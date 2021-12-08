mod zbg_spot;
mod zbg_swap;

use crypto_market_type::MarketType;

use crate::{OrderBookMsg, TradeMsg};

use simple_error::SimpleError;

pub(crate) fn extract_symbol(market_type: MarketType, msg: &str) -> Result<String, SimpleError> {
    if market_type == MarketType::Spot {
        zbg_spot::extract_symbol(msg)
    } else {
        zbg_swap::extract_symbol(market_type, msg)
    }
}

pub(crate) fn parse_trade(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<TradeMsg>, SimpleError> {
    if market_type == MarketType::Spot {
        zbg_spot::parse_trade(msg)
    } else {
        zbg_swap::parse_trade(market_type, msg)
    }
}

pub(crate) fn parse_l2(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<OrderBookMsg>, SimpleError> {
    if market_type == MarketType::Spot {
        zbg_spot::parse_l2(msg)
    } else {
        zbg_swap::parse_l2(market_type, msg)
    }
}
