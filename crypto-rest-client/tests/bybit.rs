use std::collections::HashMap;

use crypto_market_type::MarketType;
use crypto_rest_client::fetch_l2_snapshot;
use serde_json::Value;
use test_case::test_case;

#[test_case(MarketType::InverseSwap, "BTCUSD")]
#[test_case(MarketType::LinearSwap, "BTCUSDT")]
fn test_l2_snapshot(market_type: MarketType, symbol: &str) {
    let text = fetch_l2_snapshot("bybit", market_type, symbol, Some(3)).unwrap();

    let obj = serde_json::from_str::<HashMap<String, Value>>(&text).unwrap();
    let result = obj.get("result").unwrap();

    assert!(result.is_array());
    assert_eq!(result.as_array().unwrap().len(), 50);
}
