mod zbg_spot;
mod zbg_swap;

pub use zbg_spot::ZbgSpotRestClient;
pub use zbg_swap::ZbgSwapRestClient;

use crate::error::Result;
use crypto_markets::MarketType;

pub(crate) fn fetch_l2_snapshot(market_type: MarketType, symbol: &str) -> Result<String> {
    let func = match market_type {
        MarketType::Spot => zbg_spot::ZbgSpotRestClient::fetch_l2_snapshot,
        MarketType::InverseSwap | MarketType::LinearSwap => {
            zbg_swap::ZbgSwapRestClient::fetch_l2_snapshot
        }
        _ => panic!("ZBG unknown market_type: {}", market_type),
    };

    func(symbol)
}
