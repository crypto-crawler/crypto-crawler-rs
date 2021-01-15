use std::collections::HashMap;

use super::super::utils::http_get;
use crate::error::Result;

use serde::{Deserialize, Serialize};
use serde_json::Value;

// see <https://www.okex.com/docs/en/#spot-currency>
#[derive(Serialize, Deserialize)]
struct SpotMarket {
    instrument_id: String,
    base_currency: String,
    quote_currency: String,
    min_size: String,
    size_increment: String,
    tick_size: String,
    category: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

// see <https://www.okex.com/docs/en/#spot-currency>
fn fetch_spot_markets_raw() -> Result<Vec<SpotMarket>> {
    let txt = http_get("https://www.okex.com/api/spot/v3/instruments", None)?;
    let markets = serde_json::from_str::<Vec<SpotMarket>>(&txt).unwrap();
    Ok(markets)
}

pub(super) fn fetch_spot_symbols() -> Result<Vec<String>> {
    let symbols = fetch_spot_markets_raw()?
        .into_iter()
        .map(|m| m.instrument_id)
        .collect::<Vec<String>>();
    Ok(symbols)
}
