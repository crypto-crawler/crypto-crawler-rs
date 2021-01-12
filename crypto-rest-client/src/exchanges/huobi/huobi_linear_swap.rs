use serde_json::Value;

use super::super::utils::http_get;
use crate::error::Result;
use std::collections::HashMap;

const BASE_URL: &'static str = "https://api.hbdm.com/linear-swap-ex";

/// Huobi Linear Swap market.
///
/// Linear Swap market uses USDT as collateral.
///
/// * REST API doc: <https://huobiapi.github.io/docs/usdt_swap/v1/en/>
/// * Trading at: <https://futures.huobi.com/en-us/linear_swap/exchange/>
pub struct HuobiLinearSwapRestClient {
    _api_key: Option<String>,
    _api_secret: Option<String>,
}

impl_contract!(HuobiLinearSwapRestClient);

impl HuobiLinearSwapRestClient {
    /// Get active trading symbols.
    pub fn fetch_symbols() -> Result<Vec<String>> {
        let text = gen_api!("https://api.hbdm.com/linear-swap-api/v1/swap_contract_info")?;
        let obj = serde_json::from_str::<HashMap<String, Value>>(&text).unwrap();
        if obj.get("status").unwrap().as_str().unwrap() != "ok" {
            return Err(crate::Error(text));
        }

        let arr = obj.get("data").unwrap().as_array().unwrap();
        let symbols = arr
            .into_iter()
            .map(|v| v.as_object().unwrap())
            .filter(|obj| obj.get("contract_status").unwrap().as_i64().unwrap() == 1)
            .map(|obj| {
                obj.get("contract_code")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .to_string()
            })
            .collect::<Vec<String>>();
        Ok(symbols)
    }
}
