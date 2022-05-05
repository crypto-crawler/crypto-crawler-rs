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

pub(super) const EXCHANGE_NAME: &str = "bithumb";

const WEBSOCKET_URL: &str = "wss://global-api.bithumb.pro/message/realtime";

/// The WebSocket client for Bithumb.
///
/// Bithumb has only Spot market.
///
///   * WebSocket API doc: <https://github.com/bithumb-pro/bithumb.pro-official-api-docs/blob/master/ws-api.md>
///   * Trading at: <https://en.bithumb.com/trade/order/BTC_KRW>
pub struct BithumbWSClient {
    client: WSClientInternal<BithumbMessageHandler>,
    translator: BithumbCommandTranslator,
}

impl_new_constructor!(
    BithumbWSClient,
    EXCHANGE_NAME,
    WEBSOCKET_URL,
    BithumbMessageHandler {},
    BithumbCommandTranslator {}
);

#[rustfmt::skip]
impl_trait!(Trade, BithumbWSClient, subscribe_trade, "TRADE");
#[rustfmt::skip]
impl_trait!(Ticker, BithumbWSClient, subscribe_ticker, "TICKER");
#[rustfmt::skip]
impl_trait!(OrderBook, BithumbWSClient, subscribe_orderbook, "ORDERBOOK");

panic_bbo!(BithumbWSClient);
panic_candlestick!(BithumbWSClient);
panic_l2_topk!(BithumbWSClient);
panic_l3_orderbook!(BithumbWSClient);

impl_ws_client_trait!(BithumbWSClient);

struct BithumbMessageHandler {}
struct BithumbCommandTranslator {}

impl MessageHandler for BithumbMessageHandler {
    fn handle_message(&mut self, msg: &str) -> MiscMessage {
        let obj = serde_json::from_str::<HashMap<String, Value>>(msg).unwrap();
        let code = obj.get("code").unwrap().as_str().unwrap();
        let code = code.parse::<i64>().unwrap();
        if code < 10000 {
            match code {
                0 => MiscMessage::Pong,
                6 => {
                    let arr = obj.get("data").unwrap().as_array();
                    if arr != None && arr.unwrap().is_empty() {
                        // ignore empty data
                        MiscMessage::Other
                    } else {
                        MiscMessage::Normal
                    }
                }
                7 => MiscMessage::Normal,
                _ => {
                    debug!("Received {} from {}", msg, EXCHANGE_NAME);
                    MiscMessage::Other
                }
            }
        } else {
            panic!("Received {} from {}", msg, EXCHANGE_NAME);
        }
    }

    fn get_ping_msg_and_interval(&self) -> Option<(Message, u64)> {
        Some((Message::Text(r#"{"cmd":"ping"}"#.to_string()), 60))
    }
}

impl BithumbCommandTranslator {
    fn topics_to_command(topics: &[(String, String)], subscribe: bool) -> String {
        let raw_channels: Vec<String> = topics
            .iter()
            .map(|(channel, symbol)| format!("{}:{}", channel, symbol))
            .collect();
        format!(
            r#"{{"cmd":"{}","args":{}}}"#,
            if subscribe {
                "subscribe"
            } else {
                "unsubscribe"
            },
            serde_json::to_string(&raw_channels).unwrap()
        )
    }
}

impl CommandTranslator for BithumbCommandTranslator {
    fn translate_to_commands(&self, subscribe: bool, topics: &[(String, String)]) -> Vec<String> {
        vec![Self::topics_to_command(topics, subscribe)]
    }

    fn translate_to_candlestick_commands(
        &self,
        _subscribe: bool,
        _symbol_interval_list: &[(String, usize)],
    ) -> Vec<String> {
        panic!("Bithumb does NOT have candlestick channel");
    }
}

#[cfg(test)]
mod tests {
    use crate::common::command_translator::CommandTranslator;

    #[test]
    fn test_two_symbols() {
        let translator = super::BithumbCommandTranslator {};
        let commands = translator.translate_to_commands(
            true,
            &vec![
                ("TRADE".to_string(), "BTC-USDT".to_string()),
                ("TRADE".to_string(), "ETH-USDT".to_string()),
            ],
        );

        assert_eq!(1, commands.len());
        assert_eq!(
            r#"{"cmd":"subscribe","args":["TRADE:BTC-USDT","TRADE:ETH-USDT"]}"#,
            commands[0]
        );
    }

    #[test]
    fn test_two_channels() {
        let translator = super::BithumbCommandTranslator {};
        let commands = translator.translate_to_commands(
            true,
            &vec![
                ("TRADE".to_string(), "BTC-USDT".to_string()),
                ("ORDERBOOK".to_string(), "BTC-USDT".to_string()),
            ],
        );

        assert_eq!(1, commands.len());
        assert_eq!(
            r#"{"cmd":"subscribe","args":["TRADE:BTC-USDT","ORDERBOOK:BTC-USDT"]}"#,
            commands[0]
        );
    }
}
