mod binance_all;
mod binance_option;

use crypto_market_type::MarketType;

use crate::{FundingRateMsg, OrderBookMsg, TradeMsg};

use serde_json::Result;

pub(crate) fn parse_trade(market_type: MarketType, msg: &str) -> Result<Vec<TradeMsg>> {
    if market_type == MarketType::Option {
        binance_option::parse_trade(msg)
    } else {
        binance_all::parse_trade(market_type, msg)
    }
}

pub(crate) fn parse_funding_rate(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<FundingRateMsg>> {
    if market_type == MarketType::InverseSwap || market_type == MarketType::LinearSwap {
        binance_all::parse_funding_rate(market_type, msg)
    } else {
        panic!("Binance {} does NOT have funding rates", market_type);
    }
}

pub(crate) fn parse_l2(_market_type: MarketType, _msg: &str) -> Result<Vec<OrderBookMsg>> {
    Ok(Vec::new())
}
