use crypto_markets::{fetch_symbols, get_market_types, MarketType};

#[macro_use]
mod utils;

const EXCHANGE_NAME: &str = "zbg";

#[test]
fn fetch_all_symbols() {
    gen_all_symbols!();
}

#[test]
fn fetch_spot_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::Spot).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        assert!(symbol.contains('_'));
        assert_eq!(symbol.to_string(), symbol.to_lowercase());
    }
}

#[test]
fn fetch_inverse_swap_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::InverseSwap).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        assert!(symbol.ends_with("_USD-R"));
    }
}

#[test]
fn fetch_linear_swap_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::LinearSwap).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        assert!(symbol.ends_with("_USDT"));
    }
}
