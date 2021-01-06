use super::super::utils::http_get;
use crate::error::Result;
use std::collections::HashMap;

const BASE_URL: &'static str = "https://contract.mxc.com";

/// MXC Swap market.
///
/// * REST API doc: <https://github.com/mxcdevelop/APIDoc/blob/master/contract/contract-api.md>
/// * Trading at: <https://contract.mxc.com/exchange>
pub struct MXCSwapRestClient {
    _api_key: Option<String>,
    _api_secret: Option<String>,
}

impl MXCSwapRestClient {
    pub fn new(api_key: Option<String>, api_secret: Option<String>) -> Self {
        MXCSwapRestClient {
            _api_key: api_key,
            _api_secret: api_secret,
        }
    }

    /// Get most recent trades.
    ///
    /// For example: <https://contract.mxc.com/api/v1/contract/deals/BTC_USDT>
    pub fn fetch_trades(symbol: &str) -> Result<String> {
        gen_api!(format!("/api/v1/contract/deals/{}", symbol))
    }

    /// Get the latest Level2 snapshot of orderbook.
    ///
    /// Top 1000 bids and asks will be returned.
    ///
    /// For example: <https://contract.mxc.com/api/v1/contract/depth_commits/BTC_USDT/1000>
    pub fn fetch_l2_snapshot(symbol: &str) -> Result<String> {
        gen_api!(format!("/api/v1/contract/depth_commits/{}/1000", symbol))
    }
}
