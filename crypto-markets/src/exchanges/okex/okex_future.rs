use std::collections::HashMap;

use super::super::utils::http_get;
use crate::error::Result;

use serde::{Deserialize, Serialize};
use serde_json::Value;

// see <https://www.okex.com/docs/en/#futures-contract_information>
#[derive(Serialize, Deserialize)]
struct FutureMarket {
    instrument_id: String,
    underlying: String,
    base_currency: String,
    quote_currency: String,
    settlement_currency: String,
    contract_val: String,
    listing: String,
    delivery: String,
    tick_size: String,
    trade_increment: String,
    alias: String,
    is_inverse: String,
    contract_val_currency: String,
    category: String,
    underlying_index: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

// see <https://www.okex.com/docs/en/#futures-contract_information>
fn fetch_future_markets_raw() -> Result<Vec<FutureMarket>> {
    let txt = http_get("https://www.okex.com/api/futures/v3/instruments", None)?;
    let markets = serde_json::from_str::<Vec<FutureMarket>>(&txt).unwrap();
    Ok(markets)
}

pub(super) fn fetch_linear_future_symbols() -> Result<Vec<String>> {
    let symbols = fetch_future_markets_raw()?
        .into_iter()
        .filter(|x| x.is_inverse == "false")
        .map(|m| m.instrument_id)
        .collect::<Vec<String>>();
    Ok(symbols)
}

pub(super) fn fetch_inverse_future_symbols() -> Result<Vec<String>> {
    let symbols = fetch_future_markets_raw()?
        .into_iter()
        .filter(|x| x.is_inverse == "true")
        .map(|m| m.instrument_id)
        .collect::<Vec<String>>();
    Ok(symbols)
}
