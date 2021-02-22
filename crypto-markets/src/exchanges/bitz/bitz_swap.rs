use std::collections::HashMap;

use super::super::utils::http_get;
use crate::error::{Error, Result};

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
struct SwapMarket {
    contractId: String,        // contract id
    symbol: String,            // symbol
    settleAnchor: String,      // settle anchor
    quoteAnchor: String,       // quote anchor
    contractAnchor: String,    // contract anchor
    contractValue: String,     // contract Value
    pair: String,              //contract market
    expiry: String,            //delivery time (non-perpetual contract)
    maxLeverage: String,       // max leverage
    maintanceMargin: String,   //maintenance margin
    makerFee: String,          // maker fee rate
    takerFee: String,          // taker fee rate
    settleFee: String,         // settlement fee rate
    priceDec: String,          // floating point decimal of price
    anchorDec: String,         // floating point decimal of quote anchor
    status: String,            // status，1: trading, 0:pending,-1:permanent stop
    isreverse: String,         // 1:reverse contract，-1: forward contract
    allowCross: String,        // Allow cross position，1:Yes，-1:No
    allowLeverages: String,    // Leverage multiple allowed by the system
    maxOrderNum: String,       // max number of unfilled orders
    maxAmount: String,         // max amount of a single order
    minAmount: String,         // min amount of a single order
    maxPositionAmount: String, //max position amount
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct Response {
    status: i64,
    msg: String,
    data: Vec<SwapMarket>,
    time: i64,
    microtime: String,
    source: String,
}

// See https://apidocv2.bitz.plus/en/#get-market-list-of-contract-transactions
fn fetch_swap_markets_raw() -> Result<Vec<SwapMarket>> {
    let txt = http_get("https://apiv2.bitz.com/V2/Market/getContractCoin", None)?;
    let resp = serde_json::from_str::<Response>(&txt)?;
    if resp.status != 200 {
        Err(Error(txt))
    } else {
        let markets: Vec<SwapMarket> = resp.data.into_iter().filter(|x| x.status == "1").collect();
        Ok(markets)
    }
}

pub(super) fn fetch_inverse_swap_symbols() -> Result<Vec<String>> {
    let symbols = fetch_swap_markets_raw()?
        .into_iter()
        .filter(|m| m.isreverse == "1")
        .map(|m| m.pair)
        .collect::<Vec<String>>();
    Ok(symbols)
}

pub(super) fn fetch_linear_swap_symbols() -> Result<Vec<String>> {
    let symbols = fetch_swap_markets_raw()?
        .into_iter()
        .filter(|m| m.isreverse == "-1" && m.settleAnchor == "USDT")
        .map(|m| m.pair)
        .collect::<Vec<String>>();
    Ok(symbols)
}
