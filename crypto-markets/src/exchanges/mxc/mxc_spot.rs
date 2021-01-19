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
    limit: Option<bool>,
    etf_mark: i64,
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
    if std::env::var("MXC_ACCESS_KEY").is_err() {
        panic!("MXC Spot REST APIs require access key, please set it to the MXC_ACCESS_KEY environment variable");
    }
    let access_key = std::env::var("MXC_ACCESS_KEY").unwrap();
    let txt = mxc_http_get(
        format!(
            "https://www.mxc.co/open/api/v2/market/symbols?api_key={}",
            access_key
        )
        .as_str(),
    )?;
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
