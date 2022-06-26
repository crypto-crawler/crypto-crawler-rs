
use crate::{BboMsg, OrderBookMsg, TradeMsg};

use serde_json::Value;
use simple_error::SimpleError;
use std::collections::HashMap;
use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;

use super::super::utils::calc_quantity_and_volume;
use super::{gate_spot_20210916, gate_spot_current, messages::WebsocketMsg, messages::RawBboMsg};
use super::EXCHANGE_NAME;

pub(super) fn parse_bbo(
    market_type: MarketType,
    msg: &str,
    received_at: Option<i64>,
) -> Result<BboMsg, SimpleError> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<RawBboMsg>>(msg).map_err(|_e| {
        SimpleError::new(format!(
            "Failed to deserialize {} to WebsocketMsg<RawBboMsg>",
            msg
        ))
    })?;
    debug_assert!(ws_msg.channel.ends_with("book_ticker"));

    let timestamp = if market_type == MarketType::Spot {
        received_at.unwrap()
    } else {
        ws_msg.result.t.unwrap()
    };
    let symbol = ws_msg.result.s.as_str();
    let pair = crypto_pair::normalize_pair(symbol, EXCHANGE_NAME).unwrap();

    let (ask_quantity_base, ask_quantity_quote, ask_quantity_contract) = calc_quantity_and_volume(
        EXCHANGE_NAME,
        market_type,
        &pair,
        ws_msg.result.a.parse::<f64>().unwrap(),
        ws_msg.result.A.parse::<f64>().unwrap(),
    );

    let (bid_quantity_base, bid_quantity_quote, bid_quantity_contract) = calc_quantity_and_volume(
        EXCHANGE_NAME,
        market_type,
        &pair,
        ws_msg.result.b.parse::<f64>().unwrap(),
        ws_msg.result.B.parse::<f64>().unwrap(),
    );

    let bbo_msg = BboMsg {
        exchange: EXCHANGE_NAME.to_string(),
        market_type,
        symbol: symbol.to_string(),
        pair,
        msg_type: MessageType::BBO,
        timestamp,
        ask_price: ws_msg.result.a.parse::<f64>().unwrap(),
        ask_quantity_base,
        ask_quantity_quote,
        ask_quantity_contract,
        bid_price: ws_msg.result.b.parse::<f64>().unwrap(),
        bid_quantity_base,
        bid_quantity_quote,
        bid_quantity_contract,
        id: Some(ws_msg.result.u),
        json: msg.to_string(),
    };
    Ok(bbo_msg)
}

pub(super) fn extract_symbol(msg: &str) -> Result<String, SimpleError> {
    let json_obj = serde_json::from_str::<HashMap<String, Value>>(msg).map_err(|_e| {
        SimpleError::new(format!(
            "Failed to deserialize {} to HashMap<String, Value>",
            msg
        ))
    })?;
    if json_obj.contains_key("params") {
        gate_spot_20210916::extract_symbol(msg)
    } else {
        gate_spot_current::extract_symbol(msg)
    }
}

pub(super) fn extract_timestamp(msg: &str) -> Result<Option<i64>, SimpleError> {
    let json_obj = serde_json::from_str::<HashMap<String, Value>>(msg).map_err(|_e| {
        SimpleError::new(format!(
            "Failed to deserialize {} to HashMap<String, Value>",
            msg
        ))
    })?;
    if json_obj.contains_key("params") {
        #[allow(deprecated)]
        gate_spot_20210916::extract_timestamp(msg)
    } else {
        gate_spot_current::extract_timestamp(msg)
    }
}

pub(super) fn parse_trade(msg: &str) -> Result<Vec<TradeMsg>, SimpleError> {
    let json_obj = serde_json::from_str::<HashMap<String, Value>>(msg).map_err(|_e| {
        SimpleError::new(format!(
            "Failed to deserialize {} to HashMap<String, Value>",
            msg
        ))
    })?;
    if json_obj.contains_key("params") {
        #[allow(deprecated)]
        gate_spot_20210916::parse_trade(msg)
    } else if json_obj.contains_key("result") {
        gate_spot_current::parse_trade(msg)
    } else {
        Err(SimpleError::new(format!("Unknown message format: {}", msg)))
    }
}

pub(crate) fn parse_l2(msg: &str, timestamp: i64) -> Result<Vec<OrderBookMsg>, SimpleError> {
    let json_obj = serde_json::from_str::<HashMap<String, Value>>(msg).map_err(|_e| {
        SimpleError::new(format!(
            "Failed to deserialize {} to HashMap<String, Value>",
            msg
        ))
    })?;
    if json_obj.contains_key("params") {
        #[allow(deprecated)]
        gate_spot_20210916::parse_l2(msg, timestamp)
    } else if json_obj.contains_key("result") {
        gate_spot_current::parse_l2(msg)
    } else {
        Err(SimpleError::new(format!("Unknown message format: {}", msg)))
    }
}
