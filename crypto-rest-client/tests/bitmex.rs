use crypto_rest_client::BitmexRestClient;

#[test]
fn test_trades() {
    let text = BitmexRestClient::fetch_trades("XBTUSD", None).unwrap();
    assert!(text.starts_with("[{"));
}

#[test]
fn test_l2_snapshot() {
    let text = BitmexRestClient::fetch_l2_snapshot("XBTUSD").unwrap();
    assert!(text.starts_with("[{"));
}
