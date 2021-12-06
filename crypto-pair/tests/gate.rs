mod utils;

use crypto_market_type::MarketType;
use crypto_pair::{get_market_type, normalize_currency, normalize_pair};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use utils::http_get;

const EXCHANGE_NAME: &'static str = "gate";

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct SpotMarket {
    id: String,
    base: String,
    quote: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

// See https://www.gateio.pro/docs/apiv4/zh_CN/index.html#611e43ef81
fn fetch_spot_markets_raw() -> Vec<SpotMarket> {
    let txt = http_get("https://api.gateio.ws/api/v4/spot/currency_pairs").unwrap();
    serde_json::from_str::<Vec<SpotMarket>>(&txt).unwrap()
}

#[test]
fn verify_spot_symbols() {
    let markets = fetch_spot_markets_raw();
    for market in markets.iter() {
        let pair = normalize_pair(&market.id, EXCHANGE_NAME).unwrap();
        let pair_expected = format!(
            "{}/{}",
            normalize_currency(&market.base, EXCHANGE_NAME),
            normalize_currency(&market.quote, EXCHANGE_NAME)
        );

        assert_eq!(pair.as_str(), pair_expected);
        assert_eq!(
            MarketType::Spot,
            get_market_type(&market.id, EXCHANGE_NAME, Some(true))
        );
    }
}

#[test]
fn verify_swap_symbols() {
    assert_eq!(
        "BTC/USD".to_string(),
        normalize_pair("BTC_USD", EXCHANGE_NAME).unwrap()
    );

    assert_eq!(
        "BTC/USDT".to_string(),
        normalize_pair("BTC_USDT", EXCHANGE_NAME).unwrap()
    );

    assert_eq!(
        MarketType::InverseSwap,
        get_market_type("BTC_USD", EXCHANGE_NAME, None)
    );
    assert_eq!(
        MarketType::LinearSwap,
        get_market_type("BTC_USDT", EXCHANGE_NAME, None)
    );
}

#[derive(Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
struct FutureMarket {
    name: String,
    underlying: String,
    cycle: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

// See https://www.gateio.pro/docs/apiv4/zh_CN/index.html#595cd9fe3c-2
fn fetch_future_markets_raw(settle: &str) -> Vec<FutureMarket> {
    let txt =
        http_get(format!("https://api.gateio.ws/api/v4/delivery/{}/contracts", settle).as_str())
            .unwrap();
    serde_json::from_str::<Vec<FutureMarket>>(&txt).unwrap()
}

#[test]
fn verify_future_symbols() {
    let markets = fetch_future_markets_raw("usdt");
    for market in markets.iter() {
        let pair = normalize_pair(&market.name, EXCHANGE_NAME).unwrap();
        let pair_expected = market.underlying.replace("_", "/");

        assert_eq!(pair, pair_expected);

        let market_type = get_market_type(&market.name, EXCHANGE_NAME, None);
        assert!(
            market_type == MarketType::InverseFuture || market_type == MarketType::LinearFuture
        );
    }
}
