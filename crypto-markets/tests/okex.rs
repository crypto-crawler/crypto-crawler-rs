use crypto_markets::{fetch_symbols, get_market_types, MarketType};

#[macro_use]
mod utils;

const EXCHANGE_NAME: &str = "okex";

#[test]
fn fetch_all_symbols() {
    gen_all_symbols!();
}

#[test]
fn fetch_spot_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::Spot).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        assert!(symbol.contains('-'));
    }
}

#[test]
fn fetch_inverse_future_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::InverseFuture).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        let date = &symbol[(symbol.len() - 6)..];
        assert!(date.parse::<i64>().is_ok());

        assert!(symbol.contains("-USD-"));
    }
}

#[test]
fn fetch_linear_future_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::LinearFuture).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        let date = &symbol[(symbol.len() - 6)..];
        assert!(date.parse::<i64>().is_ok());

        assert!(symbol.contains("-USDT-"));
    }
}

#[test]
fn fetch_inverse_swap_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::InverseSwap).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        assert!(symbol.ends_with("-USD-SWAP"));
    }
}

#[test]
fn fetch_linear_swap_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::LinearSwap).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        assert!(symbol.ends_with("-USDT-SWAP"));
    }
}

#[test]
fn fetch_option_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::Option).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        assert!(symbol.ends_with("-C") || symbol.ends_with("-P"));
    }
}
