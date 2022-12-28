mod kraken_futures;
mod kraken_spot;

use crate::{error::Result, Market, MarketType};

pub(crate) fn fetch_symbols(market_type: MarketType) -> Result<Vec<String>> {
    match market_type {
        MarketType::Spot => kraken_spot::fetch_spot_symbols(),
        MarketType::InverseFuture => kraken_futures::fetch_inverse_future_symbols(),
        MarketType::InverseSwap => kraken_futures::fetch_inverse_swap_symbols(),
        _ => panic!("Unsupported market_type: {market_type}"),
    }
}

pub(crate) fn fetch_markets(market_type: MarketType) -> Result<Vec<Market>> {
    match market_type {
        MarketType::Spot => kraken_spot::fetch_spot_markets(),
        MarketType::InverseFuture => kraken_futures::fetch_inverse_future_markets(),
        MarketType::InverseSwap => kraken_futures::fetch_inverse_swap_markets(),
        _ => panic!("Unsupported market_type: {market_type}"),
    }
}
