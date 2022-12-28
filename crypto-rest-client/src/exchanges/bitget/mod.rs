mod bitget_spot;
mod bitget_swap;

pub use bitget_spot::BitgetSpotRestClient;
pub use bitget_swap::BitgetSwapRestClient;

use crate::error::Result;
use crypto_market_type::MarketType;

pub(crate) fn fetch_l2_snapshot(market_type: MarketType, symbol: &str) -> Result<String> {
    let func = match market_type {
        MarketType::Spot => bitget_spot::BitgetSpotRestClient::fetch_l2_snapshot,
        MarketType::InverseFuture | MarketType::InverseSwap | MarketType::LinearSwap => {
            bitget_swap::BitgetSwapRestClient::fetch_l2_snapshot
        }
        _ => panic!("Bitget unknown market_type: {market_type}"),
    };

    func(symbol)
}

pub(crate) fn fetch_open_interest(market_type: MarketType, symbol: &str) -> Result<String> {
    let func = match market_type {
        MarketType::InverseFuture | MarketType::InverseSwap | MarketType::LinearSwap => {
            bitget_swap::BitgetSwapRestClient::fetch_open_interest
        }
        _ => panic!("Bitget {market_type} does not have open interest"),
    };

    func(symbol)
}
