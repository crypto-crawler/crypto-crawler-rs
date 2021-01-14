use super::utils::huobi_http_get;
use crate::error::Result;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct LinearSwapMarket {
    symbol: String,
    contract_code: String,
    contract_size: f64,
    price_tick: f64,
    delivery_time: String,
    create_date: String,
    contract_status: i64,
    settlement_date: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct Response {
    status: String,
    data: Vec<LinearSwapMarket>,
    ts: i64,
}

// see <https://huobiapi.github.io/docs/usdt_swap/v1/en/#general-query-swap-info>
fn fetch_linear_swap_markets_raw() -> Result<Vec<LinearSwapMarket>> {
    let txt = huobi_http_get("https://api.hbdm.com/linear-swap-api/v1/swap_contract_info")?;
    let resp = serde_json::from_str::<Response>(&txt).unwrap();
    Ok(resp.data)
}

pub(super) fn fetch_linear_swap_symbols() -> Result<Vec<String>> {
    let symbols = fetch_linear_swap_markets_raw()?
        .into_iter()
        .filter(|m| m.contract_status == 1)
        .map(|m| m.contract_code)
        .collect::<Vec<String>>();
    Ok(symbols)
}
