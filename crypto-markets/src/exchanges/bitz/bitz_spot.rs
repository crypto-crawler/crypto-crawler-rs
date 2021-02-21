use std::collections::HashMap;

use super::super::utils::http_get;
use crate::error::{Error, Result};

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
struct SpotMarket {
    id: String,
    symbol: String,
    baseCurrency: String,
    quoteCurrency: String,
    amountPrecision: String,
    pricePrecision: String,
    status: String,
    minOrderAmt: String,
    maxOrderAmt: String,
    buyFree: String,
    sellFree: String,
    marketBuyFloat: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct Response {
    status: i64,
    msg: String,
    data: HashMap<String, SpotMarket>,
    time: i64,
    microtime: String,
    source: String,
}

// See https://apidocv2.bitz.plus/en/#get-data-of-trading-pairs
fn fetch_spot_markets_raw() -> Result<Vec<SpotMarket>> {
    let txt = http_get("https://apiv2.bitz.com/V2/Market/symbolList", None)?;
    let resp = serde_json::from_str::<Response>(&txt)?;
    if resp.status != 200 {
        Err(Error(txt))
    } else {
        let markets = resp
            .data
            .values()
            .cloned()
            .filter(|x| x.status == "1")
            .collect::<Vec<SpotMarket>>();
        Ok(markets)
    }
}

pub(super) fn fetch_spot_symbols() -> Result<Vec<String>> {
    let markets = fetch_spot_markets_raw()?;
    let symbols: Vec<String> = markets.into_iter().map(|m| m.symbol).collect();
    Ok(symbols)
}
