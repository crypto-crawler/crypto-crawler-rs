use crypto_market_type::MarketType;
use crypto_rest_client::{fetch_l2_snapshot, BinanceInverseSwapRestClient};

#[test]
fn test_agg_trades() {
    let text =
        BinanceInverseSwapRestClient::fetch_agg_trades("BTCUSD_PERP", None, None, None).unwrap();
    assert!(text.starts_with("[{"));
}

#[test]
fn test_l2_snapshot() {
    let text = fetch_l2_snapshot("binance", MarketType::InverseSwap, "BTCUSD_PERP").unwrap();
    assert!(text.starts_with("{"));
}
