mod bitz_spot;
mod bitz_swap;

pub use bitz_spot::BitzSpotRestClient;
pub use bitz_swap::BitzSwapRestClient;

use crate::error::Result;
use crypto_market_type::MarketType;

pub(crate) fn fetch_l2_snapshot(market_type: MarketType, symbol: &str) -> Result<String> {
    let func = match market_type {
        MarketType::Spot => bitz_spot::BitzSpotRestClient::fetch_l2_snapshot,
        MarketType::InverseSwap | MarketType::LinearSwap => {
            bitz_swap::BitzSwapRestClient::fetch_l2_snapshot
        }
        _ => panic!("BitZ unknown market_type: {market_type}"),
    };

    func(symbol)
}

pub(crate) fn fetch_open_interest(market_type: MarketType, symbol: Option<&str>) -> Result<String> {
    let func = match market_type {
        MarketType::LinearSwap | MarketType::InverseSwap => {
            bitz_swap::BitzSwapRestClient::fetch_open_interest
        }
        _ => panic!("bitz {market_type} does not have open interest"),
    };

    func(symbol)
}
