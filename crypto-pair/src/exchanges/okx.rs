use crypto_market_type::MarketType;

pub(crate) fn get_market_type(symbol: &str) -> MarketType {
    if symbol.ends_with("-USD-SWAP") {
        MarketType::InverseSwap
    } else if symbol.ends_with("-USDT-SWAP") {
        MarketType::LinearSwap
    } else if symbol.ends_with("-C") || symbol.ends_with("-P") {
        MarketType::EuropeanOption
    } else if symbol[(symbol.len() - 6)..].parse::<i64>().is_ok() {
        if symbol.contains("-USD-") {
            MarketType::InverseFuture
        } else if symbol.contains("-USDT-") {
            MarketType::LinearFuture
        } else {
            MarketType::Unknown
        }
    } else if symbol.contains('-') {
        MarketType::Spot
    } else {
        MarketType::Unknown
    }
}
