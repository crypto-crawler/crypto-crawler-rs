use crypto_market_type::{get_market_types, MarketType};
use crypto_markets::{fetch_markets, fetch_symbols};
use crypto_pair::get_market_type;

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
        assert_eq!(
            MarketType::Spot,
            get_market_type(symbol, EXCHANGE_NAME, None)
        );
    }
}

#[test]
fn fetch_inverse_future_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::InverseFuture).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        assert!(symbol.starts_with("FI_"));
        let date = &symbol[(symbol.len() - 6)..];
        assert!(date.parse::<i64>().is_ok());
        assert_eq!(
            MarketType::InverseFuture,
            get_market_type(symbol, EXCHANGE_NAME, None)
        );
    }
}

#[test]
fn fetch_inverse_swap_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::InverseSwap).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        assert!(symbol.starts_with("PI_"));
        assert!(symbol.ends_with("USD"));
        assert_eq!(
            MarketType::InverseSwap,
            get_market_type(symbol, EXCHANGE_NAME, None)
        );
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
    assert_eq!(quantity_limit.min.unwrap(), 0.0001);
    assert_eq!(quantity_limit.max, None);
}

#[test]
fn fetch_inverse_future_markets() {
    let markets = fetch_markets(EXCHANGE_NAME, MarketType::InverseFuture).unwrap();
    assert!(!markets.is_empty());

    let btcusd = markets
        .iter()
        .find(|m| m.symbol.starts_with("fi_xbtusd_"))
        .unwrap()
        .clone();
    assert_eq!(btcusd.precision.tick_size, 0.5);
    assert_eq!(btcusd.precision.lot_size, 1.0);
    let quantity_limit = btcusd.quantity_limit.unwrap();
    assert_eq!(quantity_limit.min.unwrap(), 1.0);
    assert_eq!(quantity_limit.max, None);
}

#[test]
fn fetch_inverse_swap_markets() {
    let markets = fetch_markets(EXCHANGE_NAME, MarketType::InverseSwap).unwrap();
    assert!(!markets.is_empty());

    let btcusd = markets
        .iter()
        .find(|m| m.symbol == "pi_xbtusd")
        .unwrap()
        .clone();
    assert_eq!(btcusd.precision.tick_size, 0.5);
    assert_eq!(btcusd.precision.lot_size, 1.0);
    let quantity_limit = btcusd.quantity_limit.unwrap();
    assert_eq!(quantity_limit.min.unwrap(), 1.0);
    assert_eq!(quantity_limit.max, None);
}
