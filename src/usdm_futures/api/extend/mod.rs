use rust_decimal::Decimal;
use serde::Deserialize;

use crate::{
    error::{Error, Result},
    usdm_futures::{
        api::Client,
        types::{self, TimeInForce},
    },
};

pub struct ExtendClient<'a>(&'a Client);

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
pub enum NewOrder {
    Limit {
        symbol: String,
        side: types::OrderSide,
        quantity: Decimal,
        price: Decimal,
        time_in_force: TimeInForce,
        reduce_only: Option<bool>,
    },
    Market {
        symbol: String,
        side: types::OrderSide,
        quantity: Decimal,
        reduce_only: Option<bool>,
    },
    StopLimit {
        symbol: String,
        side: types::OrderSide,
        quantity: Decimal,
        price: Decimal,
        stop_price: Decimal,
        take_profit: bool,
        reduce_only: Option<bool>,
        price_protect: Option<bool>,
    },
    StopMarket {
        symbol: String,
        side: types::OrderSide,
        stop_price: Decimal,
        take_profit: bool,
        reduce_only: Option<bool>,
        close_position: Option<bool>,
        price_protect: Option<bool>,
    },
}

impl From<NewOrder> for types::request::NewOrder {
    fn from(value: NewOrder) -> Self {
        let new_client_order_id = ulid::Ulid::new().to_string();
        match value {
            NewOrder::Limit {
                symbol,
                side,
                quantity,
                price,
                time_in_force,
                reduce_only,
            } => types::request::NewOrder {
                symbol,
                side,
                order_type: types::OrderType::Limit,
                time_in_force: Some(time_in_force),
                quantity: Some(quantity),
                price: Some(price),
                new_client_order_id: Some(new_client_order_id),
                reduce_only,
                ..Default::default()
            },
            NewOrder::Market {
                symbol,
                side,
                quantity,
                reduce_only,
            } => types::request::NewOrder {
                symbol,
                side,
                order_type: types::OrderType::Market,
                quantity: Some(quantity),
                new_client_order_id: Some(new_client_order_id),
                reduce_only,
                ..Default::default()
            },
            NewOrder::StopLimit {
                symbol,
                side,
                quantity,
                price,
                stop_price,
                take_profit,
                reduce_only,
                price_protect,
            } => {
                let order_type = if take_profit {
                    types::OrderType::TakeProfit
                } else {
                    types::OrderType::Stop
                };

                types::request::NewOrder {
                    symbol,
                    side,
                    order_type,
                    quantity: Some(quantity),
                    price: Some(price),
                    stop_price: Some(stop_price),
                    new_client_order_id: Some(new_client_order_id),
                    reduce_only,
                    price_protect,
                    ..Default::default()
                }
            }
            NewOrder::StopMarket {
                symbol,
                side,
                stop_price,
                take_profit,
                mut reduce_only,
                close_position,
                price_protect,
            } => {
                let order_type = if take_profit {
                    types::OrderType::TakeProfitMarket
                } else {
                    types::OrderType::StopMarket
                };

                if close_position == Some(true) && reduce_only == Some(true) {
                    reduce_only = None;
                }

                types::request::NewOrder {
                    symbol,
                    side,
                    order_type,
                    stop_price: Some(stop_price),
                    new_client_order_id: Some(new_client_order_id),
                    reduce_only,
                    close_position,
                    price_protect,
                    ..Default::default()
                }
            }
        }
    }
}

impl<'a> ExtendClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self(client)
    }

    pub async fn kline_candlestick_data(
        &self,
        params: types::request::KlineCandlestickData,
    ) -> Result<Vec<types::response::KlineCandlestickData>> {
        let v = self.0.kline_candlestick_data(params).await?;
        let mut result = Vec::new();
        for item in v {
            let item = item.try_into()?;
            result.push(item);
        }
        Ok(result)
    }

    pub async fn symbol_ticker_price(&self, symbol: &str) -> Result<Decimal> {
        let params = types::request::OptionalSymbol {
            symbol: Some(symbol.to_string()),
        };
        let price = self.0.ticker_price(params).await?;
        let price = match price {
            types::response::TickerPrice::One(v) => v.price,
            types::response::TickerPrice::Many(v) => {
                if v.is_empty() {
                    return Err(Error::new("invalid ticker price response"));
                } else {
                    v[0].price
                }
            }
        };
        Ok(price)
    }

    pub async fn new_order(&self, params: NewOrder) -> Result<types::response::OrderInfo> {
        self.0.new_order(params.into()).await
    }
}
