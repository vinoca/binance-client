use std::{
    fmt,
    time::{Duration, SystemTime},
};

use hmac::{Hmac, Mac};
use reqwest::{Method, Proxy};
use serde::{Serialize, de::DeserializeOwned};
use sha2::Sha256;
use tracing::debug;

use crate::{
    error::{Error, Result},
    usdm_futures::types::{request, response},
};

pub mod extend;

pub struct Client {
    auth: Option<Auth>,
    client: reqwest::Client,
}

struct Auth {
    key: String,
    secret: Option<String>,
}

enum ApiVersion {
    V1,
    V2,
    V3,
    Data,
}

struct Endpoint {
    version: ApiVersion,
    endpoint: String,
}

impl Endpoint {
    fn new(version: ApiVersion, endpoint: &str) -> Self {
        Endpoint {
            version,
            endpoint: endpoint.to_string(),
        }
    }
}

impl From<&str> for Endpoint {
    fn from(endpoint: &str) -> Self {
        Endpoint::new(ApiVersion::V1, endpoint)
    }
}

impl From<(ApiVersion, &str)> for Endpoint {
    fn from((version, endpoint): (ApiVersion, &str)) -> Self {
        Endpoint::new(version, endpoint)
    }
}

impl fmt::Display for Endpoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let version_str = match self.version {
            ApiVersion::V1 => "fapi/v1",
            ApiVersion::V2 => "fapi/v2",
            ApiVersion::V3 => "fapi/v3",
            ApiVersion::Data => "futures/data",
        };
        write!(f, "{version_str}/{}", self.endpoint)
    }
}

impl Client {
    pub fn new(key: Option<&str>, secret: Option<&str>, proxy: Option<&str>) -> Result<Self> {
        let mut client_builder = reqwest::Client::builder();
        if let Some(proxy) = proxy {
            let proxy = Proxy::all(proxy)?;
            client_builder = client_builder.proxy(proxy);
        }
        let client = client_builder.build()?;

        let auth = key.map(|key| Auth {
            key: key.to_string(),
            secret: secret.map(|i| i.to_string()),
        });
        Ok(Client { auth, client })
    }

    fn url<E: Into<Endpoint>>(endpoint: E) -> String {
        let endpoint: Endpoint = endpoint.into();
        format!("https://fapi.binance.com/{endpoint}")
    }

    fn auth(&self) -> Result<&Auth> {
        self.auth
            .as_ref()
            .ok_or_else(|| Error::new("auth in client is required"))
    }

    async fn call_with_request<RESP: DeserializeOwned>(
        &self,
        request: reqwest::Request,
    ) -> Result<RESP> {
        let start_time = SystemTime::now();
        let res = self.client.execute(request).await?;
        let call_cost = fmt_duration(start_time.elapsed()?);
        if res.status().is_success() {
            let start_time = SystemTime::now();
            let s = res.text().await?;
            let read_cost = fmt_duration(start_time.elapsed()?);
            let start_time = SystemTime::now();
            let r = serde_json::from_str(&s)?;
            let serde_cost = fmt_duration(start_time.elapsed()?);
            debug!(
                "call binance api call cost {call_cost}, read cost: {read_cost}, serde cost: {serde_cost}"
            );
            Ok(r)
        } else {
            Err(Error::new(&format!(
                "binance api error, http code: {}, body: {}",
                res.status(),
                res.text().await?
            )))
        }
    }

    async fn call<E, REQ, RESP>(&self, url: E, method: Method, req: REQ) -> Result<RESP>
    where
        E: Into<Endpoint>,
        REQ: Serialize,
        RESP: DeserializeOwned,
    {
        let request = self
            .client
            .request(method, Self::url(url))
            .query(&req)
            .build()?;
        self.call_with_request(request).await
    }

    async fn call_with_key<E, REQ, RESP>(&self, url: E, method: Method, req: REQ) -> Result<RESP>
    where
        E: Into<Endpoint>,
        REQ: Serialize,
        RESP: DeserializeOwned,
    {
        let auth = self.auth()?;
        let request = self
            .client
            .request(method, Self::url(url))
            .header("X-MBX-APIKEY", &auth.key)
            .query(&req)
            .build()?;
        self.call_with_request(request).await
    }

