use crypto_market_type::{get_market_types, MarketType};
use crypto_markets::{fetch_markets, fetch_symbols};
use crypto_pair::get_market_type;
use test_case::test_case;

#[macro_use]
mod utils;

const EXCHANGE_NAME: &str = "ftx";

#[test]
fn fetch_all_symbols() {
    gen_all_symbols!();
}

#[test]
fn fetch_spot_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::Spot).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        assert!(symbol.contains('/'));
        assert_eq!(*symbol, symbol.to_uppercase());
        assert_eq!(
            MarketType::Spot,
            get_market_type(symbol, EXCHANGE_NAME, None)
        );
    }
}

#[test]
fn fetch_linear_swap_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::LinearSwap).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        assert!(symbol.ends_with("-PERP"));
        assert_eq!(
            MarketType::LinearSwap,
            get_market_type(symbol, EXCHANGE_NAME, None)
        );
    }
}

#[test]
fn fetch_linear_future_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::LinearFuture).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        let date = &symbol[(symbol.len() - 4)..];
        assert!(date.parse::<i64>().is_ok());
        assert_eq!(
            MarketType::LinearFuture,
            get_market_type(symbol, EXCHANGE_NAME, None)
        );
    }
}

#[test]
fn fetch_move_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::Move).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        assert!(symbol.contains("-MOVE-"));
        assert_eq!(
            MarketType::Move,
            get_market_type(symbol, EXCHANGE_NAME, None)
        );
    }
}

#[test]
fn fetch_bvol_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::BVOL).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        assert!(symbol.contains("BVOL/"));
        assert_eq!(
            MarketType::BVOL,
            get_market_type(symbol, EXCHANGE_NAME, None)
        );
    }
}

#[test]
fn fetch_spot_markets() {
    let markets = fetch_markets(EXCHANGE_NAME, MarketType::Spot).unwrap();
    assert!(!markets.is_empty());

    let btcusdt = markets
        .iter()
        .find(|m| m.symbol == "BTC/USDT")
        .unwrap()
        .clone();
    assert!(btcusdt.contract_value.is_none());
    assert_eq!(btcusdt.precision.tick_size, 1.0);
    assert_eq!(btcusdt.precision.lot_size, 0.0001);
    assert!(btcusdt.quantity_limit.is_none());
}

#[test]
fn fetch_linear_swap_markets() {
    let markets = fetch_markets(EXCHANGE_NAME, MarketType::LinearSwap).unwrap();
    assert!(!markets.is_empty());

    let btcusd = markets
        .iter()
        .find(|m| m.symbol == "BTC-PERP")
        .unwrap()
        .clone();
    assert_eq!(btcusd.contract_value, Some(1.0));
    assert_eq!(btcusd.precision.tick_size, 1.0);
    assert_eq!(btcusd.precision.lot_size, 0.0001);
    assert!(btcusd.quantity_limit.is_none());
}

#[test]
fn fetch_linear_future_markets() {
    let markets = fetch_markets(EXCHANGE_NAME, MarketType::LinearFuture).unwrap();
    assert!(!markets.is_empty());

    let btcusd = markets
        .iter()
        .find(|m| m.symbol.starts_with("BTC-"))
        .unwrap()
        .clone();
    assert_eq!(btcusd.contract_value, Some(1.0));
    assert_eq!(btcusd.precision.tick_size, 1.0);
    assert_eq!(btcusd.precision.lot_size, 0.0001);
    assert!(btcusd.quantity_limit.is_none());
    assert!(btcusd.delivery_date.is_some());
}

#[test]
fn fetch_move_markets() {
    let markets = fetch_markets(EXCHANGE_NAME, MarketType::Move).unwrap();
    assert!(!markets.is_empty());

    let btcusd = markets
        .iter()
        .find(|m| m.symbol.starts_with("BTC-MOVE-"))
        .unwrap()
        .clone();
    assert_eq!(btcusd.contract_value, Some(1.0));
    assert_eq!(btcusd.precision.tick_size, 1.0);
    assert_eq!(btcusd.precision.lot_size, 0.0001);
    assert!(btcusd.quantity_limit.is_none());
    assert!(btcusd.delivery_date.is_some());
}

#[test]
fn fetch_bvol_markets() {
    let markets = fetch_markets(EXCHANGE_NAME, MarketType::BVOL).unwrap();
    assert!(!markets.is_empty());

    let btcusd = markets
        .iter()
        .find(|m| m.symbol == "BVOL/USD")
        .unwrap()
        .clone();
    assert_eq!(btcusd.contract_value, None);
    assert_eq!(btcusd.precision.tick_size, 0.05);
    assert_eq!(btcusd.precision.lot_size, 0.0001);
    assert!(btcusd.quantity_limit.is_none());
    assert!(btcusd.delivery_date.is_none());
}

#[test_case(MarketType::LinearSwap)]
#[test_case(MarketType::LinearFuture)]
fn test_contract_values(market_type: MarketType) {
    check_contract_values!(EXCHANGE_NAME, market_type);
}
