pub(crate) mod mxc_spot;
pub(crate) mod mxc_swap;

use crate::error::Result;
use crypto_market_type::MarketType;

pub(crate) fn fetch_l2_snapshot(market_type: MarketType, symbol: &str) -> Result<String> {
    let func = match market_type {
        MarketType::Spot => mxc_spot::MxcSpotRestClient::fetch_l2_snapshot,
        MarketType::InverseSwap | MarketType::LinearSwap => {
            mxc_swap::MxcSwapRestClient::fetch_l2_snapshot
        }
        _ => panic!("MXC unknown market_type: {}", market_type),
    };

    func(symbol)
}
