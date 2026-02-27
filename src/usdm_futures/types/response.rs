use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::{
    ContractStatus, ContractType, MarginType, OrderSide, OrderStatus, OrderType, PositionSide,
    PriceMatch, RateLimit, SelfTradePreventionMode, SymbolFilter, TimeInForce, WorkingType,
};
use crate::error::Error;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Ticker24hr {
    One(Box<Ticker24hrItem>),
    Many(Vec<Ticker24hrItem>),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Ticker24hrItem {
    pub symbol: String,
    pub price_change: Decimal,
    pub price_change_percent: Decimal,
    pub weighted_avg_price: Decimal,
    pub last_price: Decimal,
    pub last_qty: Decimal,
    pub open_price: Decimal,
    pub high_price: Decimal,
    pub low_price: Decimal,
    pub volume: Decimal,
    pub quote_volume: Decimal,
    pub open_time: i64,
    pub close_time: i64,
    pub first_id: i64,
    pub last_id: i64,
    pub count: i64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum TickerPrice {
    One(Box<TickerPriceItem>),
    Many(Vec<TickerPriceItem>),
}

impl From<TickerPrice> for Vec<TickerPriceItem> {
    fn from(value: TickerPrice) -> Self {
        match value {
            TickerPrice::One(v) => vec![*v],
            TickerPrice::Many(v) => v,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TickerPriceItem {
    pub symbol: String,
    pub price: Decimal,
    pub time: i64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KlineCandlestickData {
    pub open: Decimal,
    pub high: Decimal,
    pub low: Decimal,
    pub close: Decimal,
    pub volume: Decimal,
    pub quote_asset_volume: Decimal,
    pub number_of_trades: i64,
    pub taker_buy_base_asset_volume: Decimal,
    pub taker_buy_quote_asset_volume: Decimal,
    pub open_time: i64,
    pub close_time: i64,
}

impl TryFrom<serde_json::Value> for KlineCandlestickData {
    type Error = Error;

    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        let v = value.as_array().ok_or_else(|| {
            Error::Serde("can not deserialize kline data, return value must be a array".to_string())
        })?;
        if v.len() < 11 {
            return Err(Error::Serde(
                "can not deserialize kline data, the array length of return value must be greater than or equal 11".to_string(),
            ));
        }

        fn parse_i64(v: &serde_json::Value) -> Result<i64, Error> {
            v.as_number()
                .and_then(|i| i.as_i64())
                .ok_or_else(|| Error::Serde("expect a json number".to_string()))
        }

        fn parse_decimal(v: &serde_json::Value) -> Result<Decimal, Error> {
            let s = v
                .as_str()
                .ok_or_else(|| Error::Serde("expect a json str".to_string()))?;
            Ok(s.parse()?)
        }

        Ok(KlineCandlestickData {
            open_time: parse_i64(&v[0])?,
            open: parse_decimal(&v[1])?,
            high: parse_decimal(&v[2])?,
            low: parse_decimal(&v[3])?,
            close: parse_decimal(&v[4])?,
            volume: parse_decimal(&v[5])?,
            close_time: parse_i64(&v[6])?,
            quote_asset_volume: parse_decimal(&v[7])?,
            number_of_trades: parse_i64(&v[8])?,
            taker_buy_base_asset_volume: parse_decimal(&v[9])?,
            taker_buy_quote_asset_volume: parse_decimal(&v[10])?,
        })
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeInfo {
    pub exchange_filters: Vec<String>,
    pub rate_limits: Vec<RateLimit>,
    pub server_time: i64,
    pub assets: Vec<ExchangeInfoAsset>,
    pub symbols: Vec<ExchangeInfoSymbol>,
    pub timezone: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeInfoAsset {
    pub asset: String,
    pub margin_available: bool,
    pub auto_asset_exchange: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeInfoSymbol {
    pub symbol: String,
    pub pair: String,
    pub contract_type: ContractType,
    pub delivery_date: i64,
    pub onboard_date: i64,
    pub status: ContractStatus,
    pub maint_margin_percent: Decimal,
    pub required_margin_percent: Decimal,
    pub base_asset: String,
    pub quote_asset: String,
    pub margin_asset: String,
    pub price_precision: i64,
    pub quantity_precision: i64,
    pub base_asset_precision: i64,
    pub quote_precision: i64,
    pub underlying_type: String,
    pub underlying_sub_type: Vec<String>,
    pub settle_plan: Option<i64>,
    pub trigger_protect: Decimal,
    pub filters: Vec<SymbolFilter>,
    pub order_types: Vec<OrderType>,
    pub time_in_force: Vec<TimeInForce>,
    pub liquidation_fee: Decimal,
    pub market_take_bound: Decimal,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenInterestHist {
    pub symbol: String,
    pub sum_open_interest: Decimal,
    pub sum_open_interest_value: Decimal,
    #[serde(rename = "CMCCirculatingSupply")]
    pub cmccirculating_supply: Decimal,
    pub timestamp: Decimal,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalTrades {
    pub id: i64,
    pub price: Decimal,
    pub qty: Decimal,
    pub quote_qty: Decimal,
    pub time: i64,
    pub is_buyer_maker: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderInfo {
    pub avg_price: Decimal,
    pub client_order_id: String,
    pub cum_quote: Decimal,
    pub executed_qty: Decimal,
    pub order_id: u64,
    pub orig_qty: Decimal,
    pub orig_type: OrderType,
    pub price: Decimal,
    pub reduce_only: bool,
    pub side: OrderSide,
    pub position_side: PositionSide,
    pub status: OrderStatus,
    pub stop_price: Decimal,
    pub close_position: bool,
    pub symbol: String,
    pub time: Option<i64>,
    pub time_in_force: TimeInForce,
    #[serde(rename = "type")]
    pub order_type: OrderType,
    pub activate_price: Option<Decimal>,
    pub price_rate: Option<Decimal>,
    pub update_time: i64,
    pub working_type: WorkingType,
    pub price_protect: bool,
    pub price_match: PriceMatch,
    pub self_trade_prevention_mode: SelfTradePreventionMode,
    pub good_till_date: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlgoOrderInfo {
    pub algo_id: u64,
    pub client_algo_id: String,
    pub algo_type: String,
    pub order_type: OrderType,
    pub symbol: String,
    pub side: OrderSide,
    pub position_side: PositionSide,
    pub time_in_force: TimeInForce,
    pub quantity: Decimal,
    pub algo_status: OrderStatus,
    pub trigger_price: Decimal,
    pub price: Decimal,
    pub iceberg_quantity: Option<Decimal>,
    pub self_trade_prevention_mode: SelfTradePreventionMode,
    pub working_type: WorkingType,
    pub price_match: PriceMatch,
    pub close_position: bool,
    pub price_protect: bool,
    pub reduce_only: bool,
    pub activate_price: Option<Decimal>,
    pub callback_rate: Option<String>,
    pub create_time: i64,
    pub update_time: i64,
    pub trigger_time: i64,
    pub good_till_date: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelAlgoOrder {
    pub algo_id: u64,
    pub client_algo_id: String,
    pub code: String,
    pub msg: String,
}

#[derive(Debug, Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountTradeList {
    pub buyer: bool,
    pub commission: Decimal,
    pub commission_asset: String,
    pub id: i64,
    pub maker: bool,
    pub order_id: i64,
    pub price: Decimal,
    pub qty: Decimal,
    pub quote_qty: Decimal,
    pub realized_pnl: Decimal,
    pub side: OrderSide,
    pub position_side: PositionSide,
    pub symbol: String,
    pub time: i64,
}

#[derive(Debug, Deserialize)]
pub struct OperationResult {
    pub code: Option<i64>,
    pub msg: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeInitialLeverage {
    pub leverage: Option<i64>,
    pub max_notional_value: Option<String>,
    pub symbol: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModifyIsolatedPositionMargin {
    #[serde(flatten)]
    pub operation_result: OperationResult,
    pub amount: Decimal,
    #[serde(rename = "type")]
    pub _type: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionInformationV3 {
    pub symbol: String,
    pub position_side: PositionSide,
    pub position_amt: Decimal,
    pub entry_price: Decimal,
    pub break_even_price: Decimal,
    pub mark_price: Decimal,
    pub un_realized_profit: Decimal,
    pub liquidation_price: Decimal,
    pub isolated_margin: Decimal,
    pub notional: Decimal,
    pub margin_asset: String,
    pub isolated_wallet: Decimal,
    pub initial_margin: Decimal,
    pub maint_margin: Decimal,
    pub position_initial_margin: Decimal,
    pub open_order_initial_margin: Decimal,
    pub adl: i64,
    pub bid_notional: Decimal,
    pub ask_notional: Decimal,
    pub update_time: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FuturesAccountBalanceV2 {
    pub account_alias: String,
    pub asset: String,
    pub balance: Decimal,
    pub cross_wallet_balance: Decimal,
    pub cross_un_pnl: Decimal,
    pub available_balance: Decimal,
    pub max_withdraw_amount: Decimal,
    pub margin_available: bool,
    pub update_time: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountInformationV3 {
    pub total_initial_margin: Decimal,
    pub total_maint_margin: Decimal,
    pub total_wallet_balance: Decimal,
    pub total_unrealized_profit: Decimal,
    pub total_margin_balance: Decimal,
    pub total_position_initial_margin: Decimal,
    pub total_open_order_initial_margin: Decimal,
    pub total_cross_wallet_balance: Decimal,
    pub total_cross_un_pnl: Decimal,
    pub available_balance: Decimal,
    pub max_withdraw_amount: Decimal,
    pub assets: Vec<AccountInformationV3Asset>,
    pub positions: Vec<AccountInformationV3Position>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountInformationV3Asset {
    pub asset: String,
    pub wallet_balance: Decimal,
    pub unrealized_profit: Decimal,
    pub margin_balance: Decimal,
    pub maint_margin: Decimal,
    pub initial_margin: Decimal,
    pub position_initial_margin: Decimal,
    pub open_order_initial_margin: Decimal,
    pub cross_wallet_balance: Decimal,
    pub cross_un_pnl: Decimal,
    pub available_balance: Decimal,
    pub max_withdraw_amount: Decimal,
    pub update_time: i64,
    pub margin_available: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountInformationV3Position {
    pub symbol: String,
    pub position_side: PositionSide,
    pub position_amt: Decimal,
    pub unrealized_profit: Decimal,
    pub isolated_margin: Decimal,
    pub notional: Decimal,
    pub isolated_wallet: Decimal,
    pub initial_margin: Decimal,
    pub maint_margin: Decimal,
    pub update_time: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SymbolConfiguration {
    pub symbol: String,
    pub margin_type: MarginType,
    pub is_auto_add_margin: bool,
    pub leverage: u8,
    pub max_notional_value: Decimal,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCurrentPositionMode {
    pub dual_side_position: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FuturesAccountConfiguration {
    pub fee_tier: u8,
    pub can_trade: bool,
    pub can_deposit: bool,
    pub can_withdraw: bool,
    pub dual_side_position: bool,
    pub update_time: i64,
    pub multi_assets_margin: bool,
    pub trade_group_id: i64,
}
