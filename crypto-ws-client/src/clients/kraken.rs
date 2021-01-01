use crate::WSClient;
use std::collections::HashMap;

use super::{
    utils::CHANNEL_PAIR_DELIMITER,
    ws_client_internal::{MiscMessage, WSClientInternal},
};

use log::*;
use serde_json::Value;
use tungstenite::Message;

pub(super) const EXCHANGE_NAME: &str = "Kraken";

const WEBSOCKET_URL: &str = "wss://ws.kraken.com";

/// The WebSocket client for Kraken Spot market(<https://docs.kraken.com/websockets/>).
pub struct KrakenSpotWSClient<'a> {
    client: WSClientInternal<'a>,
}

fn serialize_command(channels: &[String], subscribe: bool) -> Vec<String> {
    let mut name_pairs = HashMap::<String, Vec<String>>::new();
    for s in channels {
        let v: Vec<&str> = s.split(CHANNEL_PAIR_DELIMITER).collect();
        let name = v[0];
        let pair = v[1];
        match name_pairs.get_mut(name) {
            Some(pairs) => pairs.push(pair.to_string()),
            None => {
                name_pairs.insert(name.to_string(), vec![pair.to_string()]);
            }
        }
    }

    let mut commands = Vec::<String>::new();

    for (name, pairs) in name_pairs.iter() {
        let mut command = HashMap::<&str, Value>::new();
        command.insert(
            "event",
            serde_json::to_value(if subscribe {
                "subscribe"
            } else {
                "unsubscribe"
            })
            .unwrap(),
        );
        command.insert("pair", serde_json::to_value(pairs).unwrap());

        let mut subscription = HashMap::<&str, &str>::new();
        subscription.insert("name", name);
        command.insert("subscription", serde_json::to_value(subscription).unwrap());

        commands.push(serde_json::to_string(&command).unwrap());
    }

    commands
}

fn on_misc_msg(msg: &str) -> MiscMessage {
    let resp = serde_json::from_str::<Value>(&msg);
    if resp.is_err() {
        error!("{} is not a JSON string, {}", msg, EXCHANGE_NAME);
        return MiscMessage::Misc;
    }
    let value = resp.unwrap();

    if value.is_object() {
        let obj = value.as_object().unwrap();
        let event = obj.get("event").unwrap().as_str().unwrap();
        match event {
            "heartbeat" => {
                debug!("Received {} from {}", msg, EXCHANGE_NAME);
                let ping = r#"{
                    "event": "ping",
                    "reqid": 9527
                }"#;
                MiscMessage::WebSocket(Message::Text(ping.to_string()))
            }
            "pong" => {
                debug!("Received {} from {}", msg, EXCHANGE_NAME);
                MiscMessage::Misc
            }
            _ => {
                warn!("Received {} from {}", msg, EXCHANGE_NAME);
                MiscMessage::Misc
            }
        }
    } else {
        MiscMessage::Normal
    }
}

define_client!(
    KrakenSpotWSClient,
    EXCHANGE_NAME,
    WEBSOCKET_URL,
    serialize_command,
    on_misc_msg
);
