use crate::WSClient;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use super::super::ws_client_internal::{MiscMessage, WSClientInternal};
use super::super::{Candlestick, OrderBook, OrderBookTopK, Ticker, Trade, BBO};

use log::*;
use serde_json::Value;

const EXCHANGE_NAME: &str = "bitget";

const WEBSOCKET_URL: &str = "wss://csocketapi.bitget.com/ws/v1";

// https://bitgetlimited.github.io/apidoc/en/swap/#brief-introduction
// System will auto-disconnect while subscription has been done within 30sec
// or no ping command sent by user after 30sec after ws is connected
const CLIENT_PING_INTERVAL_AND_MSG: (u64, &str) = (30, "ping");

/// The WebSocket client for Bitget swap markets.
///
/// User has the opinion to subscribe 1 or more channels, total length of multiple channel can not exceeds 4096 bytes.
///
/// * WebSocket API doc: <https://bitgetlimited.github.io/apidoc/en/swap/#websocketapi>
/// * Trading at: <https://www.bitget.com/en/swap/>
pub struct BitgetSwapWSClient<'a> {
    client: WSClientInternal<'a>,
}

fn channels_to_commands(channels: &[String], subscribe: bool) -> Vec<String> {
    let mut all_commands: Vec<String> = channels
        .iter()
        .filter(|ch| ch.starts_with('{'))
        .map(|s| s.to_string())
        .collect();

    let raw_channels: Vec<&String> = channels.iter().filter(|ch| !ch.starts_with('{')).collect();
    if !raw_channels.is_empty() {
        all_commands.append(&mut vec![format!(
            r#"{{"op":"{}","args":{}}}"#,
            if subscribe {
                "subscribe"
            } else {
                "unsubscribe"
            },
            serde_json::to_string(&raw_channels).unwrap()
        )])
    };

    all_commands
}

fn on_misc_msg(msg: &str) -> MiscMessage {
    if msg == "pong" {
        return MiscMessage::Pong;
    }
    let obj = serde_json::from_str::<HashMap<String, Value>>(msg).unwrap();

    if obj.contains_key("event") {
        let event = obj.get("event").unwrap().as_str().unwrap();
        if event == "error" {
            error!("Received {} from {}", msg, EXCHANGE_NAME);
            panic!("Received {} from {}", msg, EXCHANGE_NAME);
        } else {
            info!("Received {} from {}", msg, EXCHANGE_NAME);
            MiscMessage::Misc
        }
    } else if obj.contains_key("table") && obj.contains_key("data") {
        MiscMessage::Normal
    } else if obj.contains_key("action") {
        let action = obj.get("action").unwrap().as_str().unwrap();
        if action == "ping" {
            info!("Received {} from {}", msg, EXCHANGE_NAME);
        } else {
            warn!("Received {} from {}", msg, EXCHANGE_NAME);
        }
        MiscMessage::Misc
    } else {
        warn!("Received {} from {}", msg, EXCHANGE_NAME);
        MiscMessage::Misc
    }
}

fn to_raw_channel(channel: &str, symbol: &str) -> String {
    format!("swap/{}:{}", channel, symbol)
}

#[rustfmt::skip]
impl_trait!(Trade, BitgetSwapWSClient, subscribe_trade, "trade", to_raw_channel);
#[rustfmt::skip]
impl_trait!(OrderBookTopK, BitgetSwapWSClient, subscribe_orderbook_topk, "depth5", to_raw_channel);
#[rustfmt::skip]
impl_trait!(OrderBook, BitgetSwapWSClient, subscribe_orderbook, "depth", to_raw_channel);
#[rustfmt::skip]
impl_trait!(Ticker, BitgetSwapWSClient, subscribe_ticker, "ticker", to_raw_channel);

impl<'a> BBO for BitgetSwapWSClient<'a> {
    fn subscribe_bbo(&self, _symbols: &[String]) {
        panic!("Bitget does NOT have BBO channel");
    }
}

fn to_candlestick_raw_channel(symbol: &str, interval: usize) -> String {
    let valid_set: Vec<usize> = vec![60, 300, 900, 1800, 3600, 14400, 43200, 86400, 604800];
    if !valid_set.contains(&interval) {
        let joined = valid_set
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",");
        panic!("Bitget available intervals: {}", joined);
    }
    let channel = format!("candle{}s", interval);
    to_raw_channel(&channel, symbol)
}

impl_candlestick!(BitgetSwapWSClient);

define_client!(
    BitgetSwapWSClient,
    EXCHANGE_NAME,
    WEBSOCKET_URL,
    channels_to_commands,
    on_misc_msg,
    Some(CLIENT_PING_INTERVAL_AND_MSG),
    None
);
