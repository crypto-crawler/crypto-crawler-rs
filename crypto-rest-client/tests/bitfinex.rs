use crypto_market_type::MarketType;
use crypto_rest_client::{fetch_l2_snapshot, fetch_l3_snapshot, BitfinexRestClient};

#[test]
fn test_trades() {
    let text = BitfinexRestClient::fetch_trades("tBTCUSD", None, None, None, None).unwrap();
    assert!(text.starts_with("[["));
}

#[test]
fn test_l2_snapshot() {
    let text = fetch_l2_snapshot("bitfinex", MarketType::Spot, "tBTCUSD", Some(3)).unwrap();
    assert!(text.starts_with("[["));
}

#[test]
fn test_l3_snapshot() {
    let text = fetch_l3_snapshot("bitfinex", MarketType::Spot, "tBTCUSD", Some(3)).unwrap();
    assert!(text.starts_with("[["));
}
