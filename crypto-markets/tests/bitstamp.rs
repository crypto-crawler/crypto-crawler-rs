use crypto_market_type::{get_market_types, MarketType};
use crypto_markets::{fetch_markets, fetch_symbols};

#[macro_use]
mod utils;

const EXCHANGE_NAME: &str = "bitstamp";

#[test]
fn fetch_all_symbols() {
    gen_all_symbols!();
}

#[test]
fn fetch_spot_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::Spot).unwrap();
    assert!(!symbols.is_empty());
}

#[test]
fn fetch_spot_markets() {
    let markets = fetch_markets(EXCHANGE_NAME, MarketType::Spot).unwrap();
    assert!(!markets.is_empty());

    let btcusd = markets
        .iter()
        .find(|m| m.symbol == "btcusd")
        .unwrap()
        .clone();
    assert_eq!(btcusd.precision.tick_size, 0.00000001);
    assert_eq!(btcusd.precision.lot_size, 0.01);
    assert!(btcusd.quantity_limit.is_none());
}
