use fluent_uri::Uri;
use futures_channel::mpsc::UnboundedSender;
use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{client_async_tls, connect_async, tungstenite::Message};
use tokio_util::compat::TokioAsyncReadCompatExt;
use tracing::{error, info};

use crate::error::{Error, Result};

pub mod request;
pub mod response;

const URL: &str = "wss://fstream.binance.com/stream";

struct Socks5Proxy<'a> {
    server: &'a str,
    port: Option<u16>,
    auth: Option<(&'a str, &'a str)>,
}

impl<'a> Socks5Proxy<'a> {
    async fn connect_to(&self, dest_addr: (&[u8], u16)) -> Result<TcpStream> {
        let mut connect = tokio::net::TcpStream::connect((self.server, self.port.unwrap_or(1080)))
            .await?
            .compat();
        socks5_client::connect(&mut connect, dest_addr.into(), self.auth.map(Into::into)).await?;
        Ok(connect.into_inner())
    }
}

impl<'a> TryFrom<Uri<&'a str>> for Socks5Proxy<'a> {
    type Error = Error;

    fn try_from(v: Uri<&'a str>) -> Result<Self> {
        if v.scheme().as_str() != "socks5h" {
            return Err(Error::new("invalid proxy scheme"));
        }

        let authority = v
            .authority()
            .ok_or_else(|| Error::new("invalid proxy url"))?;
        let auth = match authority.userinfo() {
            Some(u) => u.split_once(':').map(|(u, p)| (u.as_str(), p.as_str())),
            None => None,
        };
        Ok(Socks5Proxy {
            server: authority.host(),
            port: authority.port_to_u16()?,
            auth,
        })
    }
}

pub async fn receive(
    streams: Vec<request::Stream>,
    tx: UnboundedSender<response::Stream>,
    proxy: Option<&str>,
) -> Result<()> {
    let (mut stream, _) = match proxy {
        Some(proxy) => {
            let uri = Uri::parse(proxy)?;
            let socks5_info: Socks5Proxy = uri.try_into()?;
            let proxy = socks5_info
                .connect_to(("fstream.binance.com".as_bytes(), 443))
                .await?;
            client_async_tls(URL, proxy).await?
        }
        None => connect_async(URL).await?,
    };
    stream
        .send(request::Command::Subscribe(streams).to_message(0)?)
        .await?;
    while let Some(msg) = stream.next().await {
        match msg? {
            Message::Text(msg) => match serde_json::from_str(&msg)? {
                response::Response::Error { error, id } => {
                    error!(
                        "response error: id: {id}, code: {}, message: {}",
                        error.code, error.msg
                    );
                    break;
                }
                response::Response::Result { result, id } => {
                    info!("result: {result:?}, id: {id}");
                }
                response::Response::Stream { stream, data } => {
                    tx.unbounded_send(response::Stream::new(&stream, data))?
                }
                response::Response::Single { stream, data } => {
                    tx.unbounded_send(response::Stream::new(&stream, vec![*data]))?
                }
            },
            Message::Ping(payload) => stream.send(Message::Pong(payload)).await?,
            x => error!("invalid message from server: {x:?}"),
        }
    }
    Ok(())
}
