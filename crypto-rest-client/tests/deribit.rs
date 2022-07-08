use crypto_market_type::MarketType;
use crypto_rest_client::{fetch_l2_snapshot, fetch_open_interest, DeribitRestClient};
use serde_json::Value;
use std::collections::HashMap;
use test_case::test_case;

#[test_case("BTC-PERPETUAL")]
#[test_case("BTC-29APR22")]
#[test_case("BTC-29JUL22-20000-C")]
fn test_trades(symbol: &str) {
    let text = DeribitRestClient::fetch_trades(symbol).unwrap();

    let obj = serde_json::from_str::<HashMap<String, Value>>(&text).unwrap();
    let result = obj.get("result").unwrap().as_object().unwrap();

    assert!(result.get("trades").unwrap().is_array());
}

#[test_case(MarketType::InverseSwap, "BTC-PERPETUAL")]
#[test_case(MarketType::InverseFuture, "BTC-29APR22")]
#[test_case(MarketType::EuropeanOption, "BTC-29JUL22-20000-C")]
fn test_l2_snapshot(market_type: MarketType, symbol: &str) {
    let text = fetch_l2_snapshot("deribit", market_type, symbol, Some(3)).unwrap();

    let obj = serde_json::from_str::<HashMap<String, Value>>(&text).unwrap();
    let result = obj.get("result").unwrap().as_object().unwrap();

    assert!(result.get("asks").unwrap().is_array());
    assert!(result.get("bids").unwrap().is_array())
}

#[test_case(MarketType::InverseFuture)]
#[test_case(MarketType::InverseSwap)]
#[test_case(MarketType::EuropeanOption)]
fn test_open_interest(market_type: MarketType) {
    let text = fetch_open_interest("deribit", market_type, None).unwrap();
    for line in text.lines() {
        let obj = serde_json::from_str::<HashMap<String, Value>>(&line).unwrap();
        let arr = obj.get("result").unwrap().as_array().unwrap();
        assert!(!arr.is_empty());
    }
}
