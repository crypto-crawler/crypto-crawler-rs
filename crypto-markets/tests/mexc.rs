use crypto_market_type::MarketType;
use crypto_markets::{fetch_markets, fetch_symbols};
use crypto_pair::get_market_type;
use test_case::test_case;

#[macro_use]
mod utils;

const EXCHANGE_NAME: &str = "mexc";

#[test]
fn fetch_spot_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::Spot).unwrap();
    assert!(!symbols.is_empty());

    for symbol in symbols.iter() {
        assert!(symbol.contains("_"));
        assert_eq!(symbol.to_uppercase(), symbol.to_string());
        assert_eq!(
            MarketType::Spot,
            get_market_type(symbol, EXCHANGE_NAME, Some(true))
        );
    }
}

#[test]
fn fetch_linear_swap_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::LinearSwap).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        assert!(symbol.ends_with("_USDT"));
        assert_eq!(
            MarketType::LinearSwap,
            get_market_type(symbol, EXCHANGE_NAME, None)
        );
    }
}

#[test]
fn fetch_inverse_swap_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::InverseSwap).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        assert!(symbol.ends_with("_USD"));
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

    let btc_usdt = markets
        .iter()
        .find(|m| m.symbol == "BTC_USDT")
        .unwrap()
        .clone();
    assert_eq!(btc_usdt.market_type, MarketType::Spot);
    assert!(btc_usdt.contract_value.is_none());
    assert_eq!(btc_usdt.precision.tick_size, 0.01);
    assert_eq!(btc_usdt.precision.lot_size, 0.000001);
    let quantity_limit = btc_usdt.quantity_limit.unwrap();
    assert_eq!(quantity_limit.min.unwrap(), 5.0);
    assert_eq!(quantity_limit.max, Some(5000000.0));
}

#[test]
fn fetch_inverse_swap_markets() {
    let markets = fetch_markets(EXCHANGE_NAME, MarketType::InverseSwap).unwrap();
    assert!(!markets.is_empty());

    let btcusd_perp = markets
        .iter()
        .find(|m| m.symbol == "BTC_USD")
        .unwrap()
        .clone();
    assert_eq!(btcusd_perp.market_type, MarketType::InverseSwap);
    assert_eq!(btcusd_perp.contract_value, Some(100.0));
    assert_eq!(btcusd_perp.precision.tick_size, 0.5);
    assert_eq!(btcusd_perp.precision.lot_size, 1.0);
    let quantity_limit = btcusd_perp.quantity_limit.unwrap();
    assert_eq!(quantity_limit.min.unwrap(), 1.0);
    assert_eq!(quantity_limit.max, Some(10000.0));
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
    assert_eq!(btcusdt.contract_value, Some(0.0001));
    assert_eq!(btcusdt.precision.tick_size, 0.5);
    assert_eq!(btcusdt.precision.lot_size, 1.0);
    let quantity_limit = btcusdt.quantity_limit.unwrap();
    assert_eq!(quantity_limit.min.unwrap(), 1.0);
    assert_eq!(quantity_limit.max, Some(1000000.0));
}

#[test_case(MarketType::InverseSwap)]
#[test_case(MarketType::LinearSwap)]
fn test_contract_values(market_type: MarketType) {
    check_contract_values!(EXCHANGE_NAME, market_type);
}
