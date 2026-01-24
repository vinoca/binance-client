use chrono::{
    serde::{ts_milliseconds, ts_milliseconds_option},
    DateTime, Utc,
};
use rust_decimal::{
    serde::{str as de_decimal, str_option as de_decimal_opt},
    Decimal,
};
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
}

#[derive(Debug, Deserialize)]
pub struct ErrorDetail {
    pub code: u64,
    pub msg: String,
}

#[derive(Debug, Deserialize)]
pub struct StreamItem {
    #[serde(rename = "e")]
    pub event_type: String,
    #[serde(rename = "E", with = "ts_milliseconds")]
    pub event_time: DateTime<Utc>,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "c", with = "de_decimal")]
    pub close_price: Decimal,
    #[serde(rename = "o", with = "de_decimal")]
    pub open_price: Decimal,
    #[serde(rename = "h", with = "de_decimal")]
    pub high_price: Decimal,
    #[serde(rename = "l", with = "de_decimal")]
    pub low_price: Decimal,
    #[serde(rename = "v", with = "de_decimal")]
    pub total_traded_base_asset_volume: Decimal,
    #[serde(rename = "q", with = "de_decimal")]
    pub total_traded_quote_asset_volume: Decimal,
    #[serde(default, rename = "p", with = "de_decimal_opt")]
    pub price_change: Option<Decimal>,
    #[serde(default, rename = "P", with = "de_decimal_opt")]
    pub price_change_percent: Option<Decimal>,
    #[serde(default, rename = "w", with = "de_decimal_opt")]
    pub weighted_average_price: Option<Decimal>,
    #[serde(default, rename = "O", with = "ts_milliseconds_option")]
    pub statistics_open_time: Option<DateTime<Utc>>,
    #[serde(default, rename = "C", with = "ts_milliseconds_option")]
    pub statistics_close_time: Option<DateTime<Utc>>,
    #[serde(rename = "F")]
    pub first_trade_id: Option<u64>,
    #[serde(rename = "L")]
    pub last_trade_id: Option<u64>,
    #[serde(rename = "n")]
    pub total_number_of_trades: Option<u64>,
}
