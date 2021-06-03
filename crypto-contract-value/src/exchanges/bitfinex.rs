pub use crypto_market_type::MarketType;

pub(crate) fn get_contract_value(market_type: MarketType, _pair: &str) -> Option<f64> {
    match market_type {
        MarketType::Spot | MarketType::LinearSwap => Some(1.0),
        _ => None,
    }
}
