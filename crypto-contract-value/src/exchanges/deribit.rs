pub use crypto_market_type::MarketType;

pub(crate) fn get_contract_value(market_type: MarketType, _pair: &str) -> Option<f64> {
    match market_type {
        // Each inverse contract value is 1 USD, see:
        // https://docs.deribit.com/?javascript#trades-instrument_name-interval
        MarketType::InverseSwap | MarketType::InverseFuture => Some(1.0),
        // Each option contract value is 1 coin
        MarketType::EuropeanOption => Some(1.0),
        _ => None,
    }
}
