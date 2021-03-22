use crypto_markets::{fetch_symbols, get_market_types, MarketType};

#[macro_use]
mod utils;

const EXCHANGE_NAME: &str = "coinbase_pro";

#[test]
fn fetch_all_symbols() {
    gen_all_symbols!();
}

#[test]
fn fetch_spot_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::Spot).unwrap();
    assert!(!symbols.is_empty());

    for symbol in symbols.iter() {
        assert!(symbol.contains("-"));
        assert_eq!(symbol.to_string(), symbol.to_uppercase());
    }
}
