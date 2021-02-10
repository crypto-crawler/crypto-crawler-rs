use std::collections::HashMap;

use crypto_rest_client::DeribitRestClient;
use serde_json::Value;
use test_case::test_case;

#[test_case("BTC-PERPETUAL")]
#[test_case("BTC-24SEP21")]
#[test_case("BTC-31DEC21-400000-C")]
fn test_trades(symbol: &str) {
    let text = DeribitRestClient::fetch_trades(symbol).unwrap();

    let obj = serde_json::from_str::<HashMap<String, Value>>(&text).unwrap();
    let result = obj.get("result").unwrap().as_object().unwrap();

    assert!(result.get("trades").unwrap().is_array());
}

#[test_case("BTC-PERPETUAL")]
#[test_case("BTC-24SEP21")]
#[test_case("BTC-31DEC21-400000-C")]
fn test_l2_snapshot(symbol: &str) {
    let text = DeribitRestClient::fetch_l2_snapshot(symbol).unwrap();

    let obj = serde_json::from_str::<HashMap<String, Value>>(&text).unwrap();
    let result = obj.get("result").unwrap().as_object().unwrap();

    assert!(result.get("asks").unwrap().is_array());
    assert!(result.get("bids").unwrap().is_array())
}
