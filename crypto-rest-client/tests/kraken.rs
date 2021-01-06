use crypto_rest_client::KrakenRestClient;

#[test]
fn test_trades() {
    let text = KrakenRestClient::fetch_trades("XXBTZUSD", None).unwrap();
    assert!(text.starts_with("{"));
}

#[test]
fn test_l2_snapshot() {
    let text = KrakenRestClient::fetch_l2_snapshot("XXBTZUSD").unwrap();
    assert!(text.starts_with("{"));
}
