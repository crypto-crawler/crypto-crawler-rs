use crypto_rest_client::BinanceSpotRestClient;

#[test]
fn test_fetch_symbols() {
    let symbols = BinanceSpotRestClient::fetch_symbols().unwrap();
    assert!(!symbols.is_empty());
}

#[test]
fn test_agg_trades() {
    let text = BinanceSpotRestClient::fetch_agg_trades("BTCUSDT", None, None, None).unwrap();
    assert!(text.starts_with("[{"));
}

#[test]
fn test_l2_snapshot() {
    let text = BinanceSpotRestClient::fetch_l2_snapshot("BTCUSDT").unwrap();
    assert!(text.starts_with("{"));
}
