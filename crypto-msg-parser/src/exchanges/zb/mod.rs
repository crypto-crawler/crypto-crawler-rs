mod zb_spot;
mod zb_swap;

use std::collections::HashMap;

use crypto_market_type::MarketType;
use serde_json::Value;

use crate::{OrderBookMsg, TradeMsg};

use simple_error::SimpleError;

const EXCHANGE_NAME: &str = "zb";

pub(crate) fn extract_symbol(_market_type: MarketType, msg: &str) -> Result<String, SimpleError> {
    let obj = serde_json::from_str::<HashMap<String, Value>>(msg)
        .map_err(|_e| SimpleError::new(format!("Failed to parse the JSON string {}", msg)))?;
    if let Some(channel) = obj.get("channel") {
        // websocket
        let channel = channel.as_str().unwrap();
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
    } else if obj.contains_key("asks") && obj.contains_key("bids") {
        // e.g., https://api.zbex.site/data/v1/depth?market=btc_usdt&size=50
        Ok("NONE".to_string())
    } else if obj.contains_key("code") && obj.contains_key("desc") && obj.contains_key("data") {
        // ZB linear_swap RESTful
        let data = obj["data"].as_object().unwrap();
        if let Some(symbol) = data.get("symbol") {
            Ok(symbol.as_str().unwrap().to_string())
        } else {
            Ok("NONE".to_string())
        }
    } else {
        Err(SimpleError::new(format!("Unknown message format {}", msg)))
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
        zb_swap::parse_trade(market_type, msg)
    }
}

pub(crate) fn parse_l2(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<OrderBookMsg>, SimpleError> {
    if market_type == MarketType::Spot {
        zb_spot::parse_l2(msg)
    } else {
        zb_swap::parse_l2(market_type, msg)
    }
}

pub(crate) fn parse_l2_topk(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<OrderBookMsg>, SimpleError> {
    if market_type == MarketType::Spot {
        zb_spot::parse_l2_topk(msg)
    } else {
        zb_swap::parse_l2(market_type, msg)
    }
}
