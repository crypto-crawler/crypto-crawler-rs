mod okx_v3;
mod okx_v5;

use simple_error::SimpleError;
use std::collections::HashMap;

use crate::{BboMsg, FundingRateMsg, OrderBookMsg, TradeMsg, KlineMsg};
use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;
use serde_json::Value;

pub(super) const EXCHANGE_NAME: &str = "okx";

pub(crate) fn extract_symbol(_market_type: MarketType, msg: &str) -> Result<String, SimpleError> {
    let obj = serde_json::from_str::<HashMap<String, Value>>(msg).map_err(SimpleError::from)?;
    if obj.contains_key("arg") && obj.contains_key("data") {
        okx_v5::extract_symbol(msg)
    } else if obj.contains_key("table") && obj.contains_key("data") {
        okx_v3::extract_symbol(msg)
    } else if obj.contains_key("code") && obj.contains_key("msg") && obj.contains_key("data") {
        // restful v5
        okx_v5::extract_symbol(msg)
    } else {
        // restful v3
        okx_v3::extract_symbol(msg)
    }
}

pub(crate) fn extract_timestamp(
    _market_type: MarketType,
    msg: &str,
) -> Result<Option<i64>, SimpleError> {
    let obj = serde_json::from_str::<HashMap<String, Value>>(msg).map_err(SimpleError::from)?;
    if obj.contains_key("arg") && obj.contains_key("data") {
        // websocket v5
        okx_v5::extract_timestamp(msg)
    } else if obj.contains_key("table") && obj.contains_key("data") {
        // websocket v3
        okx_v3::extract_timestamp(msg)
    } else if obj.contains_key("code") && obj.contains_key("msg") && obj.contains_key("data") {
        // restful v5
        okx_v5::extract_timestamp(msg)
    } else {
        // restful v3
        okx_v3::extract_timestamp(msg)
    }
}

pub(crate) fn get_msg_type(msg: &str) -> MessageType {
    let obj = serde_json::from_str::<HashMap<String, Value>>(msg)
        .map_err(SimpleError::from)
        .unwrap();
    if obj.contains_key("arg") && obj.contains_key("data") {
        // websocket v5
        okx_v5::get_msg_type(msg)
    } else if obj.contains_key("table") && obj.contains_key("data") {
        // websocket v3
        okx_v3::get_msg_type(msg)
    } else if obj.contains_key("code") && obj.contains_key("msg") && obj.contains_key("data") {
        // restful v5
        okx_v5::get_msg_type(msg)
    } else {
        // restful v3
        okx_v3::get_msg_type(msg)
    }
}

pub(crate) fn parse_trade(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<TradeMsg>, SimpleError> {
    let obj = serde_json::from_str::<HashMap<String, Value>>(msg)
        .map_err(SimpleError::from)
        .unwrap();
    if obj.contains_key("arg") && obj.contains_key("data") {
        okx_v5::parse_trade(market_type, msg)
    } else if obj.contains_key("table") && obj.contains_key("data") {
        okx_v3::parse_trade(market_type, msg)
    } else {
        panic!("Unknown msg format {}", msg)
    }
}

pub(crate) fn parse_l2(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<OrderBookMsg>, SimpleError> {
    let obj = serde_json::from_str::<HashMap<String, Value>>(msg)
        .map_err(SimpleError::from)
        .unwrap();
    if obj.contains_key("arg") && obj.contains_key("data") {
        okx_v5::parse_l2(market_type, msg)
    } else if obj.contains_key("table") && obj.contains_key("data") {
        okx_v3::parse_l2(market_type, msg)
    } else {
        panic!("Unknown msg format {}", msg)
    }
}

pub(crate) fn parse_l2_topk(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<OrderBookMsg>, SimpleError> {
    parse_l2(market_type, msg)
}

pub(crate) fn parse_funding_rate(
    market_type: MarketType,
    msg: &str,
    received_at: i64,
) -> Result<Vec<FundingRateMsg>, SimpleError> {
    let obj = serde_json::from_str::<HashMap<String, Value>>(msg).map_err(SimpleError::from)?;
    if obj.contains_key("arg") && obj.contains_key("data") {
        okx_v5::parse_funding_rate(market_type, msg, received_at)
    } else if obj.contains_key("table") && obj.contains_key("data") {
        okx_v3::parse_funding_rate(market_type, msg, received_at)
    } else {
        panic!("Unknown msg format {}", msg)
    }
}

pub(crate) fn parse_bbo(
    market_type: MarketType,
    msg: &str,
) -> Result<BboMsg, SimpleError> {
    okx_v5::parse_bbo(market_type, msg)
}

pub(crate) fn parse_candlestick(
    market_type: MarketType,
    msg: &str,
    message_type: MessageType
) -> Result<KlineMsg, SimpleError> {
    okx_v5::parse_candlestick(market_type, msg, message_type)
}
