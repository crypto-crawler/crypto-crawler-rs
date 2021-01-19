use super::utils::huobi_http_get;
use crate::error::Result;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct InverseSwapMarket {
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
    data: Vec<InverseSwapMarket>,
    ts: i64,
}

// see <https://huobiapi.github.io/docs/coin_margined_swap/v1/en/#query-swap-info>
fn fetch_inverse_swap_markets_raw() -> Result<Vec<InverseSwapMarket>> {
    let txt = huobi_http_get("https://api.hbdm.com/swap-api/v1/swap_contract_info")?;
    let resp = serde_json::from_str::<Response>(&txt)?;
    Ok(resp.data)
}

pub(super) fn fetch_inverse_swap_symbols() -> Result<Vec<String>> {
    let symbols = fetch_inverse_swap_markets_raw()?
        .into_iter()
        .filter(|m| m.contract_status == 1)
        .map(|m| m.contract_code)
        .collect::<Vec<String>>();
    Ok(symbols)
}
