use std::collections::HashMap;

use super::super::utils::http_get;
use crate::error::{Error, Result};

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct SwapMarket {
    symbol: String,
    currencyName: String,
    lotSize: String,
    contractId: i64,
    takerFeeRatio: String,
    commodityId: i64,
    currencyId: i64,
    contractUnit: String,
    makerFeeRatio: String,
    priceTick: String,
    commodityName: Option<String>,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct ResMsg {
    message: String,
    method: Option<String>,
    code: String,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct Response {
    datas: Vec<SwapMarket>,
    resMsg: ResMsg,
}

// See https://zbgapi.github.io/docs/future/v1/en/#public-get-contracts
fn fetch_swap_markets_raw() -> Result<Vec<SwapMarket>> {
    let txt = http_get(
        "https://www.zbg.com/exchange/api/v1/future/common/contracts",
        None,
    )?;
    let resp = serde_json::from_str::<Response>(&txt)?;
    if resp.resMsg.code != "1" {
        Err(Error(txt))
    } else {
        Ok(resp.datas)
    }
}

pub(super) fn fetch_inverse_swap_symbols() -> Result<Vec<String>> {
    let symbols = fetch_swap_markets_raw()?
        .into_iter()
        .map(|m| m.symbol)
        .filter(|x| x.ends_with("_USD-R"))
        .collect::<Vec<String>>();
    Ok(symbols)
}

pub(super) fn fetch_linear_swap_symbols() -> Result<Vec<String>> {
    let symbols = fetch_swap_markets_raw()?
        .into_iter()
        .map(|m| m.symbol)
        .filter(|x| x.ends_with("_USDT"))
        .collect::<Vec<String>>();
    Ok(symbols)
}
