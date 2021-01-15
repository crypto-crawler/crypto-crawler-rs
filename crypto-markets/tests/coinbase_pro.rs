use crypto_markets::{fetch_symbols, MarketType};

#[test]
fn fetch_spot_symbols() {
    let symbols = fetch_symbols("coinbase_pro", MarketType::Spot).unwrap();
    assert!(!symbols.is_empty());
}
