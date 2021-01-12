use serde_json::Value;

use super::super::utils::http_get;
use crate::error::Result;
use std::collections::HashMap;

const BASE_URL: &'static str = "https://www.mxc.co";

/// MXC Spot market.
///
/// * REST API doc: <https://mxcdevelop.github.io/APIDoc/>
/// * Trading at: <https://www.mxc.com/trade/pro>
pub struct MXCSpotRestClient {
    access_key: String,
    _secret_key: Option<String>,
}

impl MXCSpotRestClient {
    pub fn new(access_key: String, secret_key: Option<String>) -> Self {
        MXCSpotRestClient {
            access_key,
            _secret_key: secret_key,
        }
    }

    /// Get active trading symbols.
    pub fn fetch_symbols(&self) -> Result<Vec<String>> {
        let text = gen_api!(format!(
            "/open/api/v2/market/symbols?api_key={}",
            self.access_key
        ))?;
        let obj = serde_json::from_str::<HashMap<String, Value>>(&text).unwrap();
        if obj.get("code").unwrap().as_i64().unwrap() != 200 {
            return Err(crate::Error(text));
        }

        let symbols = obj
            .get("data")
            .unwrap()
            .as_array()
            .unwrap()
            .iter()
            .filter(|x| x.get("state").unwrap().as_str().unwrap() == "ENABLED")
            .map(|x| x.get("symbol").unwrap().as_str().unwrap().to_string())
            .collect::<Vec<String>>();
        Ok(symbols)
    }

    /// Get latest trades.
    ///
    /// 1000 trades are returned.
    ///
    /// For example: <https://www.mxc.co/open/api/v2/market/deals?symbol=BTC_USDT&limit=1000&api_key=your-access-key>
    #[allow(non_snake_case)]
    pub fn fetch_trades(&self, symbol: &str) -> Result<String> {
        gen_api!(format!(
            "/open/api/v2/market/deals?symbol={}&limit=1000&api_key={}",
            symbol, self.access_key
        ))
    }

    /// Get latest Level2 snapshot of orderbook.
    ///
    /// Top 2000 bids and asks will be returned.
    ///
    /// For example: <https://www.mxc.co/open/api/v2/market/depth?symbol=BTC_USDT&depth=2000&api_key=your-access-key>
    pub fn fetch_l2_snapshot(&self, symbol: &str) -> Result<String> {
        gen_api!(format!(
            "/open/api/v2/market/depth?symbol={}&depth=2000&api_key={}",
            symbol, self.access_key
        ))
    }
}
