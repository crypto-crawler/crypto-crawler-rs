use crypto_market_type::{get_market_types, MarketType};
use crypto_markets::{fetch_markets, fetch_symbols};
use crypto_pair::get_market_type;
use test_case::test_case;

#[macro_use]
mod utils;

const EXCHANGE_NAME: &str = "zbg";

#[test]
fn fetch_all_symbols() {
    gen_all_symbols!();
}

#[test]
fn fetch_spot_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::Spot).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        assert!(symbol.contains('_'));
        assert_eq!(symbol.to_string(), symbol.to_lowercase());
        assert_eq!(
            MarketType::Spot,
            get_market_type(symbol, EXCHANGE_NAME, None)
        );
    }
}

#[test]
fn fetch_inverse_swap_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::InverseSwap).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        assert!(symbol.ends_with("_USD-R"));
        assert_eq!(
            MarketType::InverseSwap,
            get_market_type(symbol, EXCHANGE_NAME, None)
        );
    }
}

#[test]
fn fetch_linear_swap_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::LinearSwap).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        assert!(symbol.ends_with("_USDT") || symbol.ends_with("_ZUSD"));
        assert_eq!(
            MarketType::LinearSwap,
            get_market_type(symbol, EXCHANGE_NAME, None)
        );
    }
}

#[test]
fn fetch_spot_markets() {
    let markets = fetch_markets(EXCHANGE_NAME, MarketType::Spot).unwrap();
    assert!(!markets.is_empty());

    let btc_usdt = markets
        .iter()
        .find(|m| m.symbol == "btc_usdt")
        .unwrap()
        .clone();
    assert_eq!(btc_usdt.market_type, MarketType::Spot);
    assert!(btc_usdt.contract_value.is_none());
    assert_eq!(btc_usdt.precision.tick_size, 0.1);
    assert_eq!(btc_usdt.precision.lot_size, 0.000001);
    let quantity_limit = btc_usdt.quantity_limit.unwrap();
    assert_eq!(quantity_limit.min.unwrap(), 0.000001);
    assert!(quantity_limit.max.is_none());
}

#[test]
fn fetch_inverse_swap_markets() {
    let markets = fetch_markets(EXCHANGE_NAME, MarketType::InverseSwap).unwrap();
    assert!(!markets.is_empty());

    let btcusd_perp = markets
        .iter()
        .find(|m| m.symbol == "BTC_USD-R")
        .unwrap()
        .clone();
    assert_eq!(btcusd_perp.market_type, MarketType::InverseSwap);
    assert_eq!(btcusd_perp.contract_value, Some(1.0));
    assert_eq!(btcusd_perp.precision.tick_size, 0.5);
    assert_eq!(btcusd_perp.precision.lot_size, 1.0);
    assert!(btcusd_perp.quantity_limit.is_none());
}

#[test]
fn fetch_linear_swap_markets() {
    let markets = fetch_markets(EXCHANGE_NAME, MarketType::LinearSwap).unwrap();
    assert!(!markets.is_empty());

    let btcusdt = markets
        .iter()
        .find(|m| m.symbol == "BTC_USDT")
        .unwrap()
        .clone();
    assert_eq!(btcusdt.market_type, MarketType::LinearSwap);
    assert_eq!(btcusdt.contract_value, Some(0.01));
    assert_eq!(btcusdt.precision.tick_size, 0.5);
    assert_eq!(btcusdt.precision.lot_size, 1.0);
    assert!(btcusdt.quantity_limit.is_none());
}

#[test_case(MarketType::InverseSwap)]
#[test_case(MarketType::LinearSwap)]
fn test_contract_values(market_type: MarketType) {
    check_contract_values!(EXCHANGE_NAME, market_type);
}
