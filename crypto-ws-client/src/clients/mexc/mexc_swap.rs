use async_trait::async_trait;
use std::collections::HashMap;
use tokio_tungstenite::tungstenite::Message;

use super::EXCHANGE_NAME;
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

pub(super) const SWAP_WEBSOCKET_URL: &str = "wss://contract.mexc.com/ws";

/// MEXC Swap market.
///
///   * WebSocket API doc: <https://mxcdevelop.github.io/APIDoc/contract.api.en.html#websocket-api>
///   * Trading at: <https://contract.mexc.com/exchange>
pub struct MexcSwapWSClient {
    client: WSClientInternal<MexcMessageHandler>,
    translator: MexcCommandTranslator,
}

impl_new_constructor!(
    MexcSwapWSClient,
    EXCHANGE_NAME,
    SWAP_WEBSOCKET_URL,
    MexcMessageHandler {},
    MexcCommandTranslator {}
);

#[rustfmt::skip]
impl_trait!(Trade, MexcSwapWSClient, subscribe_trade, "deal");
#[rustfmt::skip]
impl_trait!(Ticker, MexcSwapWSClient, subscribe_ticker, "ticker");
#[rustfmt::skip]
impl_trait!(OrderBook, MexcSwapWSClient, subscribe_orderbook, "depth");
#[rustfmt::skip]
impl_trait!(OrderBookTopK, MexcSwapWSClient, subscribe_orderbook_topk, "depth.full");
impl_candlestick!(MexcSwapWSClient);

panic_bbo!(MexcSwapWSClient);
panic_l3_orderbook!(MexcSwapWSClient);

impl_ws_client_trait!(MexcSwapWSClient);

struct MexcMessageHandler {}
struct MexcCommandTranslator {}

impl MessageHandler for MexcMessageHandler {
    fn handle_message(&mut self, msg: &str) -> MiscMessage {
        let obj = serde_json::from_str::<HashMap<String, Value>>(msg).unwrap();
        if obj.contains_key("channel") && obj.contains_key("data") && obj.contains_key("ts") {
            let channel = obj.get("channel").unwrap().as_str().unwrap();
            match channel {
                "pong" => MiscMessage::Pong,
                "rs.error" => {
                    error!("Received {} from {}", msg, EXCHANGE_NAME);
                    panic!("Received {} from {}", msg, EXCHANGE_NAME);
                }
                _ => {
                    if obj.contains_key("symbol") && channel.starts_with("push.") {
                        MiscMessage::Normal
                    } else {
                        info!("Received {} from {}", msg, EXCHANGE_NAME);
                        MiscMessage::Other
                    }
                }
            }
        } else {
            error!("Received {} from {}", msg, SWAP_WEBSOCKET_URL);
            MiscMessage::Other
        }
    }

    fn get_ping_msg_and_interval(&self) -> Option<(Message, u64)> {
        // more than 60 seconds no response, close the channel
        Some((Message::Text(r#"{"method":"ping"}"#.to_string()), 60))
    }
}

impl MexcCommandTranslator {
    fn topic_to_command(channel: &str, symbol: &str, subscribe: bool) -> String {
        format!(
            r#"{{"method":"{}.{}","param":{{"symbol":"{}"}}}}"#,
            if subscribe { "sub" } else { "unsub" },
            channel,
            symbol
        )
    }

    fn interval_to_string(interval: usize) -> String {
        let tmp = match interval {
            60 => "Min1",
            300 => "Min5",
            900 => "Min15",
            1800 => "Min30",
            3600 => "Min60",
            14400 => "Hour4",
            28800 => "Hour8",
            86400 => "Day1",
            604800 => "Week1",
            2592000 => "Month1",
            _ => panic!(
                "MEXC has intervals Min1,Min5,Min15,Min30,Min60,Hour4,Hour8,Day1,Week1,Month1"
            ),
        };
        tmp.to_string()
    }
}

impl CommandTranslator for MexcCommandTranslator {
    fn translate_to_commands(&self, subscribe: bool, topics: &[(String, String)]) -> Vec<String> {
        topics
            .iter()
            .map(|(channel, symbol)| {
                MexcCommandTranslator::topic_to_command(channel, symbol, subscribe)
            })
            .collect()
    }

    fn translate_to_candlestick_commands(
        &self,
        subscribe: bool,
        symbol_interval_list: &[(String, usize)],
    ) -> Vec<String> {
        symbol_interval_list
            .iter()
            .map(|(symbol, interval)| {
                format!(
                    r#"{{"method":"{}.kline","param":{{"symbol":"{}","interval":"{}"}}}}"#,
                    if subscribe { "sub" } else { "unsub" },
                    symbol,
                    Self::interval_to_string(*interval)
                )
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::common::command_translator::CommandTranslator;

    #[test]
    fn test_one_topic() {
        let translator = super::MexcCommandTranslator {};
        let commands =
            translator.translate_to_commands(true, &[("deal".to_string(), "BTC_USDT".to_string())]);

        assert_eq!(1, commands.len());
        assert_eq!(r#"{"method":"sub.deal","param":{"symbol":"BTC_USDT"}}"#, commands[0]);
    }

    #[test]
    fn test_two_topic() {
        let translator = super::MexcCommandTranslator {};
        let commands = translator.translate_to_commands(
            true,
            &[
                ("deal".to_string(), "BTC_USDT".to_string()),
                ("depth".to_string(), "ETH_USDT".to_string()),
            ],
        );

        assert_eq!(2, commands.len());
        assert_eq!(r#"{"method":"sub.deal","param":{"symbol":"BTC_USDT"}}"#, commands[0]);
        assert_eq!(r#"{"method":"sub.depth","param":{"symbol":"ETH_USDT"}}"#, commands[1]);
    }

    #[test]
    fn test_candlestick() {
        let translator = super::MexcCommandTranslator {};
        let commands =
            translator.translate_to_candlestick_commands(true, &[("BTC_USDT".to_string(), 60)]);

        assert_eq!(1, commands.len());
        assert_eq!(
            r#"{"method":"sub.kline","param":{"symbol":"BTC_USDT","interval":"Min1"}}"#,
            commands[0]
        );
    }
}
