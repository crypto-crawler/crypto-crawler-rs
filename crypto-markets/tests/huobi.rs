use crypto_markets::{fetch_symbols, MarketType};

#[test]
fn fetch_spot_symbols() {
    let symbols = fetch_symbols("huobi", MarketType::Spot).unwrap();
    assert!(!symbols.is_empty());
}

#[test]
fn fetch_inverse_future_symbols() {
    let symbols = fetch_symbols("huobi", MarketType::InverseFuture).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        assert!(
            symbol.ends_with("_CW")
                || symbol.ends_with("_NW")
                || symbol.ends_with("_CQ")
                || symbol.ends_with("_NQ")
        );
    }
}

#[test]
fn fetch_inverse_swap_symbols() {
    let symbols = fetch_symbols("huobi", MarketType::InverseSwap).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        assert!(symbol.ends_with("-USD"));
    }
}

#[test]
fn fetch_linear_swap_symbols() {
    let symbols = fetch_symbols("huobi", MarketType::LinearSwap).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        assert!(symbol.ends_with("-USDT"));
    }
}

#[test]
fn fetch_linear_option_symbols() {
    let symbols = fetch_symbols("huobi", MarketType::LinearOption).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        assert!(symbol.contains("-C-") || symbol.contains("-P-"));
    }
}
