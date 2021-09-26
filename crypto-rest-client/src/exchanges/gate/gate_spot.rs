use super::super::utils::http_get;
use crate::error::Result;
use std::collections::HashMap;

const BASE_URL: &str = "https://api.gateio.ws/api/v4";

/// The RESTful client for Gate spot market.
///
/// * RESTful API doc: <https://www.gate.io/docs/apiv4/en/index.html>
/// * Trading at: <https://www.gateio.pro/cn/trade/BTC_USDT>
/// * Rate Limits: <https://www.gate.io/docs/apiv4/en/index.html#frequency-limit-rule>
///   * 300 read operations per IP per second
pub struct GateSpotRestClient {
    _api_key: Option<String>,
    _api_secret: Option<String>,
}

impl GateSpotRestClient {
    pub fn new(api_key: Option<String>, api_secret: Option<String>) -> Self {
        GateSpotRestClient {
            _api_key: api_key,
            _api_secret: api_secret,
        }
    }

    /// Get the latest Level2 snapshot of orderbook.
    ///
    /// Top 1000 asks and bids are returned.
    ///
    /// For example: <https://api.gateio.ws/api/v4/spot/order_book?currency_pair=BTC_USDT&limit=1000>,
    pub fn fetch_l2_snapshot(symbol: &str) -> Result<String> {
        gen_api!(format!(
            "/spot/order_book?currency_pair={}&limit=1000",
            symbol
        ))
    }
}
