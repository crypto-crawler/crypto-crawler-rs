mod gate_future;
mod gate_spot;
mod gate_swap;

use crate::{error::Result, Market, MarketType};

pub(crate) fn fetch_symbols(market_type: MarketType) -> Result<Vec<String>> {
    match market_type {
        MarketType::Spot => gate_spot::fetch_spot_symbols(),
        MarketType::InverseSwap => gate_swap::fetch_inverse_swap_symbols(),
        MarketType::LinearSwap => gate_swap::fetch_linear_swap_symbols(),
        MarketType::InverseFuture => gate_future::fetch_inverse_future_symbols(),
        MarketType::LinearFuture => gate_future::fetch_linear_future_symbols(),
        _ => panic!("Unsupported market_type: {market_type}"),
    }
}

pub(crate) fn fetch_markets(market_type: MarketType) -> Result<Vec<Market>> {
    match market_type {
        MarketType::Spot => gate_spot::fetch_spot_markets(),
        MarketType::InverseSwap => gate_swap::fetch_inverse_swap_markets(),
        MarketType::LinearSwap => gate_swap::fetch_linear_swap_markets(),
        MarketType::InverseFuture => gate_future::fetch_inverse_future_markets(),
        MarketType::LinearFuture => gate_future::fetch_linear_future_markets(),
        _ => panic!("Unsupported market_type: {market_type}"),
    }
}
