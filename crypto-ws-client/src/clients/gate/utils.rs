use crate::clients::utils::CHANNEL_PAIR_DELIMITER;
use std::collections::HashMap;

use super::super::ws_client_internal::MiscMessage;

use log::*;
use serde_json::Value;

pub(super) const EXCHANGE_NAME: &str = "gate";

pub(super) const CLIENT_PING_INTERVAL_AND_MSG: (u64, &str) = (10, r#"{"channel":"futures.ping"}"#);

fn channel_pairs_to_command(channel: &str, pairs: &[String], subscribe: bool) -> Vec<String> {
    match channel {
        "trades" | "tickers" => {
            vec![format!(
                r#"{{"channel":"{}","event":"{}", "payload":{}}}"#,
                format!("futures.{}", channel),
                if subscribe {
                    "subscribe"
                } else {
                    "unsubscribe"
                },
                serde_json::to_string(pairs).unwrap(),
            )]
        }
        _ => {
            let commands: Vec<String> = pairs
                .iter()
                .map(|pair| {
                    let command = serde_json::json!({
                        "channel": format!("futures.{}", channel),
                        "event": if subscribe {
                            "subscribe"
                        } else {
                            "unsubscribe"
                        },
                        "payload": match channel {
                            "order_book" => [pair, "20", "0"],
                            _ => panic!("Unknown channel {}", channel),
                        }
                    });
                    serde_json::to_string(&command).unwrap()
                })
                .collect();
            commands
        }
    }
}

pub(super) fn channels_to_commands(channels: &[String], subscribe: bool) -> Vec<String> {
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
        let mut commands = channel_pairs_to_command(channel, pairs, subscribe);
        all_commands.append(&mut commands);
    }

    all_commands
}

pub(super) fn to_raw_channel(channel: &str, pair: &str) -> String {
    format!("{}{}{}", channel, CHANNEL_PAIR_DELIMITER, pair)
}

pub(super) fn on_misc_msg(msg: &str) -> MiscMessage {
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
        // see https://www.gate.io/docs/futures/ws/index.html#error
        panic!("Received {} from {}", msg, EXCHANGE_NAME);
    } else {
        let channel = obj.get("channel").unwrap().as_str().unwrap();
        let event = obj.get("event").unwrap().as_str().unwrap();
        if channel == "futures.pong" {
            debug!("Received {} from {}", msg, EXCHANGE_NAME);
            MiscMessage::Pong
        } else if event == "all" || event == "update" {
            MiscMessage::Normal
        } else if event == "subscribe" || event == "unsubscribe" {
            debug!("Received {} from {}", msg, EXCHANGE_NAME);
            MiscMessage::Misc
        } else {
            warn!("Received {} from {}", msg, EXCHANGE_NAME);
            MiscMessage::Misc
        }
    }
}

pub(super) fn to_candlestick_raw_channel(pair: &str, interval: u32) -> String {
    let interval_str = match interval {
        10 => "10s",
        60 => "1m",
        300 => "5m",
        900 => "15m",
        1800 => "30m",
        3600 => "1h",
        14400 => "4h",
        28800 => "8h",
        86400 => "1d",
        604800 => "7d",
        _ => panic!("Gate available intervals 10s,1m,5m,15m,30m,1h,4h,8h,1d,7d"),
    };
    format!(
        r#"{{"channel": "futures.candlesticks", "event": "subscribe", "payload" : ["{}", "{}"]}}"#,
        interval_str, pair
    )
}
