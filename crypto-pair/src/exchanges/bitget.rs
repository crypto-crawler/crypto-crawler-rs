use crypto_market_type::MarketType;

pub(crate) fn normalize_pair(symbol: &str) -> Option<String> {
    if symbol.ends_with("_SPBL") || symbol.ends_with("_UMCBL") || symbol.ends_with("_DMCBL") {
        let pos = symbol.find('_').unwrap();
        let pair = &symbol[..pos];
        if pair.ends_with("USDT") {
            Some(format!("{}/USDT", pair.strip_suffix("USDT").unwrap()))
        } else if pair.ends_with("USD") {
            Some(format!("{}/USD", pair.strip_suffix("USD").unwrap()))
        } else if pair.ends_with("ETH") {
            Some(format!("{}/ETH", pair.strip_suffix("ETH").unwrap()))
        } else if pair.ends_with("BTC") {
            Some(format!("{}/BTC", pair.strip_suffix("BTC").unwrap()))
        } else {
            panic!("Failed to parse {}", symbol);
        }
    } else {
        #[allow(clippy::collapsible_else_if)]
        if symbol.starts_with("cmt_") {
            // linear swap
            assert!(symbol.ends_with("usdt"));
            let base = &symbol[4..symbol.len() - 4];
            Some(format!("{}/usdt", base).to_uppercase())
        } else if symbol.contains('_') {
            // spot
            Some(symbol.replace('_', "/").to_uppercase())
        } else if symbol.ends_with("usd") {
            // inverse swap
            let base = symbol.strip_suffix("usd").unwrap();
            Some(format!("{}/usd", base).to_uppercase())
        } else {
            None
        }
    }
}

pub(crate) fn get_market_type(symbol: &str) -> MarketType {
    if symbol.ends_with("_SPBL") || symbol.ends_with("_UMCBL") || symbol.ends_with("_DMCBL") {
        // bitget v3 API
        if symbol.ends_with("_SPBL") {
            MarketType::Spot
        } else if symbol.ends_with("_UMCBL") {
            MarketType::LinearSwap
        } else {
            MarketType::InverseSwap
        }
    } else {
        // deprecated bitget v1 API
        if symbol.starts_with("cmt_") {
            MarketType::LinearSwap
        } else if symbol.contains('_') {
            MarketType::Spot
        } else if symbol.ends_with("usd") {
            MarketType::InverseSwap
        } else {
            MarketType::Unknown
        }
    }
}
