use crypto_market_type::MarketType;

pub(crate) fn get_market_type(symbol: &str, is_spot: Option<bool>) -> MarketType {
    if symbol.ends_with("_USD") {
        MarketType::InverseSwap
    } else if symbol.ends_with("_USDT") {
        if let Some(is_spot) = is_spot {
            if is_spot {
                MarketType::Spot
            } else {
                MarketType::LinearSwap
            }
        } else {
            MarketType::LinearSwap
        }
    } else if symbol.contains('_') {
        MarketType::Spot
    } else {
        MarketType::Unknown
    }
}
