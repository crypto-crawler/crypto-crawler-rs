use crypto_market_type::MarketType;
use crypto_pair::{get_market_type, normalize_pair};

const EXCHANGE_NAME: &'static str = "bithumb";

#[test]
fn verify_spot_symbols() {
    assert_eq!(
        "ETH/USDT".to_string(),
        normalize_pair("ETH-USDT", EXCHANGE_NAME).unwrap()
    );
    assert_eq!(
        MarketType::Spot,
        get_market_type("ETH-USDT", EXCHANGE_NAME, None)
    );
}
