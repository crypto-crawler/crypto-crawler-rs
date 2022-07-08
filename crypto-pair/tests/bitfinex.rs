use crypto_market_type::MarketType;
use crypto_pair::{get_market_type, normalize_pair};

const EXCHANGE_NAME: &str = "bitfinex";

#[test]
fn verify_spot_symbols() {
    assert_eq!(
        "BTC/USD".to_string(),
        normalize_pair("BTCUSD", EXCHANGE_NAME).unwrap()
    );
    assert_eq!(
        "BTC/USDT".to_string(),
        normalize_pair("BTCUST", EXCHANGE_NAME).unwrap()
    );
    assert_eq!(
        "BTC/USDT".to_string(),
        normalize_pair("tBTCUST", EXCHANGE_NAME).unwrap()
    );

    assert_eq!(
        MarketType::Spot,
        get_market_type("BTCUSD", EXCHANGE_NAME, None)
    );
    assert_eq!(
        MarketType::Spot,
        get_market_type("BTCUST", EXCHANGE_NAME, None)
    );
    assert_eq!(
        MarketType::Spot,
        get_market_type("tBTCUST", EXCHANGE_NAME, None)
    );
}

#[test]
fn verify_linear_swap_symbols() {
    assert_eq!(
        "BTC/USDT".to_string(),
        normalize_pair("BTCF0:USTF0", EXCHANGE_NAME).unwrap()
    );
    assert_eq!(
        "BTC/USDT".to_string(),
        normalize_pair("tBTCF0:USTF0", EXCHANGE_NAME).unwrap()
    );

    assert_eq!(
        MarketType::LinearSwap,
        get_market_type("BTCF0:USTF0", EXCHANGE_NAME, None)
    );
    assert_eq!(
        MarketType::LinearSwap,
        get_market_type("tBTCF0:USTF0", EXCHANGE_NAME, None)
    );
}
