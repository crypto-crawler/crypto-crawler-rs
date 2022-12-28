mod zb_spot;
mod zb_swap;

use crate::{error::Result, Market, MarketType};

pub(crate) fn fetch_symbols(market_type: MarketType) -> Result<Vec<String>> {
    match market_type {
        MarketType::Spot => zb_spot::fetch_spot_symbols(),
        MarketType::LinearSwap => zb_swap::fetch_linear_swap_symbols(),
        _ => panic!("Unkown market_type: {market_type}"),
    }
}

pub(crate) fn fetch_markets(market_type: MarketType) -> Result<Vec<Market>> {
    match market_type {
        MarketType::Spot => zb_spot::fetch_spot_markets(),
        MarketType::LinearSwap => zb_swap::fetch_linear_swap_markets(),
        _ => panic!("Unkown market_type: {market_type}"),
    }
}
