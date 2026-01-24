use std::fmt::{Display, Formatter};

use serde::Serialize;
use tokio_tungstenite::tungstenite::Message;

use crate::{
    error::Result,
    usdm_futures::types::{ContractType, KlineInterval, Symbol},
};

#[derive(Debug)]
pub enum Command {
    Subscribe(Vec<Stream>),
    Unsubscribe(Vec<Stream>),
    ListSubscriptions,
    SetProperty,
    GetProperty,
}

impl Command {
    pub fn to_message(&self, id: u64) -> Result<Message> {
        #[derive(Serialize)]
        struct CommandMessage {
            method: &'static str,
            #[serde(skip_serializing_if = "Option::is_none")]
            params: Option<Vec<String>>,
            id: u64,
        }
        let (method, params) = match self {
            Command::Subscribe(s) => ("SUBSCRIBE", Some(s)),
            Command::Unsubscribe(s) => ("UNSUBSCRIBE", Some(s)),
            Command::ListSubscriptions => ("LIST_SUBSCRIPTIONS", None),
            Command::SetProperty => ("SET_PROPERTY", None),
            Command::GetProperty => ("GET_PROPERTY", None),
        };
        Ok(Message::Text(
            serde_json::to_string(&CommandMessage {
                method,
                params: params.map(|s| s.iter().map(|s| s.to_string()).collect()),
                id,
            })?
            .into(),
        ))
    }
}

/*
impl Display for ContractType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ContractType::Perpetual => write!(f, "perpetual"),
            ContractType::CurrentQuarter => write!(f, "current_quarter"),
            ContractType::NextQuarter => write!(f, "next_quarter"),
            ContractType::TradifiPerpetual => write!(f, "tradifi_perpetual"),
        }
    }
}
*/

#[derive(Debug)]
pub enum Stream {
    /// Aggregate Trade Streams
    AggregateTrade(Symbol),
    /// Mark Price Stream
    MarkPrice(Symbol),
    /// Mark Price Stream for All market
    MarkPriceAllMarket,
    /// Kline/Candlestick Streams
    Kline {
        symbol: Symbol,
        interval: KlineInterval,
    },
    /// Continuous Contract Kline/Candlestick Streams
    ContinuousContractKline {
        pair: String,
        contract_type: ContractType,
        interval: KlineInterval,
    },
    /// Individual Symbol Mini Ticker Stream
    IndividualSymbolMiniTicker { symbol: Symbol },
    /// All Market Tickers Streams
    AllMarketTickers,
    /// Individual Symbol Ticker Streams
    IndividualSymbolTicker { symbol: Symbol },
    /// All Market Mini Tickers Stream
    AllMarketMiniTickers,
}

impl Display for Stream {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Stream::AggregateTrade(s) => write!(f, "{s}@aggTrade"),
            Stream::MarkPrice(s) => write!(f, "{s}@markPrice"),
            Stream::MarkPriceAllMarket => write!(f, "!markPrice@arr"),
            Stream::Kline { symbol, interval } => write!(f, "{symbol}@kline_{interval}>"),
            Stream::ContinuousContractKline {
                pair,
                contract_type,
                interval,
            } => write!(f, "{pair}_{contract_type}@continuousKline_{interval}"),
            Stream::IndividualSymbolMiniTicker { symbol } => write!(f, "{symbol}@miniTicker"),
            Stream::AllMarketTickers => write!(f, "!ticker@arr"),
            Stream::IndividualSymbolTicker { symbol } => write!(f, "{symbol}@ticker"),
            Stream::AllMarketMiniTickers => write!(f, "!miniTicker@arr"),
        }
    }
}
