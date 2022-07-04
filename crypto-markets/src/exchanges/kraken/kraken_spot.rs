use std::collections::HashMap;

use super::super::utils::http_get;
use crate::{
    error::{Error, Result},
    Fees, Market, MarketType, Precision, QuantityLimit,
};

use serde::{Deserialize, Serialize};
use serde_json::Value;

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
        return Err(Error(resp));
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

// see <https://docs.kraken.com/rest/#operation/getTradableAssetPairs>
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

pub(super) fn fetch_spot_symbols() -> Result<Vec<String>> {
    let symbols = fetch_spot_markets_raw()?
        .into_iter()
        .filter_map(|m| m.wsname)
        .collect::<Vec<String>>();
    Ok(symbols)
}

pub(super) fn fetch_spot_markets() -> Result<Vec<Market>> {
    let markets = fetch_spot_markets_raw()?
        .into_iter()
        .map(|m| {
            let info = serde_json::to_value(&m)
                .unwrap()
                .as_object()
                .unwrap()
                .clone();
            let symbol = m.wsname.unwrap();
            let pair = crypto_pair::normalize_pair(&symbol, "kraken").unwrap();
            let (base, quote) = {
                let v: Vec<&str> = pair.split('/').collect();
                (v[0].to_string(), v[1].to_string())
            };
            Market {
                exchange: "kraken".to_string(),
                market_type: MarketType::Spot,
                symbol,
                base_id: m.base,
                quote_id: m.quote,
                settle_id: None,
                base,
                quote,
                settle: None,
                active: true,
                margin: false,
                // see https://support.kraken.com/hc/en-us/articles/360000526126-What-are-Maker-and-Taker-fees-
                fees: Fees {
                    maker: 0.0016,
                    taker: 0.0026,
                },
                precision: Precision {
                    tick_size: 1.0 / (10_i64.pow(m.pair_decimals as u32) as f64),
                    lot_size: 1.0 / (10_i64.pow(m.lot_decimals as u32) as f64),
                },
                quantity_limit: Some(QuantityLimit {
                    min: m.ordermin.parse::<f64>().ok(),
                    max: None,
                    notional_min: None,
                    notional_max: None,
                }),
                contract_value: None,
                delivery_date: None,
                info,
            }
        })
        .collect::<Vec<Market>>();
    Ok(markets)
}
