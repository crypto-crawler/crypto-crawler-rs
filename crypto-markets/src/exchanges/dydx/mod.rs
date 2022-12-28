mod dydx_swap;

use crate::{error::Result, Market, MarketType};

pub(crate) fn fetch_symbols(market_type: MarketType) -> Result<Vec<String>> {
    match market_type {
        MarketType::LinearSwap => dydx_swap::fetch_linear_swap_symbols(),
        _ => panic!("dydX does NOT have the {market_type} market"),
    }
}

pub(crate) fn fetch_markets(market_type: MarketType) -> Result<Vec<Market>> {
    match market_type {
        MarketType::LinearSwap => dydx_swap::fetch_linear_swap_markets(),
        _ => panic!("dydX does NOT have the {market_type} market"),
    }
}
