use serde::Serialize;
use tokio_tungstenite::tungstenite::Message;

use crate::error::Result;

#[derive(Debug, Serialize)]
pub struct Command {
    method: CommandMethod,
    params: Vec<String>,
    id: u64,
}

impl Command {
    pub fn new(method: CommandMethod, params: &[CommandParam], id: u64) -> Command {
        Command {
            method,
            params: params.iter().map(|i| i.serialize()).collect(),
            id,
        }
    }

    pub fn to_message(&self) -> Result<Message> {
        Ok(Message::Text(serde_json::to_string(&self)?.into()))
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CommandMethod {
    Subscribe,
    // Unsubscribe,
    // ListSubscriptions,
    // SetProperty,
    // GetProperty,
}

pub struct CommandParam {
    kind: CommandParamKind,
    interval: u64,
}

impl CommandParam {
    pub fn new(kind: CommandParamKind, interval: u64) -> Self {
        CommandParam { kind, interval }
    }

    fn serialize(&self) -> String {
        format!("{}@{}ms", self.kind.serialize(), self.interval)
    }
}

pub enum CommandParamKind {
    MiniTicker(String),
    Ticker {
        symbol: String,
        /// 1h,4h,1d
        period: String,
    },
}

impl CommandParamKind {
    fn serialize(&self) -> String {
        fn symbol_join(symbol: &str, stream_name: &str) -> String {
            if symbol == "!" {
                format!("!{stream_name}@arr")
            } else {
                format!("{symbol}@{stream_name}")
            }
        }
        match self {
            CommandParamKind::MiniTicker(symbol) => symbol_join(symbol, "miniTicker"),
            CommandParamKind::Ticker { symbol, period } => {
                symbol_join(symbol, &format!("ticker_{period}"))
            }
        }
    }
}
