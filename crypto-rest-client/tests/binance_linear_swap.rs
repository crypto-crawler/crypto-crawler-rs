use crypto_markets::MarketType;
use crypto_rest_client::{fetch_l2_snapshot, BinanceLinearSwapRestClient};

#[test]
fn test_agg_trades() {
    let text = BinanceLinearSwapRestClient::fetch_agg_trades("BTCUSDT", None, None, None).unwrap();
    assert!(text.starts_with("[{"));
}

#[test]
fn test_l2_snapshot() {
    let text = fetch_l2_snapshot("binance", MarketType::LinearSwap, "BTCUSDT").unwrap();
    assert!(text.starts_with("{"));
}
