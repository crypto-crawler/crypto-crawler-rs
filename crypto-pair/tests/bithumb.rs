use crypto_pair::normalize_pair;

const EXCHANGE_NAME: &'static str = "bithumb";

#[test]
fn verify_spot_symbols() {
    assert_eq!(
        "ETH/USDT".to_string(),
        normalize_pair("ETH-USDT", EXCHANGE_NAME).unwrap()
    );
}
