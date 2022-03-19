use async_trait::async_trait;
use std::collections::HashMap;

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

pub(super) const EXCHANGE_NAME: &str = "bitstamp";

const WEBSOCKET_URL: &str = "wss://ws.bitstamp.net";

/// The WebSocket client for Bitstamp Spot market.
///
/// Bitstamp has only Spot market.
///
///   * WebSocket API doc: <https://www.bitstamp.net/websocket/v2/>
///   * Trading at: <https://www.bitstamp.net/market/tradeview/>
pub struct BitstampWSClient {
    client: WSClientInternal<BitstampMessageHandler>,
    translator: BitstampCommandTranslator,
}

impl_new_constructor!(
    BitstampWSClient,
    EXCHANGE_NAME,
    WEBSOCKET_URL,
    BitstampMessageHandler {},
    BitstampCommandTranslator {}
);

impl_trait!(Trade, BitstampWSClient, subscribe_trade, "live_trades");
#[rustfmt::skip]
impl_trait!(OrderBook, BitstampWSClient, subscribe_orderbook, "diff_order_book");
#[rustfmt::skip]
impl_trait!(OrderBookTopK, BitstampWSClient, subscribe_orderbook_topk, "order_book");
#[rustfmt::skip]
impl_trait!(Level3OrderBook, BitstampWSClient, subscribe_l3_orderbook, "live_orders");

panic_bbo!(BitstampWSClient);
impl_candlestick!(BitstampWSClient);
panic_ticker!(BitstampWSClient);

impl_ws_client_trait!(BitstampWSClient);

struct BitstampMessageHandler {}
struct BitstampCommandTranslator {}

impl MessageHandler for BitstampMessageHandler {
    fn handle_message(&mut self, msg: &str) -> MiscMessage {
        let resp = serde_json::from_str::<HashMap<String, Value>>(msg);
        if resp.is_err() {
            error!("{} is not a JSON string, {}", msg, EXCHANGE_NAME);
            return MiscMessage::Other;
        }
        let obj = resp.unwrap();

        let event = obj.get("event").unwrap().as_str().unwrap();
        match event {
            "bts:subscription_succeeded" | "bts:unsubscription_succeeded" | "bts:heartbeat" => {
                debug!("Received {} from {}", msg, EXCHANGE_NAME);
                MiscMessage::Other
            }
            "bts:error" => {
                error!("Received {} from {}", msg, EXCHANGE_NAME);
                panic!("Received {} from {}", msg, EXCHANGE_NAME);
            }
            "bts:request_reconnect" => {
                warn!(
                    "Received {}, which means Bitstamp is under maintenance",
                    msg
                );
                std::thread::sleep(std::time::Duration::from_secs(20));
                MiscMessage::Reconnect
            }
            _ => MiscMessage::Normal,
        }
    }

    fn get_ping_msg_and_interval(&self) -> Option<(String, u64)> {
        // See "Heartbeat" at https://www.bitstamp.net/websocket/v2/
        Some((r#"{"event": "bts:heartbeat"}"#.to_string(), 10))
    }
}

impl CommandTranslator for BitstampCommandTranslator {
    fn translate_to_commands(&self, subscribe: bool, topics: &[(String, String)]) -> Vec<String> {
        topics
            .iter()
            .map(|(channel, symbol)| {
                format!(
                    r#"{{"event":"bts:{}","data":{{"channel":"{}_{}"}}}}"#,
                    if subscribe {
                        "subscribe"
                    } else {
                        "unsubscribe"
                    },
                    channel,
                    symbol,
                )
            })
            .collect()
    }

    fn translate_to_candlestick_commands(
        &self,
        _subscribe: bool,
        _symbol_interval_list: &[(String, usize)],
    ) -> Vec<String> {
        panic!("Bitstamp does NOT have candlestick channel");
    }
}

#[cfg(test)]
mod tests {
    use crate::common::command_translator::CommandTranslator;

    #[test]
    fn test_one_topic() {
        let translator = super::BitstampCommandTranslator {};
        let commands = translator.translate_to_commands(
            true,
            &vec![("live_trades".to_string(), "btcusd".to_string())],
        );

        assert_eq!(1, commands.len());
        assert_eq!(
            r#"{"event":"bts:subscribe","data":{"channel":"live_trades_btcusd"}}"#,
            commands[0]
        );
    }

    #[test]
    fn test_two_topics() {
        let translator = super::BitstampCommandTranslator {};
        let commands = translator.translate_to_commands(
            true,
            &vec![
                ("live_trades".to_string(), "btcusd".to_string()),
                ("diff_order_book".to_string(), "btcusd".to_string()),
            ],
        );

        assert_eq!(2, commands.len());
        assert_eq!(
            r#"{"event":"bts:subscribe","data":{"channel":"live_trades_btcusd"}}"#,
            commands[0]
        );
        assert_eq!(
            r#"{"event":"bts:subscribe","data":{"channel":"diff_order_book_btcusd"}}"#,
            commands[1]
        );
    }
}
