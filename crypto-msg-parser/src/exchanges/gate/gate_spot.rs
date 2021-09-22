use crate::{OrderBookMsg, TradeMsg};

use serde_json::{Result, Value};
use std::collections::HashMap;

use super::{gate_spot_20210916, gate_spot_current};

pub(super) fn extract_symbol(msg: &str) -> Option<String> {
    let json_obj = serde_json::from_str::<HashMap<String, Value>>(msg).unwrap();
    if json_obj.contains_key("params") {
        gate_spot_20210916::extract_symbol(msg)
    } else if json_obj.contains_key("result") {
        gate_spot_current::extract_symbol(msg)
    } else {
        panic!("Unknown message format: {}", msg);
    }
}

pub(super) fn parse_trade(msg: &str) -> Result<Vec<TradeMsg>> {
    let json_obj = serde_json::from_str::<HashMap<String, Value>>(msg)?;
    if json_obj.contains_key("params") {
        #[allow(deprecated)]
        gate_spot_20210916::parse_trade(msg)
    } else if json_obj.contains_key("result") {
        gate_spot_current::parse_trade(msg)
    } else {
        panic!("Unknown message format: {}", msg);
    }
}

pub(crate) fn parse_l2(msg: &str, timestamp: i64) -> Result<Vec<OrderBookMsg>> {
    let json_obj = serde_json::from_str::<HashMap<String, Value>>(msg)?;
    if json_obj.contains_key("params") {
        #[allow(deprecated)]
        gate_spot_20210916::parse_l2(msg, timestamp)
    } else if json_obj.contains_key("result") {
        gate_spot_current::parse_l2(msg)
    } else {
        panic!("Unknown message format: {}", msg);
    }
}
