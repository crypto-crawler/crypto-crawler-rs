mod utils;

use crypto_pair::{normalize_currency, normalize_pair};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use utils::http_get;

const EXCHANGE_NAME: &str = "bitz";

#[derive(Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
struct SpotMarket {
    symbol: String,
    baseCurrency: String,
    quoteCurrency: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct SpotResponse {
    status: i64,
    msg: String,
    data: HashMap<String, SpotMarket>,
    time: i64,
    microtime: String,
    source: String,
}

// See https://apidocv2.bitz.plus/en/#get-data-of-trading-pairs
fn fetch_spot_markets_raw() -> Vec<SpotMarket> {
    let txt = http_get("https://apiv2.bitz.com/V2/Market/symbolList").unwrap();
    let resp = serde_json::from_str::<SpotResponse>(&txt).unwrap();

    resp.data.values().cloned().collect::<Vec<SpotMarket>>()
}

#[derive(Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
struct SwapMarket {
    symbol: String,
    settleAnchor: String,   // settle anchor
    quoteAnchor: String,    // quote anchor
    contractAnchor: String, // contract anchor
    pair: String,           // contract market
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct SwapResponse {
    status: i64,
    msg: String,
    data: Vec<SwapMarket>,
    time: i64,
    microtime: String,
    source: String,
}

// See https://apidocv2.bitz.plus/en/#get-market-list-of-contract-transactions
fn fetch_swap_markets_raw() -> Vec<SwapMarket> {
    let txt = http_get("https://apiv2.bitz.com/V2/Market/getContractCoin").unwrap();
    let resp = serde_json::from_str::<SwapResponse>(&txt).unwrap();
    resp.data
}

#[test]
#[ignore = "bitz.com has shutdown since October 2021"]
fn verify_spot_symbols() {
    let markets = fetch_spot_markets_raw();
    for market in markets.iter() {
        let pair = normalize_pair(&market.symbol, EXCHANGE_NAME).unwrap();
        let pair_expected = format!(
            "{}/{}",
            normalize_currency(&market.baseCurrency, EXCHANGE_NAME),
            normalize_currency(&market.quoteCurrency, EXCHANGE_NAME)
        );

        assert_eq!(pair.as_str(), pair_expected);
    }
}

#[test]
#[ignore = "bitz.com has shutdown since October 2021"]
fn verify_swap_symbols() {
    let markets = fetch_swap_markets_raw();
    for market in markets.iter() {
        let pair = normalize_pair(&market.pair, EXCHANGE_NAME).unwrap();
        let pair_expected = format!(
            "{}/{}",
            normalize_currency(&market.symbol, EXCHANGE_NAME),
            normalize_currency(&market.quoteAnchor, EXCHANGE_NAME)
        );

        assert_eq!(pair.as_str(), pair_expected);
    }
}
