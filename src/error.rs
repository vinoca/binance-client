use std::num::ParseIntError;

use thiserror::Error;

use crate::usdm_futures::stream::response;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Message(String),

    #[error("{0}")]
    Serde(String),

    #[error(transparent)]
    StdIo(#[from] std::io::Error),

    #[error(transparent)]
    ParseInt(#[from] ParseIntError),

    #[error(transparent)]
    SystemTime(#[from] std::time::SystemTimeError),

    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    Hmac(#[from] hmac::digest::InvalidLength),

    #[error(transparent)]
    RustDecimal(#[from] rust_decimal::Error),

    #[error(transparent)]
    Tungstenite(#[from] tokio_tungstenite::tungstenite::Error),

    #[error(transparent)]
    FluentUri(#[from] fluent_uri::ParseError),

    #[error(transparent)]
    Socks5Client(#[from] socks5_client::Error),

    #[error(transparent)]
    FuturesChannel(#[from] futures_channel::mpsc::TrySendError<response::Stream>),
}

impl Error {
    pub fn new(message: &str) -> Self {
        Error::Message(message.to_string())
    }
}
