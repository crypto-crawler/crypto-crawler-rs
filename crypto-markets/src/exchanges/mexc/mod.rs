mod utils;

pub(super) mod mexc_spot;
pub(super) mod mexc_swap;

use crate::{error::Result, Market, MarketType};

pub(super) const EXCHANGE_NAME: &str = "mexc";

pub(crate) fn fetch_symbols(market_type: MarketType) -> Result<Vec<String>> {
    match market_type {
        MarketType::Spot => mexc_spot::fetch_spot_symbols(),
        MarketType::InverseSwap => mexc_swap::fetch_inverse_swap_symbols(),
        MarketType::LinearSwap => mexc_swap::fetch_linear_swap_symbols(),
        _ => panic!("Unsupported market_type: {}", market_type),
    }
}

pub(crate) fn fetch_markets(market_type: MarketType) -> Result<Vec<Market>> {
    match market_type {
        MarketType::Spot => mexc_spot::fetch_spot_markets(),
        MarketType::InverseSwap => mexc_swap::fetch_inverse_swap_markets(),
        MarketType::LinearSwap => mexc_swap::fetch_linear_swap_markets(),
        _ => panic!("Unsupported market_type: {}", market_type),
    }
}
