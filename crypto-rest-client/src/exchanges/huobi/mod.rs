#[macro_use]
mod utils;

pub(crate) mod huobi_future;
pub(crate) mod huobi_inverse_swap;
pub(crate) mod huobi_linear_swap;
pub(crate) mod huobi_option;
pub(crate) mod huobi_spot;

use crate::error::Result;
use crypto_market_type::MarketType;

pub(crate) fn fetch_l2_snapshot(market_type: MarketType, symbol: &str) -> Result<String> {
    let func = match market_type {
        MarketType::Spot => huobi_spot::HuobiSpotRestClient::fetch_l2_snapshot,
        MarketType::InverseFuture => huobi_future::HuobiFutureRestClient::fetch_l2_snapshot,
        MarketType::LinearSwap => huobi_linear_swap::HuobiLinearSwapRestClient::fetch_l2_snapshot,
        MarketType::InverseSwap => {
            huobi_inverse_swap::HuobiInverseSwapRestClient::fetch_l2_snapshot
        }
        MarketType::EuropeanOption => huobi_option::HuobiOptionRestClient::fetch_l2_snapshot,
        _ => panic!("Binance unknown market_type: {market_type}"),
    };

    func(symbol)
}

pub(crate) fn fetch_open_interest(market_type: MarketType, symbol: Option<&str>) -> Result<String> {
    let func = match market_type {
        MarketType::InverseFuture => huobi_future::HuobiFutureRestClient::fetch_open_interest,
        MarketType::LinearSwap => huobi_linear_swap::HuobiLinearSwapRestClient::fetch_open_interest,
        MarketType::InverseSwap => {
            huobi_inverse_swap::HuobiInverseSwapRestClient::fetch_open_interest
        }
        _ => panic!("Huobi {market_type} does not have open interest"),
    };

    func(symbol)
}
