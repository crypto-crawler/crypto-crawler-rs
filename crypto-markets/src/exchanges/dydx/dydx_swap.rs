use std::collections::HashMap;

use super::super::utils::http_get;
use crate::error::Result;

use serde::{Deserialize, Serialize};
use serde_json::Value;

const BASE_URL: &str = "https://api.dydx.exchange";

#[derive(Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
struct PerpetualMarket {
    market: String,
    status: String,
    baseAsset: String,
    quoteAsset: String,
    stepSize: String,
    tickSize: String,
    minOrderSize: String,
    #[serde(rename = "type")]
    type_: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
struct MarketsResponse {
    markets: HashMap<String, PerpetualMarket>,
}

// See https://docs.dydx.exchange/#get-markets
fn fetch_markets_raw() -> Result<Vec<PerpetualMarket>> {
    let txt = http_get(format!("{}/v3/markets", BASE_URL).as_str(), None)?;
    let resp = serde_json::from_str::<MarketsResponse>(&txt)?;
    Ok(resp
        .markets
        .values()
        .cloned()
        .filter(|x| x.status == "ONLINE")
        .collect::<Vec<PerpetualMarket>>())
}

pub(super) fn fetch_linear_swap_symbols() -> Result<Vec<String>> {
    let markets = fetch_markets_raw()?;
    let symbols = markets
        .into_iter()
        .map(|m| m.market)
        .collect::<Vec<String>>();
    Ok(symbols)
}
