use crypto_market_type::MarketType;
use crypto_rest_client::{fetch_l2_snapshot, BinanceOptionRestClient};

#[test]
fn test_agg_trades() {
    let text = BinanceOptionRestClient::fetch_trades("BTC-210129-40000-C", None).unwrap();
    assert!(text.starts_with("{"));
}

#[test]
fn test_l2_snapshot() {
    let text = fetch_l2_snapshot("binance", MarketType::Option, "BTC-210129-40000-C").unwrap();
    assert!(text.starts_with("{"));
}
