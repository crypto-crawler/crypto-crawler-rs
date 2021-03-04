use std::collections::HashMap;

use crypto_market_type::MarketType;
use crypto_rest_client::fetch_l2_snapshot;
use serde_json::Value;
use test_case::test_case;

#[test_case(MarketType::Spot, "btc_usdt")]
#[test_case(MarketType::InverseSwap, "BTC_USD-R")]
#[test_case(MarketType::LinearSwap, "BTC_USDT")]
fn test_l2_snapshot(market_type: MarketType, symbol: &str) {
    let text = fetch_l2_snapshot("zbg", market_type, symbol).unwrap();
    let obj = serde_json::from_str::<HashMap<String, Value>>(&text).unwrap();

    assert_eq!(
        obj.get("resMsg")
            .unwrap()
            .as_object()
            .unwrap()
            .get("code")
            .unwrap()
            .as_str()
            .unwrap(),
        "1"
    );

    let data = obj.get("datas").unwrap().as_object().unwrap();
    assert!(data.get("asks").unwrap().as_array().unwrap().len() > 0);
    assert!(data.get("bids").unwrap().as_array().unwrap().len() > 0);
}
