use crypto_markets::{fetch_markets, fetch_symbols, MarketType};

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
    }
}

#[test]
fn fetch_inverse_swap_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::InverseSwap).unwrap();
    assert!(!symbols.is_empty());

    for symbol in symbols.iter() {
        assert!(symbol.ends_with("-PERPETUAL"));
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
    }
}

#[test]
fn fetch_inverse_future_markets() {
    let markets = fetch_markets(EXCHANGE_NAME, MarketType::InverseFuture).unwrap();
    assert!(!markets.is_empty());

    let btcusd = markets.iter().find(|m| m.base_id == "BTC").unwrap();
    assert_eq!(btcusd.precision.tick_size, 0.5);
    assert_eq!(btcusd.precision.lot_size, 10.0);
}

#[test]
fn fetch_inverse_swap_markets() {
    let markets = fetch_markets(EXCHANGE_NAME, MarketType::InverseSwap).unwrap();
    assert!(!markets.is_empty());

    let btcusd = markets.iter().find(|m| m.base_id == "BTC").unwrap();
    assert_eq!(btcusd.precision.tick_size, 0.5);
    assert_eq!(btcusd.precision.lot_size, 10.0);
}

#[test]
fn fetch_option_markets() {
    let markets = fetch_markets(EXCHANGE_NAME, MarketType::EuropeanOption).unwrap();
    assert!(!markets.is_empty());

    let btcusd = markets.iter().find(|m| m.base_id == "BTC").unwrap();
    assert_eq!(btcusd.precision.tick_size, 0.0005);
    assert_eq!(btcusd.precision.lot_size, 0.1);
}
