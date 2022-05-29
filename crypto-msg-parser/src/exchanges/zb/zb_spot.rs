use std::collections::HashMap;

use crate::{OrderBookMsg, TradeMsg};

use serde_json::Value;
use simple_error::SimpleError;

pub(super) fn extract_timestamp(msg: &str) -> Result<Option<i64>, SimpleError> {
    let obj = serde_json::from_str::<HashMap<String, Value>>(msg).map_err(|_e| {
        SimpleError::new(format!(
            "Failed to deserialize {} to HashMap<String, Value>",
            msg
        ))
    })?;
    let raw_channel = obj["channel"].as_str().unwrap();
    let channel = raw_channel.split('_').nth(1).unwrap();
    match channel {
        "ticker" => Ok(Some(obj["date"].as_str().unwrap().parse::<i64>().unwrap())),
        "depth" => Ok(obj.get("timestamp").map(|x| x.as_i64().unwrap() * 1000)),
        "trades" => {
            let arr = obj["data"]
                .as_array()
                .unwrap()
                .iter()
                .map(|x| x["date"].as_i64().unwrap())
                .collect::<Vec<i64>>();
            let timestamp = arr.iter().fold(std::i64::MIN, |a, t| a.max(*t));
            if timestamp == std::i64::MIN {
                Err(SimpleError::new(format!("data is empty in {}", msg)))
            } else {
                Ok(Some(timestamp * 1000))
            }
        }
        "kline" => {
            let arr = obj["datas"]["data"]
                .as_array()
                .unwrap()
                .iter()
                .map(|x| x[0].as_i64().unwrap())
                .collect::<Vec<i64>>();
            let timestamp = arr.iter().fold(std::i64::MIN, |a, t| a.max(*t));
            if timestamp == std::i64::MIN {
                Err(SimpleError::new(format!("data is empty in {}", msg)))
            } else {
                Ok(Some(timestamp))
            }
        }
        _ => Err(SimpleError::new(format!(
            "Failed to extract timestamp from {}",
            msg
        ))),
    }
}

pub(super) fn parse_trade(_msg: &str) -> Result<Vec<TradeMsg>, SimpleError> {
    Err(SimpleError::new("Not implemented"))
}

pub(crate) fn parse_l2(_msg: &str) -> Result<Vec<OrderBookMsg>, SimpleError> {
    Err(SimpleError::new("Not implemented"))
}
