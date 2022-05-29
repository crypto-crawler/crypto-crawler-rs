mod zb_spot;
mod zb_swap;

use std::collections::HashMap;

use crypto_market_type::MarketType;
use serde_json::Value;

use crate::{OrderBookMsg, TradeMsg};

use simple_error::SimpleError;

// const EXCHANGE_NAME: &str = "zb";

pub(crate) fn extract_symbol(_market_type: MarketType, msg: &str) -> Result<String, SimpleError> {
    let obj = serde_json::from_str::<HashMap<String, Value>>(msg).map_err(|_e| {
        SimpleError::new(format!(
            "Failed to deserialize {} to HashMap<String, Value>",
            msg
        ))
    })?;
    let channel = obj["channel"].as_str().unwrap();

    if channel.contains('.') {
        let symbol = channel.split('.').next().unwrap();
        Ok(symbol.to_string())
    } else if channel.contains('_') {
        let symbol = channel.split('_').next().unwrap();
        Ok(symbol.to_string())
    } else {
        Err(SimpleError::new(format!(
            "Failed to extract symbol from {}",
            msg
        )))
    }
}

pub(crate) fn extract_timestamp(
    market_type: MarketType,
    msg: &str,
) -> Result<Option<i64>, SimpleError> {
    if market_type == MarketType::Spot {
        zb_spot::extract_timestamp(msg)
    } else {
        zb_swap::extract_timestamp(market_type, msg)
    }
}

pub(crate) fn parse_trade(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<TradeMsg>, SimpleError> {
    if market_type == MarketType::Spot {
        zb_spot::parse_trade(msg)
    } else {
        zb_swap::parse_trade(msg)
    }
}

pub(crate) fn parse_l2(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<OrderBookMsg>, SimpleError> {
    if market_type == MarketType::Spot {
        zb_spot::parse_l2(msg)
    } else {
        zb_swap::parse_l2(msg)
    }
}
