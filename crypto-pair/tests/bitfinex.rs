use crypto_pair::normalize_pair;

const EXCHANGE_NAME: &'static str = "bitfinex";

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
}
