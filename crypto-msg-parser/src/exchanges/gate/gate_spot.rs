use crate::{OrderBookMsg, TradeMsg};

use serde_json::Value;
use simple_error::SimpleError;
use std::collections::HashMap;

use super::{gate_spot_20210916, gate_spot_current};

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
