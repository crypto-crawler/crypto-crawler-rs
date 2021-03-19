#[macro_use]
mod utils;

pub(crate) mod binance_inverse;
pub(crate) mod binance_linear;
pub(crate) mod binance_option;
pub(crate) mod binance_spot;

use crate::error::Result;
use crypto_market_type::MarketType;

pub(crate) fn fetch_l2_snapshot(market_type: MarketType, symbol: &str) -> Result<String> {
    let func = match market_type {
        MarketType::Spot => binance_spot::BinanceSpotRestClient::fetch_l2_snapshot,
        MarketType::InverseFuture | MarketType::InverseSwap => {
            binance_inverse::BinanceInverseRestClient::fetch_l2_snapshot
        }
        MarketType::LinearFuture | MarketType::LinearSwap => {
            binance_linear::BinanceLinearRestClient::fetch_l2_snapshot
        }
        MarketType::Option => binance_option::BinanceOptionRestClient::fetch_l2_snapshot,
        _ => panic!("Binance unknown market_type: {}", market_type),
    };

    func(symbol)
}
