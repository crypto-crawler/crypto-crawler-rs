use crypto_rest_client::BitstampRestClient;

#[test]
fn test_trades() {
    let text = BitstampRestClient::fetch_trades("btcusd", Some("minute".to_string())).unwrap();
    assert!(text.starts_with("[{"));
}

#[test]
fn test_l2_snapshot() {
    let text = BitstampRestClient::fetch_l2_snapshot("btcusd").unwrap();
    assert!(text.starts_with("{"));
}

#[test]
fn test_l3_snapshot() {
    let text = BitstampRestClient::fetch_l3_snapshot("btcusd").unwrap();
    assert!(text.starts_with("{"));
}
