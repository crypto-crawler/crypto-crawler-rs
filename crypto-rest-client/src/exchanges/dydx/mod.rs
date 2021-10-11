pub(crate) mod dydx_swap;

use crate::error::Result;
use crypto_market_type::MarketType;

pub(crate) fn fetch_l2_snapshot(market_type: MarketType, symbol: &str) -> Result<String> {
    let func = match market_type {
        MarketType::LinearSwap => dydx_swap::DydxSwapRestClient::fetch_l2_snapshot,
        _ => panic!("dYdX does not have the {} market type", market_type),
    };

    func(symbol)
}
