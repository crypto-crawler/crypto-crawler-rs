use crate::WSClient;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use super::utils::ensure_frame_size;
use super::ws_client_internal::{MiscMessage, WSClientInternal};
use super::{Candlestick, Level3OrderBook, OrderBook, OrderBookTopK, Ticker, Trade, BBO};

use log::*;
use serde_json::Value;
use tungstenite::Message;

pub(super) const EXCHANGE_NAME: &str = "deribit";

const WEBSOCKET_URL: &str = "wss://www.deribit.com/ws/api/v2/";

// -32600	"request entity too large"
/// single frame in websocket connection frame exceeds the limit (32 kB)
const WS_FRAME_SIZE: usize = 32 * 1024;

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

fn topics_to_command(chunk: &[String], subscribe: bool) -> String {
    format!(
        r#"{{"method": "public/{}", "params": {{"channels": {}}}}}"#,
        if subscribe {
            "subscribe"
        } else {
            "unsubscribe"
        },
        serde_json::to_string(chunk).unwrap()
    )
}

fn channels_to_commands(channels: &[String], subscribe: bool) -> Vec<String> {
    let mut all_commands: Vec<String> =
        ensure_frame_size(channels, subscribe, topics_to_command, WS_FRAME_SIZE, None);

    all_commands
        .push(r#"{"method": "public/set_heartbeat", "params": {"interval": 10}}"#.to_string());

    all_commands
}

fn on_misc_msg(msg: &str) -> MiscMessage {
    let obj = serde_json::from_str::<HashMap<String, Value>>(msg).unwrap();

    if obj.contains_key("error") {
        panic!("Received {} from {}", msg, EXCHANGE_NAME);
    } else if obj.contains_key("result") {
        info!("Received {} from {}", msg, EXCHANGE_NAME);
        MiscMessage::Misc
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

fn to_candlestick_raw_channel(pair: &str, interval: usize) -> String {
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

panic_l3_orderbook!(DeribitWSClient);

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
