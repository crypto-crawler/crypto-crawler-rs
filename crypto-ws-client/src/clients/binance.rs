use crate::WSClient;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

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

fn serialize_command(channels: &[String], subscribe: bool) -> String {
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

    let id = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        % 9999;
    object.insert("id".to_string(), json!(id));

    serde_json::to_string(&object).unwrap()
}

impl WSClient for BinanceSpotWSClient {
    type Exchange = BinanceSpotWSClient;

    fn init(on_msg: fn(String)) -> BinanceSpotWSClient {
        BinanceSpotWSClient {
            client: WSClientInternal::new(SPOT_WEBSOCKET_URL, on_msg, serialize_command),
        }
    }

    fn subscribe(&mut self, channels: &[String]) {
        self.client.subscribe(channels);
    }

    fn unsubscribe(&mut self, channels: &[String]) {
        self.client.unsubscribe(channels);
    }

    fn run(&mut self) {
        self.client.run();
    }

    fn close(&mut self) {
        self.client.close();
    }
}

impl WSClient for BinanceFuturesWSClient {
    type Exchange = BinanceFuturesWSClient;

    fn init(on_msg: fn(String)) -> BinanceFuturesWSClient {
        BinanceFuturesWSClient {
            client: WSClientInternal::new(FUTURES_WEBSOCKET_URL, on_msg, serialize_command),
        }
    }

    fn subscribe(&mut self, channels: &[String]) {
        self.client.subscribe(channels);
    }

    fn unsubscribe(&mut self, channels: &[String]) {
        self.client.unsubscribe(channels);
    }

    fn run(&mut self) {
        self.client.run();
    }

    fn close(&mut self) {
        self.client.close();
    }
}

impl WSClient for BinanceDeliveryWSClient {
    type Exchange = BinanceDeliveryWSClient;

    fn init(on_msg: fn(String)) -> BinanceDeliveryWSClient {
        BinanceDeliveryWSClient {
            client: WSClientInternal::new(DELIVERY_WEBSOCKET_URL, on_msg, serialize_command),
        }
    }

    fn subscribe(&mut self, channels: &[String]) {
        self.client.subscribe(channels);
    }

    fn unsubscribe(&mut self, channels: &[String]) {
        self.client.unsubscribe(channels);
    }

    fn run(&mut self) {
        self.client.run();
    }

    fn close(&mut self) {
        self.client.close();
    }
}
