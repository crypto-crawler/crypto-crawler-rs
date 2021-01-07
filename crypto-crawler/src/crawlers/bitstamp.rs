use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use crate::{msg::Message, MarketType, MessageType};
use crypto_rest_client::*;
use crypto_ws_client::*;
use log::*;
use serde_json::Value;

const EXCHANGE_NAME: &str = "Bitstamp";

fn extract_symbol(json: &str) -> String {
    let obj = serde_json::from_str::<HashMap<String, Value>>(&json).unwrap();
    let channel = obj.get("channel").unwrap().as_str().unwrap();
    let pos = channel.rfind('_').unwrap();
    (&channel[(pos + 1)..]).to_string()
}

fn convert_to_message(json: String, market_type: MarketType, msg_type: MessageType) -> Message {
    Message::new(
        EXCHANGE_NAME.to_string(),
        market_type,
        extract_symbol(&json),
        msg_type,
        json,
    )
}

fn check_market_type(market_type: MarketType) {
    if market_type != MarketType::Spot {
        error!("Bitstamp has only Spot market");
        panic!("Bitstamp has only Spot market");
    }
}

pub(crate) fn crawl_trade<'a>(
    market_type: MarketType,
    symbols: &[String],
    mut on_msg: Box<dyn FnMut(Message) + 'a>,
    duration: Option<u64>,
) {
    check_market_type(market_type);
    let on_msg_ext = |msg: String| {
        let message = convert_to_message(msg.to_string(), market_type, MessageType::Trade);
        on_msg(message);
    };
    let mut ws_client = BitstampWSClient::new(Box::new(on_msg_ext), None);
    ws_client.subscribe_trade(symbols);
    ws_client.run(duration);
}

pub(crate) fn crawl_l2_event<'a>(
    market_type: MarketType,
    symbols: &[String],
    mut on_msg: Box<dyn FnMut(Message) + 'a>,
    duration: Option<u64>,
) {
    check_market_type(market_type);
    let on_msg_ext = |msg: String| {
        let message = convert_to_message(msg.to_string(), market_type, MessageType::L2Event);
        on_msg(message);
    };
    let mut ws_client = BitstampWSClient::new(Box::new(on_msg_ext), None);
    ws_client.subscribe_orderbook(symbols);
    ws_client.run(duration);
}

pub(crate) fn crawl_l3_event<'a>(
    market_type: MarketType,
    symbols: &[String],
    mut on_msg: Box<dyn FnMut(Message) + 'a>,
    duration: Option<u64>,
) {
    check_market_type(market_type);
    let on_msg_ext = |msg: String| {
        let message = convert_to_message(msg.to_string(), market_type, MessageType::L3Event);
        on_msg(message);
    };
    let mut ws_client = BitstampWSClient::new(Box::new(on_msg_ext), None);
    ws_client.subscribe_l3_orderbook(symbols);
    ws_client.run(duration);
}

pub(crate) fn crawl_l2_snapshot<'a>(
    market_type: MarketType,
    symbols: &[String],
    mut on_msg: Box<dyn FnMut(Message) + 'a>,
    duration: Option<u64>,
) {
    check_market_type(market_type);
    gen_crawl_snapshot!(
        market_type,
        symbols,
        on_msg,
        duration,
        MessageType::L2Snapshot,
        BitstampRestClient::fetch_l2_snapshot
    )
}

pub(crate) fn crawl_l3_snapshot<'a>(
    market_type: MarketType,
    symbols: &[String],
    mut on_msg: Box<dyn FnMut(Message) + 'a>,
    duration: Option<u64>,
) {
    check_market_type(market_type);
    gen_crawl_snapshot!(
        market_type,
        symbols,
        on_msg,
        duration,
        MessageType::L3Snapshot,
        BitstampRestClient::fetch_l3_snapshot
    )
}
