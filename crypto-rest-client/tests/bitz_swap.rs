use std::collections::HashMap;

use crypto_market_type::MarketType;
use crypto_rest_client::{fetch_l2_snapshot, fetch_open_interest};
use serde_json::Value;
use test_case::test_case;

#[test_case(MarketType::InverseSwap, "BTC_USD"; "inconclusive 1")]
#[test_case(MarketType::LinearSwap, "BTC_USDT"; "inconclusive 2")]
fn test_l2_snapshot(market_type: MarketType, symbol: &str) {
    let text = fetch_l2_snapshot("bitz", market_type, symbol, Some(3)).unwrap();
    let obj = serde_json::from_str::<HashMap<String, Value>>(&text).unwrap();

    assert_eq!(obj.get("status").unwrap().as_i64().unwrap(), 200);

    let data = obj.get("data").unwrap().as_object().unwrap();
    assert!(data.get("asks").unwrap().as_array().unwrap().len() > 0);
    assert!(data.get("bids").unwrap().as_array().unwrap().len() > 0);
}

#[test_case(MarketType::InverseSwap; "inconclusive 1")]
#[test_case(MarketType::LinearSwap; "inconclusive 2")]
fn test_open_interest(market_type: MarketType) {
    let text = fetch_open_interest("bitz", market_type, None).unwrap();
    let obj = serde_json::from_str::<HashMap<String, Value>>(&text).unwrap();
    let arr = obj.get("data").unwrap().as_array().unwrap();
    assert!(!arr.is_empty());
}
