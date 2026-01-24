use futures_channel::mpsc::UnboundedSender;
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use tracing::{error, info};

use crate::error::Result;

mod request;
mod response;

pub use request::{Command, CommandMethod, CommandParam};
pub use response::{Response, StreamItem};

// const URL: &str = "wss://stream.binance.com/stream";
const URL: &str = "wss://data-stream.binance.vision/stream";

pub async fn receive(
    params: Vec<CommandParam>,
    tx: UnboundedSender<(String, Vec<StreamItem>)>,
) -> Result<()> {
    let (mut stream, _) = connect_async(URL).await?;
    let subscribe_msg = Command::new(CommandMethod::Subscribe, &params, 0);
    stream.send(subscribe_msg.to_message()?).await?;
    while let Some(msg) = stream.next().await {
        match msg? {
            Message::Text(msg) => match serde_json::from_str(&msg)? {
                Response::Error { error, id } => {
                    error!(
                        "error, id: {id}, code: {}, message: {}",
                        error.code, error.msg
                    );
                    break;
                }
                Response::Result { result, id } => {
                    info!("result: {result:?}, id: {id}");
                }
                Response::Stream { stream, data } => tx.unbounded_send((stream, data))?,
            },
            Message::Ping(payload) => stream.send(Message::Pong(payload)).await?,
            x => error!("invalid message from server: {x:?}"),
        }
    }
    Ok(())
}
