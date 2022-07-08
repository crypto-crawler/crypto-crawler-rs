use crypto_market_type::MarketType;
use crypto_rest_client::{fetch_l2_snapshot, BinanceOptionRestClient};

#[ignore]
#[test]
fn test_agg_trades() {
    let text = BinanceOptionRestClient::fetch_trades("BTC-220610-30000-C", None).unwrap();
    assert!(text.starts_with("{"));
}

#[ignore]
#[test]
fn test_l2_snapshot() {
    let text = fetch_l2_snapshot(
        "binance",
        MarketType::EuropeanOption,
        "BTC-220610-30000-C",
        Some(3),
    )
    .unwrap();
    assert!(text.starts_with("{"));
}
