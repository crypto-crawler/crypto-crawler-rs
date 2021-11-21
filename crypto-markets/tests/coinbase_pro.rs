use crypto_market_type::{get_market_types, MarketType};
use crypto_markets::{fetch_markets, fetch_symbols};

#[macro_use]
mod utils;

const EXCHANGE_NAME: &str = "coinbase_pro";

#[test]
fn fetch_all_symbols() {
    gen_all_symbols!();
}

#[test]
fn fetch_spot_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::Spot).unwrap();
    assert!(!symbols.is_empty());

    for symbol in symbols.iter() {
        assert!(symbol.contains("-"));
        assert_eq!(symbol.to_string(), symbol.to_uppercase());
    }
}

#[test]
fn fetch_spot_markets() {
    let markets = fetch_markets(EXCHANGE_NAME, MarketType::Spot).unwrap();
    assert!(!markets.is_empty());

    let btcusd = markets
        .iter()
        .find(|m| m.symbol == "BTC-USD")
        .unwrap()
        .clone();
    assert_eq!(btcusd.precision.tick_size, 0.01);
    assert_eq!(btcusd.precision.lot_size, 0.00000001);
    let quantity_limit = btcusd.quantity_limit.unwrap();
    assert_eq!(quantity_limit.min, 0.000021);
    assert_eq!(quantity_limit.max, Some(280.0));
}
