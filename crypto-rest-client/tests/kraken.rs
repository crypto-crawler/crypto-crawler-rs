use std::collections::HashMap;

use crypto_market_type::MarketType;
use crypto_rest_client::{fetch_l2_snapshot, KrakenSpotRestClient};
use serde_json::Value;
use test_case::test_case;

#[test]
fn test_trades() {
    let text = KrakenSpotRestClient::fetch_trades("XBTUSD", None).unwrap();
    let obj = serde_json::from_str::<HashMap<String, Value>>(&text).unwrap();
    assert!(obj.get("error").unwrap().as_array().unwrap().is_empty());

    let text = KrakenSpotRestClient::fetch_trades("XBT/USD", None).unwrap();
    let obj = serde_json::from_str::<HashMap<String, Value>>(&text).unwrap();
    assert!(obj.get("error").unwrap().as_array().unwrap().is_empty());
}

#[test]
fn test_restful_accepts_two_symbols() {
    let text = fetch_l2_snapshot("kraken", MarketType::Spot, "XBTUSD", Some(3)).unwrap();
    let obj = serde_json::from_str::<HashMap<String, Value>>(&text).unwrap();
    assert!(obj.get("error").unwrap().as_array().unwrap().is_empty());

    let text = fetch_l2_snapshot("kraken", MarketType::Spot, "XBT/USD", Some(3)).unwrap();
    let obj = serde_json::from_str::<HashMap<String, Value>>(&text).unwrap();
    assert!(obj.get("error").unwrap().as_array().unwrap().is_empty());
}

#[test_case(MarketType::Spot, "XBTUSD")]
#[test_case(MarketType::Spot, "XBT/USD")]
#[test_case(MarketType::InverseFuture, "FI_XBTUSD_220930")]
#[test_case(MarketType::InverseSwap, "PI_XBTUSD")]
fn test_l2_snapshot(market_type: MarketType, symbol: &str) {
    let text = fetch_l2_snapshot("kraken", market_type, symbol, None).unwrap();
    let obj = serde_json::from_str::<HashMap<String, Value>>(&text).unwrap();
    if market_type == MarketType::Spot {
        assert!(obj.get("error").unwrap().as_array().unwrap().is_empty());
    } else {
        assert_eq!(obj.get("result").unwrap().as_str().unwrap(), "success");
    }
}
