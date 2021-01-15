use crypto_markets::{fetch_symbols, MarketType};

#[test]
fn fetch_spot_symbols() {
    let symbols = fetch_symbols("kraken", MarketType::Spot).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        assert!(symbol.contains("/"));
    }
}
