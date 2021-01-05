use crypto_rest_client::CoinbaseProRestClient;

#[test]
fn test_trades() {
    let text = CoinbaseProRestClient::fetch_trades("BTC-USD").unwrap();
    assert!(text.starts_with("[{"));
}

#[test]
fn test_l2_snapshot() {
    let text = CoinbaseProRestClient::fetch_l2_snapshot("BTC-USD").unwrap();
    assert!(text.starts_with("{"));
}

#[test]
fn test_l3_snapshot() {
    let text = CoinbaseProRestClient::fetch_l3_snapshot("BTC-USD").unwrap();
    assert!(text.starts_with("{"));
}
