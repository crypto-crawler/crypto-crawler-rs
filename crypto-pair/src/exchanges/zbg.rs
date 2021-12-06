use crypto_market_type::MarketType;

pub(crate) fn normalize_pair(symbol: &str) -> Option<String> {
    if symbol.ends_with("_USD-R") {
        let base = symbol.strip_suffix("_USD-R").unwrap();
        Some(format!("{}/USD", base))
    } else {
        Some(symbol.replace("_", "/").to_uppercase())
    }
}

pub(crate) fn get_market_type(symbol: &str) -> MarketType {
    if symbol.ends_with("_USD-R") {
        MarketType::InverseSwap
    } else if symbol.ends_with("_USDT") || symbol.ends_with("_ZUSD") {
        MarketType::LinearSwap
    } else {
        MarketType::Spot
    }
}
