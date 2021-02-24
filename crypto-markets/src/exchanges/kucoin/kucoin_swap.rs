use std::collections::HashMap;

use super::super::utils::http_get;
use crate::error::{Error, Result};

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
struct SwapMarket {
    symbol: String,
    rootSymbol: String,
    #[serde(rename = "type")]
    type_: String,
    baseCurrency: String,
    quoteCurrency: String,
    settleCurrency: String,
    maxOrderQty: i64,
    maxPrice: f64,
    lotSize: f64,
    tickSize: f64,
    indexPriceTickSize: f64,
    multiplier: f64,
    initialMargin: f64,
    maintainMargin: f64,
    maxRiskLimit: i64,
    minRiskLimit: i64,
    riskStep: i64,
    makerFeeRate: f64,
    takerFeeRate: f64,
    takerFixFee: f64,
    makerFixFee: f64,
    isDeleverage: bool,
    isQuanto: bool,
    isInverse: bool,
    markMethod: String,
    fairMethod: Option<String>,
    status: String,
    fundingFeeRate: Option<f64>,
    predictedFundingFeeRate: Option<f64>,
    openInterest: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct Response {
    code: String,
    data: Vec<SwapMarket>,
}

// See https://docs.kucoin.com/#get-symbols-list
fn fetch_swap_markets_raw() -> Result<Vec<SwapMarket>> {
    let txt = http_get(
        "https://api-futures.kucoin.com/api/v1/contracts/active",
        None,
    )?;
    let resp = serde_json::from_str::<Response>(&txt)?;
    if resp.code != "200000" {
        Err(Error(txt))
    } else {
        let markets = resp
            .data
            .into_iter()
            .filter(|x| x.status == "Open")
            .collect::<Vec<SwapMarket>>();
        Ok(markets)
    }
}

pub(super) fn fetch_inverse_swap_symbols() -> Result<Vec<String>> {
    let markets = fetch_swap_markets_raw()?;
    let symbols: Vec<String> = markets
        .into_iter()
        .filter(|x| x.isInverse && x.type_ == "FFWCSX")
        .map(|m| m.symbol)
        .collect();
    Ok(symbols)
}

pub(super) fn fetch_linear_swap_symbols() -> Result<Vec<String>> {
    let markets = fetch_swap_markets_raw()?;
    let symbols: Vec<String> = markets
        .into_iter()
        .filter(|x| !x.isInverse && x.type_ == "FFWCSX")
        .map(|m| m.symbol)
        .collect();
    Ok(symbols)
}

pub(super) fn fetch_inverse_future_symbols() -> Result<Vec<String>> {
    let markets = fetch_swap_markets_raw()?;
    let symbols: Vec<String> = markets
        .into_iter()
        .filter(|x| x.isInverse && x.type_ == "FFICSX")
        .map(|m| m.symbol)
        .collect();
    Ok(symbols)
}
