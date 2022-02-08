use crate::WSClient;
use std::collections::HashMap;
use std::sync::mpsc::Sender;

use super::{
    utils::CHANNEL_PAIR_DELIMITER,
    ws_client_internal::{MiscMessage, WSClientInternal},
    Candlestick, Level3OrderBook, OrderBook, OrderBookTopK, Ticker, Trade, BBO,
};

use log::*;
use serde_json::Value;
use tungstenite::Message;

pub(super) const EXCHANGE_NAME: &str = "kraken_futures";

const WEBSOCKET_URL: &str = "wss://futures.kraken.com/ws/v1";

// Client can ping server to determine whether connection is alive
// https://docs.kraken.com/websockets/#message-ping
const CLIENT_PING_INTERVAL_AND_MSG: (u64, &str) = (10, r#"{"event":"ping"}"#);

/// The WebSocket client for Kraken.
///
/// Kraken has only Spot market.
///
///   * WebSocket API doc: <https://support.kraken.com/hc/en-us/sections/360003562371-Websocket-API-Public>
///   * Trading at: <https://futures.kraken.com/>

pub struct KrakenFuturesWSClient {
    client: WSClientInternal,
}

fn name_pairs_to_command(name: &str, pairs: &[String], subscribe: bool) -> String {
    format!(
        r#"{{"event":"{}","pair":{},"subscription":{{"name":"{}"}}}}"#,
        if subscribe {
            "subscribe"
        } else {
            "unsubscribe"
        },
        serde_json::to_string(pairs).unwrap(),
        name
    )
}

fn channels_to_commands(channels: &[String], subscribe: bool) -> Vec<String> {
    let mut all_commands: Vec<String> = channels
        .iter()
        .filter(|ch| ch.starts_with('{'))
        .map(|s| s.to_string())
        .collect();

    let mut name_pairs = HashMap::<String, Vec<String>>::new();
    for s in channels.iter().filter(|ch| !ch.starts_with('{')) {
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

    for (name, pairs) in name_pairs.iter() {
        all_commands.push(name_pairs_to_command(name, pairs, subscribe));
    }

    all_commands
}

fn on_misc_msg(msg: &str) -> MiscMessage {
    let resp = serde_json::from_str::<Value>(msg);
    if resp.is_err() {
        error!("{} is not a JSON string, {}", msg, EXCHANGE_NAME);
        return MiscMessage::Misc;
    }
    let value = resp.unwrap();

    if value.is_object() {
        let obj = value.as_object().unwrap();
        if !obj.contains_key("event") {
            return MiscMessage::Normal;
        }
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
            "pong" => MiscMessage::Pong,
            "subscriptionStatus" => {
                let status = obj.get("status").unwrap().as_str().unwrap();
                match status {
                    "subscribed" | "unsubscribed" => {
                        info!("Received {} from {}", msg, EXCHANGE_NAME)
                    }
                    "error" => {
                        error!("Received {} from {}", msg, EXCHANGE_NAME);
                        let error_msg = obj.get("errorMessage").unwrap().as_str().unwrap();
                        if error_msg.starts_with("Currency pair not supported") {
                            panic!("Received {} from {}", msg, EXCHANGE_NAME)
                        }
                    }
                    _ => warn!("Received {} from {}", msg, EXCHANGE_NAME),
                }

                MiscMessage::Misc
            }
            "systemStatus" => {
                let status = obj.get("status").unwrap().as_str().unwrap();
                match status {
                    "maintenance" | "cancel_only" => {
                        warn!(
                            "Received {}, which means Kraken is in maintenance mode",
                            msg
                        );
                        std::thread::sleep(std::time::Duration::from_secs(20));
                        MiscMessage::Reconnect
                    }
                    _ => {
                        info!("Received {} from {}", msg, EXCHANGE_NAME);
                        MiscMessage::Misc
                    }
                }
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

fn to_raw_channel(channel: &str, pair: &str) -> String {
    format!("{}{}{}", channel, CHANNEL_PAIR_DELIMITER, pair)
}

#[rustfmt::skip]
impl_trait!(Trade, KrakenFuturesWSClient, subscribe_trade, "trade", to_raw_channel);
#[rustfmt::skip]
impl_trait!(Ticker, KrakenFuturesWSClient, subscribe_ticker, "ticker", to_raw_channel);

impl OrderBook for KrakenFuturesWSClient {
    fn subscribe_orderbook(&self, pairs: &[String]) {
        let command = format!(
            r#"{{"event":"subscribe","product_ids":{},"feed": "book"}}"#,
            serde_json::to_string(pairs).unwrap(),
        );
        let channels = vec![command];

        self.client.subscribe(&channels);
    }
}

impl OrderBookTopK for KrakenFuturesWSClient {
    fn subscribe_orderbook_topk(&self, _pairs: &[String]) {
        panic!("Kraken does NOT have orderbook snapshot channel");
    }
}

panic_bbo!(KrakenFuturesWSClient);

panic_candlestick!(KrakenFuturesWSClient);

panic_l3_orderbook!(KrakenFuturesWSClient);

impl_new_constructor!(
    KrakenFuturesWSClient,
    EXCHANGE_NAME,
    WEBSOCKET_URL,
    channels_to_commands,
    on_misc_msg,
    Some(CLIENT_PING_INTERVAL_AND_MSG),
    None
);
impl_ws_client_trait!(KrakenFuturesWSClient);

#[cfg(test)]
mod tests {
    #[test]
    fn test_one_pair() {
        assert_eq!(
            r#"{"event":"subscribe","pair":["XBT/USD"],"subscription":{"name":"trade"}}"#,
            super::name_pairs_to_command("trade", &vec!["XBT/USD".to_string()], true)
        );

        assert_eq!(
            r#"{"event":"unsubscribe","pair":["XBT/USD"],"subscription":{"name":"trade"}}"#,
            super::name_pairs_to_command("trade", &vec!["XBT/USD".to_string()], false)
        );
    }

    #[test]
    fn test_two_pairs() {
        assert_eq!(
            r#"{"event":"subscribe","pair":["XBT/USD","ETH/USD"],"subscription":{"name":"trade"}}"#,
            super::name_pairs_to_command(
                "trade",
                &vec!["XBT/USD".to_string(), "ETH/USD".to_string()],
                true
            )
        );

        assert_eq!(
            r#"{"event":"unsubscribe","pair":["XBT/USD","ETH/USD"],"subscription":{"name":"trade"}}"#,
            super::name_pairs_to_command(
                "trade",
                &vec!["XBT/USD".to_string(), "ETH/USD".to_string()],
                false
            )
        );
    }
}
