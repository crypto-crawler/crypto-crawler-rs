mod utils;

use crypto_pair::{normalize_currency, normalize_pair};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use utils::http_get;

const EXCHANGE_NAME: &'static str = "bitmex";

#[derive(Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
struct Instrument {
    symbol: String,
    rootSymbol: String,
    state: String,
    positionCurrency: String,
    underlying: String,
    quoteCurrency: String,
    underlyingSymbol: String,
    isQuanto: bool,
    isInverse: bool,
    expiry: Option<String>,
    hasLiquidity: bool,
    openInterest: i64,
    fairMethod: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

fn fetch_instruments() -> Vec<Instrument> {
    let text = http_get("https://www.bitmex.com/api/v1/instrument/active").unwrap();
    let instruments: Vec<Instrument> = serde_json::from_str::<Vec<Instrument>>(&text)
        .unwrap()
        .into_iter()
        .filter(|x| x.state == "Open" && x.hasLiquidity && x.openInterest > 0)
        .collect();
    instruments
}

#[test]
fn verify_normalize_pair() {
    let instruments = fetch_instruments();
    for instrument in instruments.iter() {
        let symbol = instrument.symbol.as_str();
        let pair = normalize_pair(symbol, EXCHANGE_NAME).unwrap();

        let base_id = instrument.underlying.as_str();
        let quote_id = instrument.quoteCurrency.as_str();
        let pair_expected = format!(
            "{}/{}",
            normalize_currency(base_id, EXCHANGE_NAME),
            normalize_currency(quote_id, EXCHANGE_NAME)
        );

        assert_eq!(pair, pair_expected);
    }
}
