use std::collections::HashMap;

use crypto_market_type::MarketType;
use crypto_rest_client::fetch_l2_snapshot;
use serde_json::Value;

#[test]
fn test_spot_l2_snapshot() {
    let text = fetch_l2_snapshot("zb", MarketType::Spot, "btc_usdt", Some(3)).unwrap();
    let obj = serde_json::from_str::<HashMap<String, Value>>(&text).unwrap();

    assert!(!obj.get("asks").unwrap().as_array().unwrap().is_empty());
    assert!(!obj.get("bids").unwrap().as_array().unwrap().is_empty());
}

#[test]
fn test_swap_l2_snapshot() {
    let text = fetch_l2_snapshot("zb", MarketType::LinearSwap, "BTC_USDT", Some(3)).unwrap();
    let obj = serde_json::from_str::<HashMap<String, Value>>(&text).unwrap();

    assert_eq!(10000, obj["code"].as_i64().unwrap());

    let data = obj.get("data").unwrap().as_object().unwrap();
    assert!(!data.get("asks").unwrap().as_array().unwrap().is_empty());
    assert!(!data.get("bids").unwrap().as_array().unwrap().is_empty());
}
