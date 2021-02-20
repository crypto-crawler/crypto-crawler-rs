use std::collections::HashMap;

use crypto_rest_client::BybitRestClient;
use serde_json::Value;
use test_case::test_case;

#[test_case("BTCUSD")]
#[test_case("BTCUSDT")]
fn test_l2_snapshot(symbol: &str) {
    let text = BybitRestClient::fetch_l2_snapshot(symbol).unwrap();

    let obj = serde_json::from_str::<HashMap<String, Value>>(&text).unwrap();
    let result = obj.get("result").unwrap();

    assert!(result.is_array());
    assert_eq!(result.as_array().unwrap().len(), 50);
}
