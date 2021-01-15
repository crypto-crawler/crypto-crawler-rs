use std::collections::HashMap;

use super::super::utils::http_get;
use crate::error::Result;

use serde::{Deserialize, Serialize};
use serde_json::Value;

// see <https://www.okex.com/docs/en/#swap-swap---contract_information>
#[derive(Serialize, Deserialize)]
struct SwapMarket {
    instrument_id: String,
    underlying: String,
    base_currency: String,
    quote_currency: String,
    settlement_currency: String,
    contract_val: String,
    listing: String,
    delivery: String,
    tick_size: String,
    is_inverse: String,
    contract_val_currency: String,
    category: String,
    underlying_index: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

// see <https://www.okex.com/docs/en/#swap-swap---contract_information>
fn fetch_swap_markets_raw() -> Result<Vec<SwapMarket>> {
    let txt = http_get("https://www.okex.com/api/swap/v3/instruments", None)?;
    let markets = serde_json::from_str::<Vec<SwapMarket>>(&txt).unwrap();
    Ok(markets)
}

pub(super) fn fetch_linear_swap_symbols() -> Result<Vec<String>> {
    let symbols = fetch_swap_markets_raw()?
        .into_iter()
        .filter(|x| x.is_inverse == "false")
        .map(|m| m.instrument_id)
        .collect::<Vec<String>>();
    Ok(symbols)
}

pub(super) fn fetch_inverse_swap_symbols() -> Result<Vec<String>> {
    let symbols = fetch_swap_markets_raw()?
        .into_iter()
        .filter(|x| x.is_inverse == "true")
        .map(|m| m.instrument_id)
        .collect::<Vec<String>>();
    Ok(symbols)
}
