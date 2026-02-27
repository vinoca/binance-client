use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::{
    KlineInterval, MarginType, NewOrderRespType, OrderSide, OrderType, PositionSide, PriceMatch,
    SelfTradePreventionMode, TimeInForce, WorkingType,
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct OptionalSymbol {
    pub symbol: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderId {
    symbol: String,
    #[serde(flatten)]
    order_id: OrderIdInner,
}

impl OrderId {
    pub fn new_bn(symbol: &str, bn_id: i64) -> Self {
        Self {
            symbol: symbol.to_string(),
            order_id: OrderIdInner::OrderId(bn_id),
        }
    }

    pub fn new_client(symbol: &str, client_id: &str) -> Self {
        Self {
            symbol: symbol.to_string(),
            order_id: OrderIdInner::OrigClientOrderId(client_id.to_string()),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
enum OrderIdInner {
    OrderId(i64),
    OrigClientOrderId(String),
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AlgoOrderId {
    symbol: String,
    #[serde(flatten)]
    order_id: AlgoOrderIdInner,
}

impl From<OrderId> for AlgoOrderId {
    fn from(value: OrderId) -> Self {
        let order_id = match value.order_id {
            OrderIdInner::OrderId(v) => AlgoOrderIdInner::AlgoId(v),
            OrderIdInner::OrigClientOrderId(v) => AlgoOrderIdInner::ClientAlgoId(v),
        };
        AlgoOrderId {
            symbol: value.symbol,
            order_id,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum AlgoOrderIdInner {
    AlgoId(i64),
    ClientAlgoId(String),
}

impl From<(&str, i64)> for AlgoOrderId {
    fn from((symbol, id): (&str, i64)) -> Self {
        AlgoOrderId {
            symbol: symbol.to_string(),
            order_id: AlgoOrderIdInner::AlgoId(id),
        }
    }
}

impl From<(&str, &str)> for AlgoOrderId {
    fn from((symbol, id): (&str, &str)) -> Self {
        AlgoOrderId {
            symbol: symbol.to_string(),
            order_id: AlgoOrderIdInner::ClientAlgoId(id.to_string()),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AllOrders {
    pub symbol: String,
    pub order_id: Option<u64>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub limit: Option<u64>,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewOrder {
    pub symbol: String,
    pub side: OrderSide,
    #[serde(rename = "type")]
    pub order_type: OrderType,
    pub position_side: Option<PositionSide>,
    pub time_in_force: Option<TimeInForce>,
    pub quantity: Option<Decimal>,
    pub reduce_only: Option<bool>,
    pub price: Option<Decimal>,
    /// A unique id among open orders. Automatically generated if not sent. Can only be string following the rule: `^[\.A-Z\:/a-z0-9_-]{1,36}$`
    pub new_client_order_id: Option<String>,
    /// Used with `STOP/STOP_MARKET` or `TAKE_PROFIT/TAKE_PROFIT_MARKET` orders.
    pub stop_price: Option<Decimal>,
    /// Close-All，used with `STOP_MARKET` or `TAKE_PROFIT_MARKET`.
    pub close_position: Option<bool>,
    /// Used with `TRAILING_STOP_MARKET` orders, default as the latest price(supporting different `workingType`)
    pub activation_price: Option<Decimal>,
    /// Used with `TRAILING_STOP_MARKET` orders, min 0.1, max 10 where 1 for 1%
    pub callback_rate: Option<Decimal>,
    pub working_type: Option<WorkingType>,
    /// "TRUE" or "FALSE", default "FALSE". Used with `STOP/STOP_MARKET` or `TAKE_PROFIT/TAKE_PROFIT_MARKET` orders.
    pub price_protect: Option<bool>,
    pub new_order_resp_type: Option<NewOrderRespType>,
    pub price_match: Option<PriceMatch>,
    pub self_trade_prevention_mode: Option<SelfTradePreventionMode>,
    /// order cancel time for timeInForce `GTD`, mandatory when `timeInforce` set to `GTD`; order the timestamp only retains second-level precision, ms part will be ignored; The goodTillDate timestamp must be greater than the current time plus 600 seconds and smaller than 253402300799000
    pub good_till_date: Option<i64>,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewAlgoOrder {
    /// Only support CONDITIONAL
    pub algo_type: String,
    pub symbol: String,
    pub side: OrderSide,
    pub position_side: Option<PositionSide>,
    /// For CONDITIONAL algoType, STOP_MARKET/TAKE_PROFIT_MARKET/STOP/TAKE_PROFIT/TRAILING_STOP_MARKET as order type
    #[serde(rename = "type")]
    pub order_type: OrderType,
    pub time_in_force: Option<TimeInForce>,
    pub quantity: Option<Decimal>,
    pub price: Option<Decimal>,
    pub trigger_price: Option<Decimal>,
    pub working_type: Option<WorkingType>,
    pub price_match: Option<PriceMatch>,
    pub close_position: Option<bool>,
    /// "TRUE" or "FALSE", default "FALSE". Used with `STOP/STOP_MARKET` or `TAKE_PROFIT/TAKE_PROFIT_MARKET` orders.
    pub price_protect: Option<bool>,
    pub reduce_only: Option<bool>,
    /// Used with `TRAILING_STOP_MARKET` orders, default as the latest price(supporting different `workingType`)
    pub activation_price: Option<Decimal>,
    /// Used with `TRAILING_STOP_MARKET` orders, min 0.1, max 10 where 1 for 1%
    pub callback_rate: Option<Decimal>,
    /// A unique id among open orders. Automatically generated if not sent. Can only be string following the rule: ^[\.A-Z\:/a-z0-9_-]{1,36}$
    pub client_algo_id: Option<String>,
    pub new_order_resp_type: Option<NewOrderRespType>,
    pub self_trade_prevention_mode: Option<SelfTradePreventionMode>,
    /// order cancel time for timeInForce GTD, mandatory when timeInforce set to GTD; order the timestamp only retains second-level precision, ms part will be ignored; The goodTillDate timestamp must be greater than the current time plus 600 seconds and smaller than 253402300799000
    pub good_till_date: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountTradeList {
    pub symbol: String,
    pub order_id: Option<u64>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub from_id: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeMarginType {
    pub symbol: String,
    pub margin_type: MarginType,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeInitialLeverage {
    pub symbol: String,
    pub leverage: u8,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeMultiAssetsMode {
    pub multi_assets_margin: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangePositionMode {
    pub dual_side_position: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModifyIsolatedPositionMargin {
    pub symbol: String,
    pub amount: Decimal,
    // 1: Add position margin，2: Reduce position margin
    #[serde(rename = "type")]
    pub _type: u8,
    pub position_side: Option<PositionSide>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModifyOrder {
    #[serde(flatten)]
    pub order_id: OrderId,
    pub side: OrderSide,
    pub quantity: Decimal,
    pub price: Decimal,
    pub price_match: Option<PriceMatch>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KlineCandlestickData {
    pub symbol: String,
    pub interval: KlineInterval,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenInterestHist {
    pub symbol: String,
    pub period: KlineInterval,
    pub limit: Option<i64>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalTrades {
    pub symbol: String,
    pub limit: Option<i64>,
    pub from_id: Option<i64>,
}
