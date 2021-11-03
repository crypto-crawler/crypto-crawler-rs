pub use crypto_market_type::MarketType;

pub(crate) fn get_contract_value(market_type: MarketType, pair: &str) -> Option<f64> {
    match market_type {
        MarketType::InverseSwap | MarketType::InverseFuture => {
            if pair.starts_with("BTC") {
                Some(10.0)
            } else {
                Some(1.0)
            }
        }
        // Each option contract value is 1 coin
        MarketType::EuropeanOption => Some(1.0),
        _ => None,
    }
}
