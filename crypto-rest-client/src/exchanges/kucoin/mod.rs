mod kucoin_spot;
mod kucoin_swap;

pub use kucoin_spot::KuCoinSpotRestClient;
pub use kucoin_swap::KuCoinSwapRestClient;

use crate::error::Result;
use crypto_markets::MarketType;

pub(crate) fn fetch_l2_snapshot(market_type: MarketType, symbol: &str) -> Result<String> {
    let func = match market_type {
        MarketType::Spot => kucoin_spot::KuCoinSpotRestClient::fetch_l2_snapshot,
        MarketType::InverseSwap | MarketType::LinearSwap | MarketType::InverseFuture => {
            kucoin_swap::KuCoinSwapRestClient::fetch_l2_snapshot
        }
        _ => panic!("Bitget unknown market_type: {}", market_type),
    };

    func(symbol)
}

pub(crate) fn fetch_l3_snapshot(market_type: MarketType, symbol: &str) -> Result<String> {
    let func = match market_type {
        MarketType::Spot => kucoin_spot::KuCoinSpotRestClient::fetch_l3_snapshot,
        MarketType::InverseSwap | MarketType::LinearSwap | MarketType::InverseFuture => {
            kucoin_swap::KuCoinSwapRestClient::fetch_l3_snapshot
        }
        _ => panic!("Bitget unknown market_type: {}", market_type),
    };

    func(symbol)
}
