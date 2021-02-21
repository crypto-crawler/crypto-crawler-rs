mod bitz_spot;
mod bitz_swap;

use crate::{error::Result, Market, MarketType};

pub(crate) fn fetch_symbols(market_type: MarketType) -> Result<Vec<String>> {
    match market_type {
        MarketType::Spot => bitz_spot::fetch_spot_symbols(),
        MarketType::InverseSwap => bitz_swap::fetch_inverse_swap_symbols(),
        MarketType::LinearSwap => bitz_swap::fetch_linear_swap_symbols(),
        _ => panic!("Unsupported market_type: {}", market_type),
    }
}

pub(crate) fn fetch_markets(_market_type: MarketType) -> Result<Vec<Market>> {
    Ok(Vec::new())
}
