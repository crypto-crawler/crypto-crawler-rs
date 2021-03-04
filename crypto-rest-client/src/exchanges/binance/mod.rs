#[macro_use]
mod utils;

pub(crate) mod binance_future;
pub(crate) mod binance_inverse_swap;
pub(crate) mod binance_linear_swap;
pub(crate) mod binance_option;
pub(crate) mod binance_spot;

use crate::error::Result;
use crypto_market_type::MarketType;

pub(crate) fn fetch_l2_snapshot(market_type: MarketType, symbol: &str) -> Result<String> {
    let func = match market_type {
        MarketType::Spot => binance_spot::BinanceSpotRestClient::fetch_l2_snapshot,
        MarketType::InverseFuture => binance_future::BinanceFutureRestClient::fetch_l2_snapshot,
        MarketType::LinearSwap => {
            binance_linear_swap::BinanceLinearSwapRestClient::fetch_l2_snapshot
        }
        MarketType::InverseSwap => {
            binance_inverse_swap::BinanceInverseSwapRestClient::fetch_l2_snapshot
        }
        MarketType::Option => binance_option::BinanceOptionRestClient::fetch_l2_snapshot,
        _ => panic!("Binance unknown market_type: {}", market_type),
    };

    func(symbol)
}
