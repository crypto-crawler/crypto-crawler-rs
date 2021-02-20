use std::collections::HashMap;

use super::utils::http_get;
use crate::{error::Result, Market, MarketType};

use serde::{Deserialize, Serialize};
use serde_json::Value;

pub(crate) fn fetch_symbols(market_type: MarketType) -> Result<Vec<String>> {
    match market_type {
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

#[derive(Serialize, Deserialize)]
struct BybitMarket {
    name: String,
    base_currency: String,
    quote_currency: String,
    price_scale: i64,
    taker_fee: String,
    maker_fee: String,
    leverage_filter: LeverageFilter,
    price_filter: PriceFilter,
    lot_size_filter: LotSizeFilter,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct Response {
    ret_code: i64,
    ret_msg: String,
    ext_code: String,
    ext_info: String,
    result: Vec<BybitMarket>,
}

// See https://bybit-exchange.github.io/docs/inverse/#t-querysymbol
fn fetch_markets_raw() -> Result<Vec<BybitMarket>> {
    let txt = http_get("https://api.bybit.com/v2/public/symbols", None)?;
    let resp = serde_json::from_str::<Response>(&txt)?;
    assert_eq!(resp.ret_code, 0);
    Ok(resp.result)
}

fn fetch_inverse_swap_symbols() -> Result<Vec<String>> {
    let symbols = fetch_markets_raw()?
        .into_iter()
        .filter(|m| m.quote_currency == "USD")
        .map(|m| m.name)
        .collect::<Vec<String>>();
    Ok(symbols)
}

fn fetch_linear_swap_symbols() -> Result<Vec<String>> {
    let symbols = fetch_markets_raw()?
        .into_iter()
        .filter(|m| m.quote_currency == "USDT")
        .map(|m| m.name)
        .collect::<Vec<String>>();
    Ok(symbols)
}
