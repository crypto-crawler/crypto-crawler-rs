use std::collections::HashMap;

use crypto_markets::MarketType;
use crypto_rest_client::fetch_l2_snapshot;
use serde_json::Value;
use test_case::test_case;

#[test_case(MarketType::InverseSwap, "BTC_USD")]
#[test_case(MarketType::LinearSwap, "BTC_USDT")]
fn test_l2_snapshot(market_type: MarketType, symbol: &str) {
    let text = fetch_l2_snapshot("bitz", market_type, symbol).unwrap();
    let obj = serde_json::from_str::<HashMap<String, Value>>(&text).unwrap();

    assert_eq!(obj.get("status").unwrap().as_i64().unwrap(), 200);

    let data = obj.get("data").unwrap().as_object().unwrap();
    assert!(data.get("asks").unwrap().as_array().unwrap().len() > 0);
    assert!(data.get("bids").unwrap().as_array().unwrap().len() > 0);
}
