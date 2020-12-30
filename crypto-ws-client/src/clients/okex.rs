use crate::WSClient;
use std::collections::HashMap;

use super::ws_client_internal::WSClientInternal;
use serde_json::{json, Value};

const WEBSOCKET_URL: &str = "wss://real.okex.com:8443/ws/v3";

/// The WebSocket client for OKEx, including Spot, Futures, Swap and Option(<https://www.okex.com/docs/en/>).
pub struct OKExWSClient {
    client: WSClientInternal,
}

fn serialize_command(channels: &[String], subscribe: bool) -> Vec<String> {
    let mut object = HashMap::<&str, Value>::new();

    object.insert(
        "op",
        serde_json::to_value(if subscribe {
            "subscribe"
        } else {
            "unsubscribe"
        })
        .unwrap(),
    );

    object.insert("args", json!(channels));

    vec![serde_json::to_string(&object).unwrap()]
}

define_client!(OKExWSClient, WEBSOCKET_URL, serialize_command);
