use crypto_market_type::{get_market_types, MarketType};
use crypto_markets::{fetch_markets, fetch_symbols};
use crypto_pair::get_market_type;
use test_case::test_case;

#[macro_use]
mod utils;

const EXCHANGE_NAME: &str = "huobi";

#[test]
fn fetch_all_symbols() {
    gen_all_symbols!();
}

#[test]
fn fetch_spot_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::Spot).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        assert_eq!(symbol.to_lowercase(), symbol.to_string());
        assert_eq!(
            MarketType::Spot,
            get_market_type(symbol, EXCHANGE_NAME, None)
        );
    }
}

#[test]
fn fetch_inverse_future_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::InverseFuture).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        assert!(
            symbol.ends_with("_CW")
                || symbol.ends_with("_NW")
                || symbol.ends_with("_CQ")
                || symbol.ends_with("_NQ")
        );
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
        assert!(symbol.ends_with("-USD"));
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
        assert!(symbol.ends_with("-USDT"));
        assert_eq!(
            MarketType::LinearSwap,
            get_market_type(symbol, EXCHANGE_NAME, None)
        );
    }
}

#[test]
#[ignore]
fn fetch_option_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::EuropeanOption).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        assert!(symbol.contains("-C-") || symbol.contains("-P-"));
        assert_eq!(
            MarketType::EuropeanOption,
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
        .find(|m| m.symbol == "btcusdt")
        .unwrap()
        .clone();
    assert_eq!(btc_usdt.precision.tick_size, 0.01);
    assert_eq!(btc_usdt.precision.lot_size, 0.000001);
    let quantity_limit = btc_usdt.quantity_limit.unwrap();
    assert_eq!(quantity_limit.min.unwrap(), 0.0001);
    assert_eq!(quantity_limit.max, Some(1000.0));
}

#[test]
fn fetch_inverse_future_markets() {
    let markets = fetch_markets(EXCHANGE_NAME, MarketType::InverseFuture).unwrap();
    assert!(!markets.is_empty());

    let btc_usd = markets
        .iter()
        .find(|m| m.symbol == "BTC_CW")
        .unwrap()
        .clone();
    assert_eq!(btc_usd.precision.tick_size, 0.01);
    assert_eq!(btc_usd.precision.lot_size, 1.0);
    assert!(btc_usd.quantity_limit.is_none());
}

#[test]
fn fetch_inverse_swap_markets() {
    let markets = fetch_markets(EXCHANGE_NAME, MarketType::InverseSwap).unwrap();
    assert!(!markets.is_empty());

    let btc_usd = markets
        .iter()
        .find(|m| m.symbol == "BTC-USD")
        .unwrap()
        .clone();
    assert_eq!(btc_usd.precision.tick_size, 0.1);
    assert_eq!(btc_usd.precision.lot_size, 1.0);
    assert!(btc_usd.quantity_limit.is_none());
}

#[test]
fn fetch_linear_swap_markets() {
    let markets = fetch_markets(EXCHANGE_NAME, MarketType::LinearSwap).unwrap();
    assert!(!markets.is_empty());

    let btc_usdt = markets
        .iter()
        .find(|m| m.symbol == "BTC-USDT")
        .unwrap()
        .clone();
    assert_eq!(btc_usdt.precision.tick_size, 0.1);
    assert_eq!(btc_usdt.precision.lot_size, 1.0);
    assert!(btc_usdt.quantity_limit.is_none());
}

#[test_case(MarketType::InverseFuture)]
#[test_case(MarketType::InverseSwap)]
#[test_case(MarketType::LinearSwap)]
fn test_contract_values(market_type: MarketType) {
    check_contract_values!(EXCHANGE_NAME, market_type);
}
