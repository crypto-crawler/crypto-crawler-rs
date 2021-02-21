use std::collections::HashMap;

use super::utils::http_get;
use crate::{
    error::{Error, Result},
    Market, MarketType,
};

use serde::{Deserialize, Serialize};
use serde_json::Value;

pub(crate) fn fetch_symbols(market_type: MarketType) -> Result<Vec<String>> {
    match market_type {
        MarketType::Spot => fetch_spot_symbols(),
        MarketType::InverseSwap => fetch_inverse_swap_symbols(),
        MarketType::LinearSwap => fetch_linear_swap_symbols(),
        _ => panic!("Unsupported market_type: {}", market_type),
    }
}

pub(crate) fn fetch_markets(_market_type: MarketType) -> Result<Vec<Market>> {
    Ok(Vec::new())
}

#[derive(Serialize, Deserialize)]
struct LeverageFilter {
    min_leverage: i64,
    max_leverage: i64,
    leverage_step: String,
}

#[derive(Serialize, Deserialize)]
struct PriceFilter {
    min_price: String,
    max_price: String,
    tick_size: String,
}

#[derive(Serialize, Deserialize)]
struct LotSizeFilter {
    max_trading_qty: i64,
    min_trading_qty: f64,
    qty_step: f64,
}

// See https://github.com/BitgetLimited/API_Docs_en/wiki/REST_api_reference#get-datav1commonsymbols--query-all-trading-pairs-and-accuracy-supported-in-the-station
#[derive(Serialize, Deserialize)]
struct SpotMarket {
    base_currency: String,
    quote_currency: String,
    symbol: String,
    tick_size: String,
    size_increment: String,
    status: String,
    base_asset_precision: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct Response {
    status: String,
    ts: i64,
    data: Vec<SpotMarket>,
}

// See https://bitgetlimited.github.io/apidoc/en/swap/#contract-information
#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct SwapMarket {
    symbol: String,
    underlying_index: String,
    quote_currency: String,
    coin: String,
    contract_val: String,
    size_increment: String,
    tick_size: String,
    forwardContractFlag: bool,
    priceEndStep: i64,
    minLeverage: i64,
    maxLeverage: i64,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

fn fetch_spot_markets_raw() -> Result<Vec<SpotMarket>> {
    let txt = http_get("https://api.bitget.com/data/v1/common/symbols", None)?;
    let resp = serde_json::from_str::<Response>(&txt)?;
    if resp.status != "ok" {
        Err(Error(txt))
    } else {
        Ok(resp.data)
    }
}

fn fetch_swap_markets_raw() -> Result<Vec<SwapMarket>> {
    let txt = http_get("https://capi.bitget.com/api/swap/v3/market/contracts", None)?;
    let markets = serde_json::from_str::<Vec<SwapMarket>>(&txt)?;
    Ok(markets)
}

fn fetch_spot_symbols() -> Result<Vec<String>> {
    let markets = fetch_spot_markets_raw()?;
    let symbols: Vec<String> = markets
        .into_iter()
        .filter(|m| m.status == "1")
        .map(|m| m.symbol)
        .collect();
    Ok(symbols)
}

fn fetch_inverse_swap_symbols() -> Result<Vec<String>> {
    let symbols = fetch_swap_markets_raw()?
        .into_iter()
        .filter(|m| !m.forwardContractFlag)
        .map(|m| m.symbol)
        .collect::<Vec<String>>();
    Ok(symbols)
}

fn fetch_linear_swap_symbols() -> Result<Vec<String>> {
    let symbols = fetch_swap_markets_raw()?
        .into_iter()
        .filter(|m| m.forwardContractFlag)
        .map(|m| m.symbol)
        .collect::<Vec<String>>();
    Ok(symbols)
}
