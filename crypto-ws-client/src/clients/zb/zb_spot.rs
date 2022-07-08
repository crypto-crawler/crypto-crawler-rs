use std::collections::HashMap;

use async_trait::async_trait;
use log::*;
use serde_json::Value;
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

// If you're in China, use wss://api.zbex.site/websocket instead
const WEBSOCKET_URL: &str = "wss://api.zb.com/websocket";

/// The WebSocket client for ZB spot market.
///
/// * WebSocket API doc: <https://www.zb.com/en/api>
/// * Trading at: <https://www.zb.com/en/kline/btc_usdt>
pub struct ZbSpotWSClient {
    client: WSClientInternal<ZbMessageHandler>,
    translator: ZbCommandTranslator,
}

impl_new_constructor!(
    ZbSpotWSClient,
    EXCHANGE_NAME,
    WEBSOCKET_URL,
    ZbMessageHandler {},
    ZbCommandTranslator {}
);

#[rustfmt::skip]
impl_trait!(Trade, ZbSpotWSClient, subscribe_trade, "trades");
#[rustfmt::skip]
impl_trait!(OrderBookTopK, ZbSpotWSClient, subscribe_orderbook_topk, "depth");
#[rustfmt::skip]
impl_trait!(Ticker, ZbSpotWSClient, subscribe_ticker, "ticker");
impl_candlestick!(ZbSpotWSClient);

panic_bbo!(ZbSpotWSClient);
panic_l2!(ZbSpotWSClient);
panic_l3_orderbook!(ZbSpotWSClient);

impl_ws_client_trait!(ZbSpotWSClient);

struct ZbMessageHandler {}
struct ZbCommandTranslator {}

impl MessageHandler for ZbMessageHandler {
    fn handle_message(&mut self, msg: &str) -> MiscMessage {
        let obj = serde_json::from_str::<HashMap<String, Value>>(msg).unwrap();
        let channel = obj["channel"].as_str().unwrap();

        if channel == "pong" {
            return MiscMessage::Pong;
        }
        if let Some(code) = obj.get("code") {
            let code = code.as_i64().unwrap();
            if code != 1000 {
                if code == 1007 {
                    panic!("Received {} from {}", msg, EXCHANGE_NAME);
                } else {
                    error!("Received {} from {}", msg, EXCHANGE_NAME);
                }
                return MiscMessage::Other;
            }
        }
        MiscMessage::Normal
    }

    fn get_ping_msg_and_interval(&self) -> Option<(Message, u64)> {
        Some((
            Message::Text(r#"{"channel":"ping","event":"addChannel"}"#.to_string()),
            3,
        ))
    }
}

impl ZbCommandTranslator {
    fn to_candlestick_raw_channel(&self, symbol: &str, interval: usize) -> String {
        let interval_str = match interval {
            60 => "1min",
            180 => "3min",
            300 => "5min",
            900 => "15min",
            1800 => "30min",
            3600 => "1hour",
            7200 => "2hour",
            14400 => "4hour",
            21600 => "6hour",
            43200 => "12hour",
            86400 => "1day",
            259200 => "3day",
            604800 => "1week",
            _ => panic!("ZB spot available intervals: 1week, 3day, 1day, 12hour, 6hour, 4hour, 2hour, 1hour, 30min, 15min, 5min, 3min, 1min"),
        };
        format!("{}_kline_{}", symbol.replace('_', ""), interval_str,)
    }
}

impl CommandTranslator for ZbCommandTranslator {
    fn translate_to_commands(&self, subscribe: bool, topics: &[(String, String)]) -> Vec<String> {
        topics
            .iter()
            .map(|(channel, symbol)| {
                format!(
                    r#"{{"event":"{}","channel":"{}_{}"}}"#,
                    if subscribe {
                        "addChannel"
                    } else {
                        "removeChannel"
                    },
                    symbol.replace('_', ""),
                    channel,
                )
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
                    r#"{{"event":"{}","channel":"{}"}}"#,
                    if subscribe {
                        "addChannel"
                    } else {
                        "removeChannel"
                    },
                    self.to_candlestick_raw_channel(symbol, *interval),
                )
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::common::command_translator::CommandTranslator;

    #[tokio::test(flavor = "multi_thread")]
    async fn test_one_topic() {
        let translator = super::ZbCommandTranslator {};
        let commands = translator
            .translate_to_commands(true, &[("trades".to_string(), "btc_usdt".to_string())]);

        assert_eq!(1, commands.len());
        assert_eq!(
            r#"{"event":"addChannel","channel":"btcusdt_trades"}"#,
            commands[0]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_two_topic() {
        let translator = super::ZbCommandTranslator {};
        let commands = translator.translate_to_commands(
            true,
            &[
                ("trades".to_string(), "btc_usdt".to_string()),
                ("depth".to_string(), "eth_usdt".to_string()),
            ],
        );

        assert_eq!(2, commands.len());
        assert_eq!(
            r#"{"event":"addChannel","channel":"btcusdt_trades"}"#,
            commands[0]
        );
        assert_eq!(
            r#"{"event":"addChannel","channel":"ethusdt_depth"}"#,
            commands[1]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_candlestick() {
        let translator = super::ZbCommandTranslator {};
        let commands =
            translator.translate_to_candlestick_commands(true, &[("btc_usdt".to_string(), 60)]);

        assert_eq!(1, commands.len());
        assert_eq!(
            r#"{"event":"addChannel","channel":"btcusdt_kline_1min"}"#,
            commands[0]
        );
    }
}
