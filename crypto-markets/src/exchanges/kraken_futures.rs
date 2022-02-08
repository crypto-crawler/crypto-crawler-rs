use std::collections::HashMap;

use super::utils::http_get;
use crate::{
    error::{Error, Result},
    Fees, Market, MarketType, Precision,
};

use serde::{Deserialize, Serialize};
use serde_json::Value;

pub(crate) fn fetch_symbols(market_type: MarketType) -> Result<Vec<String>> {
    match market_type {
        MarketType::InverseFuture => fetch_inverse_future_symbols(),
        _ => panic!("Unsupported market_type: {}", market_type),
    }
}

pub(crate) fn fetch_markets(market_type: MarketType) -> Result<Vec<Market>> {
    match market_type {
        MarketType::InverseFuture => fetch_inverse_future_markets(),
        _ => panic!("Unsupported market_type: {}", market_type),
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct InverseFutureMarket {
    symbol: String,
    #[serde(alias = "type")]
    market_type: String,
    underlying: String,
    #[serde(alias = "tickSize")]
    tick_size: f64,
    #[serde(alias = "contractSize")]
    contract_size: f64,
    tradeable: bool,
    isin: String,
    #[serde(alias = "lastTradingTime")]
    last_trading_time: Option<String>,
    #[serde(alias = "marginLevels")]
    margin_levels: Vec<MarginLevel>,
    #[serde(alias = "retailMarginLevels")]
    retail_margin_levels: Vec<MarginLevel>,
}

#[derive(Clone, Serialize, Deserialize)]
struct MarginLevel {
    contracts: u64,
    #[serde(alias = "initialMargin")]
    initial_margin: f64,
    #[serde(alias = "maintenanceMargin")]
    maintenance_margin: f64,
}

#[derive(Serialize, Deserialize)]
struct Response {
    result: HashMap<String, InverseFutureMarket>,
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
fn fetch_inverse_future_markets_raw() -> Result<Vec<InverseFutureMarket>> {
    let txt = kraken_http_get("https://futures.kraken.com/derivatives/api/v3/instruments")?;
    let obj = serde_json::from_str::<HashMap<String, Value>>(&txt)?;
    let markets = obj
        .get("instruments")
        .unwrap()
        .as_array()
        .unwrap()
        .into_iter()
        .filter(|x| x.as_object().unwrap().contains_key("tickSize"))
        .map(|x| serde_json::from_value::<InverseFutureMarket>(x.clone()).unwrap())
        .collect::<Vec<InverseFutureMarket>>();
    Ok(markets)
}

fn fetch_inverse_future_symbols() -> Result<Vec<String>> {
    let symbols = fetch_inverse_future_markets_raw()?
        .into_iter()
        .map(|m| m.symbol)
        .collect::<Vec<String>>();
    Ok(symbols)
}

fn fetch_inverse_future_markets() -> Result<Vec<Market>> {
    let markets = fetch_inverse_future_markets_raw()?
        .into_iter()
        .map(|m| {
            let info = serde_json::to_value(&m)
                .unwrap()
                .as_object()
                .unwrap()
                .clone();
            let symbol = m.symbol;
            let pair = crypto_pair::normalize_pair(&symbol, "kraken_futures").unwrap();
            let (base, quote) = {
                pair.split('_')
                    .skip(1)
                    .collect::<Vec<&str>>()
                    .get(0)
                    .unwrap()
                    .split_at(3)
                    .into()
            };
            Market {
                exchange: "kraken_futures".to_string(),
                market_type: MarketType::InverseFuture,
                symbol,
                base_id: base.into(),
                quote_id: quote.into(),
                settle_id: None,
                base: base.into(),
                quote: quote.into(),
                settle: None,
                active: true,
                margin: false,
                // No fees for kraken futures: https://support.kraken.com/hc/en-us/articles/360031091951-Overview-of-fees-on-Kraken-Futures
                fees: Fees {
                    maker: 0.0,
                    taker: 0.0,
                },
                precision: Precision {
                    tick_size: m.tick_size,
                    lot_size: m.contract_size,
                },
                quantity_limit: None,
                contract_value: None,
                delivery_date: None,
                info,
            }
        })
        .collect::<Vec<Market>>();
    Ok(markets)
}
