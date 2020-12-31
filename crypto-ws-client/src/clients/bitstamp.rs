use crate::WSClient;
use std::collections::HashMap;

use super::ws_client_internal::{MiscMessage, WSClientInternal};
use log::*;
use serde_json::{json, Value};

pub(super) const EXCHANGE_NAME: &str = "Bitstamp";

const WEBSOCKET_URL: &str = "wss://ws.bitstamp.net";

/// The WebSocket client for Bitstamp Spot market(<https://www.bitstamp.net/websocket/v2/>).
pub struct BitstampSpotWSClient<'a> {
    client: WSClientInternal<'a>,
}

fn serialize_command(channels: &[String], subscribe: bool) -> Vec<String> {
    let mut commands = Vec::<String>::new();
    for channel in channels {
        let mut object = HashMap::<&str, Value>::new();

        object.insert(
            "event",
            serde_json::to_value(if subscribe {
                "bts:subscribe"
            } else {
                "bts:unsubscribe"
            })
            .unwrap(),
        );

        let mut data = HashMap::new();
        data.insert("channel", channel);
        object.insert("data", json!(data));

        commands.push(serde_json::to_string(&object).unwrap());
    }

    commands
}

fn on_misc_msg(msg: &str) -> MiscMessage {
    let resp = serde_json::from_str::<HashMap<String, Value>>(&msg);
    if resp.is_err() {
        error!("{} is not a JSON string, {}", msg, EXCHANGE_NAME);
        return MiscMessage::Misc;
    }
    let obj = resp.unwrap();

    let event = obj.get("event").unwrap().as_str().unwrap();
    match event {
        "bts:subscription_succeeded" | "bts:unsubscription_succeeded" => {
            debug!("Received {} from {}", msg, EXCHANGE_NAME);
            MiscMessage::Misc
        }
        "bts:error" => {
            error!("Received {} from {}", msg, EXCHANGE_NAME);
            MiscMessage::Misc
        }
        "bts:request_reconnect" => {
            warn!("Received {} from {}", msg, EXCHANGE_NAME);
            MiscMessage::Reconnect
        }
        _ => MiscMessage::Normal,
    }
}

define_client!(
    BitstampSpotWSClient,
    EXCHANGE_NAME,
    WEBSOCKET_URL,
    serialize_command,
    on_misc_msg
);
