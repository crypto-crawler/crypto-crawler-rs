use crypto_market_type::MarketType;
use crypto_rest_client::{fetch_l2_snapshot, fetch_open_interest};
use serde_json::Value;
use std::collections::HashMap;
use test_case::test_case;

#[test_case(MarketType::Spot, "BTC_USDT")]
#[test_case(MarketType::InverseSwap, "BTC_USD")]
#[test_case(MarketType::LinearSwap, "BTC_USDT")]
#[test_case(MarketType::InverseFuture, "BTC_USD_20220930")]
#[test_case(MarketType::LinearFuture, "BTC_USDT_20220930")]
fn test_l2_snapshot(market_type: MarketType, symbol: &str) {
    let text = fetch_l2_snapshot("gate", market_type, symbol, Some(3)).unwrap();
    let obj = serde_json::from_str::<HashMap<String, Value>>(&text).unwrap();

    let asks = obj.get("asks").unwrap().as_array().unwrap();
    assert!(!asks.is_empty());

    let bids = obj.get("bids").unwrap().as_array().unwrap();
    assert!(!bids.is_empty());
}

#[test_case(MarketType::InverseSwap, "BTC_USD")]
#[test_case(MarketType::LinearSwap, "BTC_USDT")]
fn test_open_interest(market_type: MarketType, symbol: &str) {
    let text = fetch_open_interest("gate", market_type, Some(symbol)).unwrap();
    let arr = serde_json::from_str::<Vec<Value>>(&text).unwrap();
    assert!(!arr.is_empty());
}
