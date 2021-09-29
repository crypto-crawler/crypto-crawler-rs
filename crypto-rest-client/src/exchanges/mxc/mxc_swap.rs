use super::super::utils::http_get;
use crate::error::Result;
use std::collections::BTreeMap;

const BASE_URL: &str = "https://contract.mexc.com";

/// MXC Swap market.
///
/// * REST API doc: <https://mxcdevelop.github.io/APIDoc/>
/// * Trading at: <https://contract.mxc.com/exchange>
pub struct MxcSwapRestClient {
    _api_key: Option<String>,
    _api_secret: Option<String>,
}

impl MxcSwapRestClient {
    pub fn new(api_key: Option<String>, api_secret: Option<String>) -> Self {
        MxcSwapRestClient {
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
    /// Top 2000 bids and asks will be returned.
    ///
    /// For example: <https://contract.mxc.com/api/v1/contract/depth/BTC_USDT?limit=2000>
    ///
    /// Rate limit: 20 times /2 seconds
    pub fn fetch_l2_snapshot(symbol: &str) -> Result<String> {
        gen_api!(format!("/api/v1/contract/depth/{}?limit=2000", symbol))
    }
}
