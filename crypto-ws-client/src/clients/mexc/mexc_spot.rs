use async_trait::async_trait;
use std::collections::HashMap;

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

pub(super) const SPOT_WEBSOCKET_URL: &str = "wss://wbs.mexc.com/raw/ws";

/// MEXC Spot market.
///
///   * WebSocket API doc: <https://github.com/mxcdevelop/APIDoc/blob/master/websocket/spot/websocket-api.md>
///   * Trading at: <https://www.mexc.com/exchange/BTC_USDT>
pub struct MexcSpotWSClient {
    client: WSClientInternal<MexcMessageHandler>,
    translator: MexcCommandTranslator,
}

impl_new_constructor!(
    MexcSpotWSClient,
    EXCHANGE_NAME,
    SPOT_WEBSOCKET_URL,
    MexcMessageHandler {},
    MexcCommandTranslator {}
);

#[rustfmt::skip]
impl_trait!(Trade, MexcSpotWSClient, subscribe_trade, "deal");
#[rustfmt::skip]
impl_trait!(OrderBook, MexcSpotWSClient, subscribe_orderbook, "depth");
#[rustfmt::skip]
impl_trait!(OrderBookTopK, MexcSpotWSClient, subscribe_orderbook_topk, "limit.depth");
impl_candlestick!(MexcSpotWSClient);

panic_bbo!(MexcSpotWSClient);
panic_ticker!(MexcSpotWSClient);
panic_l3_orderbook!(MexcSpotWSClient);

impl_ws_client_trait!(MexcSpotWSClient);

struct MexcMessageHandler {}
struct MexcCommandTranslator {}

impl MessageHandler for MexcMessageHandler {
    fn handle_message(&mut self, msg: &str) -> MiscMessage {
        if msg == "pong" {
            return MiscMessage::Pong;
        }
        if let Ok(obj) = serde_json::from_str::<HashMap<String, Value>>(msg) {
            if obj.contains_key("channel") && obj.contains_key("data") {
                let channel = obj.get("channel").unwrap().as_str().unwrap();
                match channel {
                    "push.deal" | "push.depth" | "push.limit.depth" | "push.kline" => {
                        if obj.contains_key("symbol") {
                            MiscMessage::Normal
                        } else {
                            warn!("Received {} from {}", msg, EXCHANGE_NAME);
                            MiscMessage::Other
                        }
                    }
                    "push.overview" => MiscMessage::Normal,
                    _ => {
                        warn!("Received {} from {}", msg, EXCHANGE_NAME);
                        MiscMessage::Other
                    }
                }
            } else {
                warn!("Received {} from {}", msg, EXCHANGE_NAME);
                MiscMessage::Other
            }
        } else {
            warn!("Received {} from {}", msg, EXCHANGE_NAME);
            MiscMessage::Other
        }
    }

    fn get_ping_msg_and_interval(&self) -> Option<(String, u64)> {
        Some(("ping".to_string(), 5))
    }
}

impl MexcCommandTranslator {
    fn topic_to_command(channel: &str, symbol: &str, subscribe: bool) -> String {
        if channel == "limit.depth" {
            format!(
                r#"{{"op":"{}.{}","symbol":"{}","depth": 5}}"#,
                if subscribe { "sub" } else { "unsub" },
                channel,
                symbol
            )
        } else {
            format!(
                r#"{{"op":"{}.{}","symbol":"{}"}}"#,
                if subscribe { "sub" } else { "unsub" },
                channel,
                symbol
            )
        }
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
                    r#"{{"op":"{}.kline","symbol":"{}","interval":"{}"}}"#,
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
        let commands = translator
            .translate_to_commands(true, &vec![("deal".to_string(), "BTC_USDT".to_string())]);

        assert_eq!(1, commands.len());
        assert_eq!(r#"{"op":"sub.deal","symbol":"BTC_USDT"}"#, commands[0]);
    }

    #[test]
    fn test_two_topic() {
        let translator = super::MexcCommandTranslator {};
        let commands = translator.translate_to_commands(
            true,
            &vec![
                ("deal".to_string(), "BTC_USDT".to_string()),
                ("depth".to_string(), "ETH_USDT".to_string()),
            ],
        );

        assert_eq!(2, commands.len());
        assert_eq!(r#"{"op":"sub.deal","symbol":"BTC_USDT"}"#, commands[0]);
        assert_eq!(r#"{"op":"sub.depth","symbol":"ETH_USDT"}"#, commands[1]);
    }

    #[test]
    fn test_candlestick() {
        let translator = super::MexcCommandTranslator {};
        let commands =
            translator.translate_to_candlestick_commands(true, &vec![("BTC_USDT".to_string(), 60)]);

        assert_eq!(1, commands.len());
        assert_eq!(
            r#"{"op":"sub.kline","symbol":"BTC_USDT","interval":"Min1"}"#,
            commands[0]
        );
    }
}
