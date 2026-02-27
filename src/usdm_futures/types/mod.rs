use std::{fmt, str::FromStr};

use chrono::{DateTime, Duration, Utc};
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
    GteGtc,
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
    pub fn num_seconds(&self) -> i64 {
        let interval: Duration = (*self).into();
        interval.num_seconds()
    }

    pub fn get_start_time(&self, time: DateTime<Utc>) -> DateTime<Utc> {
        let seconds = self.num_seconds();
        let truncated_ts = (time.timestamp() / seconds) * seconds;
        DateTime::from_timestamp(truncated_ts, 0).unwrap_or_default()
    }

    pub fn get_previous_time(&self, time: DateTime<Utc>) -> DateTime<Utc> {
        let seconds = self.num_seconds() / 2;
        let truncated_ts = (time.timestamp() / seconds) * seconds;
        DateTime::from_timestamp(truncated_ts, 0).unwrap_or_default()
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
