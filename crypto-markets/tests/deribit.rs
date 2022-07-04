use crypto_market_type::MarketType;
use crypto_markets::{fetch_markets, fetch_symbols};
use crypto_pair::get_market_type;
use test_case::test_case;

#[macro_use]
mod utils;

const EXCHANGE_NAME: &str = "deribit";

#[test]
fn fetch_inverse_future_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::InverseFuture).unwrap();
    assert!(!symbols.is_empty());

    for symbol in symbols.iter() {
        let year = &symbol[(symbol.len() - 2)..];
        assert!(year.parse::<i64>().is_ok());

        let date = &symbol[(symbol.len() - 7)..(symbol.len() - 5)];
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
        assert!(symbol.ends_with("-PERPETUAL"));
        assert_eq!(
            MarketType::InverseSwap,
            get_market_type(symbol, EXCHANGE_NAME, None)
        );
    }
}

#[test]
fn fetch_option_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::EuropeanOption).unwrap();
    assert!(!symbols.is_empty());

    for symbol in symbols.iter() {
        let arr: Vec<&str> = symbol.split('-').collect();

        assert!(arr[0] == "BTC" || arr[0] == "ETH");

        let date = &arr[1][..(arr[1].len() - 5)];
        assert!(date.parse::<i64>().is_ok());

        let year = &arr[1][(arr[1].len() - 2)..];
        assert!(year.parse::<i64>().is_ok());

        assert!(arr[2].parse::<i64>().is_ok());
        assert!(arr[3] == "C" || arr[3] == "P");

        assert_eq!(
            MarketType::EuropeanOption,
            get_market_type(symbol, EXCHANGE_NAME, None)
        );
    }
}

#[test]
fn fetch_inverse_future_markets() {
    let markets = fetch_markets(EXCHANGE_NAME, MarketType::InverseFuture).unwrap();
    assert!(!markets.is_empty());

    let btcusd = markets.iter().find(|m| m.base_id == "BTC").unwrap().clone();
    assert_eq!(btcusd.contract_value, Some(10.0));
    assert_eq!(btcusd.precision.tick_size, 0.5);
    assert_eq!(btcusd.precision.lot_size, 10.0);
    let quantity_limit = btcusd.quantity_limit.unwrap();
    assert_eq!(quantity_limit.min.unwrap(), 10.0);
    assert_eq!(quantity_limit.max, None);
}

#[test]
fn fetch_inverse_swap_markets() {
    let markets = fetch_markets(EXCHANGE_NAME, MarketType::InverseSwap).unwrap();
    assert!(!markets.is_empty());

    let btcusd = markets.iter().find(|m| m.base_id == "BTC").unwrap().clone();
    assert_eq!(btcusd.contract_value, Some(10.0));
    assert_eq!(btcusd.precision.tick_size, 0.5);
    assert_eq!(btcusd.precision.lot_size, 10.0);
    let quantity_limit = btcusd.quantity_limit.unwrap();
    assert_eq!(quantity_limit.min.unwrap(), 10.0);
    assert_eq!(quantity_limit.max, None);
}

#[test]
fn fetch_option_markets() {
    let markets = fetch_markets(EXCHANGE_NAME, MarketType::EuropeanOption).unwrap();
    assert!(!markets.is_empty());

    let btcusd = markets.iter().find(|m| m.base_id == "BTC").unwrap().clone();
    assert_eq!(btcusd.contract_value, Some(1.0));
    assert_eq!(btcusd.precision.tick_size, 0.0005);
    assert_eq!(btcusd.precision.lot_size, 0.1);
    let quantity_limit = btcusd.quantity_limit.unwrap();
    assert_eq!(quantity_limit.min.unwrap(), 0.1);
    assert_eq!(quantity_limit.max, None);
}

#[test_case(MarketType::InverseFuture)]
#[test_case(MarketType::InverseSwap)]
#[test_case(MarketType::EuropeanOption)]
fn test_contract_values(market_type: MarketType) {
    check_contract_values!(EXCHANGE_NAME, market_type);
}
