use crypto_markets::MarketType;
use crypto_rest_client::{fetch_l2_snapshot, fetch_l3_snapshot, CoinbaseProRestClient};

#[test]
fn test_trades() {
    let text = CoinbaseProRestClient::fetch_trades("BTC-USD").unwrap();
    assert!(text.starts_with("[{"));
}

#[test]
fn test_l2_snapshot() {
    let text = fetch_l2_snapshot("coinbase_pro", MarketType::Spot, "BTC-USD").unwrap();
    assert!(text.starts_with("{"));
}

#[test]
fn test_l3_snapshot() {
    let text = fetch_l3_snapshot("coinbase_pro", MarketType::Spot, "BTC-USD").unwrap();
    assert!(text.starts_with("{"));
}
