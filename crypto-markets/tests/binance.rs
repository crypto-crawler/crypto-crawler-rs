use crypto_market_type::{get_market_types, MarketType};
use crypto_markets::{fetch_markets, fetch_symbols};
use crypto_pair::get_market_type;
use test_case::test_case;

#[macro_use]
mod utils;

const EXCHANGE_NAME: &str = "binance";

#[test]
fn fetch_all_symbols() {
    gen_all_symbols!();
}

#[test]
fn fetch_spot_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::Spot).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        assert_eq!(symbol.to_uppercase(), symbol.to_string());
        assert_eq!(
            MarketType::Spot,
            get_market_type(&symbol, EXCHANGE_NAME, Some(true))
        );
    }
}

#[test]
fn fetch_inverse_future_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::InverseFuture).unwrap();
    assert!(!symbols.is_empty());

    for symbol in symbols.iter() {
        let date = &symbol[(symbol.len() - 6)..];
        assert!(date.parse::<i64>().is_ok());

        let quote = &symbol[(symbol.len() - 10)..(symbol.len() - 7)];
        assert_eq!(quote, "USD");
        assert_eq!(
            MarketType::InverseFuture,
            get_market_type(&symbol, EXCHANGE_NAME, None)
        );
    }
}

#[test]
fn fetch_linear_future_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::LinearFuture).unwrap();
    assert!(!symbols.is_empty());

    for symbol in symbols.iter() {
        let date = &symbol[(symbol.len() - 6)..];
        assert!(date.parse::<i64>().is_ok());

        let quote = &symbol[(symbol.len() - 11)..(symbol.len() - 7)];
        assert_eq!(quote, "USDT");
        assert_eq!(
            MarketType::LinearFuture,
            get_market_type(&symbol, EXCHANGE_NAME, None)
        );
    }
}

#[test]
fn fetch_inverse_swap_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::InverseSwap).unwrap();
    assert!(!symbols.is_empty());

    for symbol in symbols.iter() {
        assert!(symbol.ends_with("USD_PERP"));
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
        assert!(symbol.ends_with("USDT") || symbol.ends_with("BUSD"));
        assert_eq!(
            MarketType::LinearSwap,
            get_market_type(&symbol, EXCHANGE_NAME, None)
        );
    }
}

#[ignore]
#[test]
fn fetch_option_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::EuropeanOption).unwrap();
    assert!(!symbols.is_empty());

    for symbol in symbols.iter() {
        assert!(symbol.ends_with("-P") || symbol.ends_with("-C"));
        assert_eq!(
            MarketType::EuropeanOption,
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
        .find(|m| m.symbol == "BTCUSDT")
        .unwrap()
        .clone();
    assert!(btc_usdt.contract_value.is_none());
    assert_eq!(btc_usdt.precision.tick_size, 0.01);
    assert_eq!(btc_usdt.precision.lot_size, 0.00001);
    let quantity_limit = btc_usdt.quantity_limit.unwrap();
    assert_eq!(quantity_limit.min.unwrap(), 0.00001);
    assert_eq!(quantity_limit.max, Some(9000.0));
}

#[test]
fn fetch_inverse_future_markets() {
    let markets = fetch_markets(EXCHANGE_NAME, MarketType::InverseFuture).unwrap();
    assert!(!markets.is_empty());

    let btcusd = markets
        .iter()
        .find(|m| m.symbol.starts_with("BTCUSD_"))
        .unwrap()
        .clone();
    assert_eq!(btcusd.contract_value, Some(100.0));
    assert_eq!(btcusd.precision.tick_size, 0.1);
    assert_eq!(btcusd.precision.lot_size, 1.0);
    let quantity_limit = btcusd.quantity_limit.unwrap();
    assert_eq!(quantity_limit.min.unwrap(), 1.0);
    assert_eq!(quantity_limit.max, Some(1000000.0));
}

#[test]
fn fetch_inverse_swap_markets() {
    let markets = fetch_markets(EXCHANGE_NAME, MarketType::InverseSwap).unwrap();
    assert!(!markets.is_empty());

    let btcusd_perp = markets
        .iter()
        .find(|m| m.symbol == "BTCUSD_PERP")
        .unwrap()
        .clone();
    assert_eq!(btcusd_perp.contract_value, Some(100.0));
    assert_eq!(btcusd_perp.precision.tick_size, 0.1);
    assert_eq!(btcusd_perp.precision.lot_size, 1.0);
    let quantity_limit = btcusd_perp.quantity_limit.unwrap();
    assert_eq!(quantity_limit.min.unwrap(), 1.0);
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
    assert_eq!(btcusdt.contract_value, Some(1.0));
    assert_eq!(btcusdt.precision.tick_size, 0.01);
    assert_eq!(btcusdt.precision.lot_size, 0.001);
    let quantity_limit = btcusdt.quantity_limit.unwrap();
    assert_eq!(quantity_limit.min.unwrap(), 0.001);
    assert_eq!(quantity_limit.max, Some(1000.0));
}

#[ignore]
#[test]
fn fetch_option_markets() {
    let markets = fetch_markets(EXCHANGE_NAME, MarketType::EuropeanOption).unwrap();
    assert!(!markets.is_empty());

    let btcusd = markets
        .iter()
        .find(|m| m.symbol.starts_with("BTC-"))
        .unwrap()
        .clone();
    assert_eq!(btcusd.contract_value, Some(1.0));
    assert_eq!(btcusd.precision.tick_size, 0.01);
    assert_eq!(btcusd.precision.lot_size, 0.0001);
    let quantity_limit = btcusd.quantity_limit.unwrap();
    assert_eq!(quantity_limit.min.unwrap(), 0.0002);
    assert_eq!(quantity_limit.max, Some(10000.0));
}

#[test_case(MarketType::InverseFuture)]
#[test_case(MarketType::LinearFuture)]
#[test_case(MarketType::InverseSwap)]
#[test_case(MarketType::LinearSwap)]
#[test_case(MarketType::EuropeanOption)]
fn test_contract_values(market_type: MarketType) {
    check_contract_values!(EXCHANGE_NAME, market_type);
}
