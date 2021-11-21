use crypto_market_type::{get_market_types, MarketType};
use crypto_markets::{fetch_markets, fetch_symbols};

#[macro_use]
mod utils;

const EXCHANGE_NAME: &str = "kraken";

#[test]
fn fetch_all_symbols() {
    gen_all_symbols!();
}

#[test]
fn fetch_spot_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::Spot).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        assert!(symbol.contains("/"));
    }
}

#[test]
fn fetch_spot_markets() {
    let markets = fetch_markets(EXCHANGE_NAME, MarketType::Spot).unwrap();
    assert!(!markets.is_empty());

    let btcusd = markets
        .iter()
        .find(|m| m.symbol == "XBT/USD")
        .unwrap()
        .clone();
    assert_eq!(btcusd.precision.tick_size, 0.1);
    assert_eq!(btcusd.precision.lot_size, 0.00000001);
    let quantity_limit = btcusd.quantity_limit.unwrap();
    assert_eq!(quantity_limit.min, 0.0001);
    assert_eq!(quantity_limit.max, None);
}
