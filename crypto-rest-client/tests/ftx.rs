use std::collections::HashMap;

use crypto_market_type::MarketType;
use crypto_rest_client::{fetch_l2_snapshot, fetch_open_interest};
use serde_json::Value;
use test_case::test_case;

#[test_case(MarketType::Spot, "BTC/USD")]
#[test_case(MarketType::LinearSwap, "BTC-PERP")]
#[test_case(MarketType::LinearFuture, "BTC-0930")]
#[test_case(MarketType::Move, "BTC-MOVE-2022Q3")]
#[test_case(MarketType::BVOL, "BVOL/USD")]
fn test_l2_snapshot(market_type: MarketType, symbol: &str) {
    let text = fetch_l2_snapshot("ftx", market_type, symbol, Some(3)).unwrap();

    let obj = serde_json::from_str::<HashMap<String, Value>>(&text).unwrap();
    let result = obj.get("result").unwrap().as_object().unwrap();

    assert!(result.get("asks").unwrap().is_array());
    assert!(result.get("bids").unwrap().is_array())
}

#[test]
fn test_open_interest() {
    let text = fetch_open_interest("ftx", MarketType::Unknown, None).unwrap();
    let obj = serde_json::from_str::<HashMap<String, Value>>(&text).unwrap();
    let result = obj.get("result").unwrap().as_array().unwrap();
    assert!(!result.is_empty())
}
