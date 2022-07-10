mod kraken_futures;
mod kraken_spot;

use std::collections::HashMap;

use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;

use crate::{BboMsg, OrderBookMsg, TradeMsg};

use serde_json::Value;
use simple_error::SimpleError;

pub(crate) fn extract_symbol(market_type: MarketType, msg: &str) -> Result<String, SimpleError> {
    match market_type {
        MarketType::Spot => kraken_spot::extract_symbol(msg),
        MarketType::InverseFuture | MarketType::InverseSwap => kraken_futures::extract_symbol(msg),
        _ => panic!("Kraken unknown market_type: {}", market_type),
    }
}

pub(crate) fn extract_timestamp(
    market_type: MarketType,
    msg: &str,
) -> Result<Option<i64>, SimpleError> {
    match market_type {
        MarketType::Spot => kraken_spot::extract_timestamp(msg),
        MarketType::InverseFuture | MarketType::InverseSwap => {
            kraken_futures::extract_timestamp(msg)
        }
        _ => panic!("Kraken unknown market_type: {}", market_type),
    }
}

pub(crate) fn get_msg_type(msg: &str) -> MessageType {
    if let Ok(arr) = serde_json::from_str::<Vec<Value>>(msg) {
        // spot
        let channel = arr[arr.len() - 2].as_str().unwrap();
        if channel == "ticker" {
            MessageType::Ticker
        } else if channel == "trade" {
            MessageType::Trade
        } else if channel == "spread" {
            MessageType::BBO
        } else if channel.starts_with("book-") {
            MessageType::L2Event
        } else if channel.starts_with("ohlc-") {
            MessageType::Candlestick
        } else {
            MessageType::Other
        }
    } else if let Ok(obj) = serde_json::from_str::<HashMap<String, Value>>(msg) {
        // futures
        if let Some(feed) = obj.get("feed") {
            match feed.as_str().unwrap() {
                "trade" | "trade_snapshot" => MessageType::Trade,
                "ticker" => MessageType::Ticker,
                "book" | "book_snapshot" => MessageType::L2Event,
                _ => MessageType::Other,
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
    match market_type {
        MarketType::Spot => kraken_spot::parse_trade(msg),
        MarketType::InverseFuture | MarketType::InverseSwap => kraken_futures::parse_trade(msg),
        _ => panic!("Kraken unknown market_type: {}", market_type),
    }
}

pub(crate) fn parse_l2(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<OrderBookMsg>, SimpleError> {
    match market_type {
        MarketType::Spot => kraken_spot::parse_l2(msg),
        MarketType::InverseFuture | MarketType::InverseSwap => kraken_futures::parse_l2(msg),
        _ => panic!("Kraken unknown market_type: {}", market_type),
    }
}


pub(crate) fn parse_bbo(
    market_type: MarketType,
    msg: &str,
    received_at: Option<i64>,
) -> Result<BboMsg, SimpleError> {
    match market_type {

        MarketType::Spot => kraken_spot::parse_bbo_spot(market_type, msg, received_at),
        _ => Err(SimpleError::new("Not implemented")),
    }
}

