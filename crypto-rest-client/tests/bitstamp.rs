use crypto_market_type::MarketType;
use crypto_rest_client::{fetch_l2_snapshot, fetch_l3_snapshot, BitstampRestClient};

#[test]
fn test_trades() {
    let text = BitstampRestClient::fetch_trades("btcusd", Some("minute".to_string())).unwrap();
    assert!(text.starts_with("[{"));
}

#[test]
fn test_l2_snapshot() {
    let text = fetch_l2_snapshot("bitstamp", MarketType::Spot, "btcusd", Some(3)).unwrap();
    assert!(text.starts_with('{'));
}

#[test]
fn test_l3_snapshot() {
    let text = fetch_l3_snapshot("bitstamp", MarketType::Spot, "btcusd", Some(3)).unwrap();
    assert!(text.starts_with('{'));
}
