use async_trait::async_trait;
use std::{
    collections::HashMap,
    time::{SystemTime, UNIX_EPOCH},
};
use tokio_tungstenite::tungstenite::Message;

use crate::{
    clients::common_traits::{
        Candlestick, Level3OrderBook, OrderBook, OrderBookTopK, Ticker, Trade, BBO,
    },
    common::{
        command_translator::CommandTranslator,
        message_handler::{MessageHandler, MiscMessage},
        ws_client_internal::WSClientInternal,
    },
    WSClient,
};

use log::*;
use serde_json::Value;

use super::EXCHANGE_NAME;

const WEBSOCKET_URL: &str = "wss://wsapi.bitz.plus/";

/// The WebSocket client for Bitz spot market.
///
/// * WebSocket API doc: <https://apidocv2.bitz.plus/en/#websocket-url>
/// * Trading at <https://www.bitz.plus/exchange>
pub struct BitzSpotWSClient {
    client: WSClientInternal<BitzMessageHandler>,
    translator: BitzCommandTranslator,
}

impl_new_constructor!(
    BitzSpotWSClient,
    EXCHANGE_NAME,
    WEBSOCKET_URL,
    BitzMessageHandler {},
    BitzCommandTranslator {}
);

#[rustfmt::skip]
impl_trait!(Trade, BitzSpotWSClient, subscribe_trade, "order");
#[rustfmt::skip]
impl_trait!(OrderBook, BitzSpotWSClient, subscribe_orderbook, "depth");
#[rustfmt::skip]
impl_trait!(Ticker, BitzSpotWSClient, subscribe_ticker, "market");
impl_candlestick!(BitzSpotWSClient);

panic_bbo!(BitzSpotWSClient);
panic_l2_topk!(BitzSpotWSClient);
panic_l3_orderbook!(BitzSpotWSClient);

impl_ws_client_trait!(BitzSpotWSClient);

struct BitzMessageHandler {}
struct BitzCommandTranslator {}

impl MessageHandler for BitzMessageHandler {
    fn handle_message(&mut self, msg: &str) -> MiscMessage {
        if msg == "pong" {
            return MiscMessage::Pong;
        }
        let obj = serde_json::from_str::<HashMap<String, Value>>(msg).unwrap();

        if obj.contains_key("action")
            && obj.get("action").unwrap().as_str().unwrap().starts_with("Pushdata.")
        {
            MiscMessage::Normal
        } else if obj.contains_key("status") {
            let status = obj.get("status").unwrap().as_i64().unwrap();
            // see https://apidocv2.bitz.plus/en/#error
            match status {
                -101001 => {
                    error!("Subscription type parameter error: {}", msg);
                    panic!("Subscription type parameter error: {msg}");
                }
                -101002 => {
                    error!("Fail to get subscribed symbol of trading pair: {}", msg);
                    panic!("Fail to get subscribed symbol of trading pair: {msg}");
                }
                -101003 => {
                    error!("k-line scale resolution error: {}", msg);
                    panic!("k-line scale resolution error: {msg}");
                }
                _ => warn!("Received {} from {}", msg, EXCHANGE_NAME),
            }
            MiscMessage::Other
        } else {
            warn!("Received {} from {}", msg, EXCHANGE_NAME);
            MiscMessage::Other
        }
    }

    fn get_ping_msg_and_interval(&self) -> Option<(Message, u64)> {
        // See https://apidocv2.bitz.plus/en/#heartbeat-and-persistent-connection-strategy
        Some((Message::Text("ping".to_string()), 10))
    }
}

impl BitzCommandTranslator {
    fn symbol_channels_to_command(pair: &str, channels: &[String], subscribe: bool) -> String {
        format!(
            r#"{{"action":"Topic.{}", "data":{{"symbol":"{}", "type":"{}", "_CDID":"100002", "dataType":"1"}}, "msg_id":{}}}"#,
            if subscribe { "sub" } else { "unsub" },
            pair,
            channels.join(","),
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis(),
        )
    }

    fn to_candlestick_command(symbol: &str, interval: usize, subscribe: bool) -> String {
        let interval_str = match interval {
            60 => "1min",
            300 => "5min",
            900 => "15min",
            1800 => "30min",
            3600 => "60min",
            14400 => "4hour",
            86400 => "1day",
            432000 => "5day",
            604800 => "1week",
            2592000 => "1mon",
            _ => panic!(
                "Bitz available intervals 1min,5min,15min,30min,60min,4hour,1day,5day,1week,1mon"
            ),
        };
        format!(
            r#"{{"action":"Topic.{}", "data":{{"symbol":"{}", "type":"kline", "resolution":"{}", "_CDID":"100002", "dataType":"1"}}, "msg_id":{}}}"#,
            if subscribe { "sub" } else { "unsub" },
            symbol,
            interval_str,
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis(),
        )
    }
}

impl CommandTranslator for BitzCommandTranslator {
    fn translate_to_commands(&self, subscribe: bool, topics: &[(String, String)]) -> Vec<String> {
        let mut commands: Vec<String> = Vec::new();

        let mut symbol_channels = HashMap::<String, Vec<String>>::new();
        for (channel, symbol) in topics {
            match symbol_channels.get_mut(symbol) {
                Some(channels) => channels.push(channel.to_string()),
                None => {
                    symbol_channels.insert(symbol.to_string(), vec![channel.to_string()]);
                }
            }
        }

        for (symbol, channels) in symbol_channels.iter() {
            commands.push(Self::symbol_channels_to_command(symbol, channels, subscribe));
        }

        commands
    }

    fn translate_to_candlestick_commands(
        &self,
        subscribe: bool,
        symbol_interval_list: &[(String, usize)],
    ) -> Vec<String> {
        symbol_interval_list
            .iter()
            .map(|(symbol, interval)| Self::to_candlestick_command(symbol, *interval, subscribe))
            .collect::<Vec<String>>()
    }
}
