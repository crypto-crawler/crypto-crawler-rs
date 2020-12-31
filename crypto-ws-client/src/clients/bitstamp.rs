use crate::WSClient;
use std::collections::HashMap;

use super::ws_client_internal::WSClientInternal;
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

define_client!(
    BitstampSpotWSClient,
    EXCHANGE_NAME,
    WEBSOCKET_URL,
    serialize_command
);
