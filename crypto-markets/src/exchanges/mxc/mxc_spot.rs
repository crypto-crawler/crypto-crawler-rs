use super::utils::mxc_http_get;
use crate::error::Result;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct SpotMarket {
    symbol: String,
    state: String,
    price_scale: i64,
    quantity_scale: i64,
    min_amount: String,
    max_amount: String,
    maker_fee_rate: String,
    taker_fee_rate: String,
    limited: bool,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct Response {
    code: i64,
    data: Vec<SpotMarket>,
}

// see <https://mxcdevelop.github.io/APIDoc/open.api.v2.en.html#all-symbols>
fn fetch_spot_markets_raw() -> Result<Vec<SpotMarket>> {
    let txt = mxc_http_get("https://www.mexc.com/open/api/v2/market/symbols")?;
    let resp = serde_json::from_str::<Response>(&txt)?;
    Ok(resp.data)
}

pub(super) fn fetch_spot_symbols() -> Result<Vec<String>> {
    let symbols = fetch_spot_markets_raw()?
        .into_iter()
        .filter(|m| m.state == "ENABLED")
        .map(|m| m.symbol)
        .collect::<Vec<String>>();
    Ok(symbols)
}
