mod utils;

use crypto_market_type::MarketType;
use crypto_pair::{get_market_type, normalize_currency, normalize_pair};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use utils::http_get;

const EXCHANGE_NAME: &'static str = "kucoin";

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
struct Response<T: Sized> {
    code: String,
    data: Vec<T>,
}

// See https://docs.kucoin.com/#get-symbols-list
fn fetch_spot_markets_raw() -> Vec<SpotMarket> {
    let txt = http_get("https://api.kucoin.com/api/v1/symbols").unwrap();
    let resp = serde_json::from_str::<Response<SpotMarket>>(&txt).unwrap();
    resp.data
}

// See https://docs.kucoin.com/#get-symbols-list
fn fetch_swap_markets_raw() -> Vec<SpotMarket> {
    let txt = http_get("https://api-futures.kucoin.com/api/v1/contracts/active").unwrap();
    let resp = serde_json::from_str::<Response<SpotMarket>>(&txt).unwrap();
    resp.data
}

#[test]
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
        assert_eq!(
            MarketType::Spot,
            get_market_type(&market.symbol, EXCHANGE_NAME, None)
        );
    }
}

#[test]
fn verify_swap_symbols() {
    let markets = fetch_swap_markets_raw();
    for market in markets.iter() {
        let pair = normalize_pair(&market.symbol, EXCHANGE_NAME).unwrap();
        let pair_expected = format!(
            "{}/{}",
            normalize_currency(&market.baseCurrency, EXCHANGE_NAME),
            normalize_currency(&market.quoteCurrency, EXCHANGE_NAME)
        );

        assert_eq!(pair.as_str(), pair_expected);

        let market_type = get_market_type(&market.symbol, EXCHANGE_NAME, None);
        assert!(
            market_type == MarketType::LinearSwap
                || market_type == MarketType::InverseSwap
                || market_type == MarketType::InverseFuture
        );
    }
}
