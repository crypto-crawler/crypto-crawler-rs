use std::collections::HashMap;

use crypto_market_type::MarketType;
use crypto_rest_client::{fetch_l2_snapshot, fetch_open_interest};
use serde_json::Value;
use test_case::test_case;

#[test_case(MarketType::Spot, "btc_usdt")]
#[test_case(MarketType::InverseSwap, "BTC_USD-R")]
#[test_case(MarketType::LinearSwap, "BTC_USDT")]
fn test_l2_snapshot(market_type: MarketType, symbol: &str) {
    let text = fetch_l2_snapshot("zbg", market_type, symbol, Some(3)).unwrap();
    let obj = serde_json::from_str::<HashMap<String, Value>>(&text).unwrap();

    let code = obj
        .get("resMsg")
        .unwrap()
        .as_object()
        .unwrap()
        .get("code")
        .unwrap()
        .as_str()
        .unwrap();
    if code == "6001" || code == "6020" {
        // system in maintainance
        return;
    }

    assert_eq!("1", code);

    let data = obj.get("datas").unwrap().as_object().unwrap();
    assert!(!data.get("asks").unwrap().as_array().unwrap().is_empty());
    assert!(!data.get("bids").unwrap().as_array().unwrap().is_empty());
}

#[test_case(MarketType::InverseSwap, "BTC_USD-R")]
#[test_case(MarketType::LinearSwap, "BTC_USDT")]
fn test_open_interest(market_type: MarketType, symbol: &str) {
    let text = fetch_open_interest("zbg", market_type, Some(symbol)).unwrap();
    let obj = serde_json::from_str::<HashMap<String, Value>>(&text).unwrap();
    assert!(obj.contains_key("datas"));
}