    async fn signed_call<E, REQ, RESP>(&self, url: E, method: Method, req: REQ) -> Result<RESP>
    where
        E: Into<Endpoint>,
        REQ: Serialize,
        RESP: DeserializeOwned,
    {
        let auth = self.auth()?;
        let timestamp = chrono::Utc::now().timestamp_millis();

        let mut request = self
            .client
            .request(method, Self::url(url))
            .header("X-MBX-APIKEY", &auth.key)
            .query(&req)
            .query(&[("timestamp", timestamp.to_string())])
            .build()?;

        let secret = auth
            .secret
            .as_ref()
            .ok_or_else(|| Error::new("secret is required"))?;
        let mut mac: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes())?;
        mac.update(request.url().query().unwrap_or_default().as_bytes());
        let mac_result = mac.finalize();
        let signature = hex::encode(mac_result.into_bytes());
        request
            .url_mut()
            .query_pairs_mut()
            .append_pair("signature", &signature);

        self.call_with_request(request).await
    }
}

// market data
impl Client {
    pub async fn exchange_info(&self) -> Result<response::ExchangeInfo> {
        self.call("exchangeInfo", Method::GET, None::<()>).await
    }

    pub async fn ticker_24hr(
        &self,
        params: request::OptionalSymbol,
    ) -> Result<response::Ticker24hr> {
        self.call("ticker/24hr", Method::GET, params).await
    }

    pub async fn ticker_price(
        &self,
        params: request::OptionalSymbol,
    ) -> Result<response::TickerPrice> {
        self.call((ApiVersion::V2, "ticker/price"), Method::GET, params)
            .await
    }

    pub async fn kline_candlestick_data(
        &self,
        params: request::KlineCandlestickData,
    ) -> Result<Vec<serde_json::Value>> {
        self.call("klines", Method::GET, params).await
    }

    pub async fn open_interest_hist(
        &self,
        params: request::OpenInterestHist,
    ) -> Result<Vec<response::OpenInterestHist>> {
        self.call((ApiVersion::Data, "openInterestHist"), Method::GET, params)
            .await
    }

    pub async fn historical_trades(
        &self,
        params: request::HistoricalTrades,
    ) -> Result<Vec<response::HistoricalTrades>> {
        self.call_with_key("historicalTrades", Method::GET, params)
            .await
    }
}

// trade
impl Client {
    /// Send in a new order
    pub async fn new_order(&self, params: request::NewOrder) -> Result<response::OrderInfo> {
        self.signed_call("order", Method::POST, params).await
    }

