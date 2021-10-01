use super::super::utils::http_get;
use crate::error::Result;
use std::collections::BTreeMap;

const BASE_URL: &str = "https://api.gateio.ws/api/v4";

/// The RESTful client for Gate Future markets.
///
/// * RESTful API doc: <https://www.gate.io/docs/apiv4/en/index.html#delivery>
/// * Trading at: <https://www.gateio.pro/cn/futures-delivery/usdt>
pub struct GateFutureRestClient {
    _api_key: Option<String>,
    _api_secret: Option<String>,
}

impl GateFutureRestClient {
    pub fn new(api_key: Option<String>, api_secret: Option<String>) -> Self {
        GateFutureRestClient {
            _api_key: api_key,
            _api_secret: api_secret,
        }
    }

    /// Get the latest Level2 snapshot of orderbook.
    ///
    /// Top 50 asks and bids are returned.
    ///
    /// For example:
    ///
    /// - <https://api.gateio.ws/api/v4/delivery/usdt/order_book?contract=BTC_USDT_20211015&limit=50>
    pub fn fetch_l2_snapshot(symbol: &str) -> Result<String> {
        let without_date = &symbol[..(symbol.len() - 8)];
        let settle = if without_date.ends_with("_USD_") {
            "btc"
        } else if without_date.ends_with("_USDT_") {
            "usdt"
        } else {
            panic!("Unknown symbol {}", symbol);
        };
        gen_api!(format!(
            "/delivery/{}/order_book?contract={}&limit=50",
            settle, symbol
        ))
    }
}
