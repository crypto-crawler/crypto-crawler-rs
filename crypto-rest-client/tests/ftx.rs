use std::collections::HashMap;

use crypto_markets::MarketType;
use crypto_rest_client::fetch_l2_snapshot;
use serde_json::Value;
use test_case::test_case;

#[test_case(MarketType::Spot, "BTC/USD")]
#[test_case(MarketType::LinearSwap, "BTC-PERP")]
#[test_case(MarketType::LinearFuture, "BTC-0326")]
#[test_case(MarketType::Move, "BTC-MOVE-2021Q1")]
#[test_case(MarketType::BVOL, "BVOL/USD")]
fn test_l2_snapshot(market_type: MarketType, symbol: &str) {
    let text = fetch_l2_snapshot("ftx", market_type, symbol).unwrap();

    let obj = serde_json::from_str::<HashMap<String, Value>>(&text).unwrap();
    let result = obj.get("result").unwrap().as_object().unwrap();

    assert!(result.get("asks").unwrap().is_array());
    assert!(result.get("bids").unwrap().is_array())
}
