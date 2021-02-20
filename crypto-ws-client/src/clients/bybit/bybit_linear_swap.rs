use crate::WSClient;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use super::super::ws_client_internal::{MiscMessage, WSClientInternal};
use super::super::{Candlestick, OrderBook, OrderBookSnapshot, Ticker, Trade, BBO};

use log::*;
use serde_json::Value;

pub(super) const EXCHANGE_NAME: &str = "bybit";

const WEBSOCKET_URL: &str = "wss://stream.bybit.com/realtime_public";

const PING_INTERVAL_AND_MSG: (u64, &str) = (60, r#"{"op":"ping"}"#);

/// Bybit LinearSwap market.
///
/// * WebSocket API doc: <https://bybit-exchange.github.io/docs/inverse/#t-websocket>
/// * Trading at: <https://www.bybit.com/trade/inverse/>
pub struct BybitLinearSwapWSClient<'a> {
    client: WSClientInternal<'a>,
}

fn channels_to_commands(channels: &[String], subscribe: bool) -> Vec<String> {
    let mut all_commands: Vec<String> = channels
        .iter()
        .filter(|ch| ch.starts_with('{'))
        .map(|s| s.to_string())
        .collect();

    let channels_to_parse: Vec<&String> =
        channels.iter().filter(|ch| !ch.starts_with('{')).collect();
    if !channels_to_parse.is_empty() {
        all_commands.append(&mut vec![format!(
            r#"{{"op":"{}","args":{}}}"#,
            if subscribe {
                "subscribe"
            } else {
                "unsubscribe"
            },
            serde_json::to_string(&channels_to_parse).unwrap()
        )])
    };

    all_commands
}

fn on_misc_msg(msg: &str) -> MiscMessage {
    let obj = serde_json::from_str::<HashMap<String, Value>>(&msg).unwrap();

    if obj.contains_key("topic") && obj.contains_key("data") {
        MiscMessage::Normal
    } else {
        if obj.contains_key("success") {
            if obj.get("success").unwrap().as_bool().unwrap() {
                info!("Received {} from {}", msg, EXCHANGE_NAME);
            } else {
                error!("Received {} from {}", msg, EXCHANGE_NAME);
                panic!("Received {} from {}", msg, EXCHANGE_NAME);
            }
        } else {
            warn!("Received {} from {}", msg, EXCHANGE_NAME);
        }
        MiscMessage::Misc
    }
}

fn to_raw_channel(channel: &str, pair: &str) -> String {
    format!("{}.{}", channel, pair)
}

#[rustfmt::skip]
impl_trait!(Trade, BybitLinearSwapWSClient, subscribe_trade, "trade", to_raw_channel);
#[rustfmt::skip]
impl_trait!(BBO, BybitLinearSwapWSClient, subscribe_bbo, "orderBookL2_25", to_raw_channel);
#[rustfmt::skip]
impl_trait!(OrderBook, BybitLinearSwapWSClient, subscribe_orderbook, "orderBook_200.100ms", to_raw_channel);

impl<'a> OrderBookSnapshot for BybitLinearSwapWSClient<'a> {
    fn subscribe_orderbook_snapshot(&self, _pairs: &[String]) {
        panic!("FTX does NOT have orderbook snapshot channel");
    }
}

impl<'a> Ticker for BybitLinearSwapWSClient<'a> {
    fn subscribe_ticker(&self, _pairs: &[String]) {
        panic!("FTX does NOT have ticker channel");
    }
}

fn to_candlestick_raw_channel(pair: &str, interval: u32) -> String {
    let interval_str = match interval {
        60 => "1",
        180 => "3",
        300 => "5",
        900 => "15",
        1800 => "30",
        3600 => "60",
        7200 => "120",
        14400 => "240",
        21600 => "360",
        86400 => "D",
        604800 => "W",
        2592000 => "M",
        _ => panic!("Huobi has intervals 1min,5min,15min,30min,60min,4hour,1day,1week,1mon"),
    };
    format!("candle.{}.{}", interval_str, pair)
}

impl_candlestick!(BybitLinearSwapWSClient);

define_client!(
    BybitLinearSwapWSClient,
    EXCHANGE_NAME,
    WEBSOCKET_URL,
    channels_to_commands,
    on_misc_msg,
    Some(PING_INTERVAL_AND_MSG)
);
