use crypto_market_type::MarketType;
use crypto_rest_client::fetch_l2_snapshot;
use serde_json::Value;
use std::collections::HashMap;

#[test]
fn test_l2_snapshot() {
    let text = fetch_l2_snapshot("bitget", MarketType::Spot, "BTCUSDT_SPBL", Some(3)).unwrap();
    let obj = serde_json::from_str::<HashMap<String, Value>>(&text).unwrap();

    assert_eq!(obj.get("msg").unwrap().as_str().unwrap(), "success");

    let data = obj.get("data").unwrap().as_object().unwrap();
    assert!(data.get("asks").unwrap().as_array().unwrap().len() > 0);
    assert!(data.get("bids").unwrap().as_array().unwrap().len() > 0);
}
