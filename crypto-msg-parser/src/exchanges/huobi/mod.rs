mod funding_rate;
mod huobi_inverse;
mod huobi_linear;
mod huobi_spot;

use crypto_market_type::MarketType;

use crate::{FundingRateMsg, OrderBookMsg, TradeMsg};

use serde_json::Result;

pub(crate) fn parse_trade(market_type: MarketType, msg: &str) -> Result<Vec<TradeMsg>> {
    match market_type {
        MarketType::Spot => huobi_spot::parse_trade(msg),
        MarketType::InverseFuture | MarketType::InverseSwap => {
            huobi_inverse::parse_trade(market_type, msg)
        }
        MarketType::LinearFuture | MarketType::LinearSwap | MarketType::EuropeanOption => {
            huobi_linear::parse_trade(market_type, msg)
        }
        _ => panic!("Unknown market type {}", market_type),
    }
}

pub(crate) fn parse_funding_rate(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<FundingRateMsg>> {
    if market_type == MarketType::InverseSwap || market_type == MarketType::LinearSwap {
        funding_rate::parse_funding_rate(market_type, msg)
    } else {
        panic!("Huobi {} does NOT have funding rates", market_type);
    }
}

pub(crate) fn parse_l2(_market_type: MarketType, _msg: &str) -> Result<Vec<OrderBookMsg>> {
    Ok(Vec::new())
}
