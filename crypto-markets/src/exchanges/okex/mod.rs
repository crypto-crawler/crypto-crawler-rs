pub(super) mod okex_future;
pub(super) mod okex_option;
pub(super) mod okex_spot;
pub(super) mod okex_swap;

use crate::{error::Result, Market, MarketType};

pub(crate) fn fetch_symbols(market_type: MarketType) -> Result<Vec<String>> {
    match market_type {
        MarketType::Spot => okex_spot::fetch_spot_symbols(),
        MarketType::InverseFuture => okex_future::fetch_inverse_future_symbols(),
        MarketType::LinearFuture => okex_future::fetch_linear_future_symbols(),
        MarketType::InverseSwap => okex_swap::fetch_inverse_swap_symbols(),
        MarketType::LinearSwap => okex_swap::fetch_linear_swap_symbols(),
        MarketType::EuropeanOption => okex_option::fetch_option_symbols(),
        _ => panic!("Unsupported market_type: {}", market_type),
    }
}

pub(crate) fn fetch_markets(market_type: MarketType) -> Result<Vec<Market>> {
    match market_type {
        MarketType::Spot => okex_spot::fetch_spot_markets(),
        MarketType::InverseFuture => okex_future::fetch_inverse_future_markets(),
        MarketType::LinearFuture => okex_future::fetch_linear_future_markets(),
        MarketType::InverseSwap => okex_swap::fetch_inverse_swap_markets(),
        MarketType::LinearSwap => okex_swap::fetch_linear_swap_markets(),
        MarketType::EuropeanOption => okex_option::fetch_option_markets(),
        _ => panic!("Unsupported market_type: {}", market_type),
    }
}
