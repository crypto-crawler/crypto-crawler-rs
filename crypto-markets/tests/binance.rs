use crypto_markets::{fetch_markets, fetch_symbols, MarketType};

#[test]
fn fetch_spot_symbols() {
    let symbols = fetch_symbols("binance", MarketType::Spot).unwrap();
    assert!(!symbols.is_empty());
}

#[test]
fn fetch_inverse_future_symbols() {
    let symbols = fetch_symbols("binance", MarketType::InverseFuture).unwrap();
    assert!(!symbols.is_empty());

    for symbol in symbols.iter() {
        let date = &symbol[(symbol.len() - 6)..];
        assert!(date.parse::<i64>().is_ok());

        let quote = &symbol[(symbol.len() - 10)..(symbol.len() - 7)];
        assert_eq!(quote, "USD");
    }
}

#[test]
fn fetch_inverse_swap_symbols() {
    let symbols = fetch_symbols("binance", MarketType::InverseSwap).unwrap();
    assert!(!symbols.is_empty());

    for symbol in symbols.iter() {
        assert!(symbol.ends_with("USD_PERP"));
    }
}

#[test]
fn fetch_linear_swap_symbols() {
    let symbols = fetch_symbols("binance", MarketType::LinearSwap).unwrap();
    assert!(!symbols.is_empty());

    for symbol in symbols.iter() {
        assert!(symbol.ends_with("USDT"));
    }
}

#[test]
fn fetch_linear_option_symbols() {
    let symbols = fetch_symbols("binance", MarketType::LinearOption).unwrap();
    assert!(!symbols.is_empty());

    for symbol in symbols.iter() {
        assert!(symbol.ends_with("-P") || symbol.ends_with("-C"));
    }
}

#[test]
fn fetch_spot_markets() {
    let markets = fetch_markets("binance", MarketType::Spot).unwrap();
    assert!(!markets.is_empty());
}

#[test]
fn fetch_inverse_future_markets() {
    let markets = fetch_markets("binance", MarketType::InverseFuture).unwrap();
    assert!(!markets.is_empty());
}

#[test]
fn fetch_inverse_swap_markets() {
    let markets = fetch_markets("binance", MarketType::InverseSwap).unwrap();
    assert!(!markets.is_empty());
}

#[test]
fn fetch_linear_swap_markets() {
    let markets = fetch_markets("binance", MarketType::LinearSwap).unwrap();
    assert!(!markets.is_empty());
}

#[test]
fn fetch_linear_option_markets() {
    let markets = fetch_markets("binance", MarketType::LinearOption).unwrap();
    assert!(!markets.is_empty());
}
