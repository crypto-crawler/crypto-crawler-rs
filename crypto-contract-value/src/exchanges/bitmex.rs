pub use crypto_market_type::MarketType;

pub(crate) fn get_contract_value(market_type: MarketType, _pair: &str) -> Option<f64> {
    match market_type {
        MarketType::InverseSwap | MarketType::InverseFuture => Some(1.0),
        MarketType::LinearFuture => Some(1.0),
        MarketType::QuantoSwap | MarketType::QuantoFuture => panic!("not implemented"),
        _ => None,
    }
}
