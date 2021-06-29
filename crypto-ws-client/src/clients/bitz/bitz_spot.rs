use crate::{clients::utils::CHANNEL_PAIR_DELIMITER, WSClient};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

use super::super::ws_client_internal::{MiscMessage, WSClientInternal};
use super::super::{Candlestick, OrderBook, OrderBookSnapshot, Ticker, Trade, BBO};

use log::*;
use serde_json::Value;

const EXCHANGE_NAME: &str = "bitz";

const WEBSOCKET_URL: &str = "wss://wsapi.bitz.plus/";

// See https://apidocv2.bitz.plus/en/#heartbeat-and-persistent-connection-strategy
const CLIENT_PING_INTERVAL_AND_MSG: (u64, &str) = (10, "ping");

/// The WebSocket client for Bitz spot market.
///
/// * WebSocket API doc: <https://apidocv2.bitz.plus/en/#websocket-url>
/// * Trading at <https://www.bitz.plus/exchange>
pub struct BitzSpotWSClient<'a> {
    client: WSClientInternal<'a>,
}

fn pair_channels_to_command(pair: &str, channels: &[String], subscribe: bool) -> String {
    format!(
        r#"{{"action":"Topic.{}", "data":{{"symbol":"{}", "type":"{}", "_CDID":"100002", "dataType":"1"}}, "msg_id":{}}}"#,
        if subscribe { "sub" } else { "unsub" },
        pair,
        channels.join(","),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis(),
    )
}

fn channels_to_commands(channels: &[String], subscribe: bool) -> Vec<String> {
    let mut all_commands: Vec<String> = channels
        .iter()
        .filter(|ch| ch.starts_with('{'))
        .map(|s| s.to_string())
        .collect();

    let mut pair_channels = HashMap::<String, Vec<String>>::new();
    for s in channels.iter().filter(|ch| !ch.starts_with('{')) {
        let v: Vec<&str> = s.split(CHANNEL_PAIR_DELIMITER).collect();
        let channel = v[0];
        let pair = v[1];
        match pair_channels.get_mut(pair) {
            Some(channels) => channels.push(channel.to_string()),
            None => {
                pair_channels.insert(pair.to_string(), vec![channel.to_string()]);
            }
        }
    }

    for (pair, channels) in pair_channels.iter() {
        all_commands.push(pair_channels_to_command(pair, channels, subscribe));
    }

    all_commands
}

fn on_misc_msg(msg: &str) -> MiscMessage {
    if msg == "pong" {
        info!("Received {} from {}", msg, EXCHANGE_NAME);
        return MiscMessage::Pong;
    }
    let obj = serde_json::from_str::<HashMap<String, Value>>(&msg).unwrap();

    if obj.contains_key("action")
        && obj
            .get("action")
            .unwrap()
            .as_str()
            .unwrap()
            .starts_with("Pushdata.")
    {
        MiscMessage::Normal
    } else if obj.contains_key("status") {
        let status = obj.get("status").unwrap().as_i64().unwrap();
        // see https://apidocv2.bitz.plus/en/#error
        match status {
            -101001 => {
                error!("Subscription type parameter error: {}", msg);
                panic!("Subscription type parameter error: {}", msg);
            }
            -101002 => {
                error!("Fail to get subscribed symbol of trading pair: {}", msg);
                panic!("Fail to get subscribed symbol of trading pair: {}", msg);
            }
            -101003 => {
                error!("k-line scale resolution error: {}", msg);
                panic!("k-line scale resolution error: {}", msg);
            }
            _ => warn!("Received {} from {}", msg, EXCHANGE_NAME),
        }
        MiscMessage::Misc
    } else {
        warn!("Received {} from {}", msg, EXCHANGE_NAME);
        MiscMessage::Misc
    }
}

fn to_raw_channel(channel: &str, pair: &str) -> String {
    format!("{}{}{}", channel, CHANNEL_PAIR_DELIMITER, pair)
}

#[rustfmt::skip]
impl_trait!(Trade, BitzSpotWSClient, subscribe_trade, "order", to_raw_channel);
#[rustfmt::skip]
impl_trait!(OrderBook, BitzSpotWSClient, subscribe_orderbook, "depth", to_raw_channel);
#[rustfmt::skip]
impl_trait!(Ticker, BitzSpotWSClient, subscribe_ticker, "market", to_raw_channel);

impl<'a> BBO for BitzSpotWSClient<'a> {
    fn subscribe_bbo(&self, _pairs: &[String]) {
        panic!("Bitz does NOT have BBO channel");
    }
}

impl<'a> OrderBookSnapshot for BitzSpotWSClient<'a> {
    fn subscribe_orderbook_snapshot(&self, _pairs: &[String]) {
        panic!("Bitz does NOT have orderbook snapshot channel");
    }
}

fn to_candlestick_raw_channel(pair: &str, interval: u32) -> String {
    let interval_str = match interval {
        60 => "1min",
        300 => "5min",
        900 => "15min",
        1800 => "30min",
        3600 => "60min",
        14400 => "4hour",
        86400 => "1day",
        432000 => "5day",
        604800 => "1week",
        2592000 => "1mon",
        _ => panic!(
            "Bitz available intervals 1min,5min,15min,30min,60min,4hour,1day,5day,1week,1mon"
        ),
    };
    format!(
        r#"{{"action":"Topic.sub", "data":{{"symbol":"{}", "type":"kline", "resolution":"{}", "_CDID":"100002", "dataType":"1"}}, "msg_id":{}}}"#,
        pair,
        interval_str,
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis(),
    )
}

impl_candlestick!(BitzSpotWSClient);

define_client!(
    BitzSpotWSClient,
    EXCHANGE_NAME,
    WEBSOCKET_URL,
    channels_to_commands,
    on_misc_msg,
    Some(CLIENT_PING_INTERVAL_AND_MSG)
);
