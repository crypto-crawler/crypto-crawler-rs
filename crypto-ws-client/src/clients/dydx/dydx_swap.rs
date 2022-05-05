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
        ws_client_internal::WSClientInternal,
    },
    WSClient,
};

use super::EXCHANGE_NAME;
use log::*;
use serde_json::Value;

const WEBSOCKET_URL: &str = "wss://api.dydx.exchange/v3/ws";

/// The WebSocket client for dYdX perpetual markets.
///
/// * WebSocket API doc: <https://docs.dydx.exchange/#v3-websocket-api>
/// * Trading at: <https://trade.dydx.exchange/trade>
pub struct DydxSwapWSClient {
    client: WSClientInternal<DydxMessageHandler>,
    translator: DydxCommandTranslator,
}

impl_new_constructor!(
    DydxSwapWSClient,
    EXCHANGE_NAME,
    WEBSOCKET_URL,
    DydxMessageHandler {},
    DydxCommandTranslator {}
);

impl_trait!(Trade, DydxSwapWSClient, subscribe_trade, "v3_trades");
#[rustfmt::skip]
impl_trait!(OrderBook, DydxSwapWSClient, subscribe_orderbook, "v3_orderbook");

panic_ticker!(DydxSwapWSClient);
panic_bbo!(DydxSwapWSClient);
panic_l2_topk!(DydxSwapWSClient);
panic_l3_orderbook!(DydxSwapWSClient);
panic_candlestick!(DydxSwapWSClient);

impl_ws_client_trait!(DydxSwapWSClient);

struct DydxMessageHandler {}
struct DydxCommandTranslator {}

impl MessageHandler for DydxMessageHandler {
    fn handle_message(&mut self, msg: &str) -> MiscMessage {
        let obj = serde_json::from_str::<HashMap<String, Value>>(msg).unwrap();

        match obj.get("type").unwrap().as_str().unwrap() {
            "error" => {
                error!("Received {} from {}", msg, EXCHANGE_NAME);
                if obj.contains_key("message")
                    && obj
                        .get("message")
                        .unwrap()
                        .as_str()
                        .unwrap()
                        .starts_with("Invalid subscription id for channel")
                {
                    panic!("Received {} from {}", msg, EXCHANGE_NAME);
                } else {
                    MiscMessage::Other
                }
            }
            "connected" | "pong" => {
                debug!("Received {} from {}", msg, EXCHANGE_NAME);
                MiscMessage::Other
            }
            "channel_data" | "subscribed" => MiscMessage::Normal,
            _ => {
                warn!("Received {} from {}", msg, EXCHANGE_NAME);
                MiscMessage::Other
            }
        }
    }

    fn get_ping_msg_and_interval(&self) -> Option<(Message, u64)> {
        // https://docs.dydx.exchange/#v3-websocket-api
        // The server will send pings every 30s and expects a pong within 10s.
        // The server does not expect pings, but will respond with a pong if sent one.
        Some((Message::Text(r#"{"type":"ping"}"#.to_string()), 30))
    }
}

impl DydxCommandTranslator {
    fn topic_to_command(topic: &(String, String), subscribe: bool) -> String {
        format!(
            r#"{{"type": "{}", "channel": "{}", "id": "{}"}}"#,
            if subscribe {
                "subscribe"
            } else {
                "unsubscribe"
            },
            topic.0,
            topic.1,
        )
    }
}

impl CommandTranslator for DydxCommandTranslator {
    fn translate_to_commands(&self, subscribe: bool, topics: &[(String, String)]) -> Vec<String> {
        topics
            .iter()
            .map(|t| Self::topic_to_command(t, subscribe))
            .collect()
    }

    fn translate_to_candlestick_commands(
        &self,
        _subscribe: bool,
        _symbol_interval_list: &[(String, usize)],
    ) -> Vec<String> {
        panic!("dYdX does NOT have candlestick channel");
    }
}

#[cfg(test)]
mod tests {
    use crate::common::command_translator::CommandTranslator;

    #[test]
    fn test_one_topic() {
        let translator = super::DydxCommandTranslator {};
        let commands = translator.translate_to_commands(
            true,
            &vec![("v3_trades".to_string(), "BTC-USD".to_string())],
        );

        assert_eq!(1, commands.len());
        assert_eq!(
            r#"{"type": "subscribe", "channel": "v3_trades", "id": "BTC-USD"}"#,
            commands[0]
        );
    }

    #[test]
    fn test_two_topic() {
        let translator = super::DydxCommandTranslator {};
        let commands = translator.translate_to_commands(
            true,
            &vec![
                ("v3_trades".to_string(), "BTC-USD".to_string()),
                ("v3_orderbook".to_string(), "BTC-USD".to_string()),
            ],
        );

        assert_eq!(2, commands.len());
        assert_eq!(
            r#"{"type": "subscribe", "channel": "v3_trades", "id": "BTC-USD"}"#,
            commands[0]
        );
        assert_eq!(
            r#"{"type": "subscribe", "channel": "v3_orderbook", "id": "BTC-USD"}"#,
            commands[1]
        );
    }
}
