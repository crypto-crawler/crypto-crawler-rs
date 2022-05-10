mod kucoin_spot;
mod kucoin_swap;
mod message;

use crypto_market_type::MarketType;

use crate::{OrderBookMsg, TradeMsg};

use serde_json::Value;
use simple_error::SimpleError;

use self::message::WebsocketMsg;

pub(crate) fn extract_symbol(msg: &str) -> Result<String, SimpleError> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<Value>>(msg)
        .map_err(|_e| SimpleError::new(format!("Failed to deserialize {} to WebsocketMsg", msg)))?;
    let pos = ws_msg.topic.rfind(':').unwrap();
    let symbol = &ws_msg.topic[pos + 1..];
    Ok(symbol.to_string())
}

pub(crate) fn extract_timestamp(
    market_type: MarketType,
    msg: &str,
) -> Result<Option<i64>, SimpleError> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<Value>>(msg)
        .map_err(|_e| SimpleError::new(format!("Failed to deserialize {} to WebsocketMsg", msg)))?;
    let topic = ws_msg.topic.as_str();
    if market_type == MarketType::Spot {
        if topic.starts_with("/market/match:") {
            Ok(Some(
                ws_msg.data["time"]
                    .as_str()
                    .unwrap()
                    .parse::<i64>()
                    .unwrap()
                    / 1000000,
            ))
        } else if topic.starts_with("/market/level2:") {
            Ok(None)
        } else {
            Err(SimpleError::new(format!("Unknown Kucoin topic in {}", msg)))
        }
    } else {
        #[allow(clippy::collapsible_else_if)]
        if topic.starts_with("/contractMarket/execution:") {
            Ok(Some(ws_msg.data["ts"].as_i64().unwrap() / 1000000))
        } else if topic.starts_with("/contractMarket/level2:") {
            Ok(Some(ws_msg.data["timestamp"].as_i64().unwrap()))
        } else {
            Err(SimpleError::new(format!("Unknown Kucoin topic in {}", msg)))
        }
    }
}

pub(crate) fn parse_trade(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<TradeMsg>, SimpleError> {
    if market_type == MarketType::Spot {
        kucoin_spot::parse_trade(msg)
    } else {
        kucoin_swap::parse_trade(market_type, msg)
    }
}

pub(crate) fn parse_l2(
    market_type: MarketType,
    msg: &str,
    timestamp: Option<i64>,
) -> Result<Vec<OrderBookMsg>, SimpleError> {
    if market_type == MarketType::Spot {
        kucoin_spot::parse_l2(msg, timestamp.unwrap())
    } else {
        kucoin_swap::parse_l2(market_type, msg)
    }
}
