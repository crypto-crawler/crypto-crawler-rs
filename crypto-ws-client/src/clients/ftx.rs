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

use log::*;
use serde_json::Value;

pub(super) const EXCHANGE_NAME: &str = "ftx";

const WEBSOCKET_URL: &str = "wss://ftx.com/ws/";

/// The WebSocket client for FTX.
///
/// FTX has Spot, LinearFuture, LinearSwap, Option, Move and BVOL markets.
///
/// * WebSocket API doc: <https://docs.ftx.com/#websocket-api>
/// * Trading at <https://ftx.com/markets>
pub struct FtxWSClient {
    client: WSClientInternal<FtxMessageHandler>,
    translator: FtxCommandTranslator,
}

impl_new_constructor!(
    FtxWSClient,
    EXCHANGE_NAME,
    WEBSOCKET_URL,
    FtxMessageHandler {},
    FtxCommandTranslator {}
);

impl_trait!(Trade, FtxWSClient, subscribe_trade, "trades");
impl_trait!(BBO, FtxWSClient, subscribe_bbo, "ticker");
#[rustfmt::skip]
impl_trait!(OrderBook, FtxWSClient, subscribe_orderbook, "orderbook");
panic_candlestick!(FtxWSClient);
panic_l2_topk!(FtxWSClient);
panic_l3_orderbook!(FtxWSClient);
panic_ticker!(FtxWSClient);

impl_ws_client_trait!(FtxWSClient);

struct FtxMessageHandler {}
struct FtxCommandTranslator {}

impl MessageHandler for FtxMessageHandler {
    fn handle_message(&mut self, msg: &str) -> MiscMessage {
        let obj = serde_json::from_str::<HashMap<String, Value>>(msg).unwrap();
        let msg_type = obj.get("type").unwrap().as_str().unwrap();

        match msg_type {
            // see https://docs.ftx.com/#response-format
            "pong" => MiscMessage::Pong,
            "subscribed" | "unsubscribed" | "info" => {
                info!("Received {} from {}", msg, EXCHANGE_NAME);
                MiscMessage::Other
            }
            "partial" | "update" => MiscMessage::Normal,
            "error" => {
                let code = obj.get("code").unwrap().as_i64().unwrap();
                match code {
                    400 => {
                        // Already subscribed
                        warn!("Received {} from {}", msg, EXCHANGE_NAME);
                    }
                    _ => panic!("Received {msg} from {EXCHANGE_NAME}"),
                }
                MiscMessage::Other
            }
            _ => {
                warn!("Received {} from {}", msg, EXCHANGE_NAME);
                MiscMessage::Other
            }
        }
    }

    fn get_ping_msg_and_interval(&self) -> Option<(Message, u64)> {
        // Send pings at regular intervals (every 15 seconds): {'op': 'ping'}.
        // You will see an {'type': 'pong'} response.
        Some((Message::Text(r#"{"op":"ping"}"#.to_string()), 15))
    }
}

impl CommandTranslator for FtxCommandTranslator {
    fn translate_to_commands(&self, subscribe: bool, topics: &[(String, String)]) -> Vec<String> {
        topics
            .iter()
            .map(|(channel, symbol)| {
                format!(
                    r#"{{"op":"{}","channel":"{}","market":"{}"}}"#,
                    if subscribe { "subscribe" } else { "unsubscribe" },
                    channel,
                    symbol
                )
            })
            .collect()
    }

    fn translate_to_candlestick_commands(
        &self,
        _subscribe: bool,
        _symbol_interval_list: &[(String, usize)],
    ) -> Vec<String> {
        panic!("FTX does NOT have candlestick channel");
    }
}

#[cfg(test)]
mod tests {
    use crate::common::command_translator::CommandTranslator;

    #[test]
    fn test_one_topic() {
        let translator = super::FtxCommandTranslator {};
        let commands = translator
            .translate_to_commands(true, &[("trades".to_string(), "BTC/USD".to_string())]);

        assert_eq!(1, commands.len());
        assert_eq!(r#"{"op":"subscribe","channel":"trades","market":"BTC/USD"}"#, commands[0]);
    }

    #[test]
    fn test_two_topic() {
        let translator = super::FtxCommandTranslator {};
        let commands = translator.translate_to_commands(
            true,
            &[
                ("trades".to_string(), "BTC/USD".to_string()),
                ("orderbook".to_string(), "BTC/USD".to_string()),
            ],
        );

        assert_eq!(2, commands.len());
        assert_eq!(r#"{"op":"subscribe","channel":"trades","market":"BTC/USD"}"#, commands[0]);
        assert_eq!(r#"{"op":"subscribe","channel":"orderbook","market":"BTC/USD"}"#, commands[1]);
    }
}
