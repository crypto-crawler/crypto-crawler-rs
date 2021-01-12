use serde_json::Value;

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

    /// Get active trading symbols.
    ///
    /// For example: <https://contract.mxc.com/api/v1/contract/detail>
    pub fn fetch_symbols() -> Result<Vec<String>> {
        let text = gen_api!("/api/v1/contract/detail")?;
        let obj = serde_json::from_str::<HashMap<String, Value>>(&text).unwrap();
        if !obj.get("success").unwrap().as_bool().unwrap() {
            return Err(crate::Error(text));
        }

        let symbols = obj
            .get("data")
            .unwrap()
            .as_array()
            .unwrap()
            .iter()
            .filter(|x| {
                x.get("state").unwrap().as_i64().unwrap() == 0
                    && x.get("isHidden").unwrap().as_bool().unwrap() == false
            })
            .map(|x| x.get("symbol").unwrap().as_str().unwrap().to_string())
            .collect::<Vec<String>>();
        Ok(symbols)
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
