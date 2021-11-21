use crypto_market_type::{get_market_types, MarketType};
use crypto_markets::{fetch_markets, fetch_symbols};
use test_case::test_case;

#[macro_use]
mod utils;

const EXCHANGE_NAME: &str = "bybit";

#[test]
fn fetch_all_symbols() {
    gen_all_symbols!();
}

#[test]
fn fetch_inverse_swap_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::InverseSwap).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        assert!(symbol.ends_with("USD"));
    }
}

#[test]
fn fetch_linear_swap_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::LinearSwap).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        assert!(symbol.ends_with("USDT"));
    }
}

#[test]
fn fetch_inverse_future_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::InverseFuture).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        let date = &symbol[(symbol.len() - 2)..];
        assert!(date.parse::<i64>().is_ok());
    }
}

#[test]
fn fetch_inverse_swap_markets() {
    let markets = fetch_markets(EXCHANGE_NAME, MarketType::InverseSwap).unwrap();
    assert!(!markets.is_empty());

    let btcusd = markets
        .iter()
        .find(|m| m.symbol == "BTCUSD")
        .unwrap()
        .clone();
    assert_eq!(btcusd.precision.tick_size, 0.5);
    assert_eq!(btcusd.precision.lot_size, 1.0);
    let quantity_limit = btcusd.quantity_limit.unwrap();
    assert_eq!(quantity_limit.min, 1.0);
    assert_eq!(quantity_limit.max, Some(1000000.0));
}

#[test]
fn fetch_linear_swap_markets() {
    let markets = fetch_markets(EXCHANGE_NAME, MarketType::LinearSwap).unwrap();
    assert!(!markets.is_empty());

    let btcusdt = markets
        .iter()
        .find(|m| m.symbol == "BTCUSDT")
        .unwrap()
        .clone();
    assert_eq!(btcusdt.precision.tick_size, 0.5);
    assert_eq!(btcusdt.precision.lot_size, 0.001);
    let quantity_limit = btcusdt.quantity_limit.unwrap();
    assert_eq!(quantity_limit.min, 0.001);
    assert_eq!(quantity_limit.max, Some(100.0));
}

#[test]
fn fetch_inverse_future_markets() {
    let markets = fetch_markets(EXCHANGE_NAME, MarketType::InverseFuture).unwrap();
    assert!(!markets.is_empty());

    let btcusd = markets
        .iter()
        .find(|m| m.symbol == "BTCUSDZ21")
        .unwrap()
        .clone();
    assert_eq!(btcusd.precision.tick_size, 0.5);
    assert_eq!(btcusd.precision.lot_size, 1.0);
    let quantity_limit = btcusd.quantity_limit.unwrap();
    assert_eq!(quantity_limit.min, 1.0);
    assert_eq!(quantity_limit.max, Some(1000000.0));
}

#[test_case(MarketType::InverseFuture)]
#[test_case(MarketType::InverseSwap)]
#[test_case(MarketType::LinearSwap)]
fn test_contract_values(market_type: MarketType) {
    check_contract_values!(EXCHANGE_NAME, market_type);
}
