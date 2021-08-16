pub(crate) fn normalize_currency(currency: &str) -> String {
    if currency == "XBT" { "BTC" } else { currency }.to_string()
}

pub(crate) fn normalize_pair(mut symbol: &str) -> Option<String> {
    if symbol[(symbol.len() - 2)..].parse::<f64>().is_ok() {
        symbol = &symbol[..(symbol.len() - 3)]
    }

    let (base, quote) = if symbol.ends_with("USD") {
        (
            symbol.strip_suffix("USD").unwrap().to_string(),
            "USD".to_string(),
        )
    } else if symbol.ends_with("USDT") {
        (
            symbol.strip_suffix("USDT").unwrap().to_string(),
            "USDT".to_string(),
        )
    } else if symbol.ends_with("EUR") {
        (
            symbol.strip_suffix("EUR").unwrap().to_string(),
            "EUR".to_string(),
        )
    } else {
        let base_symbol = symbol;
        let quote_symbol = if base_symbol == "XBT" { "USD" } else { "XBT" };
        (base_symbol.to_string(), quote_symbol.to_string())
    };

    Some(format!(
        "{}/{}",
        normalize_currency(&base),
        normalize_currency(&quote)
    ))
}
