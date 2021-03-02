use std::collections::HashMap;

use super::super::utils::http_get;
use crate::error::Result;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
struct SwapMarket {
    funding_rate_indicative: String,
    mark_price_round: String,
    in_delisting: bool,
    order_size_min: i64,
    order_size_max: i64,
    name: String,
    maintenance_rate: String,
    mark_type: String,
    #[serde(rename = "type")]
    type_: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

// See https://www.gateio.pro/docs/apiv4/zh_CN/index.html#595cd9fe3c
fn fetch_swap_markets_raw(settle: &str) -> Result<Vec<SwapMarket>> {
    let txt = http_get(
        format!("https://api.gateio.ws/api/v4/futures/{}/contracts", settle).as_str(),
        None,
    )?;
    let markets = serde_json::from_str::<Vec<SwapMarket>>(&txt)?;
    Ok(markets
        .into_iter()
        .filter(|x| !x.in_delisting)
        .collect::<Vec<SwapMarket>>())
}

pub(super) fn fetch_inverse_swap_symbols() -> Result<Vec<String>> {
    let symbols = fetch_swap_markets_raw("btc")?
        .into_iter()
        .map(|m| m.name)
        .collect::<Vec<String>>();
    Ok(symbols)
}

pub(super) fn fetch_linear_swap_symbols() -> Result<Vec<String>> {
    let symbols = fetch_swap_markets_raw("usdt")?
        .into_iter()
        .map(|m| m.name)
        .collect::<Vec<String>>();
    Ok(symbols)
}
