mod before20220429;
mod bitget_mix;

use std::collections::HashMap;

use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{FundingRateMsg, OrderBookMsg, TradeMsg};

use simple_error::SimpleError;

const EXCHANGE_NAME: &str = "bitget";

pub(crate) fn extract_symbol(market_type: MarketType, msg: &str) -> Result<String, SimpleError> {
    let obj = serde_json::from_str::<HashMap<String, Value>>(msg)
        .map_err(|_e| SimpleError::new(format!("Failed to parse JSON string {}", msg)))?;
    if obj.contains_key("data") && obj.contains_key("table") {
        before20220429::extract_symbol(market_type, msg)
    } else if obj.contains_key("data") && obj.contains_key("arg") {
        bitget_mix::extract_symbol(msg)
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
    let obj = serde_json::from_str::<HashMap<String, Value>>(msg)
        .map_err(|_e| SimpleError::new(format!("Failed to parse JSON string {}", msg)))?;
    if obj.contains_key("data") && obj.contains_key("table") {
        before20220429::extract_timestamp(market_type, msg)
    } else if obj.contains_key("data") && obj.contains_key("arg") {
        bitget_mix::extract_timestamp(msg)
    } else {
        Err(SimpleError::new(format!(
            "Failed to extract timestamp from {}",
            msg
        )))
    }
}

pub(crate) fn get_msg_type(msg: &str) -> MessageType {
    let obj = serde_json::from_str::<HashMap<String, Value>>(msg)
        .map_err(|_e| SimpleError::new(format!("Failed to parse JSON string {}", msg)))
        .unwrap();
    if obj.contains_key("data") && obj.contains_key("table") {
        before20220429::get_msg_type(msg)
    } else if obj.contains_key("data") && obj.contains_key("arg") {
        let channel = obj["arg"]["channel"].as_str().unwrap();
        match channel {
            "trade" => MessageType::Trade,
            "books" => MessageType::L2Event,
            "books5" | "books15" => MessageType::L2TopK,
            "ticker" => MessageType::Ticker,
            _ => {
                if channel.starts_with("candle") {
                    MessageType::Candlestick
                } else {
                    MessageType::Other
                }
            }
        }
    } else {
        MessageType::Other
    }
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct Arg {
    instType: String,
    channel: String,
    instId: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct WebsocketMsg<T: Sized> {
    action: String,
    arg: Arg,
    data: Vec<T>,
}

pub(crate) fn parse_trade(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<TradeMsg>, SimpleError> {
    let obj = serde_json::from_str::<HashMap<String, Value>>(msg)
        .map_err(|_e| SimpleError::new(format!("Failed to parse JSON string {}", msg)))?;
    if obj.contains_key("data") && obj.contains_key("table") {
        before20220429::parse_trade(market_type, msg)
    } else if obj.contains_key("data") && obj.contains_key("arg") {
        bitget_mix::parse_trade(msg)
    } else {
        Err(SimpleError::new(format!(
            "Unsupported Trade message {}",
            msg
        )))
    }
}

pub(crate) fn parse_l2(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<OrderBookMsg>, SimpleError> {
    let obj = serde_json::from_str::<HashMap<String, Value>>(msg)
        .map_err(|_e| SimpleError::new(format!("Failed to parse JSON string {}", msg)))?;
    if obj.contains_key("data") && obj.contains_key("table") {
        before20220429::parse_l2(market_type, msg)
    } else if obj.contains_key("data") && obj.contains_key("arg") {
        bitget_mix::parse_l2(msg)
    } else {
        Err(SimpleError::new(format!(
            "Unsupported L2Event message {}",
            msg
        )))
    }
}

pub(crate) fn parse_l2_topk(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<OrderBookMsg>, SimpleError> {
    match parse_l2(market_type, msg) {
        Ok(mut orderbooks) => {
            for ob in orderbooks.iter_mut() {
                ob.snapshot = true;
                ob.msg_type = MessageType::L2TopK;
            }
            Ok(orderbooks)
        }
        Err(err) => Err(err),
    }
}

pub(crate) fn parse_funding_rate(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<FundingRateMsg>, SimpleError> {
    let obj = serde_json::from_str::<HashMap<String, Value>>(msg)
        .map_err(|_e| SimpleError::new(format!("Failed to parse JSON string {}", msg)))?;
    if obj.contains_key("data") && obj.contains_key("table") {
        before20220429::parse_funding_rate(market_type, msg)
    } else if obj.contains_key("data") && obj.contains_key("arg") {
        Err(SimpleError::new("Not implemented"))
    } else {
        Err(SimpleError::new(format!(
            "Unsupported FundingRate message {}",
            msg
        )))
    }
}
