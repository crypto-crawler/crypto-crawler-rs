use crate::{clients::utils::CHANNEL_PAIR_DELIMITER, WSClient};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use super::super::ws_client_internal::{MiscMessage, WSClientInternal};
use super::super::{Candlestick, OrderBook, OrderBookSnapshot, Ticker, Trade, BBO};

use log::*;
use serde_json::Value;

const EXCHANGE_NAME: &str = "gate";

const WEBSOCKET_URL: &str = "wss://ws.gate.io/v3/";

const PING_INTERVAL_AND_MSG: (u64, &str) =
    (10, r#"{"id":9527,"method":"server.ping", "params":[]}"#);

/// The WebSocket client for Gate spot market.
///
/// * WebSocket API doc: <https://www.gate.io/docs/websocket/index.html>
/// * Trading at <https://www.gateio.pro/cn/trade/BTC_USDT>
pub struct GateSpotWSClient<'a> {
    client: WSClientInternal<'a>,
}

fn channel_pairs_to_command(channel: &str, pairs: &[String], subscribe: bool) -> String {
    let params: Vec<Value> = pairs
        .iter()
        .map(|pair| {
            if channel == "depth" {
                serde_json::json!([pair, 30, "0"])
            } else {
                serde_json::json!(pair)
            }
        })
        .collect();

    format!(
        r#"{{"id":9527, "method":"{}.{}", "params":{}}}"#,
        channel,
        if subscribe {
            "subscribe"
        } else {
            "unsubscribe"
        },
        serde_json::to_string(&params).unwrap(),
    )
}

fn channels_to_commands(channels: &[String], subscribe: bool) -> Vec<String> {
    let mut all_commands: Vec<String> = channels
        .iter()
        .filter(|ch| ch.starts_with('{'))
        .map(|s| s.to_string())
        .collect();

    let mut channel_pairs = HashMap::<String, Vec<String>>::new();
    for s in channels.iter().filter(|ch| !ch.starts_with('{')) {
        let v: Vec<&str> = s.split(CHANNEL_PAIR_DELIMITER).collect();
        let channel = v[0];
        let pair = v[1];
        match channel_pairs.get_mut(channel) {
            Some(pairs) => pairs.push(pair.to_string()),
            None => {
                channel_pairs.insert(channel.to_string(), vec![pair.to_string()]);
            }
        }
    }

    for (channel, pairs) in channel_pairs.iter() {
        all_commands.push(channel_pairs_to_command(channel, pairs, subscribe));
    }

    all_commands
}

fn on_misc_msg(msg: &str) -> MiscMessage {
    let obj = serde_json::from_str::<HashMap<String, Value>>(&msg).unwrap();

    // null for success; object with code and message for failure
    let error = match obj.get("error") {
        None => serde_json::Value::Null,
        Some(err) => {
            if err.is_null() {
                serde_json::Value::Null
            } else {
                err.clone()
            }
        }
    };
    if !error.is_null() {
        let err = error.as_object().unwrap();
        // see https://www.gatecn.io/docs/websocket/index.html#error
        let code = err.get("code").unwrap().as_i64().unwrap();
        match code {
            1 | 4 => panic!("Received {} from {}", msg, EXCHANGE_NAME),
            _ => error!("Received {} from {}", msg, EXCHANGE_NAME),
        }
        MiscMessage::Misc
    } else if obj.contains_key("method")
        && obj
            .get("method")
            .unwrap()
            .as_str()
            .unwrap()
            .ends_with(".update")
    {
        MiscMessage::Normal
    } else if obj.contains_key("result") {
        let result = obj.get("result").unwrap().clone();
        if result == serde_json::json!("pong") || result == serde_json::json!({"status": "success"})
        {
            debug!("Received {} from {}", msg, EXCHANGE_NAME);
        } else {
            warn!("Received {} from {}", msg, EXCHANGE_NAME);
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
impl_trait!(Trade, GateSpotWSClient, subscribe_trade, "trades", to_raw_channel);
#[rustfmt::skip]
impl_trait!(OrderBook, GateSpotWSClient, subscribe_orderbook, "depth", to_raw_channel);
#[rustfmt::skip]
impl_trait!(Ticker, GateSpotWSClient, subscribe_ticker, "ticker", to_raw_channel);

impl<'a> BBO for GateSpotWSClient<'a> {
    fn subscribe_bbo(&self, _pairs: &[String]) {
        panic!("Bitz does NOT have BBO channel");
    }
}

impl<'a> OrderBookSnapshot for GateSpotWSClient<'a> {
    fn subscribe_orderbook_snapshot(&self, _pairs: &[String]) {
        panic!("Bitz does NOT have orderbook snapshot channel");
    }
}

fn to_candlestick_raw_channel(pair: &str, interval: u32) -> String {
    let valid_set: Vec<u32> = vec![
        10, 60, 300, 900, 1800, 3600, 14400, 28800, 86400, 604800, 2592000,
    ];
    if !valid_set.contains(&interval) {
        let joined = valid_set
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",");
        panic!("Gate available intervals {}", joined);
    }

    let arr = serde_json::json!([pair, interval]);

    format!(
        r#"{{"id":9527, "method":"kline.subscribe", "params":{}}}"#,
        serde_json::to_string(&arr).unwrap(),
    )
}

impl_candlestick!(GateSpotWSClient);

define_client!(
    GateSpotWSClient,
    EXCHANGE_NAME,
    WEBSOCKET_URL,
    channels_to_commands,
    on_misc_msg,
    Some(PING_INTERVAL_AND_MSG)
);
