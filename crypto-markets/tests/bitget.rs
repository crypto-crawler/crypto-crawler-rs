use crypto_markets::{fetch_markets, fetch_symbols, get_market_types, MarketType};
use test_case::test_case;

#[macro_use]
mod utils;

const EXCHANGE_NAME: &str = "bitget";

#[test]
fn fetch_all_symbols() {
    gen_all_symbols!();
}

#[test]
fn fetch_spot_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::Spot).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        assert!(symbol.ends_with("_USDT") || symbol.ends_with("_BTC"));
    }
}

#[test]
fn fetch_inverse_swap_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::InverseSwap).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        assert!(symbol.ends_with("usd"));
    }
}

#[test]
fn fetch_linear_swap_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::LinearSwap).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        assert!(symbol.starts_with("cmt_"));
        assert!(symbol.ends_with("usdt"));
    }
}

#[test]
fn fetch_spot_markets() {
    let markets = fetch_markets(EXCHANGE_NAME, MarketType::Spot).unwrap();
    assert!(!markets.is_empty());

    let btcusdt = markets
        .iter()
        .find(|m| m.symbol == "BTC_USDT")
        .unwrap()
        .clone();
    assert_eq!(btcusdt.precision.tick_size, 0.01);
    assert_eq!(btcusdt.precision.lot_size, 0.0001);
    assert!(btcusdt.quantity_limit.is_none());
}

#[test]
fn fetch_inverse_swap_markets() {
    let markets = fetch_markets(EXCHANGE_NAME, MarketType::InverseSwap).unwrap();
    assert!(!markets.is_empty());

    let btcusd = markets
        .iter()
        .find(|m| m.symbol == "btcusd")
        .unwrap()
        .clone();
    assert_eq!(btcusd.contract_value, Some(1.0));
    assert_eq!(btcusd.precision.tick_size, 0.1);
    assert_eq!(btcusd.precision.lot_size, 1.0);
    assert!(btcusd.quantity_limit.is_none());
}

#[test]
fn fetch_linear_swap_markets() {
    let markets = fetch_markets(EXCHANGE_NAME, MarketType::LinearSwap).unwrap();
    assert!(!markets.is_empty());

    let btcusdt = markets
        .iter()
        .find(|m| m.symbol == "cmt_btcusdt")
        .unwrap()
        .clone();
    assert_eq!(btcusdt.contract_value, Some(0.001));
    assert_eq!(btcusdt.precision.tick_size, 0.1);
    assert_eq!(btcusdt.precision.lot_size, 1.0);
    assert!(btcusdt.quantity_limit.is_none());
}

#[test_case(MarketType::InverseSwap)]
#[test_case(MarketType::LinearSwap)]
fn test_contract_values(market_type: MarketType) {
    check_contract_values!(EXCHANGE_NAME, market_type);
}
