use crypto_market_type::MarketType;

pub(crate) fn normalize_pair(symbol: &str) -> Option<String> {
    let (base, quote) = if symbol.ends_with("USDT") {
        // linear swap
        let base = symbol.strip_suffix("USDT").unwrap();
        (base, "USDT")
    } else if symbol.ends_with("USD") {
        // inverse swap
        let base = symbol.strip_suffix("USD").unwrap();
        (base, "USD")
    } else if symbol[symbol.len() - 2..].parse::<i64>().is_ok() {
        // inverse future
        let base = &symbol[..symbol.len() - 6];
        (base, "USD")
    } else {
        panic!("Unknown symbol {}", symbol);
    };
    Some(format!("{}/{}", base, quote))
}

pub(crate) fn get_market_type(symbol: &str) -> MarketType {
    if symbol.ends_with("USDT") {
        MarketType::LinearSwap
    } else if symbol.ends_with("USD") {
        MarketType::InverseSwap
    } else if symbol[symbol.len() - 2..].parse::<i64>().is_ok() {
        MarketType::InverseFuture
    } else {
        MarketType::Unknown
    }
}