    /// Place Multiple Orders
    pub async fn place_multiple_orders(
        &self,
        params: Vec<request::NewOrder>,
    ) -> Result<Vec<response::OrderInfo>> {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            batch_orders: serde_json::Value,
        }
        let params = Params {
            batch_orders: serde_json::to_value(params)?,
        };
        self.signed_call("batchOrders", Method::POST, params).await
    }

    /// Modify Order
    pub async fn modify_order(&self, params: request::ModifyOrder) -> Result<response::OrderInfo> {
        self.signed_call("order", Method::PUT, params).await
    }

    // TODO: Modify Multiple Orders
    // TODO: Get order modification history

    /// Cancel an active order.
    pub async fn cancel_order(&self, params: request::OrderId) -> Result<response::OrderInfo> {
        self.signed_call("order", Method::DELETE, params).await
    }

    // TODO: Cancel Multiple Orders
    // TODO: Cancel All Open Orders
    // TODO: Auto-Cancel All Open Orders

    /// Check an order's status
    pub async fn query_order(&self, params: request::OrderId) -> Result<response::OrderInfo> {
        self.signed_call("order", Method::GET, params).await
    }

    /// Get all account orders; active, canceled, or filled
    pub async fn all_orders(&self, params: request::AllOrders) -> Result<Vec<response::OrderInfo>> {
        self.signed_call("allOrders", Method::GET, params).await
    }

    /// Get all open orders on a symbol
    pub async fn current_all_open_orders(
        &self,
        params: request::OptionalSymbol,
    ) -> Result<Vec<response::OrderInfo>> {
        self.signed_call("openOrders", Method::GET, params).await
    }

    /// Query open order
    pub async fn query_current_open_order(
        &self,
        params: request::OrderId,
    ) -> Result<response::OrderInfo> {
        self.signed_call("openOrder", Method::GET, params).await
    }

    // TODO: Query user's Force Orders

    /// Get trades for a specific account and symbol
    pub async fn account_trade_list(
        &self,
        params: request::AccountTradeList,
    ) -> Result<Vec<response::AccountTradeList>> {
        self.signed_call("userTrades", Method::GET, params).await
    }

    /// Change symbol level margin type
    pub async fn change_margin_type(
        &self,
        params: request::ChangeMarginType,
    ) -> Result<response::OperationResult> {
        self.signed_call("marginType", Method::POST, params).await
    }

    /// Change user's position mode (Hedge Mode or One-way Mode ) on EVERY symbol
    pub async fn change_position_mode(
        &self,
        params: request::ChangePositionMode,
    ) -> Result<response::OperationResult> {
        self.signed_call("positionSide/dual", Method::POST, params)
            .await
    }

    /// Change user's initial leverage of specific symbol market.
    pub async fn change_initial_leverage(
        &self,
        params: request::ChangeInitialLeverage,
    ) -> Result<response::ChangeInitialLeverage> {
        self.signed_call("leverage", Method::POST, params).await
    }

    /// Change user's Multi-Assets mode (Multi-Assets Mode or Single-Asset Mode) on Every symbol
    pub async fn change_multi_assets_mode(
        &self,
        params: request::ChangeMultiAssetsMode,
    ) -> Result<response::OperationResult> {
        self.signed_call("multiAssetsMargin", Method::POST, params)
            .await
    }

    /// # Modify Isolated Position Margin
    pub async fn modify_isolated_position_margin(
        &self,
        params: request::ModifyIsolatedPositionMargin,
    ) -> Result<response::ModifyIsolatedPositionMargin> {
        self.signed_call("positionMargin", Method::POST, params)
            .await
    }

    // TODO: Position Information V2

    /// # Position Information V3
    /// Get current position information(only symbol that has position or open orders will be returned).
    pub async fn position_information_v3(
        &self,
        params: request::OptionalSymbol,
    ) -> Result<Vec<response::PositionInformationV3>> {
        self.signed_call((ApiVersion::V3, "positionRisk"), Method::GET, params)
            .await
    }

    // TODO: Position ADL Quantile Estimation
    // TODO: Get Position Margin Change History

    /// Testing order request, this order will not be submitted to matching engine
    pub async fn test_order(&self, params: request::NewOrder) -> Result<response::OrderInfo> {
        self.signed_call("order/test", Method::POST, params).await
    }
}

// account
impl Client {
    /// Futures Account Balance V3
    pub async fn futures_account_balance_v3(
        &self,
    ) -> Result<Vec<response::FuturesAccountBalanceV2>> {
        self.signed_call((ApiVersion::V3, "balance"), Method::GET, None::<()>)
            .await
    }

    /// Account Information V3
    pub async fn account_information_v3(&self) -> Result<response::AccountInformationV3> {
        self.signed_call((ApiVersion::V3, "account"), Method::GET, None::<()>)
            .await
    }

    /// Futures Account Configuration
    pub async fn futures_account_configuration(
        &self,
    ) -> Result<response::FuturesAccountConfiguration> {
        self.signed_call((ApiVersion::V1, "accountConfig"), Method::GET, None::<()>)
            .await
    }

    /// Symbol Configuration
    pub async fn symbol_configuration(
        &self,
        params: request::OptionalSymbol,
    ) -> Result<Vec<response::SymbolConfiguration>> {
        self.signed_call((ApiVersion::V1, "symbolConfig"), Method::GET, params)
            .await
    }

    /// Get Current Position Mode
    pub async fn get_current_position_mode(&self) -> Result<response::GetCurrentPositionMode> {
        self.signed_call(
            (ApiVersion::V1, "positionSide/dual"),
            Method::GET,
            None::<()>,
        )
        .await
    }
}

fn fmt_duration(d: Duration) -> String {
    if d.as_millis() == 0 {
        format!("{}us", d.as_micros())
    } else {
        let millis = d.as_millis();
        if millis >= 1000 {
            format!("{}s", millis as f64 / 1000.0)
        } else {
            format!("{}ms", millis)
        }
    }
}
