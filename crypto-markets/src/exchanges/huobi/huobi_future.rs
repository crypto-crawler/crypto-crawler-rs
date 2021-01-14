use super::utils::huobi_http_get;
use crate::error::Result;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct FutureMarket {
    symbol: String,
    contract_code: String,
    contract_type: String,
    contract_size: f64,
    price_tick: f64,
    delivery_date: String,
    delivery_time: String,
    create_date: String,
    contract_status: i64,
    settlement_time: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct Response {
    status: String,
    data: Vec<FutureMarket>,
    ts: i64,
}

// see <https://huobiapi.github.io/docs/dm/v1/en/#get-contract-info>
fn fetch_future_markets_raw() -> Result<Vec<FutureMarket>> {
    let txt = huobi_http_get("https://api.hbdm.com/api/v1/contract_contract_info")?;
    let resp = serde_json::from_str::<Response>(&txt).unwrap();
    Ok(resp.data)
}

pub(super) fn fetch_inverse_future_symbols() -> Result<Vec<String>> {
    let symbols = fetch_future_markets_raw()?
        .into_iter()
        .filter(|m| m.contract_status == 1)
        .map(|m| {
            m.symbol.to_string()
                + match m.contract_type.as_str() {
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
