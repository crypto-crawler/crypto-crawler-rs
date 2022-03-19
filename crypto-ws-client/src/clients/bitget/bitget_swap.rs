use async_trait::async_trait;
use std::collections::HashMap;

use crate::{
    clients::common_traits::{
        Candlestick, Level3OrderBook, OrderBook, OrderBookTopK, Ticker, Trade, BBO,
    },
    common::{
        command_translator::CommandTranslator,
        message_handler::{MessageHandler, MiscMessage},
        utils::ensure_frame_size,
        ws_client_internal::WSClientInternal,
    },
    WSClient,
};

use super::EXCHANGE_NAME;
use log::*;
use serde_json::Value;

const WEBSOCKET_URL: &str = "wss://csocketapi.bitget.com/ws/v1";

// User has the opinion to subscribe 1 or more channels, total length
// of multiple channel can not exceeds 4096 bytes.
const WS_FRAME_SIZE: usize = 4096;

/// The WebSocket client for Bitget swap markets.
///
/// * WebSocket API doc: <https://bitgetlimited.github.io/apidoc/en/swap/#websocketapi>
/// * Trading at: <https://www.bitget.com/en/swap/>
pub struct BitgetSwapWSClient {
    client: WSClientInternal<BitgetMessageHandler>,
    translator: BitgetCommandTranslator,
}

impl_new_constructor!(
    BitgetSwapWSClient,
    EXCHANGE_NAME,
    WEBSOCKET_URL,
    BitgetMessageHandler {},
    BitgetCommandTranslator {}
);

impl_trait!(Trade, BitgetSwapWSClient, subscribe_trade, "trade");
#[rustfmt::skip]
impl_trait!(OrderBookTopK, BitgetSwapWSClient, subscribe_orderbook_topk, "depth5");
impl_trait!(OrderBook, BitgetSwapWSClient, subscribe_orderbook, "depth");
impl_trait!(Ticker, BitgetSwapWSClient, subscribe_ticker, "ticker");
impl_candlestick!(BitgetSwapWSClient);

panic_bbo!(BitgetSwapWSClient);
panic_l3_orderbook!(BitgetSwapWSClient);

impl_ws_client_trait!(BitgetSwapWSClient);

struct BitgetMessageHandler {}
struct BitgetCommandTranslator {}

impl BitgetCommandTranslator {
    fn topics_to_command(topics: &[(String, String)], subscribe: bool) -> String {
        let raw_channels = topics
            .iter()
            .map(|(channel, symbol)| format!("swap/{}:{}", channel, symbol))
            .collect::<Vec<String>>();
        format!(
            r#"{{"op":"{}","args":{}}}"#,
            if subscribe {
                "subscribe"
            } else {
                "unsubscribe"
            },
            serde_json::to_string(&raw_channels).unwrap()
        )
    }

    fn to_candlestick_channel(interval: usize) -> String {
        let valid_set: Vec<usize> = vec![60, 300, 900, 1800, 3600, 14400, 43200, 86400, 604800];
        if !valid_set.contains(&interval) {
            let joined = valid_set
                .into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(",");
            panic!("Bitget available intervals: {}", joined);
        }
        format!("candle{}s", interval)
    }
}

impl MessageHandler for BitgetMessageHandler {
    fn handle_message(&mut self, msg: &str) -> MiscMessage {
        if msg == "pong" {
            return MiscMessage::Pong;
        }
        let obj = serde_json::from_str::<HashMap<String, Value>>(msg).unwrap();

        if obj.contains_key("event") {
            let event = obj.get("event").unwrap().as_str().unwrap();
            if event == "error" {
                error!("Received {} from {}", msg, EXCHANGE_NAME);
                panic!("Received {} from {}", msg, EXCHANGE_NAME);
            } else {
                info!("Received {} from {}", msg, EXCHANGE_NAME);
                MiscMessage::Other
            }
        } else if obj.contains_key("table") && obj.contains_key("data") {
            if let Some(arr) = obj.get("data").unwrap().as_array() {
                if arr.is_empty() {
                    info!("data field is empty {} from {}", msg, EXCHANGE_NAME);
                    MiscMessage::Other
                } else {
                    MiscMessage::Normal
                }
            } else {
                MiscMessage::Normal
            }
        } else if obj.contains_key("action") {
            let action = obj.get("action").unwrap().as_str().unwrap();
            if action == "ping" {
                info!("Received {} from {}", msg, EXCHANGE_NAME);
            } else {
                warn!("Received {} from {}", msg, EXCHANGE_NAME);
            }
            MiscMessage::Other
        } else {
            warn!("Received {} from {}", msg, EXCHANGE_NAME);
            MiscMessage::Other
        }
    }

    fn get_ping_msg_and_interval(&self) -> Option<(String, u64)> {
        // https://bitgetlimited.github.io/apidoc/en/swap/#brief-introduction
        // System will auto-disconnect while subscription has been done within 30sec
        // or no ping command sent by user after 30sec after ws is connected
        Some(("ping".to_string(), 30))
    }
}

impl CommandTranslator for BitgetCommandTranslator {
    fn translate_to_commands(&self, subscribe: bool, topics: &[(String, String)]) -> Vec<String> {
        ensure_frame_size(
            topics,
            subscribe,
            Self::topics_to_command,
            WS_FRAME_SIZE,
            None,
        )
    }

    fn translate_to_candlestick_commands(
        &self,
        subscribe: bool,
        symbol_interval_list: &[(String, usize)],
    ) -> Vec<String> {
        let topics = symbol_interval_list
            .iter()
            .map(|(symbol, interval)| {
                let channel = Self::to_candlestick_channel(*interval);
                (channel, symbol.to_string())
            })
            .collect::<Vec<(String, String)>>();
        self.translate_to_commands(subscribe, &topics)
    }
}

#[cfg(test)]
mod tests {
    use crate::common::command_translator::CommandTranslator;

    #[test]
    fn test_one_topic() {
        let translator = super::BitgetCommandTranslator {};
        let commands = translator
            .translate_to_commands(true, &vec![("trade".to_string(), "btcusd".to_string())]);

        assert_eq!(1, commands.len());
        assert_eq!(
            r#"{"op":"subscribe","args":["swap/trade:btcusd"]}"#,
            commands[0]
        );
    }

    #[test]
    fn test_two_topics() {
        let translator = super::BitgetCommandTranslator {};
        let commands = translator.translate_to_commands(
            true,
            &vec![
                ("trade".to_string(), "btcusd".to_string()),
                ("depth".to_string(), "btcusd".to_string()),
            ],
        );

        assert_eq!(1, commands.len());
        assert_eq!(
            r#"{"op":"subscribe","args":["swap/trade:btcusd","swap/depth:btcusd"]}"#,
            commands[0]
        );
    }

    #[test]
    fn test_candlestick() {
        let translator = super::BitgetCommandTranslator {};
        let commands = translator.translate_to_candlestick_commands(
            true,
            &vec![("btcusd".to_string(), 60), ("btcusd".to_string(), 300)],
        );

        assert_eq!(1, commands.len());
        assert_eq!(
            r#"{"op":"subscribe","args":["swap/candle60s:btcusd","swap/candle300s:btcusd"]}"#,
            commands[0]
        );
    }
}
