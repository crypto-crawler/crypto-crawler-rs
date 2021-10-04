use crate::clients::utils::CHANNEL_PAIR_DELIMITER;
use std::collections::HashMap;

use super::super::ws_client_internal::MiscMessage;

use log::*;
use serde_json::Value;

pub(super) const EXCHANGE_NAME: &str = "gate";

// https://www.gate.io/docs/futures/ws/en/#ping-and-pong
// https://www.gate.io/docs/delivery/ws/en/#ping-and-pong
pub(super) const CLIENT_PING_INTERVAL_AND_MSG: (u64, &str) = (60, r#"{"channel":"futures.ping"}"#);

fn channel_pairs_to_command(channel: &str, pairs: &[String], subscribe: bool) -> Vec<String> {
    if channel.contains(".order_book") {
        pairs
            .iter()
            .map(|pair| {
                format!(
                    r#"{{"channel":"{}", "event":"{}", "payload":{}}}"#,
                    channel,
                    if subscribe {
                        "subscribe"
                    } else {
                        "unsubscribe"
                    },
                    if channel.ends_with(".order_book") {
                        if channel.starts_with("spot.") {
                            serde_json::to_string(&[pair, "20", "1000ms"]).unwrap()
                        } else if channel.starts_with("futures.") {
                            serde_json::to_string(&[pair, "20", "0"]).unwrap()
                        } else {
                            panic!("unexpected channel: {}", channel)
                        }
                    } else if channel.ends_with(".order_book_update") {
                        if channel.starts_with("spot.") {
                            serde_json::to_string(&[pair, "100ms"]).unwrap()
                        } else if channel.starts_with("futures.") {
                            serde_json::to_string(&[pair, "100ms", "20"]).unwrap()
                        } else {
                            panic!("unexpected channel: {}", channel)
                        }
                    } else {
                        panic!("unexpected channel: {}", channel)
                    },
                )
            })
            .collect()
    } else {
        vec![format!(
            r#"{{"channel":"{}", "event":"{}", "payload":{}}}"#,
            channel,
            if subscribe {
                "subscribe"
            } else {
                "unsubscribe"
            },
            serde_json::to_string(pairs).unwrap(),
        )]
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
        all_commands.extend(channel_pairs_to_command(channel, pairs, subscribe));
    }

    all_commands
}

pub(super) fn to_raw_channel(channel: &str, pair: &str) -> String {
    format!("{}{}{}", channel, CHANNEL_PAIR_DELIMITER, pair)
}

pub(super) fn on_misc_msg(msg: &str) -> MiscMessage {
    let obj = serde_json::from_str::<HashMap<String, Value>>(msg).unwrap();

    // https://www.gate.io/docs/apiv4/ws/en/#server-response
    // Null if the server accepts the client request; otherwise, the detailed reason why request is rejected.
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
        // https://www.gate.io/docs/apiv4/ws/en/#schema_error
        // https://www.gate.io/docs/futures/ws/en/#error
        let code = err.get("code").unwrap().as_i64().unwrap();
        match code {
            1 | 2 => panic!("Received {} from {}", msg, EXCHANGE_NAME), // client side errors
            _ => error!("Received {} from {}", msg, EXCHANGE_NAME),     // server side errors
        }
        return MiscMessage::Misc;
    }

    let channel = obj.get("channel").unwrap().as_str().unwrap();
    let event = obj.get("event").unwrap().as_str().unwrap();

    if channel == "spot.pong" || channel == "futures.pong" {
        MiscMessage::Pong
    } else if event == "update" || event == "all" {
        MiscMessage::Normal
    } else if event == "subscribe" || event == "unsubscribe" {
        debug!("Received {} from {}", msg, EXCHANGE_NAME);
        MiscMessage::Misc
    } else {
        warn!("Received {} from {}", msg, EXCHANGE_NAME);
        MiscMessage::Misc
    }
}

pub(super) fn to_candlestick_raw_channel_shared(
    market_type: &str,
    pair: &str,
    interval: usize,
) -> String {
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
        r#"{{"channel": "{}.candlesticks", "event": "subscribe", "payload" : ["{}", "{}"]}}"#,
        market_type, interval_str, pair
    )
}
