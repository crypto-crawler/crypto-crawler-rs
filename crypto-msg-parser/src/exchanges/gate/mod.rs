mod gate_spot;
mod gate_spot_20210916;
mod gate_spot_current;
mod gate_swap;
mod messages;

use crypto_market_type::MarketType;
use crypto_message::BboMsg;

use crate::{OrderBookMsg, TradeMsg};

use simple_error::SimpleError;

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
    match market_type {
        MarketType::Spot => gate_spot::parse_l2(
            msg,
            timestamp.expect("Gate spot orderbook messages don't have timestamp"),
        ),
        MarketType::InverseFuture | MarketType::LinearFuture => {
            gate_swap::parse_l2_topk(market_type, msg)
        }
        MarketType::InverseSwap | MarketType::LinearSwap => gate_swap::parse_l2(market_type, msg),
        _ => Err(SimpleError::new(format!(
            "Unsupported market type: {:?}",
            market_type
        ))),
    }
}

pub(crate) fn parse_l2_topk(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<OrderBookMsg>, SimpleError> {
    if market_type == MarketType::Spot {
        gate_spot::parse_l2_topk(msg)
    } else {
        gate_swap::parse_l2_topk(market_type, msg)
    }
}

pub(crate) fn parse_bbo(market_type: MarketType, msg: &str) -> Result<Vec<BboMsg>, SimpleError> {
    if market_type == MarketType::Spot {
        gate_spot::parse_bbo(msg)
    } else {
        gate_swap::parse_bbo(market_type, msg)
    }
}
