use std::collections::HashMap;

use reqwest::{header, Result};
use serde_json::Value;

async fn http_get(url: &str) -> Result<String> {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/json"),
    );

    let client = reqwest::Client::builder()
         .default_headers(headers)
         .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/87.0.4280.88 Safari/537.36")
         .gzip(true)
         .build()?;
    let response = client.get(url).send().await?;

    match response.error_for_status() {
        Ok(resp) => Ok(resp.text().await?),
        Err(error) => Err(error),
    }
}

// See https://zbgapi.github.io/docs/spot/v1/en/#public-get-all-supported-trading-symbols
pub(super) async fn fetch_symbol_id_map_spot() -> HashMap<String, String> {
    let mut symbol_id_map: HashMap<String, String> = HashMap::new();

    if let Ok(txt) = http_get("https://www.zbg.com/exchange/api/v1/common/symbols").await {
        if let Ok(obj) = serde_json::from_str::<HashMap<String, Value>>(&txt) {
            if obj
                .get("resMsg")
                .unwrap()
                .as_object()
                .unwrap()
                .get("code")
                .unwrap()
                .as_str()
                .unwrap()
                == "1"
            {
                let arr = obj.get("datas").unwrap().as_array().unwrap();
                for v in arr.iter() {
                    let obj = v.as_object().unwrap();
                    let symbol = obj.get("symbol").unwrap().as_str().unwrap();
                    let id = obj.get("id").unwrap().as_str().unwrap();

                    symbol_id_map.insert(symbol.to_string(), id.to_string());
                }
            }
        }
    }

    symbol_id_map
}

// See https://zbgapi.github.io/docs/future/v1/en/#public-get-contracts
pub(super) async fn fetch_symbol_contract_id_map_swap() -> HashMap<String, i64> {
    let mut symbol_contract_id_map: HashMap<String, i64> = HashMap::new();

    if let Ok(txt) = http_get("https://www.zbg.com/exchange/api/v1/future/common/contracts").await {
        if let Ok(obj) = serde_json::from_str::<HashMap<String, Value>>(&txt) {
            if obj
                .get("resMsg")
                .unwrap()
                .as_object()
                .unwrap()
                .get("code")
                .unwrap()
                .as_str()
                .unwrap()
                == "1"
            {
                let arr = obj.get("datas").unwrap().as_array().unwrap();
                for v in arr.iter() {
                    let obj = v.as_object().unwrap();
                    let symbol = obj.get("symbol").unwrap().as_str().unwrap();
                    let contract_id = obj.get("contractId").unwrap().as_i64().unwrap();

                    symbol_contract_id_map.insert(symbol.to_string(), contract_id);
                }
            }
        }
    }

    symbol_contract_id_map
}
