use crypto_market_type::{get_market_types, MarketType};
use crypto_markets::{fetch_markets, fetch_symbols};
use test_case::test_case;

#[macro_use]
mod utils;

const EXCHANGE_NAME: &str = "gate";

#[test]
fn fetch_all_symbols() {
    gen_all_symbols!();
}

#[test]
fn fetch_spot_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::Spot).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        if symbol.ends_with("_USD") {
            println!("{}", symbol);
        }

        assert!(symbol.contains('_'));
        assert_eq!(symbol.to_string(), symbol.to_uppercase());
    }
}

#[test]
fn fetch_inverse_swap_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::InverseSwap).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        assert!(symbol.ends_with("_USD"));
    }
}

#[test]
fn fetch_linear_swap_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::LinearSwap).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        assert!(symbol.ends_with("_USDT"));
    }
}

#[test]
fn fetch_inverse_future_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::InverseFuture).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        let date = &symbol[(symbol.len() - 8)..];
        assert!(date.parse::<i64>().is_ok());
        assert!(symbol.contains("_USD_"));
    }
}

#[test]
fn fetch_linear_future_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::LinearFuture).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        let date = &symbol[(symbol.len() - 8)..];
        assert!(date.parse::<i64>().is_ok());
        assert!(symbol.contains("_USDT_"));
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
    assert_eq!(btc_usdt.precision.lot_size, 0.0001);
    let quantity_limit = btc_usdt.quantity_limit.unwrap();
    assert_eq!(quantity_limit.min.unwrap(), 0.0001);
    assert!(quantity_limit.max.is_none());
}

#[test]
fn fetch_inverse_swap_markets() {
    let markets = fetch_markets(EXCHANGE_NAME, MarketType::InverseSwap).unwrap();
    assert!(!markets.is_empty());

    let btc_usd = markets
        .iter()
        .find(|m| m.symbol == "BTC_USD")
        .unwrap()
        .clone();
    assert_eq!(btc_usd.market_type, MarketType::InverseSwap);
    assert_eq!(btc_usd.contract_value, Some(1.0));
    assert_eq!(btc_usd.precision.tick_size, 0.1);
    assert_eq!(btc_usd.precision.lot_size, 1.0);
    let quantity_limit = btc_usd.quantity_limit.unwrap();
    assert_eq!(quantity_limit.min.unwrap(), 1.0);
    assert_eq!(quantity_limit.max, Some(1000000.0));
}

#[test]
fn fetch_linear_swap_markets() {
    let markets = fetch_markets(EXCHANGE_NAME, MarketType::LinearSwap).unwrap();
    assert!(!markets.is_empty());

    let btc_usdt = markets
        .iter()
        .find(|m| m.symbol == "BTC_USDT")
        .unwrap()
        .clone();
    assert_eq!(btc_usdt.market_type, MarketType::LinearSwap);
    assert_eq!(btc_usdt.contract_value, Some(0.0001));
    assert_eq!(btc_usdt.precision.tick_size, 0.1);
    assert_eq!(btc_usdt.precision.lot_size, 0.0001);
    let quantity_limit = btc_usdt.quantity_limit.unwrap();
    assert_eq!(quantity_limit.min.unwrap(), 1.0);
    assert_eq!(quantity_limit.max, Some(1000000.0));
}

#[test]
fn fetch_inverse_future_markets() {
    let markets = fetch_markets(EXCHANGE_NAME, MarketType::InverseFuture).unwrap();
    assert!(!markets.is_empty());

    let btc_usd = markets
        .iter()
        .find(|m| m.symbol.starts_with("BTC_USD_"))
        .unwrap()
        .clone();
    assert_eq!(btc_usd.market_type, MarketType::InverseFuture);
    assert_eq!(btc_usd.contract_value, Some(1.0));
    assert!(btc_usd.delivery_date.is_some());
    assert_eq!(btc_usd.precision.tick_size, 0.1);
    assert_eq!(btc_usd.precision.lot_size, 1.0);
    let quantity_limit = btc_usd.quantity_limit.unwrap();
    assert_eq!(quantity_limit.min.unwrap(), 1.0);
    assert_eq!(quantity_limit.max, Some(1000000.0));
}

#[test]
fn fetch_linear_future_markets() {
    let markets = fetch_markets(EXCHANGE_NAME, MarketType::LinearFuture).unwrap();
    assert!(!markets.is_empty());

    let btc_usdt = markets
        .iter()
        .find(|m| m.symbol.starts_with("BTC_USDT_"))
        .unwrap()
        .clone();
    assert_eq!(btc_usdt.market_type, MarketType::LinearFuture);
    assert_eq!(btc_usdt.contract_value, Some(0.0001));
    assert!(btc_usdt.delivery_date.is_some());
    assert_eq!(btc_usdt.precision.tick_size, 0.1);
    assert_eq!(btc_usdt.precision.lot_size, 0.0001);
    let quantity_limit = btc_usdt.quantity_limit.unwrap();
    assert_eq!(quantity_limit.min.unwrap(), 1.0);
    assert_eq!(quantity_limit.max, Some(1000000.0));
}

#[test_case(MarketType::LinearFuture)]
#[test_case(MarketType::LinearSwap)]
// TODO: ETH_USD is actually a quanto swap contract
// #[test_case(MarketType::InverseFuture)]
// #[test_case(MarketType::InverseSwap)]
fn test_contract_values(market_type: MarketType) {
    check_contract_values!(EXCHANGE_NAME, market_type);
}
