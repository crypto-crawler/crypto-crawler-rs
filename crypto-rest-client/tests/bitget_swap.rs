use std::collections::HashMap;

use crypto_rest_client::BitgetSwapRestClient;
use serde_json::Value;
use test_case::test_case;

#[test_case("btcusd")]
#[test_case("cmt_btcusdt")]
fn test_l2_snapshot(symbol: &str) {
    let text = BitgetSwapRestClient::fetch_l2_snapshot(symbol).unwrap();
    let obj = serde_json::from_str::<HashMap<String, Value>>(&text).unwrap();

    assert!(obj.get("asks").unwrap().as_array().unwrap().len() > 0);
    assert!(obj.get("bids").unwrap().as_array().unwrap().len() > 0);
}
