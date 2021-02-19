use std::collections::HashMap;

use crypto_rest_client::FtxRestClient;
use serde_json::Value;
use test_case::test_case;

#[test_case("BTC/USD")]
#[test_case("BTC-PERP")]
#[test_case("BTC-0326")]
#[test_case("BTC-MOVE-2021Q1")]
#[test_case("BVOL/USD")]
fn test_l2_snapshot(symbol: &str) {
    let text = FtxRestClient::fetch_l2_snapshot(symbol).unwrap();

    let obj = serde_json::from_str::<HashMap<String, Value>>(&text).unwrap();
    let result = obj.get("result").unwrap().as_object().unwrap();

    assert!(result.get("asks").unwrap().is_array());
    assert!(result.get("bids").unwrap().is_array())
}
