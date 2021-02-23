use std::collections::HashMap;

use crypto_markets::MarketType;
use crypto_rest_client::{fetch_l2_snapshot, BithumbRestClient};
use serde_json::Value;

#[test]
fn test_trades() {
    let text = BithumbRestClient::fetch_trades("BTC-USDT").unwrap();
    let obj = serde_json::from_str::<HashMap<String, Value>>(&text).unwrap();
    assert_eq!(obj.get("code").unwrap().as_str().unwrap(), "0");

    let data = obj.get("data").unwrap().as_array().unwrap();
    assert!(data.len() > 0);
}

#[test]
fn test_l2_snapshot() {
    let text = fetch_l2_snapshot("bithumb", MarketType::Spot, "BTC-USDT").unwrap();
    let obj = serde_json::from_str::<HashMap<String, Value>>(&text).unwrap();
    assert_eq!(obj.get("code").unwrap().as_str().unwrap(), "0");

    let data = obj.get("data").unwrap().as_object().unwrap();
    let buy = data.get("b").unwrap().as_array().unwrap();
    let sell = data.get("s").unwrap().as_array().unwrap();
    assert!(buy.len() > 0);
    assert!(sell.len() > 0);
}
