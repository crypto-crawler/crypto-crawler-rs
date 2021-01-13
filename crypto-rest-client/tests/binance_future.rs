use crypto_rest_client::BinanceFutureRestClient;

#[test]
fn test_fetch_symbols() {
    let symbols = BinanceFutureRestClient::fetch_symbols().unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        let date = &symbol[(symbol.len() - 6)..];
        assert!(date.parse::<i64>().is_ok());

        let quote = &symbol[(symbol.len() - 10)..(symbol.len() - 7)];
        assert_eq!(quote, "USD");
    }
}

#[test]
fn test_agg_trades() {
    let text =
        BinanceFutureRestClient::fetch_agg_trades("BTCUSD_210625", None, None, None).unwrap();
    assert!(text.starts_with("[{"));
}

#[test]
fn test_l2_snapshot() {
    let text = BinanceFutureRestClient::fetch_l2_snapshot("BTCUSD_210625").unwrap();
    assert!(text.starts_with("{"));
}
