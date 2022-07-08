use crypto_market_type::MarketType;

pub(crate) fn normalize_currency(currency: &str) -> String {
    if currency == "XBT" {
        "BTC"
    } else if currency == "BCHSV" {
        "BSV"
    } else if currency == "R" {
        "REV"
    } else if currency == "WAX" {
        "WAXP"
    } else if currency == "LOKI" {
        "OXEN"
    } else if currency == "GALAX" {
        "GALA"
    } else {
        currency
    }
    .to_uppercase()
}

pub(crate) fn normalize_pair(symbol: &str) -> Option<String> {
    let (base, quote) = if symbol.ends_with("USDM") {
        // inverse swap
        (
            symbol.strip_suffix("USDM").unwrap().to_string(),
            "USD".to_string(),
        )
    } else if symbol.ends_with("USDTM") {
        // linear swap
        (
            symbol.strip_suffix("USDTM").unwrap().to_string(),
            "USDT".to_string(),
        )
    } else if symbol[(symbol.len() - 2)..].parse::<i64>().is_ok() {
        // inverse future
        let base = &symbol[..symbol.len() - 4];
        (base.to_string(), "USD".to_string())
    } else if symbol.contains('-') {
        // spot
        let v: Vec<&str> = symbol.split('-').collect();
        (v[0].to_string(), v[1].to_string())
    } else {
        panic!("Unknown symbol {}", symbol);
    };

    Some(format!(
        "{}/{}",
        normalize_currency(&base),
        normalize_currency(&quote)
    ))
}

pub(crate) fn get_market_type(symbol: &str) -> MarketType {
    if symbol.ends_with("USDM") {
        MarketType::InverseSwap
    } else if symbol.ends_with("USDTM") {
        MarketType::LinearSwap
    } else if symbol[(symbol.len() - 2)..].parse::<i64>().is_ok() {
        MarketType::InverseFuture
    } else if symbol.contains('-') {
        MarketType::Spot
    } else {
        MarketType::Unknown
    }
}
