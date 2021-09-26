use super::super::utils::http_get;
use crate::error::Result;
use std::collections::HashMap;

const BASE_URL: &str = "https://capi.bitget.com/api/swap";

/// The RESTful client for Bitget swap markets.
///
/// * RESTful API doc: <https://bitgetlimited.github.io/apidoc/en/swap/>
/// * Trading at: <https://www.bitget.com/en/swap/>
pub struct BitgetSwapRestClient {
    _api_key: Option<String>,
    _api_secret: Option<String>,
}

impl BitgetSwapRestClient {
    pub fn new(api_key: Option<String>, api_secret: Option<String>) -> Self {
        BitgetSwapRestClient {
            _api_key: api_key,
            _api_secret: api_secret,
        }
    }

    /// Get the latest Level2 snapshot of orderbook.
    ///
    /// For example: <https://capi.bitget.com/api/swap/v3/market/depth?symbol=btcusd&limit=1000>
    ///
    /// Rate Limit：20 requests per 2 seconds
    pub fn fetch_l2_snapshot(symbol: &str) -> Result<String> {
        gen_api!(format!("/v3/market/depth?symbol={}&limit=1000", symbol))
    }
}
