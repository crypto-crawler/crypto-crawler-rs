use crypto_rest_client::BinanceOptionRestClient;

#[test]
fn test_fetch_symbols() {
    let symbols = BinanceOptionRestClient::fetch_symbols().unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        assert!(symbol.ends_with("-P") || symbol.ends_with("-C"));
    }
}

#[test]
fn test_agg_trades() {
    let text = BinanceOptionRestClient::fetch_trades("BTC-210129-40000-C", None).unwrap();
    assert!(text.starts_with("{"));
}

#[test]
fn test_l2_snapshot() {
    let text = BinanceOptionRestClient::fetch_l2_snapshot("BTC-210129-40000-C").unwrap();
    assert!(text.starts_with("{"));
}
