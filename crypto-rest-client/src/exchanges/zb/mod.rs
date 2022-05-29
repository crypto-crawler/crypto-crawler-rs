mod zb_spot;
mod zb_swap;

pub use zb_spot::ZbSpotRestClient;
pub use zb_swap::ZbSwapRestClient;

use crate::error::Result;
use crypto_market_type::MarketType;

pub(crate) fn fetch_l2_snapshot(market_type: MarketType, symbol: &str) -> Result<String> {
    let func = match market_type {
        MarketType::Spot => zb_spot::ZbSpotRestClient::fetch_l2_snapshot,
        MarketType::InverseSwap | MarketType::LinearSwap => {
            zb_swap::ZbSwapRestClient::fetch_l2_snapshot
        }
        _ => panic!("ZBG unknown market_type: {}", market_type),
    };

    func(symbol)
}
