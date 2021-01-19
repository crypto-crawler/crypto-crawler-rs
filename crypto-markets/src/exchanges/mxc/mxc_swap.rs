use super::utils::mxc_http_get;
use crate::error::Result;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct SwapMarket {
    symbol: String,
    displayName: String,
    displayNameEn: String,
    positionOpenType: i64,
    baseCoin: String,
    quoteCoin: String,
    settleCoin: String,
    contractSize: f64,
    minLeverage: i64,
    maxLeverage: i64,
    priceScale: i64,
    volScale: i64,
    amountScale: i64,
    priceUnit: f64,
    volUnit: i64,
    minVol: i64,
    maxVol: i64,
    bidLimitPriceRate: f64,
    askLimitPriceRate: f64,
    takerFeeRate: f64,
    makerFeeRate: f64,
    maintenanceMarginRate: f64,
    initialMarginRate: f64,
    state: i64,
    isNew: bool,
    isHot: bool,
    isHidden: bool,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct Response {
    success: bool,
    code: i64,
    data: Vec<SwapMarket>,
}

// see <https://github.com/mxcdevelop/APIDoc/blob/master/contract/contract-api.md#contract-interface-public>
fn fetch_swap_markets_raw() -> Result<Vec<SwapMarket>> {
    let txt = mxc_http_get("https://contract.mxc.com/api/v1/contract/detail")?;
    let resp = serde_json::from_str::<Response>(&txt)?;
    Ok(resp.data)
}

pub(super) fn fetch_linear_swap_symbols() -> Result<Vec<String>> {
    let symbols = fetch_swap_markets_raw()?
        .into_iter()
        .filter(|m| m.state == 0 && !m.isHidden)
        .filter(|m| m.settleCoin == m.quoteCoin)
        .map(|m| m.symbol)
        .collect::<Vec<String>>();
    Ok(symbols)
}

pub(super) fn fetch_inverse_swap_symbols() -> Result<Vec<String>> {
    let symbols = fetch_swap_markets_raw()?
        .into_iter()
        .filter(|m| m.state == 0 && !m.isHidden)
        .filter(|m| m.settleCoin == m.baseCoin)
        .map(|m| m.symbol)
        .collect::<Vec<String>>();
    Ok(symbols)
}
