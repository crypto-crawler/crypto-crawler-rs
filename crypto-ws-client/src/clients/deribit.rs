use crate::WSClient;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use super::ws_client_internal::{MiscMessage, WSClientInternal};
use super::{Candlestick, OrderBook, OrderBookTopK, Ticker, Trade, BBO};

use log::*;
use serde_json::Value;
use tungstenite::Message;

pub(super) const EXCHANGE_NAME: &str = "deribit";

const WEBSOCKET_URL: &str = "wss://www.deribit.com/ws/api/v2/";

/// The WebSocket client for Deribit.
///
/// Deribit has InverseFuture, InverseSwap and Option markets.
///
/// * WebSocket API doc: <https://docs.deribit.com/?shell#subscriptions>
/// * Trading at:
///     * Future <https://www.deribit.com/main#/futures>
///     * Option <https://www.deribit.com/main#/options>
pub struct DeribitWSClient<'a> {
    client: WSClientInternal<'a>,
}

fn channels_to_commands(channels: &[String], subscribe: bool) -> Vec<String> {
    let raw_channels: Vec<&String> = channels.iter().filter(|ch| !ch.starts_with('{')).collect();
    let mut all_commands: Vec<String> = channels
        .iter()
        .filter(|ch| ch.starts_with('{'))
        .map(|s| s.to_string())
        .collect();

    if !raw_channels.is_empty() {
        let n = raw_channels.len();
        // make sure each body size doesn't exceed the websocket frame limit 32KiB
        for i in (0..n).step_by(512) {
            let chunk: Vec<&String> = (&raw_channels[i..(std::cmp::min(i + 512, n))]).to_vec();
            all_commands.append(&mut vec![format!(
                r#"{{"method": "public/{}", "params": {{"channels": {}}}}}"#,
                if subscribe {
                    "subscribe"
                } else {
                    "unsubscribe"
                },
                serde_json::to_string(&chunk).unwrap()
            )])
        }
    };

    all_commands
        .push(r#"{"method": "public/set_heartbeat", "params": {"interval": 10}}"#.to_string());

    all_commands
}

fn on_misc_msg(msg: &str) -> MiscMessage {
    let obj = serde_json::from_str::<HashMap<String, Value>>(msg).unwrap();

    if obj.contains_key("error") {
        error!("Received {} from {}", msg, EXCHANGE_NAME);
        panic!("Received {} from {}", msg, EXCHANGE_NAME);
    } else if obj.contains_key("result") {
        let result = obj.get("result").unwrap();
        if result.is_string() {
            if result.as_str().unwrap() == "ok" {
                info!("Received {} from {}", msg, EXCHANGE_NAME);
            } else {
                warn!("Received {} from {}", msg, EXCHANGE_NAME);
            }
            MiscMessage::Misc
        } else if result.is_array() {
            let arr = result.as_array().unwrap();
            if !arr.is_empty() {
                if arr[0].is_object() {
                    MiscMessage::Normal
                } else {
                    MiscMessage::Misc
                }
            } else {
                panic!("Subscribed invalid symbols, {}, {}", msg, EXCHANGE_NAME);
            }
        } else if result.is_object() {
            if result.as_object().unwrap().contains_key("version") {
                MiscMessage::Misc
            } else {
                MiscMessage::Normal
            }
        } else {
            warn!("Received {} from {}", msg, EXCHANGE_NAME);
            MiscMessage::Misc
        }
    } else if obj.contains_key("method") && obj.contains_key("params") {
        match obj.get("method").unwrap().as_str().unwrap() {
            "subscription" => MiscMessage::Normal,
            "heartbeat" => {
                let param_type = obj
                    .get("params")
                    .unwrap()
                    .as_object()
                    .unwrap()
                    .get("type")
                    .unwrap()
                    .as_str()
                    .unwrap();
                if param_type == "test_request" {
                    let ws_msg = Message::Text(r#"{"method": "public/test"}"#.to_string());
                    MiscMessage::WebSocket(ws_msg)
                } else {
                    info!("Received {} from {}", msg, EXCHANGE_NAME);
                    MiscMessage::Misc
                }
            }
            _ => {
                warn!("Received {} from {}", msg, EXCHANGE_NAME);
                MiscMessage::Misc
            }
        }
    } else {
        error!("Received {} from {}", msg, EXCHANGE_NAME);
        MiscMessage::Misc
    }
}

fn to_raw_channel(channel: &str, pair: &str) -> String {
    match channel {
        "trade" => format!("trades.{}.raw", pair),
        "ticker" => format!("ticker.{}.100ms", pair),
        "orderbook" => format!("book.{}.100ms", pair),
        "orderbook_snapshot" => format!("book.{}.5.10.100ms", pair),
        "bbo" => format!("quote.{}", pair),
        _ => panic!("Unknown channel {}", channel),
    }
}

#[rustfmt::skip]
impl_trait!(Trade, DeribitWSClient, subscribe_trade, "trade", to_raw_channel);
#[rustfmt::skip]
impl_trait!(Ticker, DeribitWSClient, subscribe_ticker, "ticker", to_raw_channel);
#[rustfmt::skip]
impl_trait!(OrderBook, DeribitWSClient, subscribe_orderbook, "orderbook", to_raw_channel);
#[rustfmt::skip]
impl_trait!(OrderBookTopK, DeribitWSClient, subscribe_orderbook_topk, "orderbook_snapshot", to_raw_channel);
#[rustfmt::skip]
impl_trait!(BBO, DeribitWSClient, subscribe_bbo, "bbo", to_raw_channel);

fn to_candlestick_raw_channel(pair: &str, interval: u32) -> String {
    let interval_str = match interval {
        60 => "1",
        180 => "3",
        300 => "5",
        600 => "10",
        900 => "15",
        1800 => "30",
        3600 => "60",
        7200 => "120",
        10800 => "180",
        21600 => "360",
        43200 => "720",
        86400 => "1D",
        _ => panic!("Unknown interval {}", interval),
    };
    format!("chart.trades.{}.{}", pair, interval_str)
}

impl_candlestick!(DeribitWSClient);

define_client!(
    DeribitWSClient,
    EXCHANGE_NAME,
    WEBSOCKET_URL,
    channels_to_commands,
    on_misc_msg,
    None,
    None
);

#[cfg(test)]
mod tests {
    #[test]
    fn test_one_channel() {
        let commands =
            super::channels_to_commands(&vec!["trades.BTC-26MAR21.raw".to_string()], true);
        assert_eq!(2, commands.len());
        assert_eq!(
            r#"{"method": "public/subscribe", "params": {"channels": ["trades.BTC-26MAR21.raw"]}}"#,
            commands[0]
        );
        assert_eq!(
            r#"{"method": "public/set_heartbeat", "params": {"interval": 10}}"#,
            commands[1]
        );
    }

    #[test]
    fn test_two_channel() {
        let commands = super::channels_to_commands(
            &vec![
                "trades.BTC-26MAR21.raw".to_string(),
                "ticker.BTC-26MAR21.100ms".to_string(),
            ],
            true,
        );
        assert_eq!(2, commands.len());
        assert_eq!(
            r#"{"method": "public/subscribe", "params": {"channels": ["trades.BTC-26MAR21.raw","ticker.BTC-26MAR21.100ms"]}}"#,
            commands[0]
        );
        assert_eq!(
            r#"{"method": "public/set_heartbeat", "params": {"interval": 10}}"#,
            commands[1]
        );
    }
}
