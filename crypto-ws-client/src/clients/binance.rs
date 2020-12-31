use crate::WSClient;
use std::collections::HashMap;

use super::ws_client_internal::{MiscMessage, WSClientInternal};
use log::*;
use serde_json::{json, Value};

pub(super) const EXCHANGE_NAME: &str = "Binance";

const SPOT_WEBSOCKET_URL: &str = "wss://stream.binance.com:9443/stream";
const FUTURES_WEBSOCKET_URL: &str = "wss://fstream.binance.com/stream";
const DELIVERY_WEBSOCKET_URL: &str = "wss://dstream.binance.com/stream";

/// The WebSocket client for Binance Spot market(<https://binance-docs.github.io/apidocs/spot/en/>).
pub struct BinanceSpotWSClient<'a> {
    client: WSClientInternal<'a>,
}

/// The WebSocket client for Binance USDT Futures market(<https://binance-docs.github.io/apidocs/futures/en/>).
pub struct BinanceFuturesWSClient<'a> {
    client: WSClientInternal<'a>,
}

/// The WebSocket client for Binance Coin Dilivery market(<https://binance-docs.github.io/apidocs/delivery/en/>).
pub struct BinanceDeliveryWSClient<'a> {
    client: WSClientInternal<'a>,
}

fn serialize_command(channels: &[String], subscribe: bool) -> Vec<String> {
    let mut object = HashMap::<&str, Value>::new();

    object.insert(
        "method",
        serde_json::to_value(if subscribe {
            "SUBSCRIBE"
        } else {
            "UNSUBSCRIBE"
        })
        .unwrap(),
    );

    object.insert("params", json!(channels));
    object.insert("id", json!(9527));

    vec![serde_json::to_string(&object).unwrap()]
}

fn on_misc_msg(msg: &str) -> MiscMessage {
    let resp = serde_json::from_str::<HashMap<String, Value>>(&msg);
    if resp.is_err() {
        error!("{} is not a JSON string, {}", msg, EXCHANGE_NAME);
        return MiscMessage::Misc;
    }
    let obj = resp.unwrap();

    if let Some(result) = obj.get("result") {
        if serde_json::Value::Null == *result {
            return MiscMessage::Misc;
        }
    }

    if !obj.contains_key("stream") || !obj.contains_key("data") {
        warn!("Received {} from {}", msg, EXCHANGE_NAME);
        return MiscMessage::Misc;
    }

    MiscMessage::Normal
}

define_client!(
    BinanceSpotWSClient,
    EXCHANGE_NAME,
    SPOT_WEBSOCKET_URL,
    serialize_command,
    on_misc_msg
);
define_client!(
    BinanceFuturesWSClient,
    EXCHANGE_NAME,
    FUTURES_WEBSOCKET_URL,
    serialize_command,
    on_misc_msg
);
define_client!(
    BinanceDeliveryWSClient,
    EXCHANGE_NAME,
    DELIVERY_WEBSOCKET_URL,
    serialize_command,
    on_misc_msg
);
