use crypto_markets::{fetch_symbols, MarketType};

#[test]
fn fetch_inverse_swap_symbols() {
    let symbols = fetch_symbols("bitmex", MarketType::InverseSwap).unwrap();
    assert_eq!(1, symbols.len());
    assert_eq!("XBTUSD", symbols[0]);
}

#[test]
fn fetch_quanto_swap_symbols() {
    let symbols = fetch_symbols("bitmex", MarketType::QuantoSwap).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        assert!(symbol.ends_with("USD") || symbol.ends_with("USDT"));
    }
}

#[test]
fn fetch_inverse_future_symbols() {
    let symbols = fetch_symbols("bitmex", MarketType::InverseFuture).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        assert!(symbol.starts_with("XBT"));
        let date = &symbol[(symbol.len() - 2)..];
        assert!(date.parse::<i64>().is_ok());
    }
}

#[test]
fn fetch_quanto_future_symbols() {
    let symbols = fetch_symbols("bitmex", MarketType::QuantoFuture).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        let date = &symbol[(symbol.len() - 2)..];
        assert!(date.parse::<i64>().is_ok());

        let quote = if symbol.chars().nth(symbol.len() - 4).unwrap() == 'T' {
            &symbol[(symbol.len() - 7)..(symbol.len() - 3)]
        } else {
            &symbol[(symbol.len() - 6)..(symbol.len() - 3)]
        };
        assert!(quote == "USD" || quote == "USDT");
    }
}

#[test]
fn fetch_linear_future_symbols() {
    let symbols = fetch_symbols("bitmex", MarketType::LinearFuture).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        println!("{}", symbol);
        let date = &symbol[(symbol.len() - 2)..];
        assert!(date.parse::<i64>().is_ok());
    }
}
