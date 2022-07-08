use std::collections::HashMap;

use crypto_market_type::MarketType;
use crypto_rest_client::fetch_l2_snapshot;
use serde_json::Value;

#[test]
#[ignore = "bitz.com has shutdown since October 2021"]
fn test_l2_snapshot() {
    let text = fetch_l2_snapshot("bitz", MarketType::Spot, "btc_usdt", Some(3)).unwrap();
    let obj = serde_json::from_str::<HashMap<String, Value>>(&text).unwrap();

    assert_eq!(obj.get("status").unwrap().as_i64().unwrap(), 200);

    let data = obj.get("data").unwrap().as_object().unwrap();
    assert!(!data.get("asks").unwrap().as_array().unwrap().is_empty());
    assert!(!data.get("bids").unwrap().as_array().unwrap().is_empty());
}
