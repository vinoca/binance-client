use std::{fmt, str::FromStr};

use chrono::{DateTime, Datelike, Duration, TimeZone, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};

pub mod request;
pub mod response;

macro_rules! impl_enum_str {
    ($t:ty) => {
        impl fmt::Display for $t {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let s = serde_json::to_string(self).unwrap_or_default();
                let s = s.trim_matches('"');
                write!(f, "{s}")
            }
        }

        impl FromStr for $t {
            type Err = Error;

            fn from_str(s: &str) -> Result<Self> {
                let s = format!("\"{s}\"");
                Ok(serde_json::from_str(&s)?)
            }
        }
    };
}

pub type Symbol = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ContractType {
    Perpetual,
    CurrentMonth,
    NextMonth,
    CurrentQuarter,
    NextQuarter,
    PerpetualDelivering,
    #[serde(rename = "CURRENT_QUARTER DELIVERING")]
    CurrentQuarterDelivering,
    TradifiPerpetual,
}
impl_enum_str!(ContractType);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ContractStatus {
    PendingTrading,
    Trading,
    PreDelivering,
    Delivering,
    Delivered,
    PreSettle,
    Settling,
    Close,
}
impl_enum_str!(ContractStatus);

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderStatus {
    New,
    PartiallyFilled,
    Filled,
    Canceled,
    Rejected,
    Expired,
    ExpiredInMatch,
}
impl_enum_str!(OrderStatus);

impl OrderStatus {
    pub fn is_open(&self) -> bool {
        matches!(self, OrderStatus::New | OrderStatus::PartiallyFilled)
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderType {
    Limit,
    #[default]
    Market,
    Stop,
    StopMarket,
    TakeProfit,
    TakeProfitMarket,
    TrailingStopMarket,
}
impl_enum_str!(OrderType);

#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderSide {
    #[default]
    Buy,
    Sell,
}
impl_enum_str!(OrderSide);

impl OrderSide {
    pub fn is_short(&self) -> bool {
        matches!(self, OrderSide::Sell)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PositionSide {
    Both,
    Long,
    Short,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TimeInForce {
    #[default]
    Gtc,
    Ioc,
    Fok,
    Gtx,
    Gtd,
}
impl_enum_str!(TimeInForce);

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WorkingType {
    MarkPrice,
    ContractPrice,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NewOrderRespType {
    Ack,
    Result,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
pub enum KlineInterval {
    #[serde(rename = "1m")]
    I1m,
    #[serde(rename = "3m")]
    I3m,
    #[serde(rename = "5m")]
    I5m,
    #[serde(rename = "15m")]
    I15m,
    #[serde(rename = "30m")]
    I30m,
    #[serde(rename = "1h")]
    I1h,
    #[serde(rename = "2h")]
    I2h,
    #[serde(rename = "4h")]
    I4h,
    #[serde(rename = "6h")]
    I6h,
    #[serde(rename = "8h")]
    I8h,
    #[serde(rename = "12h")]
    I12h,
    #[serde(rename = "1d")]
    I1d,
    #[serde(rename = "3d")]
    I3d,
    #[serde(rename = "1w")]
    I1w,
    #[serde(rename = "1M")]
    I1M,
}
impl_enum_str!(KlineInterval);

impl KlineInterval {
    pub fn get_start_time(&self, time: DateTime<Utc>) -> DateTime<Utc> {
        fn truncate_seconds(time: DateTime<Utc>, seconds: i64) -> DateTime<Utc> {
            let ts = time.timestamp();
            let truncated_ts = (ts / seconds) * seconds;
            Utc.timestamp_opt(truncated_ts, 0).unwrap()
        }

        match self {
            KlineInterval::I1m => truncate_seconds(time, 60),
            KlineInterval::I3m => truncate_seconds(time, 3 * 60),
            KlineInterval::I5m => truncate_seconds(time, 5 * 60),
            KlineInterval::I15m => truncate_seconds(time, 15 * 60),
            KlineInterval::I30m => truncate_seconds(time, 30 * 60),
            KlineInterval::I1h => truncate_seconds(time, 3600),
            KlineInterval::I2h => truncate_seconds(time, 2 * 3600),
            KlineInterval::I4h => truncate_seconds(time, 4 * 3600),
            KlineInterval::I6h => truncate_seconds(time, 6 * 3600),
            KlineInterval::I8h => truncate_seconds(time, 8 * 3600),
            KlineInterval::I12h => truncate_seconds(time, 12 * 3600),
            KlineInterval::I1d => Utc
                .with_ymd_and_hms(time.year(), time.month(), time.day(), 0, 0, 0)
                .unwrap(),
            KlineInterval::I3d => truncate_seconds(time, 3 * 24 * 3600),

            KlineInterval::I1w => {
                let days_from_monday = time.weekday().num_days_from_monday();
                let date = time.date_naive() - chrono::Duration::days(days_from_monday as i64);
                date.and_hms_opt(0, 0, 0).unwrap().and_utc()
            }

            KlineInterval::I1M => Utc
                .with_ymd_and_hms(time.year(), time.month(), 1, 0, 0, 0)
                .unwrap(),
        }
    }
}

impl From<KlineInterval> for Duration {
    fn from(value: KlineInterval) -> Self {
        match value {
            KlineInterval::I1m => Duration::minutes(1),
            KlineInterval::I3m => Duration::minutes(3),
            KlineInterval::I5m => Duration::minutes(5),
            KlineInterval::I15m => Duration::minutes(15),
            KlineInterval::I30m => Duration::minutes(30),
            KlineInterval::I1h => Duration::hours(1),
            KlineInterval::I2h => Duration::hours(2),
            KlineInterval::I4h => Duration::hours(4),
            KlineInterval::I6h => Duration::hours(6),
            KlineInterval::I8h => Duration::hours(8),
            KlineInterval::I12h => Duration::hours(12),
            KlineInterval::I1d => Duration::days(1),
            KlineInterval::I3d => Duration::days(3),
            KlineInterval::I1w => Duration::weeks(1),
            KlineInterval::I1M => Duration::days(30),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SelfTradePreventionMode {
    ExpireTaker,
    ExpireBoth,
    ExpireMaker,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MarginType {
    Isolated,
    Crossed,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PriceMatch {
    None,
    Opponent,
    Opponent5,
    Opponent10,
    Opponent20,
    Queue,
    Queue5,
    Queue10,
    Queue20,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "filterType", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SymbolFilter {
    PriceFilter(PriceFilter),
    LotSize(LotSize),
    MarketLotSize(LotSize),
    MaxNumOrders(MaxNumOrders),
    MaxNumAlgoOrders(MaxNumOrders),
    PercentPrice(PercentPrice),
    MinNotional(MinNotional),
    PositionRiskControl(PositionRiskControl),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceFilter {
    pub max_price: Decimal,
    pub min_price: Decimal,
    pub tick_size: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LotSize {
    pub max_qty: Decimal,
    pub min_qty: Decimal,
    pub step_size: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaxNumOrders {
    pub limit: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PercentPrice {
    pub multiplier_up: Decimal,
    pub multiplier_down: Decimal,
    pub multiplier_decimal: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MinNotional {
    pub notional: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionRiskControl {
    pub position_control_side: String,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "rateLimitType", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RateLimit {
    RequestWeight(RateLimitDetail),
    Orders(RateLimitDetail),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RateLimitDetail {
    pub interval: RateLimitInterval,
    pub interval_num: u64,
    pub limit: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RateLimitInterval {
    Minute,
    Second,
}
