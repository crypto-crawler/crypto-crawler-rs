pub use crypto_market_type::MarketType;

pub(crate) fn get_contract_value(market_type: MarketType, pair: &str) -> Option<f64> {
    match market_type {
        MarketType::InverseSwap | MarketType::InverseFuture => {
            Some(if pair.starts_with("BTC") { 100.0 } else { 10.0 })
        }
        MarketType::LinearSwap | MarketType::LinearFuture => Some(1.0),
        MarketType::EuropeanOption => Some(1.0),
        _ => None,
    }
}
