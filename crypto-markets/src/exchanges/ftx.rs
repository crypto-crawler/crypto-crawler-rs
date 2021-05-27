use std::collections::HashMap;

use super::utils::http_get;
use crate::{error::Result, Market, MarketType};

use serde::{Deserialize, Serialize};
use serde_json::Value;

pub(crate) fn fetch_symbols(market_type: MarketType) -> Result<Vec<String>> {
    match market_type {
        MarketType::Spot => fetch_spot_symbols(),
        MarketType::LinearSwap => fetch_linear_swap_symbols(),
        MarketType::LinearFuture => fetch_linear_future_symbols(),
        MarketType::Move => fetch_move_symbols(),
        MarketType::BVOL => fetch_bvol_symbols(),
        _ => panic!("Unsupported market_type: {}", market_type),
    }
}

pub(crate) fn fetch_markets(_market_type: MarketType) -> Result<Vec<Market>> {
    Ok(Vec::new())
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct FtxMarket {
    name: String,
    baseCurrency: Option<String>,
    quoteCurrency: Option<String>,
    #[serde(rename = "type")]
    type_: String,
    underlying: Option<String>,
    enabled: bool,
    postOnly: bool,
    priceIncrement: f64,
    sizeIncrement: f64,
    restricted: bool,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct Response {
    success: bool,
    result: Vec<FtxMarket>,
}

fn fetch_markets_raw() -> Result<Vec<FtxMarket>> {
    let txt = http_get("https://ftx.com/api/markets", None)?;
    let resp = serde_json::from_str::<Response>(&txt)?;
    assert!(resp.success);
    let valid: Vec<FtxMarket> = resp
        .result
        .into_iter()
        .filter(|x| {
            x.enabled
                && if let Some(underlying) = x.underlying.clone() {
                    underlying != *"BTC-HASH"
                        && !underlying.contains("PRESIDENT")
                        && underlying != *"OLYMPICS"
                } else {
                    true
                }
        })
        .collect();
    Ok(valid)
}

fn fetch_spot_symbols() -> Result<Vec<String>> {
    let markets = fetch_markets_raw()?;
    let symbols: Vec<String> = markets
        .into_iter()
        .filter(|x| x.type_ == "spot")
        .map(|x| x.name)
        .collect();
    Ok(symbols)
}

fn fetch_linear_swap_symbols() -> Result<Vec<String>> {
    let markets = fetch_markets_raw()?;
    let symbols: Vec<String> = markets
        .into_iter()
        .filter(|x| x.type_ == "future" && x.name.ends_with("-PERP"))
        .map(|x| x.name)
        .collect();
    Ok(symbols)
}

fn fetch_linear_future_symbols() -> Result<Vec<String>> {
    let markets = fetch_markets_raw()?;
    let symbols: Vec<String> = markets
        .into_iter()
        .filter(|x| {
            x.type_ == "future"
                && !x.name.ends_with("-PERP")
                && !x.name.contains("BVOL")
                && !x.name.contains("-MOVE-")
        })
        .map(|x| x.name)
        .collect();
    Ok(symbols)
}

fn fetch_move_symbols() -> Result<Vec<String>> {
    let markets = fetch_markets_raw()?;
    let symbols: Vec<String> = markets
        .into_iter()
        .filter(|x| x.type_ == "future" && x.name.contains("-MOVE-"))
        .map(|x| x.name)
        .collect();
    Ok(symbols)
}

fn fetch_bvol_symbols() -> Result<Vec<String>> {
    let markets = fetch_markets_raw()?;
    let symbols: Vec<String> = markets
        .into_iter()
        .filter(|x| x.type_ == "spot" && x.baseCurrency.clone().unwrap().ends_with("BVOL"))
        .map(|x| x.name)
        .collect();
    Ok(symbols)
}
