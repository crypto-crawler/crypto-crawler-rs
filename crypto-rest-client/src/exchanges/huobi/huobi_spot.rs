use serde_json::Value;

use super::super::utils::http_get;
use crate::error::Result;
use std::collections::HashMap;

const BASE_URL: &'static str = "https://api.huobi.pro";

/// Huobi Spot market.
///
/// * REST API doc: <https://huobiapi.github.io/docs/spot/v1/en/>
/// * Trading at: <https://www.huobi.com/en-us/exchange/>
pub struct HuobiSpotRestClient {
    _api_key: Option<String>,
    _api_secret: Option<String>,
}

impl_contract!(HuobiSpotRestClient);

impl HuobiSpotRestClient {
    /// Get active trading symbols.
    pub fn fetch_symbols() -> Result<Vec<String>> {
        let text = gen_api!("/v1/common/symbols")?;
        let obj = serde_json::from_str::<HashMap<String, Value>>(&text).unwrap();
        if obj.get("status").unwrap().as_str().unwrap() != "ok" {
            return Err(crate::Error(text));
        }

        let arr = obj.get("data").unwrap().as_array().unwrap();
        let symbols = arr
            .into_iter()
            .map(|v| v.as_object().unwrap())
            .filter(|obj| obj.get("state").unwrap().as_str().unwrap() == "online")
            .map(|obj| obj.get("symbol").unwrap().as_str().unwrap().to_string())
            .collect::<Vec<String>>();
        Ok(symbols)
    }

    /// Get the latest Level2 orderbook snapshot.
    ///
    /// Top 150 bids and asks (aggregated) are returned.
    ///
    /// For example: <https://api.huobi.pro/market/depth?symbol=btcusdt&type=step0>
    pub fn fetch_l2_snapshot(symbol: &str) -> Result<String> {
        gen_api!(format!("/market/depth?symbol={}&type=step0", symbol))
    }
}
