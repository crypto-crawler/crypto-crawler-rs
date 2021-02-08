use super::super::utils::http_get;
use crate::error::{Error, Result};

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
    expiration_timestamp: i64,
    creation_timestamp: i64,
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

fn fetch_symbols(kind: &str) -> Result<Vec<String>> {
    let mut all_symbols: Vec<String> = Vec::new();

    let result = fetch_instruments("BTC", kind);
    match result {
        Ok(instruments) => {
            let mut symbols: Vec<String> =
                instruments.into_iter().map(|x| x.instrument_name).collect();
            all_symbols.append(&mut symbols);
        }
        Err(error) => {
            return Err(error);
        }
    }

    let result = fetch_instruments("ETH", kind);
    match result {
        Ok(instruments) => {
            let mut symbols: Vec<String> =
                instruments.into_iter().map(|x| x.instrument_name).collect();
            all_symbols.append(&mut symbols);
        }
        Err(error) => {
            return Err(error);
        }
    }

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
