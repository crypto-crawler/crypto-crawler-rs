use crypto_market_type::{get_market_types, MarketType};
use crypto_markets::{fetch_markets, fetch_symbols};
use crypto_pair::get_market_type;
// use test_case::test_case;

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
        assert!(
            symbol.ends_with("USDT_SPBL")
                || symbol.ends_with("BTC_SPBL")
                || symbol.ends_with("ETH_SPBL")
        );
        assert_eq!(
            MarketType::Spot,
            get_market_type(&symbol, EXCHANGE_NAME, None)
        );
    }
}

#[test]
fn fetch_inverse_swap_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::InverseSwap).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        assert!(symbol.ends_with("USD_DMCBL"));
        assert_eq!(
            MarketType::InverseSwap,
            get_market_type(&symbol, EXCHANGE_NAME, None)
        );
    }
}

#[test]
fn fetch_linear_swap_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::LinearSwap).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        assert!(symbol.ends_with("USDT_UMCBL"));
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

    let btcusdt = markets
        .iter()
        .find(|m| m.symbol == "BTCUSDT_SPBL")
        .unwrap()
        .clone();
    assert_eq!(btcusdt.precision.tick_size, 0.01);
    assert_eq!(btcusdt.precision.lot_size, 0.0001);
    let quantity_limit = btcusdt.quantity_limit.unwrap();
    assert_eq!(0.0001, quantity_limit.min.unwrap());
    assert!(quantity_limit.max.is_none());
}

#[test]
fn fetch_inverse_swap_markets() {
    let markets = fetch_markets(EXCHANGE_NAME, MarketType::InverseSwap).unwrap();
    assert!(!markets.is_empty());

    let btcusd = markets
        .iter()
        .find(|m| m.symbol == "BTCUSD_DMCBL")
        .unwrap()
        .clone();
    assert_eq!(btcusd.contract_value, Some(1.0));
    assert_eq!(btcusd.precision.tick_size, 0.1);
    assert_eq!(btcusd.precision.lot_size, 0.001);
    let quantity_limit = btcusd.quantity_limit.unwrap();
    assert_eq!(0.001, quantity_limit.min.unwrap());
    assert!(quantity_limit.max.is_none());
}

#[test]
fn fetch_linear_swap_markets() {
    let markets = fetch_markets(EXCHANGE_NAME, MarketType::LinearSwap).unwrap();
    assert!(!markets.is_empty());

    let btcusdt = markets
        .iter()
        .find(|m| m.symbol == "BTCUSDT_UMCBL")
        .unwrap()
        .clone();
    assert_eq!(btcusdt.contract_value, Some(1.0));
    assert_eq!(btcusdt.precision.tick_size, 0.1);
    assert_eq!(btcusdt.precision.lot_size, 0.001);
    let quantity_limit = btcusdt.quantity_limit.unwrap();
    assert_eq!(0.001, quantity_limit.min.unwrap());
    assert!(quantity_limit.max.is_none());
}

// #[test_case(MarketType::InverseSwap)]
// #[test_case(MarketType::LinearSwap)]
// fn test_contract_values(market_type: MarketType) {
//     check_contract_values!(EXCHANGE_NAME, market_type);
// }
