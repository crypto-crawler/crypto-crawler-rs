pub(super) mod bitget_spot;
pub(super) mod bitget_swap;

use crate::{error::Result, Market, MarketType};

pub(super) const EXCHANGE_NAME: &str = "bitget";

pub(crate) fn fetch_symbols(market_type: MarketType) -> Result<Vec<String>> {
    match market_type {
        MarketType::Spot => bitget_spot::fetch_spot_symbols(),
        MarketType::InverseSwap => bitget_swap::fetch_inverse_swap_symbols(),
        MarketType::LinearSwap => bitget_swap::fetch_linear_swap_symbols(),
        MarketType::InverseFuture => bitget_swap::fetch_inverse_future_symbols(),
        _ => panic!("Unsupported market_type: {}", market_type),
    }
}

pub(crate) fn fetch_markets(market_type: MarketType) -> Result<Vec<Market>> {
    match market_type {
        MarketType::Spot => bitget_spot::fetch_spot_markets(),
        MarketType::InverseSwap => bitget_swap::fetch_inverse_swap_markets(),
        MarketType::LinearSwap => bitget_swap::fetch_linear_swap_markets(),
        MarketType::InverseFuture => bitget_swap::fetch_inverse_future_markets(),
        _ => panic!("Unsupported market_type: {}", market_type),
    }
}
