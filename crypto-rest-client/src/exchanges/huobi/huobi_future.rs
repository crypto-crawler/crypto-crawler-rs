use serde_json::Value;

use super::super::utils::http_get;
use crate::error::Result;
use std::collections::HashMap;

const BASE_URL: &'static str = "https://api.hbdm.com";

/// Huobi Future market.
///
/// * REST API doc: <https://huobiapi.github.io/docs/dm/v1/en/>
/// * Trading at: <https://futures.huobi.com/en-us/contract/exchange/>
pub struct HuobiFutureRestClient {
    _api_key: Option<String>,
    _api_secret: Option<String>,
}

impl_contract!(HuobiFutureRestClient);

impl HuobiFutureRestClient {
    /// Get active trading symbols.
    pub fn fetch_symbols() -> Result<Vec<String>> {
        let text = gen_api!("/api/v1/contract_contract_info")?;
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
                obj.get("symbol").unwrap().as_str().unwrap().to_string()
                    + match obj.get("contract_type").unwrap().as_str().unwrap() {
                        "this_week" => "_CW",
                        "next_week" => "_NW",
                        "quarter" => "_CQ",
                        "next_quarter" => "_NQ",
                        contract_type => panic!("Unknown contract_type {}", contract_type),
                    }
            })
            .collect::<Vec<String>>();
        Ok(symbols)
    }
}
