use serde_json::Value;

use super::super::utils::http_get;
use crate::error::Result;
use std::collections::HashMap;

const BASE_URL: &'static str = "https://api.hbdm.com/option-ex";

/// Huobi Option market.
///
///
/// * REST API doc: <https://huobiapi.github.io/docs/option/v1/en/>
/// * Trading at: <https://futures.huobi.com/en-us/option/exchange/>
pub struct HuobiOptionRestClient {
    _api_key: Option<String>,
    _api_secret: Option<String>,
}

impl_contract!(HuobiOptionRestClient);

impl HuobiOptionRestClient {
    /// Get active trading symbols.
    pub fn fetch_symbols() -> Result<Vec<String>> {
        let text = gen_api!("https://api.hbdm.com/option-api/v1/option_contract_info")?;
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
