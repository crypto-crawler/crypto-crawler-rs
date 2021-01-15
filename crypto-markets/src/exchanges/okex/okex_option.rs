use std::collections::HashMap;

use super::super::utils::http_get;
use crate::error::Result;

use serde::{Deserialize, Serialize};
use serde_json::Value;

// see <https://www.okex.com/docs/en/#option-option---instrument>
#[derive(Serialize, Deserialize)]
struct OptionMarket {
    instrument_id: String,
    underlying: String,
    settlement_currency: String,
    contract_val: String,
    option_type: String,
    strike: String,
    tick_size: String,
    lot_size: String,
    listing: String,
    delivery: String,
    state: String,
    trading_start_time: String,
    timestamp: String,
    category: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

// see <https://www.okex.com/docs/en/#option-option---instrument>
fn fetch_option_markets_raw() -> Result<Vec<OptionMarket>> {
    let txt = http_get("https://www.okex.com/api/option/v3/underlying", None)?;
    let underlying_indexes = serde_json::from_str::<Vec<String>>(&txt).unwrap();

    let mut markets = Vec::<OptionMarket>::new();
    for index in underlying_indexes.iter() {
        let url = format!("https://www.okex.com/api/option/v3/instruments/{}", index);
        let txt = http_get(url.as_str(), None)?;
        let mut arr = serde_json::from_str::<Vec<OptionMarket>>(&txt).unwrap();
        markets.append(&mut arr);
    }

    Ok(markets)
}

pub(super) fn fetch_option_symbols() -> Result<Vec<String>> {
    let symbols = fetch_option_markets_raw()?
        .into_iter()
        .map(|m| m.instrument_id)
        .collect::<Vec<String>>();
    Ok(symbols)
}
