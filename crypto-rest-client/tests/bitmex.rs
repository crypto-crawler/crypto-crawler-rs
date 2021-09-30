use crypto_market_type::MarketType;
use crypto_rest_client::{fetch_l2_snapshot, BitmexRestClient};

#[test]
fn test_trades() {
    let text = BitmexRestClient::fetch_trades("XBTUSD", None).unwrap();
    assert!(text.starts_with("[{"));
}

#[test]
fn test_l2_snapshot() {
    let text = fetch_l2_snapshot("bitmex", MarketType::InverseSwap, "XBTUSD", Some(3)).unwrap();
    assert!(text.starts_with("[{"));
}
