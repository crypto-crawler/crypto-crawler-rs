mod dydx_swap;

use crate::{error::Result, Market, MarketType};

pub(crate) fn fetch_symbols(market_type: MarketType) -> Result<Vec<String>> {
    match market_type {
        MarketType::LinearSwap => dydx_swap::fetch_linear_swap_symbols(),
        _ => panic!("dydX does NOT have the {} market", market_type),
    }
}

pub(crate) fn fetch_markets(_market_type: MarketType) -> Result<Vec<Market>> {
    Ok(Vec::new())
}
