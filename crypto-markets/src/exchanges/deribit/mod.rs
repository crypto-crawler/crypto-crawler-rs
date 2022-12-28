mod utils;

use crate::{error::Result, Market, MarketType};

pub(crate) fn fetch_symbols(market_type: MarketType) -> Result<Vec<String>> {
    match market_type {
        MarketType::InverseFuture => utils::fetch_inverse_future_symbols(),
        MarketType::InverseSwap => utils::fetch_inverse_swap_symbols(),
        MarketType::EuropeanOption => utils::fetch_option_symbols(),
        _ => panic!("Unsupported market_type: {market_type}"),
    }
}

pub(crate) fn fetch_markets(market_type: MarketType) -> Result<Vec<Market>> {
    match market_type {
        MarketType::InverseFuture => utils::fetch_inverse_future_markets(),
        MarketType::InverseSwap => utils::fetch_inverse_swap_markets(),
        MarketType::EuropeanOption => utils::fetch_option_markets(),
        _ => panic!("Unsupported market_type: {market_type}"),
    }
}
