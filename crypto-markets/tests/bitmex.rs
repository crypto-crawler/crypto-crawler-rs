use crypto_market_type::MarketType;
use crypto_markets::{fetch_markets, fetch_symbols};
use crypto_pair::get_market_type;

#[macro_use]
mod utils;

const EXCHANGE_NAME: &str = "bitmex";

#[test]
fn fetch_all_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::Unknown).unwrap();
    assert!(!symbols.is_empty());
}

#[test]
#[ignore = "to avoid 429 Too Many Requests"]
fn fetch_inverse_swap_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::InverseSwap).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        assert!(symbol.starts_with("XBT") || symbol.starts_with("ETH"));
        assert!(symbol.ends_with("USD") || symbol.ends_with("EUR") || symbol.ends_with("_ETH"));
        assert_eq!(
            MarketType::InverseSwap,
            get_market_type(symbol, EXCHANGE_NAME, None)
        );
    }
}

#[test]
#[ignore = "to avoid 429 Too Many Requests"]
fn fetch_linear_swap_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::LinearSwap).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        assert!(symbol.ends_with("USDT"));
        assert_eq!(
            MarketType::LinearSwap,
            get_market_type(symbol, EXCHANGE_NAME, None)
        );
    }
}

#[test]
#[ignore = "to avoid 429 Too Many Requests"]
fn fetch_quanto_swap_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::QuantoSwap).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        assert!(symbol.ends_with("USD") || symbol.ends_with("USDT"));
        assert_eq!(
            MarketType::QuantoSwap,
            get_market_type(symbol, EXCHANGE_NAME, None)
        );
    }
}

#[test]
#[ignore = "to avoid 429 Too Many Requests"]
fn fetch_inverse_future_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::InverseFuture).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        assert!(symbol.starts_with("XBT") || symbol.starts_with("ETH"));
        let date = if let Some(pos) = symbol.rfind('_') {
            // e.g., ETHUSDM22_ETH
            &symbol[(pos - 2)..pos]
        } else {
            &symbol[(symbol.len() - 2)..]
        };
        assert!(date.parse::<i64>().is_ok());
        assert_eq!(
            MarketType::InverseFuture,
            get_market_type(symbol, EXCHANGE_NAME, None)
        );
    }
}

#[test]
#[ignore = "to avoid 429 Too Many Requests"]
fn fetch_quanto_future_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::QuantoFuture).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        assert!(!symbol.starts_with("XBT"));
        let date = &symbol[(symbol.len() - 2)..];
        assert!(date.parse::<i64>().is_ok());

        let quote = if symbol.chars().nth(symbol.len() - 4).unwrap() == 'T' {
            &symbol[(symbol.len() - 7)..(symbol.len() - 3)]
        } else {
            &symbol[(symbol.len() - 6)..(symbol.len() - 3)]
        };
        assert!(quote == "USD" || quote == "USDT");
        assert_eq!(
            MarketType::QuantoFuture,
            get_market_type(symbol, EXCHANGE_NAME, None)
        );
    }
}

#[test]
#[ignore = "to avoid 429 Too Many Requests"]
fn fetch_linear_future_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::LinearFuture).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        let date = &symbol[(symbol.len() - 2)..];
        assert!(date.parse::<i64>().is_ok());
        assert_eq!(
            MarketType::LinearFuture,
            get_market_type(symbol, EXCHANGE_NAME, None)
        );
    }
}

#[test]
fn fetch_inverse_swap_markets() {
    let markets = fetch_markets(EXCHANGE_NAME, MarketType::InverseSwap).unwrap();
    assert!(!markets.is_empty());

    let xbtusd = markets.iter().find(|m| m.symbol == "XBTUSD").unwrap();
    assert_eq!(xbtusd.precision.tick_size, 0.5);
    assert_eq!(xbtusd.precision.lot_size, 100.0);
    assert_eq!(xbtusd.contract_value, Some(1.0));
    assert!(xbtusd.quantity_limit.is_none());
}

#[test]
fn test_contract_values() {
    check_contract_values!(EXCHANGE_NAME, MarketType::Unknown);
}
