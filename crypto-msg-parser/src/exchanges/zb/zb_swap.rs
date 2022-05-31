use crypto_market_type::MarketType;

use crate::{OrderBookMsg, TradeMsg};

use serde_json::Value;
use simple_error::SimpleError;
use std::collections::HashMap;

pub(super) fn extract_timestamp(
    _market_type: MarketType,
    msg: &str,
) -> Result<Option<i64>, SimpleError> {
    let obj = serde_json::from_str::<HashMap<String, Value>>(msg).map_err(|_e| {
        SimpleError::new(format!(
            "Failed to deserialize {} to HashMap<String, Value>",
            msg
        ))
    })?;
    let raw_channel = obj["channel"].as_str().unwrap();
    let channel = raw_channel.split('.').nth(1).unwrap();
    match channel {
        "Trade" => {
            let timestamp = obj["data"]
                .as_array()
                .unwrap()
                .iter()
                .map(|x| x[3].as_i64().unwrap())
                .max();
            if let Some(timestamp) = timestamp {
                Ok(Some(timestamp * 1000))
            } else {
                Err(SimpleError::new(format!("data is empty in {}", msg)))
            }
        }
        "Depth" | "DepthWhole" => Ok(obj["data"]
            .get("time")
            .map(|x| x.as_str().unwrap().parse::<i64>().unwrap())),
        "Ticker" => {
            if raw_channel == "All.Ticker" {
                let m = obj["data"].as_object().unwrap();
                let timestamp = m.values().map(|x| x[6].as_i64().unwrap()).max();
                if let Some(timestamp) = timestamp {
                    Ok(Some(timestamp * 1000))
                } else {
                    Err(SimpleError::new(format!("data is empty in {}", msg)))
                }
            } else {
                let timestamp = obj["data"].as_array().unwrap()[6].as_i64().unwrap();
                Ok(Some(timestamp * 1000))
            }
        }
        _ => {
            if channel.starts_with("KLine_") {
                let timestamp = obj["data"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|x| x.as_array().unwrap()[5].as_i64().unwrap())
                    .max();
                if let Some(timestamp) = timestamp {
                    Ok(Some(timestamp * 1000))
                } else {
                    Err(SimpleError::new(format!("data is empty in {}", msg)))
                }
            } else {
                Err(SimpleError::new(format!(
                    "Failed to extract timestamp from {}",
                    msg
                )))
            }
        }
    }
}

pub(super) fn parse_trade(_msg: &str) -> Result<Vec<TradeMsg>, SimpleError> {
    Err(SimpleError::new("Not implemented"))
}

pub(crate) fn parse_l2(_msg: &str) -> Result<Vec<OrderBookMsg>, SimpleError> {
    Err(SimpleError::new("Not implemented"))
}
