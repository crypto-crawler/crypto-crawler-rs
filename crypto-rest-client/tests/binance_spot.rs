use crypto_market_type::MarketType;
use crypto_rest_client::{fetch_l2_snapshot, BinanceSpotRestClient};

#[test]
fn test_agg_trades() {
    let text = BinanceSpotRestClient::fetch_agg_trades("BTCUSDT", None, None, None).unwrap();
    assert!(text.starts_with("[{"));
}

#[test]
fn test_l2_snapshot() {
    let text = fetch_l2_snapshot("binance", MarketType::Spot, "BTCUSDT", Some(3)).unwrap();
    assert!(text.starts_with('{'));
}
