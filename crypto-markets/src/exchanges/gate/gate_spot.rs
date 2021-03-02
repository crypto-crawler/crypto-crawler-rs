use std::collections::HashMap;

use super::super::utils::http_get;
use crate::error::Result;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct SpotMarket {
    id: String,
    base: String,
    quote: String,
    fee: String,
    min_base_amount: Option<String>,
    min_quote_amount: Option<String>,
    amount_precision: i64,
    precision: i64,
    trade_status: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

// See https://www.gateio.pro/docs/apiv4/zh_CN/index.html#611e43ef81
fn fetch_spot_markets_raw() -> Result<Vec<SpotMarket>> {
    let txt = http_get("https://api.gateio.ws/api/v4/spot/currency_pairs", None)?;
    let markets = serde_json::from_str::<Vec<SpotMarket>>(&txt)?;
    Ok(markets
        .into_iter()
        .filter(|x| x.trade_status == "tradable")
        .collect::<Vec<SpotMarket>>())
}

pub(super) fn fetch_spot_symbols() -> Result<Vec<String>> {
    let markets = fetch_spot_markets_raw()?;
    let symbols: Vec<String> = markets.into_iter().map(|m| m.id).collect();
    Ok(symbols)
}
