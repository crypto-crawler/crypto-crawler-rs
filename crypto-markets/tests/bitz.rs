use crypto_market_type::{get_market_types, MarketType};
use crypto_markets::fetch_symbols;

#[macro_use]
mod utils;

const EXCHANGE_NAME: &str = "bitz";

#[test]
#[ignore = "bitz.com has shutdown since October 2021"]
fn fetch_all_symbols() {
    gen_all_symbols!();
}

#[test]
#[ignore = "bitz.com has shutdown since October 2021"]
fn fetch_spot_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::Spot).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        assert!(symbol.contains('_'));
    }
}

#[test]
#[ignore = "bitz.com has shutdown since October 2021"]
fn fetch_inverse_swap_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::InverseSwap).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        assert!(symbol.ends_with("_USD"));
    }
}

#[test]
#[ignore = "bitz.com has shutdown since October 2021"]
fn fetch_linear_swap_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::LinearSwap).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        assert!(symbol.ends_with("_USDT"));
    }
}
