use crypto_markets::MarketType;
use crypto_rest_client::{fetch_l2_snapshot, BinanceFutureRestClient};

#[test]
fn test_agg_trades() {
    let text =
        BinanceFutureRestClient::fetch_agg_trades("BTCUSD_210625", None, None, None).unwrap();
    assert!(text.starts_with("[{"));
}

#[test]
fn test_l2_snapshot() {
    let text = fetch_l2_snapshot("binance", MarketType::InverseFuture, "BTCUSD_210625").unwrap();
    assert!(text.starts_with("{"));
}
