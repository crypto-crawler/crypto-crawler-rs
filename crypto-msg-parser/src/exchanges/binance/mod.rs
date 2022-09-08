mod binance_all;
mod binance_option;
mod binance_spot;

use std::collections::HashMap;

use crypto_market_type::MarketType;
use crypto_message::CandlestickMsg;
use crypto_msg_type::MessageType;

use crate::{BboMsg, FundingRateMsg, OrderBookMsg, TradeMsg};

use serde_json::Value;
use simple_error::SimpleError;

const EXCHANGE_NAME: &str = "binance";

pub(crate) fn extract_symbol(msg: &str) -> Result<String, SimpleError> {
    let obj = serde_json::from_str::<HashMap<String, Value>>(msg)
        .map_err(|_e| SimpleError::new(format!("Failed to parse the JSON string {}", msg)))?;
    let stream = if obj.contains_key("stream") && obj["stream"].is_string() {
        obj["stream"].as_str().unwrap().to_string()
    } else {
        "".to_string()
    };
    if stream.starts_with('!') && stream.ends_with("@arr") {
        // for example: !ticker@arr, !markPrice@arr
        return Ok("ALL".to_string());
    }
    if stream.ends_with("_ALL") {
        return Ok("ALL".to_string());
    }

    let data = if obj.contains_key("stream") {
        serde_json::from_value::<HashMap<String, Value>>(obj["data"].clone()).unwrap()
    } else {
        obj
    };

    if data.contains_key("s") && data["s"].is_string() {
        let symbol = data["s"].as_str().unwrap();
        Ok(symbol.to_string())
    } else if data.contains_key("symbol") && data["symbol"].is_string() {
        let symbol = data["symbol"].as_str().unwrap();
        Ok(symbol.to_string())
    } else if stream.contains('@') && !(stream.starts_with('!') && stream.ends_with("@arr")) {
        let symbol = stream.split('@').next().unwrap();
        Ok(symbol.to_uppercase())
    } else if data.contains_key("lastUpdateId")
        && data.contains_key("asks")
        && data.contains_key("bids")
    {
        Ok("NONE".to_string())
    } else {
        Err(SimpleError::new(format!(
            "Failed to extract symbol from {}",
            msg
        )))
    }
}

pub(crate) fn extract_timestamp(msg: &str) -> Result<Option<i64>, SimpleError> {
    let obj = serde_json::from_str::<Value>(msg)
        .map_err(|_e| SimpleError::new(format!("Failed to parse the JSON string {}", msg)))?;
    let data = if let Some(data) = obj.get("data") {
        data
    } else {
        &obj
    };
    if data.is_object() {
        if let Some(e) = data.get("E") {
            Ok(Some(e.as_i64().unwrap()))
        } else if let Some(time) = data.get("time") {
            Ok(Some(time.as_i64().unwrap()))
        } else {
            Ok(None) // !bookTicker has no E field
        }
    } else if data.is_array() {
        let timestamp = data
            .as_array()
            .unwrap()
            .iter()
            .map(|x| x["E"].as_i64().unwrap())
            .max();
        Ok(timestamp)
    } else {
        Err(SimpleError::new(format!(
            "Failed to extract timestamp from {}",
            msg
        )))
    }
}

pub(crate) fn get_msg_type(msg: &str) -> MessageType {
    if let Ok(obj) = serde_json::from_str::<HashMap<String, Value>>(msg) {
        if let Some(stream) = obj.get("stream").unwrap().as_str() {
            if stream.ends_with("@aggTrade") {
                MessageType::Trade
            } else if stream.ends_with("@depth") || stream.ends_with("@depth@100ms") {
                MessageType::L2Event
            } else if stream.ends_with("@depth5")
                || stream.ends_with("@depth10")
                || stream.ends_with("depth20")
            {
                MessageType::L2TopK
            } else if stream.ends_with("@bookTicker") {
                MessageType::BBO
            } else if stream.ends_with("@ticker") {
                MessageType::Ticker
            } else if stream.contains("@kline_") {
                MessageType::Candlestick
            } else if stream.contains("markPrice") {
                MessageType::FundingRate
            } else {
                MessageType::Other
            }
        } else {
            MessageType::Other
        }
    } else {
        MessageType::Other
    }
}

pub(crate) fn parse_trade(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<TradeMsg>, SimpleError> {
    if market_type == MarketType::EuropeanOption {
        binance_option::parse_trade(msg)
    } else {
        binance_all::parse_trade(market_type, msg)
    }
}

pub(crate) fn parse_funding_rate(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<FundingRateMsg>, SimpleError> {
    binance_all::parse_funding_rate(market_type, msg)
}

pub(crate) fn parse_l2(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<OrderBookMsg>, SimpleError> {
    if market_type == MarketType::EuropeanOption {
        Ok(Vec::new())
    } else {
        binance_all::parse_l2(market_type, msg)
    }
}

pub(crate) fn parse_l2_topk(
    market_type: MarketType,
    msg: &str,
    received_at: Option<i64>,
) -> Result<Vec<OrderBookMsg>, SimpleError> {
    if market_type == MarketType::EuropeanOption {
        Err(SimpleError::new("Not implemented"))
    } else if market_type == MarketType::Spot {
        binance_spot::parse_l2_topk(msg, received_at)
    } else {
        binance_all::parse_l2_topk(market_type, msg)
    }
}

pub(crate) fn parse_bbo(
    market_type: MarketType,
    msg: &str,
    received_at: Option<i64>,
) -> Result<Vec<BboMsg>, SimpleError> {
    if market_type == MarketType::EuropeanOption {
        Err(SimpleError::new("Not implemented"))
    } else {
        binance_all::parse_bbo(market_type, msg, received_at)
    }
}

pub(crate) fn parse_candlestick(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<CandlestickMsg>, SimpleError> {
    match market_type {
        MarketType::Spot | MarketType::InverseSwap => binance_all::parse_candlestick(market_type, msg),
        _ => Err(SimpleError::new("Not implemented"))
    }
}
