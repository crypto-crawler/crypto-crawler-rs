use crypto_market_type::MarketType;
use crypto_rest_client::{fetch_l2_snapshot, fetch_l3_snapshot, fetch_open_interest};
use serde_json::Value;
use std::collections::HashMap;
use test_case::test_case;

#[test_case(MarketType::Spot, "BTC-USDT")]
#[test_case(MarketType::InverseFuture, "XBTMU22")]
#[test_case(MarketType::InverseSwap, "XBTUSDM")]
#[test_case(MarketType::LinearSwap, "XBTUSDTM")]
fn test_l2_snapshot(market_type: MarketType, symbol: &str) {
    let text = fetch_l2_snapshot("kucoin", market_type, symbol, Some(3)).unwrap();

    let obj = serde_json::from_str::<HashMap<String, Value>>(&text).unwrap();
    assert_eq!("200000", obj.get("code").unwrap().as_str().unwrap());

    let data = obj.get("data").unwrap().as_object().unwrap();

    let asks = data.get("asks").unwrap().as_array().unwrap();
    assert!(!asks.is_empty());

    let bids = data.get("bids").unwrap().as_array().unwrap();
    assert!(!bids.is_empty());
}

#[test_case(MarketType::Spot, "BTC-USDT"; "inconclusive")] // TODO: kucoin deprecated level2 and level3 snapshot APIs
#[test_case(MarketType::InverseFuture, "XBTMU22")]
#[test_case(MarketType::InverseSwap, "XBTUSDM")]
#[test_case(MarketType::LinearSwap, "XBTUSDTM")]
fn test_l3_snapshot(market_type: MarketType, symbol: &str) {
    let text = fetch_l3_snapshot("kucoin", market_type, symbol, Some(3)).unwrap();

    let obj = serde_json::from_str::<HashMap<String, Value>>(&text).unwrap();
    assert_eq!("200000", obj.get("code").unwrap().as_str().unwrap());

    let data = obj.get("data").unwrap().as_object().unwrap();

    let asks = data.get("asks").unwrap().as_array().unwrap();
    assert!(!asks.is_empty());

    let bids = data.get("bids").unwrap().as_array().unwrap();
    assert!(!bids.is_empty());
}

#[test_case(MarketType::Unknown)]
#[test_case(MarketType::LinearSwap)]
fn test_open_interest(market_type: MarketType) {
    let text = fetch_open_interest("kucoin", market_type, None).unwrap();
    let obj = serde_json::from_str::<HashMap<String, Value>>(&text).unwrap();
    let arr = obj.get("data").unwrap().as_array().unwrap();
    assert!(!arr.is_empty());
}
