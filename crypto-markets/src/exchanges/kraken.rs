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
        _ => panic!("Unsupported market_type: {}", market_type),
    }
}

pub(crate) fn fetch_markets(_market_type: MarketType) -> Result<Vec<Market>> {
    Ok(Vec::new())
}

#[derive(Clone, Serialize, Deserialize)]
struct SpotMarket {
    altname: String,
    wsname: Option<String>,
    aclass_base: String,
    base: String,
    aclass_quote: String,
    quote: String,
    lot: String,
    pair_decimals: i64,
    lot_decimals: i64,
    lot_multiplier: i64,
    fees: Vec<Vec<f64>>,
    fees_maker: Vec<Vec<f64>>,
    fee_volume_currency: String,
    margin_call: i64,
    margin_stop: i64,
    ordermin: String,
}

#[derive(Serialize, Deserialize)]
struct Response {
    result: HashMap<String, SpotMarket>,
}

fn check_error_in_body(resp: String) -> Result<String> {
    let obj = serde_json::from_str::<HashMap<String, Value>>(&resp);
    if obj.is_err() {
        return Ok(resp);
    }

    match obj.unwrap().get("error") {
        Some(err) => {
            let arr = err.as_array().unwrap();
            if arr.is_empty() {
                Ok(resp)
            } else {
                Err(Error(resp))
            }
        }
        None => Ok(resp),
    }
}

pub(super) fn kraken_http_get(url: &str) -> Result<String> {
    let ret = http_get(url, None);
    match ret {
        Ok(resp) => check_error_in_body(resp),
        Err(_) => ret,
    }
}

// see <https://www.kraken.com/features/api#get-tradable-pairs>
fn fetch_spot_markets_raw() -> Result<Vec<SpotMarket>> {
    let txt = kraken_http_get("https://api.kraken.com/0/public/AssetPairs")?;
    let obj = serde_json::from_str::<HashMap<String, Value>>(&txt)?;
    let markets = obj
        .get("result")
        .unwrap()
        .as_object()
        .unwrap()
        .values()
        .filter(|x| x.as_object().unwrap().contains_key("wsname"))
        .map(|x| serde_json::from_value::<SpotMarket>(x.clone()).unwrap())
        .collect::<Vec<SpotMarket>>();
    Ok(markets)
}

fn fetch_spot_symbols() -> Result<Vec<String>> {
    let symbols = fetch_spot_markets_raw()?
        .into_iter()
        .filter(|m| m.wsname.is_some())
        .map(|m| m.wsname.unwrap())
        .collect::<Vec<String>>();
    Ok(symbols)
}
