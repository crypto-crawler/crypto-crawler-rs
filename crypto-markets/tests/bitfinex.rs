use crypto_market_type::{get_market_types, MarketType};
use crypto_markets::{fetch_markets, fetch_symbols};
use crypto_pair::get_market_type;
use test_case::test_case;

#[macro_use]
mod utils;

const EXCHANGE_NAME: &str = "bitfinex";

#[test]
fn fetch_all_symbols() {
    gen_all_symbols!();
}

#[test]
fn fetch_spot_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::Spot).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        assert!(symbol.starts_with('t'));
        assert_eq!(
            MarketType::Spot,
            get_market_type(&symbol, EXCHANGE_NAME, None)
        );
    }
}

#[test]
fn fetch_linear_swap_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::LinearSwap).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        assert!(symbol.starts_with('t'));
        assert!(
            symbol.ends_with("F0:USTF0")
                || symbol.ends_with("F0:BTCF0")
                || symbol.ends_with("F0:EUTF0")
        );
        assert_eq!(
            MarketType::LinearSwap,
            get_market_type(&symbol, EXCHANGE_NAME, None)
        );
    }
}

#[test]
fn fetch_spot_markets() {
    let markets = fetch_markets(EXCHANGE_NAME, MarketType::Spot).unwrap();
    assert!(!markets.is_empty());

    let btc_usdt = markets
        .iter()
        .find(|m| m.symbol == "tBTCUST")
        .unwrap()
        .clone();
    assert_eq!(btc_usdt.precision.tick_size, 0.00001);
    assert_eq!(btc_usdt.precision.lot_size, 0.00000001);
    let quantity_limit = btc_usdt.quantity_limit.unwrap();
    assert_eq!(quantity_limit.min.unwrap(), 0.00006);
    assert_eq!(quantity_limit.max, Some(2000.0));
}

#[test]
fn fetch_linear_swap_markets() {
    let markets = fetch_markets(EXCHANGE_NAME, MarketType::LinearSwap).unwrap();
    assert!(!markets.is_empty());

    let btc_usdt = markets
        .iter()
        .find(|m| m.symbol == "tBTCF0:USTF0")
        .unwrap()
        .clone();
    assert_eq!(btc_usdt.precision.tick_size, 0.00001);
    assert_eq!(btc_usdt.precision.lot_size, 0.00000001);
    let quantity_limit = btc_usdt.quantity_limit.unwrap();
    assert_eq!(quantity_limit.min.unwrap(), 0.00006);
    assert_eq!(quantity_limit.max, Some(100.0));
}

#[test_case(MarketType::LinearSwap)]
fn test_contract_values(market_type: MarketType) {
    check_contract_values!(EXCHANGE_NAME, market_type);
}
