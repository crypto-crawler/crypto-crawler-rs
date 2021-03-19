mod utils;

use crypto_pair::{normalize_currency, normalize_pair};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use utils::http_get;

const EXCHANGE_NAME: &'static str = "bitget";

#[derive(Serialize, Deserialize)]
struct SpotMarket {
    base_currency: String,
    quote_currency: String,
    symbol: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct Response {
    status: String,
    ts: i64,
    data: Vec<SpotMarket>,
}

// See https://bitgetlimited.github.io/apidoc/en/swap/#contract-information
#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct SwapMarket {
    symbol: String,
    underlying_index: String,
    quote_currency: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

fn fetch_spot_markets_raw() -> Vec<SpotMarket> {
    let txt = http_get("https://api.bitget.com/data/v1/common/symbols").unwrap();
    let resp = serde_json::from_str::<Response>(&txt).unwrap();
    resp.data
}

fn fetch_swap_markets_raw() -> Vec<SwapMarket> {
    let txt = http_get("https://capi.bitget.com/api/swap/v3/market/contracts").unwrap();
    serde_json::from_str::<Vec<SwapMarket>>(&txt).unwrap()
}

#[test]
fn verify_spot_symbols() {
    let markets = fetch_spot_markets_raw();
    for market in markets.iter() {
        let pair = normalize_pair(&market.symbol, EXCHANGE_NAME).unwrap();
        let pair_expected = format!(
            "{}/{}",
            normalize_currency(&market.base_currency, EXCHANGE_NAME),
            normalize_currency(&market.quote_currency, EXCHANGE_NAME)
        );

        assert_eq!(pair.as_str(), pair_expected);
    }
}

#[test]
fn verify_swap_symbols() {
    let markets = fetch_swap_markets_raw();
    for market in markets.iter() {
        let pair = normalize_pair(&market.symbol, EXCHANGE_NAME).unwrap();
        let pair_expected = format!(
            "{}/{}",
            normalize_currency(&market.underlying_index, EXCHANGE_NAME),
            normalize_currency(&market.quote_currency, EXCHANGE_NAME)
        );

        assert_eq!(pair.as_str(), pair_expected);
    }
}
