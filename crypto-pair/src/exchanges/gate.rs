use crypto_market_type::MarketType;

pub(crate) fn get_market_type(symbol: &str, is_spot: Option<bool>) -> MarketType {
    if symbol.ends_with("_USD") {
        if let Some(is_spot) = is_spot {
            if is_spot {
                MarketType::Spot
            } else {
                MarketType::InverseSwap
            }
        } else {
            MarketType::InverseSwap
        }
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
    } else if symbol.len() > 8 && symbol[(symbol.len() - 8)..].parse::<i64>().is_ok() {
        if symbol.contains("_USD_") {
            MarketType::InverseFuture
        } else if symbol.contains("_USDT_") {
            MarketType::LinearFuture
        } else {
            MarketType::Unknown
        }
    } else if symbol.contains('_') {
        MarketType::Spot
    } else {
        MarketType::Unknown
    }
}
