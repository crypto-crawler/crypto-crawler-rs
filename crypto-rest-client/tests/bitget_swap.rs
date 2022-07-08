use crypto_market_type::MarketType;
use crypto_rest_client::{fetch_l2_snapshot, fetch_open_interest};
use serde_json::Value;
use std::collections::HashMap;
use test_case::test_case;

#[test_case(MarketType::InverseSwap, "BTCUSD_DMCBL")]
#[test_case(MarketType::LinearSwap, "BTCUSDT_UMCBL")]
fn test_l2_snapshot(market_type: MarketType, symbol: &str) {
    let text = fetch_l2_snapshot("bitget", market_type, symbol, Some(3)).unwrap();
    let obj = serde_json::from_str::<HashMap<String, Value>>(&text).unwrap();

    let data = obj.get("data").unwrap().as_object().unwrap();
    assert!(!data.get("asks").unwrap().as_array().unwrap().is_empty());
    assert!(!data.get("bids").unwrap().as_array().unwrap().is_empty());
}

#[test_case(MarketType::InverseSwap, "BTCUSD_DMCBL")]
#[test_case(MarketType::LinearSwap, "BTCUSDT_UMCBL")]
fn test_open_interest(market_type: MarketType, symbol: &str) {
    let text = fetch_open_interest("bitget", market_type, Some(symbol)).unwrap();
    let obj = serde_json::from_str::<HashMap<String, Value>>(&text).unwrap();

    let data = obj.get("data").unwrap().as_object().unwrap();
    assert!(data.contains_key("amount"));
}
