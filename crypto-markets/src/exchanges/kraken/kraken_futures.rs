use std::collections::HashMap;

use super::super::utils::http_get;
use crate::{
    error::{Error, Result},
    Fees, Market, MarketType, Precision, QuantityLimit,
};

use chrono::DateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Serialize, Deserialize)]
struct FuturesMarketPartial {
    symbol: String,
    #[serde(rename = "type")]
    type_: String,
    tradeable: bool,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
struct FuturesMarket {
    symbol: String,
    #[serde(rename = "type")]
    type_: String,
    tradeable: bool,
    underlying: Option<String>,
    lastTradingTime: Option<String>, // only applicable for futures
    tickSize: f64,
    contractSize: f64,
    isin: Option<String>,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct Response<T: Sized> {
    result: String,
    instruments: Vec<T>,
}

fn check_error_in_body(resp: String) -> Result<String> {
    let obj = serde_json::from_str::<HashMap<String, Value>>(&resp);
    if obj.is_err() {
        return Err(Error(resp));
    }
    let obj = obj.unwrap();

    if obj.get("result").unwrap() != "success" {
        return Err(Error(resp));
    }
    Ok(resp)
}

pub(super) fn kraken_http_get(url: &str) -> Result<String> {
    let ret = http_get(url, None);
    match ret {
        Ok(resp) => check_error_in_body(resp),
        Err(_) => ret,
    }
}

// see <https://www.kraken.com/features/api#get-tradable-pairs>
fn fetch_futures_markets_raw() -> Result<Vec<FuturesMarket>> {
    let txt = kraken_http_get("https://futures.kraken.com/derivatives/api/v3/instruments")?;
    let obj = serde_json::from_str::<Response<FuturesMarketPartial>>(&txt)?;
    let markets = obj
        .instruments
        .into_iter()
        .filter(|x| x.tradeable)
        .map(|x| {
            serde_json::from_str::<FuturesMarket>(serde_json::to_string(&x).unwrap().as_str())
                .unwrap()
        })
        .collect::<Vec<FuturesMarket>>();
    Ok(markets)
}

pub(super) fn fetch_inverse_future_symbols() -> Result<Vec<String>> {
    let symbols = fetch_futures_markets_raw()?
        .into_iter()
        .filter(|x| x.symbol.starts_with("fi_"))
        .map(|m| m.symbol.to_uppercase())
        .collect::<Vec<String>>();
    Ok(symbols)
}

pub(super) fn fetch_inverse_swap_symbols() -> Result<Vec<String>> {
    let symbols = fetch_futures_markets_raw()?
        .into_iter()
        .filter(|x| x.symbol.starts_with("pi_"))
        .map(|m| m.symbol.to_uppercase())
        .collect::<Vec<String>>();
    Ok(symbols)
}

pub(super) fn fetch_inverse_future_markets() -> Result<Vec<Market>> {
    let markets = fetch_futures_markets()?
        .into_iter()
        .filter(|x| x.market_type == MarketType::InverseFuture)
        .collect::<Vec<Market>>();
    Ok(markets)
}

pub(super) fn fetch_inverse_swap_markets() -> Result<Vec<Market>> {
    let markets = fetch_futures_markets()?
        .into_iter()
        .filter(|x| x.market_type == MarketType::InverseSwap)
        .collect::<Vec<Market>>();
    Ok(markets)
}

fn fetch_futures_markets() -> Result<Vec<Market>> {
    let markets = fetch_futures_markets_raw()?
        .into_iter()
        .filter(|m| m.symbol.starts_with("pi_") || m.symbol.starts_with("fi_")) // TODO: Multi-Collateral, e.g., pf_xbtusd
        .map(|m| {
            let info = serde_json::to_value(&m)
                .unwrap()
                .as_object()
                .unwrap()
                .clone();
            let pair = crypto_pair::normalize_pair(&m.symbol, "kraken").unwrap();
            let (base, quote) = {
                let v: Vec<&str> = pair.split('/').collect();
                (v[0].to_string(), v[1].to_string())
            };
            let (base_id, quote_id) = {
                let pos = m.symbol.find("usd").unwrap();
                (m.symbol[3..pos].to_string(), "usd".to_string())
            };
            Market {
                exchange: "kraken".to_string(),
                market_type: if m.symbol.starts_with("fi_") {
                    MarketType::InverseFuture
                } else if m.symbol.starts_with("pi_") {
                    MarketType::InverseSwap
                } else {
                    MarketType::Unknown
                },
                symbol: m.symbol,
                base_id: base_id.clone(),
                quote_id,
                settle_id: Some(base_id),
                base: base.clone(),
                quote,
                settle: Some(base),
                active: m.tradeable,
                margin: true,
                // see https://futures.kraken.com/derivatives/api/v3/feeschedules
                fees: Fees {
                    maker: 0.0002,
                    taker: 0.0005,
                },
                precision: Precision {
                    tick_size: m.tickSize,
                    lot_size: 1.0,
                },
                quantity_limit: Some(QuantityLimit {
                    min: Some(1.0),
                    max: None,
                    notional_min: None,
                    notional_max: None,
                }),
                contract_value: Some(m.contractSize),
                delivery_date: m
                    .lastTradingTime
                    .map(|x| DateTime::parse_from_rfc3339(&x).unwrap().timestamp_millis() as u64),
                info,
            }
        })
        .collect::<Vec<Market>>();
    Ok(markets)
}
