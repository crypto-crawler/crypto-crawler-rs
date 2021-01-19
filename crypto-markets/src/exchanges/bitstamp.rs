use super::utils::http_get;
use crate::{error::Result, Market, MarketType};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

pub(crate) fn fetch_symbols(market_type: MarketType) -> Result<Vec<String>> {
    match market_type {
        MarketType::Spot => fetch_spot_symbols(),
        _ => panic!("Unsupported market_type: {}", market_type),
    }
}

pub(crate) fn fetch_markets(_market_type: MarketType) -> Result<Vec<Market>> {
    Ok(Vec::new())
}

#[derive(Serialize, Deserialize)]
struct SpotMarket {
    base_decimals: i64,
    minimum_order: String,
    name: String,
    counter_decimals: i64,
    trading: String,
    url_symbol: String,
    description: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

// see <https://www.bitstamp.net/api/>
fn fetch_spot_markets_raw() -> Result<Vec<SpotMarket>> {
    let txt = http_get("https://www.bitstamp.net/api/v2/trading-pairs-info/", None)?;
    let markets = serde_json::from_str::<Vec<SpotMarket>>(&txt)?;
    Ok(markets)
}

fn fetch_spot_symbols() -> Result<Vec<String>> {
    let symbols = fetch_spot_markets_raw()?
        .into_iter()
        .map(|m| m.url_symbol)
        .collect::<Vec<String>>();
    Ok(symbols)
}
