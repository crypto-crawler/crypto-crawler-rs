use std::collections::HashMap;

use super::super::utils::http_get;
use crate::error::{Error, Result};

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
struct SpotMarket {
    symbol: String,
    name: String,
    baseCurrency: String,
    quoteCurrency: String,
    feeCurrency: String,
    market: String,
    baseMinSize: String,
    quoteMinSize: String,
    baseMaxSize: String,
    quoteMaxSize: String,
    baseIncrement: String,
    quoteIncrement: String,
    priceIncrement: String,
    priceLimitRate: String,
    isMarginEnabled: bool,
    enableTrading: bool,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct Response {
    code: String,
    data: Vec<SpotMarket>,
}

// See https://docs.kucoin.com/#get-symbols-list
fn fetch_spot_markets_raw() -> Result<Vec<SpotMarket>> {
    let txt = http_get("https://api.kucoin.com/api/v1/symbols", None)?;
    let resp = serde_json::from_str::<Response>(&txt)?;
    if resp.code != "200000" {
        Err(Error(txt))
    } else {
        let markets = resp
            .data
            .into_iter()
            .filter(|x| x.enableTrading)
            .collect::<Vec<SpotMarket>>();
        Ok(markets)
    }
}

pub(super) fn fetch_spot_symbols() -> Result<Vec<String>> {
    let markets = fetch_spot_markets_raw()?;
    let symbols: Vec<String> = markets.into_iter().map(|m| m.symbol).collect();
    Ok(symbols)
}
