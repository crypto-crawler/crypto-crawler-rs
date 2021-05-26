pub use crypto_market_type::MarketType;

pub(crate) fn get_contract_value(_market_type: MarketType, _pair: &str) -> Option<f32> {
    Some(1.0)
}
