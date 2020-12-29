use crate::WSClient;
use std::collections::HashMap;

use super::ws_client_internal::WSClientInternal;
use serde_json::{json, Value};

const SPOT_WEBSOCKET_URL: &str = "wss://stream.binance.com:9443/stream";
const FUTURES_WEBSOCKET_URL: &str = "wss://fstream.binance.com/stream";
const DELIVERY_WEBSOCKET_URL: &str = "wss://dstream.binance.com/stream";

/// The WebSocket client for Binance Spot market(https://binance-docs.github.io/apidocs/spot/en/).
pub struct BinanceSpotWSClient {
    client: WSClientInternal,
}

/// The WebSocket client for Binance USDT Futures market(https://binance-docs.github.io/apidocs/futures/en/).
pub struct BinanceFuturesWSClient {
    client: WSClientInternal,
}

/// The WebSocket client for Binance Coin Dilivery market(https://binance-docs.github.io/apidocs/delivery/en/).
pub struct BinanceDeliveryWSClient {
    client: WSClientInternal,
}

fn serialize_command(channels: &[String], subscribe: bool) -> Vec<String> {
    let mut object = HashMap::<String, Value>::new();
    if subscribe {
        object.insert(
            "method".to_string(),
            serde_json::to_value("SUBSCRIBE").unwrap(),
        );
    } else {
        object.insert(
            "method".to_string(),
            serde_json::to_value("UNSUBSCRIBE").unwrap(),
        );
    }

    object.insert("params".to_string(), json!(channels));
    object.insert("id".to_string(), json!(9527));

    vec![serde_json::to_string(&object).unwrap()]
}

define_client!(BinanceSpotWSClient, SPOT_WEBSOCKET_URL, serialize_command);
define_client!(
    BinanceFuturesWSClient,
    FUTURES_WEBSOCKET_URL,
    serialize_command
);
define_client!(
    BinanceDeliveryWSClient,
    DELIVERY_WEBSOCKET_URL,
    serialize_command
);
