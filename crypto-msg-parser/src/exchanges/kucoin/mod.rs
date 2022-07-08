mod kucoin_spot;
mod kucoin_swap;
mod message;

use std::collections::HashMap;

use crypto_market_type::MarketType;

use crate::{OrderBookMsg, TradeMsg};

use serde_json::Value;
use simple_error::SimpleError;

use self::message::{RestfulMsg, WebsocketMsg};

pub(crate) fn extract_symbol(msg: &str) -> Result<String, SimpleError> {
    if let Ok(ws_msg) = serde_json::from_str::<WebsocketMsg<Value>>(msg) {
        // websocket
        let symbol = ws_msg.topic.split(':').last().unwrap();
        if ws_msg.topic.contains("/candle") {
            let pos = symbol.rfind('_').unwrap();
            Ok((symbol[..pos]).to_string())
        } else if symbol == "all" {
            Ok("ALL".to_string())
        } else {
            Ok(symbol.to_string())
        }
    } else if let Ok(rest_msg) = serde_json::from_str::<RestfulMsg<HashMap<String, Value>>>(msg) {
        // RESTful
        if rest_msg.code != "200000" {
            Err(SimpleError::new(format!("Error HTTP response {}", msg)))
        } else if let Some(symbol) = rest_msg.data.get("symbol") {
            Ok(symbol.as_str().unwrap().to_string())
        } else {
            Ok("NONE".to_string())
        }
    } else if let Ok(rest_msg) = serde_json::from_str::<RestfulMsg<Vec<Value>>>(msg) {
        // RESTful
        if rest_msg.code != "200000" {
            return Err(SimpleError::new(format!("Error HTTP response {}", msg)));
        }
        #[allow(clippy::comparison_chain)]
        if rest_msg.data.len() > 1 {
            Ok("ALL".to_string())
        } else if rest_msg.data.len() == 1 {
            Ok(rest_msg.data[0]["symbol"].as_str().unwrap().to_string())
        } else {
            Ok("NONE".to_string())
        }
    } else {
        Err(SimpleError::new(format!(
            "Unsupported message format {}",
            msg
        )))
    }
}

pub(crate) fn extract_timestamp(msg: &str) -> Result<Option<i64>, SimpleError> {
    if let Ok(ws_msg) = serde_json::from_str::<WebsocketMsg<HashMap<String, Value>>>(msg) {
        // websocket
        let topic = ws_msg.topic.as_str();
        if ws_msg.data.contains_key("timestamp") && ws_msg.data["timestamp"].is_i64() {
            Ok(Some(ws_msg.data["timestamp"].as_i64().unwrap()))
        } else if ws_msg.data.contains_key("ts") && ws_msg.data["ts"].is_i64() {
            Ok(Some(ws_msg.data["ts"].as_i64().unwrap() / 1000000))
        } else if let Some(t) = ws_msg.data.get("time") {
            if topic.starts_with("/market/match:") {
                Ok(Some(t.as_str().unwrap().parse::<i64>().unwrap() / 1000000))
            } else if topic.starts_with("/market/ticker")
                || topic.starts_with("/contractMarket/candle:")
            {
                Ok(Some(t.as_i64().unwrap()))
            } else if topic.starts_with("/market/candles:") {
                Ok(Some(t.as_i64().unwrap() / 1000000))
            } else {
                Err(SimpleError::new(format!(
                    "Failed to extract timestamp from {}",
                    msg
                )))
            }
        } else if topic.starts_with("/market/level2:") {
            Ok(None)
        } else if topic.starts_with("/market/snapshot:") {
            Ok(Some(ws_msg.data["data"]["datetime"].as_i64().unwrap()))
        } else {
            Err(SimpleError::new(format!(
                "Failed to extract timestamp from {}",
                msg
            )))
        }
    } else if let Ok(rest_msg) = serde_json::from_str::<RestfulMsg<HashMap<String, Value>>>(msg) {
        // RESTful
        if rest_msg.code != "200000" {
            Err(SimpleError::new(format!("Error HTTP response {}", msg)))
        } else if let Some(t) = rest_msg.data.get("time") {
            Ok(Some(t.as_i64().unwrap()))
        } else if let Some(t) = rest_msg.data.get("ts") {
            Ok(Some(t.as_i64().unwrap() / 1000000))
        } else {
            Err(SimpleError::new(format!(
                "Unsupported message format {}",
                msg
            )))
        }
    } else if let Ok(rest_msg) = serde_json::from_str::<RestfulMsg<Vec<Value>>>(msg) {
        // RESTful
        if rest_msg.code != "200000" {
            Err(SimpleError::new(format!("Error HTTP response {}", msg)))
        } else {
            Ok(None)
        }
    } else {
        Err(SimpleError::new(format!(
            "Unsupported message format {}",
            msg
        )))
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

pub(crate) fn parse_l2_topk(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<OrderBookMsg>, SimpleError> {
    if market_type == MarketType::Spot {
        kucoin_spot::parse_l2_topk(msg)
    } else {
        kucoin_swap::parse_l2_topk(market_type, msg)
    }
}
