use std::collections::HashMap;

use super::super::utils::http_get;
use crate::error::Result;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
struct FutureMarket {
    name: String,
    underlying: String,
    cycle: String,
    #[serde(rename = "type")]
    type_: String,
    in_delisting: bool,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

// See https://www.gateio.pro/docs/apiv4/zh_CN/index.html#595cd9fe3c-2
fn fetch_future_markets_raw(settle: &str) -> Result<Vec<FutureMarket>> {
    let txt = http_get(
        format!("https://api.gateio.ws/api/v4/delivery/{}/contracts", settle).as_str(),
        None,
    )?;
    let markets = serde_json::from_str::<Vec<FutureMarket>>(&txt)?;
    Ok(markets
        .into_iter()
        .filter(|x| !x.in_delisting)
        .collect::<Vec<FutureMarket>>())
}

pub(super) fn fetch_inverse_future_symbols() -> Result<Vec<String>> {
    let symbols = fetch_future_markets_raw("btc")?
        .into_iter()
        .map(|m| m.name)
        .collect::<Vec<String>>();
    Ok(symbols)
}

pub(super) fn fetch_linear_future_symbols() -> Result<Vec<String>> {
    let symbols = fetch_future_markets_raw("usdt")?
        .into_iter()
        .map(|m| m.name)
        .collect::<Vec<String>>();
    Ok(symbols)
}
