mod bitget_swap;

use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;

use crate::{FundingRateMsg, OrderBookMsg, TradeMsg};

use simple_error::SimpleError;

pub(super) fn extract_symbol(_market_type: MarketType, msg: &str) -> Result<String, SimpleError> {
    bitget_swap::extract_symbol(msg)
}

pub(super) fn extract_timestamp(
    _market_type: MarketType,
    msg: &str,
) -> Result<Option<i64>, SimpleError> {
    bitget_swap::extract_timestamp(msg)
}

pub(super) fn get_msg_type(msg: &str) -> MessageType {
    bitget_swap::get_msg_type(msg)
}

pub(super) fn parse_trade(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<TradeMsg>, SimpleError> {
    if market_type == MarketType::Spot {
        Err(SimpleError::new("Not implemented"))
    } else {
        bitget_swap::parse_trade(market_type, msg)
    }
}

pub(super) fn parse_l2(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<OrderBookMsg>, SimpleError> {
    if market_type == MarketType::Spot {
        Err(SimpleError::new("Not implemented"))
    } else {
        bitget_swap::parse_l2(market_type, msg)
    }
}

pub(super) fn parse_funding_rate(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<FundingRateMsg>, SimpleError> {
    bitget_swap::parse_funding_rate(market_type, msg)
}
