use chrono::{DateTime, Utc, serde::ts_milliseconds};
use rust_decimal::Decimal;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Response {
    Error {
        error: ErrorDetail,
        id: u64,
    },
    Result {
        result: Option<String>,
        id: u64,
    },
    Stream {
        stream: String,
        data: Vec<StreamItem>,
    },
    Single {
        stream: String,
        data: Box<StreamItem>,
    },
}

#[derive(Debug, Deserialize)]
pub struct ErrorDetail {
    pub code: u64,
    pub msg: String,
}

pub struct Stream {
    pub name: String,
    pub streams: Vec<StreamItem>,
}

impl Stream {
    pub fn new(name: &str, streams: Vec<StreamItem>) -> Stream {
        Stream {
            name: name.to_string(),
            streams,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(tag = "e")]
pub enum StreamItem {
    #[serde(rename = "aggTrade")]
    AggTrade {
        /// Event time
        #[serde(rename = "E", with = "ts_milliseconds")]
        event_time: DateTime<Utc>,
        /// Symbol
        #[serde(rename = "s")]
        symbol: String,
        /// Aggregate trade ID
        #[serde(rename = "a")]
        aggregate_trade_id: u64,
        /// Price
        #[serde(rename = "p")]
        price: Decimal,
        /// Quantity with all the market trades
        #[serde(rename = "q")]
        quantity: Decimal,
        /// Normal quantity without the trades involving RPI orders
        #[serde(rename = "nq")]
        normal_quantity: Decimal,
        /// First trade ID
        #[serde(rename = "f")]
        first_trade_id: u64,
        /// Last trade ID
        #[serde(rename = "l")]
        last_trade_id: u64,
        /// Trade time
        #[serde(rename = "T", with = "ts_milliseconds")]
        trade_time: DateTime<Utc>,
        /// Is the buyer the market maker?
        #[serde(rename = "m")]
        is_buyer_market_maker: bool,
    },
    #[serde(rename = "markPriceUpdate")]
    MarkPriceUpdate {
        /// Event time
        #[serde(rename = "E", with = "ts_milliseconds")]
        event_time: DateTime<Utc>,
        /// Symbol
        #[serde(rename = "s")]
        symbol: String,
        /// Mark price
        #[serde(rename = "p")]
        mark_price: Decimal,
        /// Index price
        #[serde(rename = "i")]
        index_price: Decimal,
        /// Estimated Settle Price, only useful in the last hour before the settlement starts
        #[serde(rename = "P")]
        estimated_settle_price: Decimal,
        /// funding rate
        #[serde(rename = "r")]
        funding_rate: Decimal,
        /// next funding time
        #[serde(rename = "T", with = "ts_milliseconds")]
        next_funding_time: DateTime<Utc>,
    },
    #[serde(rename = "continuous_kline")]
    ContinuousKline {
        /// Event time
        #[serde(rename = "E", with = "ts_milliseconds")]
        event_time: DateTime<Utc>,
        /// Pair
        #[serde(rename = "ps")]
        symbol: String,
        /// Contract type
        #[serde(rename = "ct")]
        contract_type: String,
        /// kline
        #[serde(rename = "k")]
        kline: Kline,
    },
    #[serde(rename = "24hrMiniTicker")]
    E24hrMiniTicker {
        /// Event time
        #[serde(rename = "E", with = "ts_milliseconds")]
        event_time: DateTime<Utc>,
        /// Symbol
        #[serde(rename = "s")]
        symbol: String,
        /// Open price
        #[serde(rename = "o")]
        open_price: Decimal,
        /// Close price
        #[serde(rename = "c")]
        close_price: Decimal,
        /// High price
        #[serde(rename = "h")]
        high_price: Decimal,
        /// Low price
        #[serde(rename = "l")]
        low_price: Decimal,
        /// Total traded base asset volume
        #[serde(rename = "v")]
        base_asset_volume: Decimal,
        /// Total traded quote asset volume
        #[serde(rename = "q")]
        quote_asset_volume: Decimal,
    },
    #[serde(rename = "24hrTicker")]
    E24hrTicker {
        /// Event time
        #[serde(rename = "E", with = "ts_milliseconds")]
        event_time: DateTime<Utc>,
        /// Symbol
        #[serde(rename = "s")]
        symbol: String,
        /// Price change
        #[serde(rename = "p")]
        price_change: Decimal,
        /// Price change percent
        #[serde(rename = "P")]
        price_change_percent: Decimal,
        /// Weighted average price
        #[serde(rename = "w")]
        average_price: Decimal,
        /// Last price
        #[serde(rename = "c")]
        last_price: Decimal,
        /// Last quantity
        #[serde(rename = "Q")]
        last_quantity: Decimal,
        /// Open price
        #[serde(rename = "o")]
        open_price: Decimal,
        /// High price
        #[serde(rename = "h")]
        high_price: Decimal,
        /// Low price
        #[serde(rename = "l")]
        low_price: Decimal,
        /// Total traded base asset volume
        #[serde(rename = "v")]
        base_asset_volume: Decimal,
        /// Total traded quote asset volume
        #[serde(rename = "q")]
        quote_asset_volume: Decimal,
        /// Statistics open time
        #[serde(rename = "O", with = "ts_milliseconds")]
        statistics_open_time: DateTime<Utc>,
        /// Statistics close time
        #[serde(rename = "C", with = "ts_milliseconds")]
        statistics_close_time: DateTime<Utc>,
        /// First trade ID
        #[serde(rename = "F")]
        first_trade_id: u64,
        /// Last trade Id
        #[serde(rename = "L")]
        last_trade_id: u64,
        /// Total number of trades
        #[serde(rename = "n")]
        total_number_of_trades: u64,
    },
}

#[derive(Debug, Deserialize)]
pub struct Kline {
    /// Kline start time
    #[serde(rename = "t", with = "ts_milliseconds")]
    pub start_time: DateTime<Utc>,
    /// Kline close time
    #[serde(rename = "T", with = "ts_milliseconds")]
    pub close_time: DateTime<Utc>,
    /// Interval
    #[serde(rename = "i")]
    pub internal: String,
    /// First updateId
    #[serde(rename = "f")]
    pub first_update_id: u64,
    /// Last updateId
    #[serde(rename = "L")]
    pub last_update_id: u64,
    /// Open price
    #[serde(rename = "o")]
    pub open_price: Decimal,
    /// Close price
    #[serde(rename = "c")]
    pub close_price: Decimal,
    /// High price
    #[serde(rename = "h")]
    pub high_price: Decimal,
    /// Low price
    #[serde(rename = "l")]
    pub low_price: Decimal,
    /// volume
    #[serde(rename = "v")]
    pub volume: Decimal,
    /// Number of trades
    #[serde(rename = "n")]
    pub number_of_trades: u64,
    /// Is this kline closed?
    #[serde(rename = "x")]
    pub is_closed: bool,
    /// Quote asset volume
    #[serde(rename = "q")]
    pub quote_asset_volume: Decimal,
    /// Taker buy volume
    #[serde(rename = "V")]
    pub taker_buy_volume: Decimal,
    /// Taker buy quote asset volume
    #[serde(rename = "Q")]
    pub taker_buy_quote_asset_volume: Decimal,
}
