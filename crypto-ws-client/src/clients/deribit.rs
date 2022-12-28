use async_trait::async_trait;
use std::collections::HashMap;
use tokio_tungstenite::tungstenite::Message;

use crate::{
    clients::common_traits::{
        Candlestick, Level3OrderBook, OrderBook, OrderBookTopK, Ticker, Trade, BBO,
    },
    common::{
        command_translator::CommandTranslator,
        message_handler::{MessageHandler, MiscMessage},
        utils::{ensure_frame_size, topic_to_raw_channel},
        ws_client_internal::WSClientInternal,
    },
    WSClient,
};

use log::*;
use serde_json::Value;

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
pub struct DeribitWSClient {
    client: WSClientInternal<DeribitMessageHandler>,
    translator: DeribitCommandTranslator,
}

impl_new_constructor!(
    DeribitWSClient,
    EXCHANGE_NAME,
    WEBSOCKET_URL,
    DeribitMessageHandler {},
    DeribitCommandTranslator {}
);

#[rustfmt::skip]
impl_trait!(Trade, DeribitWSClient, subscribe_trade, "trades.SYMBOL.100ms");
#[rustfmt::skip]
impl_trait!(Ticker, DeribitWSClient, subscribe_ticker, "ticker.SYMBOL.100ms");
#[rustfmt::skip]
impl_trait!(OrderBook, DeribitWSClient, subscribe_orderbook, "book.SYMBOL.100ms");
#[rustfmt::skip]
impl_trait!(OrderBookTopK, DeribitWSClient, subscribe_orderbook_topk, "book.SYMBOL.none.20.100ms");
impl_trait!(BBO, DeribitWSClient, subscribe_bbo, "quote.SYMBOL");

impl_candlestick!(DeribitWSClient);

panic_l3_orderbook!(DeribitWSClient);

impl_ws_client_trait!(DeribitWSClient);

struct DeribitMessageHandler {}
struct DeribitCommandTranslator {}

impl DeribitCommandTranslator {
    fn topics_to_command(topics: &[(String, String)], subscribe: bool) -> String {
        let raw_channels = topics.iter().map(topic_to_raw_channel).collect::<Vec<String>>();
        format!(
            r#"{{"method": "public/{}", "params": {{"channels": {}}}}}"#,
            if subscribe { "subscribe" } else { "unsubscribe" },
            serde_json::to_string(&raw_channels).unwrap()
        )
    }

    fn to_candlestick_channel(interval: usize) -> String {
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
            _ => panic!("Unknown interval {interval}"),
        };
        format!("chart.trades.SYMBOL.{interval_str}")
    }
}

impl MessageHandler for DeribitMessageHandler {
    fn handle_message(&mut self, msg: &str) -> MiscMessage {
        let obj = serde_json::from_str::<HashMap<String, Value>>(msg).unwrap();

        if obj.contains_key("error") {
            panic!("Received {msg} from {EXCHANGE_NAME}");
        } else if obj.contains_key("result") {
            info!("Received {} from {}", msg, EXCHANGE_NAME);
            MiscMessage::Other
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
                        MiscMessage::Other
                    }
                }
                _ => {
                    warn!("Received {} from {}", msg, EXCHANGE_NAME);
                    MiscMessage::Other
                }
            }
        } else {
            error!("Received {} from {}", msg, EXCHANGE_NAME);
            MiscMessage::Other
        }
    }

    fn get_ping_msg_and_interval(&self) -> Option<(Message, u64)> {
        None
    }
}

impl CommandTranslator for DeribitCommandTranslator {
    fn translate_to_commands(&self, subscribe: bool, topics: &[(String, String)]) -> Vec<String> {
        let mut all_commands: Vec<String> =
            ensure_frame_size(topics, subscribe, Self::topics_to_command, WS_FRAME_SIZE, None);

        all_commands
            .push(r#"{"method": "public/set_heartbeat", "params": {"interval": 10}}"#.to_string());

        all_commands
    }

    fn translate_to_candlestick_commands(
        &self,
        subscribe: bool,
        symbol_interval_list: &[(String, usize)],
    ) -> Vec<String> {
        let topics = symbol_interval_list
            .iter()
            .map(|(symbol, interval)| (Self::to_candlestick_channel(*interval), symbol.clone()))
            .collect::<Vec<(String, String)>>();
        self.translate_to_commands(subscribe, &topics)
    }
}

#[cfg(test)]
mod tests {
    use crate::common::command_translator::CommandTranslator;

    #[test]
    fn test_one_channel() {
        let translator = super::DeribitCommandTranslator {};
        let commands = translator.translate_to_commands(
            true,
            &[("trades.SYMBOL.100ms".to_string(), "BTC-26MAR21".to_string())],
        );

        assert_eq!(2, commands.len());
        assert_eq!(
            r#"{"method": "public/subscribe", "params": {"channels": ["trades.BTC-26MAR21.100ms"]}}"#,
            commands[0]
        );
        assert_eq!(
            r#"{"method": "public/set_heartbeat", "params": {"interval": 10}}"#,
            commands[1]
        );
    }

    #[test]
    fn test_two_channel() {
        let translator = super::DeribitCommandTranslator {};
        let commands = translator.translate_to_commands(
            true,
            &[
                ("trades.SYMBOL.100ms".to_string(), "BTC-26MAR21".to_string()),
                ("ticker.SYMBOL.100ms".to_string(), "BTC-26MAR21".to_string()),
            ],
        );

        assert_eq!(2, commands.len());
        assert_eq!(
            r#"{"method": "public/subscribe", "params": {"channels": ["trades.BTC-26MAR21.100ms","ticker.BTC-26MAR21.100ms"]}}"#,
            commands[0]
        );
        assert_eq!(
            r#"{"method": "public/set_heartbeat", "params": {"interval": 10}}"#,
            commands[1]
        );
    }
}
