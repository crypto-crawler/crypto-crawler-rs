mod mxc_spot;
mod mxc_swap;

use std::collections::HashMap;

use crypto_market_type::MarketType;

use crate::{OrderBookMsg, TradeMsg};

use serde_json::Value;
use simple_error::SimpleError;

pub(crate) fn extract_symbol(market_type_: MarketType, msg: &str) -> Result<String, SimpleError> {
    if market_type_ == MarketType::Spot {
        let arr = serde_json::from_str::<Vec<Value>>(msg).map_err(|_e| {
            SimpleError::new(format!("Failed to deserialize {} to Vec<Value>", msg))
        })?;
        Ok(arr[1]["symbol"].as_str().unwrap().to_string())
    } else {
        let json_obj = serde_json::from_str::<HashMap<String, Value>>(msg).map_err(|_e| {
            SimpleError::new(format!(
                "Failed to deserialize {} to HashMap<String, Value>",
                msg
            ))
        })?;
        Ok(json_obj
            .get("symbol")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string())
    }
}

pub(crate) fn parse_trade(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<TradeMsg>, SimpleError> {
    if market_type == MarketType::Spot {
        mxc_spot::parse_trade(msg)
    } else {
        mxc_swap::parse_trade(market_type, msg)
    }
}

pub(crate) fn parse_l2(
    market_type: MarketType,
    msg: &str,
    timestamp: Option<i64>,
) -> Result<Vec<OrderBookMsg>, SimpleError> {
    if market_type == MarketType::Spot {
        mxc_spot::parse_l2(
            msg,
            timestamp.expect("MXC Spot orderbook messages don't have timestamp"),
        )
    } else {
        mxc_swap::parse_l2(market_type, msg)
    }
}
