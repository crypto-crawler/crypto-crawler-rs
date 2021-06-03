pub use crypto_market_type::MarketType;

pub(crate) fn get_contract_value(market_type: MarketType, _pair: &str) -> Option<f64> {
    match market_type {
        // Each inverse contract value is 1 USD, see:
        // https://www.bybit.com/data/basic/inverse/contract-detail?symbol=BTCUSD
        // https://www.bybit.com/data/basic/future-inverse/contract-detail?symbol=BTCUSD0625
        MarketType::InverseSwap | MarketType::InverseFuture => Some(1.0),
        // Each linear contract value is 1 coin, see:
        // https://www.bybit.com/data/basic/linear/contract-detail?symbol=BTCUSDT
        MarketType::LinearSwap => Some(1.0),
        _ => None,
    }
}
