mod gate_spot;
mod gate_spot_20210916;
mod gate_spot_current;
mod gate_swap;
mod messages;

use crypto_market_type::MarketType;
use crate::{BboMsg, OrderBookMsg, TradeMsg};
use simple_error::SimpleError;

const EXCHANGE_NAME: &str = "gate";

pub(crate) fn extract_symbol(market_type: MarketType, msg: &str) -> Result<String, SimpleError> {
    if market_type == MarketType::Spot {
        gate_spot::extract_symbol(msg)
    } else {
        gate_swap::extract_symbol(market_type, msg)
    }
}

pub(crate) fn extract_timestamp(
    market_type: MarketType,
    msg: &str,
) -> Result<Option<i64>, SimpleError> {
    if market_type == MarketType::Spot {
        gate_spot::extract_timestamp(msg)
    } else {
        gate_swap::extract_timestamp(msg)
    }
}

pub(crate) fn parse_trade(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<TradeMsg>, SimpleError> {
    if market_type == MarketType::Spot {
        gate_spot::parse_trade(msg)
    } else {
        gate_swap::parse_trade(market_type, msg)
    }
}

pub(crate) fn parse_l2(
    market_type: MarketType,
    msg: &str,
    timestamp: Option<i64>,
) -> Result<Vec<OrderBookMsg>, SimpleError> {
    if market_type == MarketType::Spot {
        gate_spot::parse_l2(
            msg,
            timestamp.expect("Gate spot orderbook messages don't have timestamp"),
        )
    } else {
        gate_swap::parse_l2(market_type, msg)
    }
}

pub(crate) fn parse_bbo(
    market_type: MarketType,
    msg: &str,
    received_at: Option<i64>,
) -> Result<BboMsg, SimpleError> {
    if market_type == MarketType::Spot {
        gate_spot::parse_bbo(market_type, msg, received_at)
    } else if market_type == MarketType::InverseSwap || market_type == MarketType::LinearSwap {
        gate_swap::parse_bbo(market_type, msg, received_at)
    } else {
        Err(SimpleError::new("Not implemented"))
    }
}
