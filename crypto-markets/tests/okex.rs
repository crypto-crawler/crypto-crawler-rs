use crypto_markets::{fetch_symbols, MarketType};

#[test]
fn fetch_spot_symbols() {
    let symbols = fetch_symbols("okex", MarketType::Spot).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        assert!(symbol.contains('-'));
    }
}

#[test]
fn fetch_inverse_future_symbols() {
    let symbols = fetch_symbols("okex", MarketType::InverseFuture).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        let date = &symbol[(symbol.len() - 6)..];
        assert!(date.parse::<i64>().is_ok());

        assert!(symbol.contains("-USD-"));
    }
}

#[test]
fn fetch_linear_future_symbols() {
    let symbols = fetch_symbols("okex", MarketType::LinearFuture).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        let date = &symbol[(symbol.len() - 6)..];
        assert!(date.parse::<i64>().is_ok());

        assert!(symbol.contains("-USDT-"));
    }
}

#[test]
fn fetch_inverse_swap_symbols() {
    let symbols = fetch_symbols("okex", MarketType::InverseSwap).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        assert!(symbol.ends_with("-USD-SWAP"));
    }
}

#[test]
fn fetch_linear_swap_symbols() {
    let symbols = fetch_symbols("okex", MarketType::LinearSwap).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        assert!(symbol.ends_with("-USDT-SWAP"));
    }
}

#[test]
fn fetch_option_symbols() {
    let symbols = fetch_symbols("okex", MarketType::Option).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        assert!(symbol.ends_with("-C") || symbol.ends_with("-P"));
    }
}
