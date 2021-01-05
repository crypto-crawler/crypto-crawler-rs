use crypto_rest_client::BitMEXRestClient;

#[test]
fn test_trades() {
    let text = BitMEXRestClient::fetch_trades("XBTUSD", None).unwrap();
    assert!(text.starts_with("[{"));
    println!("{}", text);
}

#[test]
fn test_l2_snapshot() {
    let text = BitMEXRestClient::fetch_l2_snapshot("XBTUSD").unwrap();
    assert!(text.starts_with("[{"));
    println!("{}", text);
}
