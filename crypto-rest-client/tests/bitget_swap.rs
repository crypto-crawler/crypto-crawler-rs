use crypto_market_type::MarketType;
use crypto_rest_client::fetch_l2_snapshot;
use serde_json::Value;
use std::collections::HashMap;
use test_case::test_case;

#[test_case(MarketType::InverseSwap, "btcusd")]
#[test_case(MarketType::LinearSwap, "cmt_btcusdt")]
fn test_l2_snapshot(market_type: MarketType, symbol: &str) {
    let text = fetch_l2_snapshot("bitget", market_type, symbol, Some(3)).unwrap();
    let obj = serde_json::from_str::<HashMap<String, Value>>(&text).unwrap();

    assert!(obj.get("asks").unwrap().as_array().unwrap().len() > 0);
    assert!(obj.get("bids").unwrap().as_array().unwrap().len() > 0);
}
