mod funding_rate;
mod huobi_inverse;
mod huobi_linear;
mod huobi_spot;
mod message;

use std::collections::HashMap;

use crypto_market_type::MarketType;
use crypto_message::BboMsg;
use crypto_msg_type::MessageType;

use crate::{FundingRateMsg, OrderBookMsg, TradeMsg};

use serde_json::Value;
use simple_error::SimpleError;

use message::WebsocketMsg;

pub(crate) fn extract_symbol(msg: &str) -> Result<String, SimpleError> {
    let json_obj = serde_json::from_str::<HashMap<String, Value>>(msg).unwrap();
    if json_obj.contains_key("data")
        && json_obj["data"].is_array()
        && json_obj["data"].as_array().unwrap().len() > 1
    {
        // open interest from RESTful API
        return Ok("ALL".to_string());
    }

    let channel = if json_obj.contains_key("ch") {
        json_obj["ch"].as_str().unwrap()
    } else if json_obj.contains_key("topic") {
        json_obj["topic"].as_str().unwrap()
    } else {
        return Err(SimpleError::new(format!(
            "No channel or topic found in {}",
            msg
        )));
    };
    if channel == "public.*.funding_rate" {
        Ok("ALL".to_string())
    } else {
        let symbol = channel.split('.').nth(1).unwrap();
        Ok(symbol.to_string())
    }
}

pub(crate) fn extract_timestamp(msg: &str) -> Result<Option<i64>, SimpleError> {
    let json_obj = serde_json::from_str::<HashMap<String, Value>>(msg).unwrap();
    Ok(Some(json_obj["ts"].as_i64().unwrap()))
}

pub(crate) fn get_msg_type(msg: &str) -> MessageType {
    if let Ok(ws_msg) = serde_json::from_str::<WebsocketMsg<Value>>(msg) {
        let channel = ws_msg.ch;
        if channel.ends_with("trade.detail") {
            MessageType::Trade
        } else if channel.ends_with("depth.size_20.high_freq")
            || channel.ends_with("depth.size_150.high_freq")
            || channel.ends_with("mbp.20")
        {
            MessageType::L2Event
        } else if channel.contains(".depth.step") {
            MessageType::L2TopK
        } else if channel.ends_with("bbo") {
            MessageType::BBO
        } else if channel.ends_with("detail") {
            MessageType::Ticker
        } else if channel.contains(".kline.") {
            MessageType::Candlestick
        } else if channel.ends_with(".funding_rate") {
            MessageType::FundingRate
        } else {
            MessageType::Other
        }
    } else if let Ok(ws_msg) = serde_json::from_str::<funding_rate::WebsocketMsg>(msg) {
        if ws_msg.topic.ends_with(".funding_rate") {
            MessageType::FundingRate
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
    match market_type {
        MarketType::Spot => huobi_spot::parse_trade(msg),
        MarketType::InverseFuture | MarketType::InverseSwap => {
            huobi_inverse::parse_trade(market_type, msg)
        }
        MarketType::LinearFuture | MarketType::LinearSwap | MarketType::EuropeanOption => {
            huobi_linear::parse_trade(market_type, msg)
        }
        _ => Err(SimpleError::new(format!(
            "Unknown huobi market type {}",
            market_type
        ))),
    }
}

pub(crate) fn parse_funding_rate(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<FundingRateMsg>, SimpleError> {
    if market_type == MarketType::InverseSwap || market_type == MarketType::LinearSwap {
        funding_rate::parse_funding_rate(market_type, msg)
    } else {
        Err(SimpleError::new(format!(
            "Huobi {} does NOT have funding rates",
            market_type
        )))
    }
}

pub(crate) fn parse_l2(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<OrderBookMsg>, SimpleError> {
    match market_type {
        MarketType::Spot => huobi_spot::parse_l2(msg),
        MarketType::InverseFuture | MarketType::InverseSwap => {
            huobi_inverse::parse_l2(market_type, msg)
        }
        MarketType::LinearFuture | MarketType::LinearSwap | MarketType::EuropeanOption => {
            huobi_inverse::parse_l2(market_type, msg)
        }
        _ => Err(SimpleError::new(format!(
            "Unknown huobi market type {}",
            market_type
        ))),
    }
}

pub(crate) fn parse_l2_topk(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<OrderBookMsg>, SimpleError> {
    parse_l2(market_type, msg)
}

pub(crate) fn parse_bbo(market_type: MarketType, msg: &str) -> Result<Vec<BboMsg>, SimpleError> {
    match market_type {
        MarketType::Spot => huobi_spot::parse_bbo(msg),
        MarketType::InverseFuture | MarketType::InverseSwap => {
            huobi_inverse::parse_bbo(market_type, msg)
        }
        MarketType::LinearFuture | MarketType::LinearSwap | MarketType::EuropeanOption => {
            huobi_inverse::parse_bbo(market_type, msg)
        }
        _ => Err(SimpleError::new(format!(
            "Unknown huobi market type {}",
            market_type
        ))),
    }
}
