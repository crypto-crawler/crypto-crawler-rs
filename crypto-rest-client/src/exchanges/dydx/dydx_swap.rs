use super::super::utils::http_get;
use crate::error::Result;
use std::collections::BTreeMap;

const BASE_URL: &str = "https://api.dydx.exchange";

/// dYdX perpetual RESTful client.
///
/// * REST API doc: <https://docs.dydx.exchange/>
/// * Trading at: <https://trade.dydx.exchange/trade>
/// * Rate Limits: <https://docs.dydx.exchange/#rate-limits>
///   * 100 requests per 10 seconds
pub struct DydxSwapRestClient {
    _api_key: Option<String>,
    _api_secret: Option<String>,
}

impl DydxSwapRestClient {
    pub fn new(api_key: Option<String>, api_secret: Option<String>) -> Self {
        DydxSwapRestClient {
            _api_key: api_key,
            _api_secret: api_secret,
        }
    }

    /// Get a Level2 orderbook snapshot.
    ///
    /// All price levels are returned.
    ///
    /// For example: <https://api.dydx.exchange/v3/orderbook/BTC-USD>
    pub fn fetch_l2_snapshot(symbol: &str) -> Result<String> {
        gen_api!(format!("/v3/orderbook/{}", symbol))
    }

    /// Get open interest.
    ///
    /// For example: <https://api.dydx.exchange/v3/markets>
    pub fn fetch_open_interest() -> Result<String> {
        gen_api!("/v3/markets")
    }
}
