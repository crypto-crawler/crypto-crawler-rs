use crypto_markets::{fetch_symbols, MarketType};

#[test]
fn fetch_spot_symbols() {
    let symbols = fetch_symbols("bitfinex", MarketType::Spot).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        assert!(symbol.starts_with('t'));
    }
}

#[test]
fn fetch_linear_swap_symbols() {
    let symbols = fetch_symbols("bitfinex", MarketType::LinearSwap).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        assert!(symbol.starts_with('t'));
        assert!(symbol.ends_with("F0:USTF0") || symbol.ends_with("F0:BTCF0"));
    }
}
