use super::super::utils::http_get;
use crate::error::Result;
use std::collections::BTreeMap;

const BASE_URL: &str = "https://api.bitget.com";

/// The RESTful client for Bitget swap markets.
///
/// * RESTful API doc: <https://bitgetlimited.github.io/apidoc/en/mix/#restapi>
/// * Trading at: <https://www.bitget.com/mix/>
pub struct BitgetSwapRestClient {
    _api_key: Option<String>,
    _api_secret: Option<String>,
}

impl BitgetSwapRestClient {
    pub fn new(api_key: Option<String>, api_secret: Option<String>) -> Self {
        BitgetSwapRestClient { _api_key: api_key, _api_secret: api_secret }
    }

    /// Get the latest Level2 snapshot of orderbook.
    ///
    /// For example: <https://api.bitget.com/api/mix/v1/market/depth?symbol=BTCUSDT_UMCBL&limit=100>
    ///
    /// Rate Limit：20 requests per 2 seconds
    pub fn fetch_l2_snapshot(symbol: &str) -> Result<String> {
        gen_api!(format!("/api/mix/v1/market/depth?symbol={}&limit=100", symbol))
    }

    /// Get open interest.
    ///
    /// For example:
    ///
    /// - <https://api.bitget.com/api/mix/v1/market/open-interest?symbol=BTCUSDT_UMCBL>
    pub fn fetch_open_interest(symbol: &str) -> Result<String> {
        gen_api!(format!("/api/mix/v1/market/open-interest?symbol={}", symbol))
    }
}
