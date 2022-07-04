use super::super::utils::http_get;
use crate::{
    error::{Error, Result},
    market::{Fees, Precision, QuantityLimit},
    Market,
};

use crypto_market_type::MarketType;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

fn check_error_in_body(body: String) -> Result<String> {
    let obj = serde_json::from_str::<HashMap<String, Value>>(&body).unwrap();
    if obj.contains_key("error") {
        Err(Error(body))
    } else {
        Ok(body)
    }
}

pub(super) fn deribit_http_get(url: &str) -> Result<String> {
    let ret = http_get(url, None);
    match ret {
        Ok(body) => check_error_in_body(body),
        Err(_) => ret,
    }
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct DeribitResponse<T> {
    id: Option<i64>,
    jsonrpc: String,
    result: Vec<T>,
    usIn: i64,
    usOut: i64,
    usDiff: i64,
    testnet: bool,

    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct Instrument {
    tick_size: f64,
    taker_commission: f64,
    strike: Option<f64>,
    settlement_period: String,
    quote_currency: String,
    min_trade_amount: f64,
    max_liquidation_commission: Option<f64>,
    max_leverage: Option<i64>,
    maker_commission: f64,
    kind: String,
    is_active: bool,
    instrument_name: String,
    expiration_timestamp: u64,
    creation_timestamp: u64,
    contract_size: f64, // TODO: why i64 panic ?
    block_trade_commission: f64,
    base_currency: String,

    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

/// Get active trading instruments.
///
/// doc: <https://docs.deribit.com/?shell#public-get_instruments>
///
/// `currency`, available values are `BTC` and `ETH`.
///
/// `kind`, available values are `future` and `option`.
///
/// Example: <https://www.deribit.com/api/v2/public/get_instruments?currency=BTC&kind=future>
fn fetch_instruments(currency: &str, kind: &str) -> Result<Vec<Instrument>> {
    let url = format!(
        "https://www.deribit.com/api/v2/public/get_instruments?currency={}&kind={}",
        currency, kind
    );
    let txt = deribit_http_get(&url)?;
    let resp = serde_json::from_str::<DeribitResponse<Instrument>>(&txt)?;
    Ok(resp.result)
}

fn fetch_raw_markets(kind: &str) -> Result<Vec<Instrument>> {
    let mut all_markets: Vec<Instrument> = Vec::new();

    let result = fetch_instruments("BTC", kind);
    match result {
        Ok(mut instruments) => {
            all_markets.append(&mut instruments);
        }
        Err(error) => {
            return Err(error);
        }
    }

    let result = fetch_instruments("ETH", kind);
    match result {
        Ok(mut instruments) => {
            all_markets.append(&mut instruments);
        }
        Err(error) => {
            return Err(error);
        }
    }

    Ok(all_markets.into_iter().filter(|x| x.is_active).collect())
}

fn fetch_symbols(kind: &str) -> Result<Vec<String>> {
    let all_markets = fetch_raw_markets(kind)?;
    let all_symbols: Vec<String> = all_markets.into_iter().map(|x| x.instrument_name).collect();
    Ok(all_symbols)
}

pub(super) fn fetch_inverse_future_symbols() -> Result<Vec<String>> {
    let result = fetch_symbols("future");
    match result {
        Ok(symbols) => Ok(symbols
            .into_iter()
            .filter(|x| !x.ends_with("-PERPETUAL"))
            .collect()),
        Err(error) => Err(error),
    }
}

pub(super) fn fetch_inverse_swap_symbols() -> Result<Vec<String>> {
    let result = fetch_symbols("future");
    match result {
        Ok(symbols) => Ok(symbols
            .into_iter()
            .filter(|x| x.ends_with("-PERPETUAL"))
            .collect()),
        Err(error) => Err(error),
    }
}

pub(super) fn fetch_option_symbols() -> Result<Vec<String>> {
    fetch_symbols("option")
}

fn to_market(raw_market: &Instrument) -> Market {
    let market_type = if raw_market.kind == "future" {
        if raw_market.instrument_name.ends_with("-PERPETUAL") {
            MarketType::InverseSwap
        } else {
            MarketType::InverseFuture
        }
    } else {
        MarketType::EuropeanOption
    };
    let pair = crypto_pair::normalize_pair(&raw_market.instrument_name, "deribit").unwrap();
    let (base, quote) = {
        let v: Vec<&str> = pair.split('/').collect();
        (v[0].to_string(), v[1].to_string())
    };
    Market {
        exchange: "deribit".to_string(),
        market_type,
        symbol: raw_market.instrument_name.to_string(),
        base_id: raw_market.base_currency.to_string(),
        quote_id: raw_market.quote_currency.to_string(),
        settle_id: Some(raw_market.base_currency.to_string()),
        base: base.clone(),
        quote,
        settle: Some(base),
        active: raw_market.is_active,
        margin: true,
        fees: Fees {
            maker: raw_market.maker_commission,
            taker: raw_market.taker_commission,
        },
        precision: Precision {
            tick_size: raw_market.tick_size,
            lot_size: raw_market.min_trade_amount,
        },
        quantity_limit: Some(QuantityLimit {
            min: Some(raw_market.min_trade_amount),
            max: None,
            notional_min: None,
            notional_max: None,
        }),
        contract_value: Some(raw_market.contract_size),
        delivery_date: if market_type == MarketType::InverseSwap {
            None
        } else {
            Some(raw_market.expiration_timestamp)
        },
        info: serde_json::to_value(raw_market)
            .unwrap()
            .as_object()
            .unwrap()
            .clone(),
    }
}

pub(super) fn fetch_inverse_future_markets() -> Result<Vec<Market>> {
    let raw_markets = fetch_raw_markets("future")?;
    let markets: Vec<Market> = raw_markets
        .into_iter()
        .filter(|x| !x.instrument_name.ends_with("-PERPETUAL"))
        .map(|x| to_market(&x))
        .collect();
    Ok(markets)
}

pub(super) fn fetch_inverse_swap_markets() -> Result<Vec<Market>> {
    let raw_markets = fetch_raw_markets("future")?;
    let markets: Vec<Market> = raw_markets
        .into_iter()
        .filter(|x| x.instrument_name.ends_with("-PERPETUAL"))
        .map(|x| to_market(&x))
        .collect();
    Ok(markets)
}

pub(super) fn fetch_option_markets() -> Result<Vec<Market>> {
    let raw_markets = fetch_raw_markets("option")?;
    let markets: Vec<Market> = raw_markets.into_iter().map(|x| to_market(&x)).collect();
    Ok(markets)
}
