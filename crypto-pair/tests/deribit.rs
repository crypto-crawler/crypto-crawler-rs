mod utils;

use crypto_pair::{normalize_currency, normalize_pair};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use utils::http_get;

const EXCHANGE_NAME: &'static str = "deribit";

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
    quote_currency: String,
    instrument_name: String,
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
fn fetch_instruments(currency: &str, kind: &str) -> Vec<Instrument> {
    let url = format!(
        "https://www.deribit.com/api/v2/public/get_instruments?currency={}&kind={}",
        currency, kind
    );
    let txt = http_get(&url).unwrap();
    let resp = serde_json::from_str::<DeribitResponse<Instrument>>(&txt).unwrap();
    resp.result
}

#[test]
fn verify_all_symbols() {
    let mut markets = fetch_instruments("BTC", "future");

    let mut tmp_markets = fetch_instruments("ETH", "future");
    markets.append(&mut tmp_markets);

    tmp_markets = fetch_instruments("BTC", "option");
    markets.append(&mut tmp_markets);

    tmp_markets = fetch_instruments("ETH", "option");
    markets.append(&mut tmp_markets);

    for market in markets.iter() {
        let pair = normalize_pair(&market.instrument_name, EXCHANGE_NAME).unwrap();
        let pair_expected = format!(
            "{}/{}",
            normalize_currency(&market.base_currency, EXCHANGE_NAME),
            normalize_currency(&market.quote_currency, EXCHANGE_NAME)
        );

        assert_eq!(pair.as_str(), pair_expected);
    }
}
