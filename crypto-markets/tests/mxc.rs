use crypto_markets::{fetch_symbols, MarketType};

#[test]
#[ignore]
fn fetch_spot_symbols() {
    let symbols = fetch_symbols("mxc", MarketType::Spot).unwrap();
    assert!(!symbols.is_empty());
}

#[test]
fn fetch_linear_swap_symbols() {
    let symbols = fetch_symbols("mxc", MarketType::LinearSwap).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        assert!(symbol.ends_with("_USDT"));
    }
}

#[test]
fn fetch_inverse_swap_symbols() {
    let symbols = fetch_symbols("mxc", MarketType::InverseSwap).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        assert!(symbol.ends_with("_USD"));
    }
}
