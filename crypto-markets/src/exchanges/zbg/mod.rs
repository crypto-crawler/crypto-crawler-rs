mod zbg_spot;
mod zbg_swap;

use crate::{error::Result, Market, MarketType};

pub(crate) fn fetch_symbols(market_type: MarketType) -> Result<Vec<String>> {
    match market_type {
        MarketType::Spot => zbg_spot::fetch_spot_symbols(),
        MarketType::InverseSwap => zbg_swap::fetch_inverse_swap_symbols(),
        MarketType::LinearSwap => zbg_swap::fetch_linear_swap_symbols(),
        _ => panic!("Unkown market_type: {}", market_type),
    }
}

pub(crate) fn fetch_markets(_market_type: MarketType) -> Result<Vec<Market>> {
    Ok(Vec::new())
}
