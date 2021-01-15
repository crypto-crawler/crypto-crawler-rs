use crypto_rest_client::BinanceInverseSwapRestClient;

#[test]
fn test_agg_trades() {
    let text =
        BinanceInverseSwapRestClient::fetch_agg_trades("BTCUSD_PERP", None, None, None).unwrap();
    assert!(text.starts_with("[{"));
}

#[test]
fn test_l2_snapshot() {
    let text = BinanceInverseSwapRestClient::fetch_l2_snapshot("BTCUSD_PERP").unwrap();
    assert!(text.starts_with("{"));
}
