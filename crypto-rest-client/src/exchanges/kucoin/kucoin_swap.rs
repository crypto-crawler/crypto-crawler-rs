use super::super::utils::http_get;
use crate::error::Result;
use std::collections::BTreeMap;

const BASE_URL: &str = "https://api-futures.kucoin.com";

/// The RESTful client for KuCoin Future and Swap markets.
///
/// * RESTful API doc: <https://docs.kucoin.com/futures>
/// * Trading at: <https://futures.kucoin.com/>
/// * Rate Limits: <https://docs.kucoin.cc/futures/#request-rate-limit>
pub struct KuCoinSwapRestClient {
    _api_key: Option<String>,
    _api_secret: Option<String>,
}

impl KuCoinSwapRestClient {
    pub fn new(api_key: Option<String>, api_secret: Option<String>) -> Self {
        KuCoinSwapRestClient {
            _api_key: api_key,
            _api_secret: api_secret,
        }
    }

    /// Get the latest Level2 snapshot of orderbook.
    ///
    /// All bids and asks are returned.
    ///
    /// For example: <https://api-futures.kucoin.com/api/v1/level2/snapshot?symbol=XBTUSDM>,
    pub fn fetch_l2_snapshot(symbol: &str) -> Result<String> {
        // the request rate limit is 30 times/3s
        gen_api!(format!("/api/v1/level2/snapshot?symbol={}", symbol))
    }

    /// Get the latest Level3 snapshot of orderbook.
    ///
    /// All bids and asks are returned.
    ///
    /// For example: <https://api-futures.kucoin.com/api/v2/level3/snapshot?symbol=XBTUSDM>,
    pub fn fetch_l3_snapshot(symbol: &str) -> Result<String> {
        gen_api!(format!("/api/v2/level3/snapshot?symbol={}", symbol))
    }

    /// Get open interest.
    ///
    /// For example:
    /// - <https://ftx.com/api/futures>
    pub fn fetch_open_interest() -> Result<String> {
        gen_api!("/api/v1/contracts/active")
    }
}
