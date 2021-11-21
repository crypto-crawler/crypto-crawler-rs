use crypto_market_type::{get_market_types, MarketType};
use crypto_markets::{fetch_markets, fetch_symbols};
use test_case::test_case;

#[macro_use]
mod utils;

const EXCHANGE_NAME: &str = "bitmex";

fn get_market_type_from_symbol(symbol: &str) -> MarketType {
    let date = &symbol[(symbol.len() - 2)..];
    if date.parse::<i64>().is_ok() {
        // future
        if symbol.starts_with("XBT") {
            if symbol.len() == 6 {
                MarketType::InverseFuture
            } else {
                let symbol_without_date = &symbol[..(symbol.len() - 3)];
                if symbol_without_date.ends_with("USD") || symbol_without_date.ends_with("EUR") {
                    // Settled in XBT, quoted in USD or EUR
                    MarketType::InverseFuture
                } else if symbol_without_date.ends_with("USDT") {
                    // Settled in USDT, quoted in USDT
                    MarketType::LinearFuture
                } else {
                    panic!("Unknown symbol {}", symbol);
                }
            }
        } else {
            let symbol_without_date = &symbol[..(symbol.len() - 3)];
            if symbol_without_date.ends_with("USD") || symbol_without_date.ends_with("EUR") {
                // Settled in XBT, quoted in USD or EUR
                MarketType::QuantoFuture
            } else if symbol_without_date.ends_with("USDT") {
                // Settled in USDT, quoted in USDT
                MarketType::LinearFuture
            } else {
                // Settled in XBT, quoted in XBT
                MarketType::LinearFuture
            }
        }
    } else {
        // swap
        if symbol.starts_with("XBT") {
            if symbol.ends_with("USD") || symbol.ends_with("EUR") {
                // Settled in XBT, quoted in USD or EUR
                MarketType::InverseSwap
            } else if symbol.ends_with("USDT") {
                // Settled in USDT, quoted in USDT
                MarketType::LinearSwap
            } else {
                panic!("Unknown symbol {}", symbol);
            }
        } else {
            if symbol.ends_with("USD") || symbol.ends_with("EUR") {
                // Settled in XBT, quoted in USD or EUR
                MarketType::QuantoSwap
            } else if symbol.ends_with("USDT") {
                // Settled in USDT, quoted in USDT
                MarketType::LinearSwap
            } else {
                panic!("Unknown symbol {}", symbol);
            }
        }
    }
}

#[test]
fn fetch_all_symbols() {
    gen_all_symbols!();
}

#[test]
fn fetch_all_symbols_via_unknown() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::Unknown).unwrap();
    assert!(!symbols.is_empty());
}

#[test]
fn fetch_inverse_swap_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::InverseSwap).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        assert!(symbol.starts_with("XBT"));
        assert!(symbol.ends_with("USD") || symbol.ends_with("EUR"));
        assert_eq!(MarketType::InverseSwap, get_market_type_from_symbol(symbol));
    }
}

#[test]
fn fetch_linear_swap_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::LinearSwap).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        assert!(symbol.ends_with("USDT"));
        assert_eq!(MarketType::LinearSwap, get_market_type_from_symbol(symbol));
    }
}

#[test]
fn fetch_quanto_swap_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::QuantoSwap).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        assert!(symbol.ends_with("USD") || symbol.ends_with("USDT"));
        assert_eq!(MarketType::QuantoSwap, get_market_type_from_symbol(symbol));
    }
}

#[test]
fn fetch_inverse_future_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::InverseFuture).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        assert!(symbol.starts_with("XBT"));
        let date = &symbol[(symbol.len() - 2)..];
        assert!(date.parse::<i64>().is_ok());
        assert_eq!(
            MarketType::InverseFuture,
            get_market_type_from_symbol(symbol)
        );
    }
}

#[test]
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
            get_market_type_from_symbol(symbol)
        );
    }
}

#[test]
fn fetch_linear_future_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::LinearFuture).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        let date = &symbol[(symbol.len() - 2)..];
        assert!(date.parse::<i64>().is_ok());
        assert_eq!(
            MarketType::LinearFuture,
            get_market_type_from_symbol(symbol)
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

#[test_case(MarketType::InverseSwap)]
#[test_case(MarketType::QuantoSwap)]
#[test_case(MarketType::InverseFuture)]
#[test_case(MarketType::QuantoFuture)]
#[test_case(MarketType::LinearFuture)]
fn test_contract_values(market_type: MarketType) {
    check_contract_values!(EXCHANGE_NAME, market_type);
}
