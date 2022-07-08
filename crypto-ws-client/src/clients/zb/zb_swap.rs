use std::{collections::HashMap, num::NonZeroU32};

use async_trait::async_trait;
use nonzero_ext::nonzero;
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
use log::*;

use super::EXCHANGE_NAME;

const WEBSOCKET_URL: &str = "wss://fapi.zb.com/ws/public/v1";

// The default limit for the number of requests for a single interface is 200 times/2s
//
// See https://github.com/ZBFuture/docs/blob/main/API%20V2%20_en.md#14-access-limit-frequency-rules
const UPLINK_LIMIT: (NonZeroU32, std::time::Duration) =
    (nonzero!(200u32), std::time::Duration::from_secs(2));

/// The WebSocket client for ZB swap market.
///
/// * WebSocket API doc: <https://github.com/ZBFuture/docs/blob/main/API%20V2%20_en.md>
/// * Trading at: <https://www.zb.com/en/futures/btc_usdt>
pub struct ZbSwapWSClient {
    client: WSClientInternal<ZbMessageHandler>,
    translator: ZbCommandTranslator,
}

impl ZbSwapWSClient {
    pub async fn new(tx: std::sync::mpsc::Sender<String>, url: Option<&str>) -> Self {
        let real_url = match url {
            Some(endpoint) => endpoint,
            None => WEBSOCKET_URL,
        };
        ZbSwapWSClient {
            client: WSClientInternal::connect(
                EXCHANGE_NAME,
                real_url,
                ZbMessageHandler {},
                Some(UPLINK_LIMIT),
                tx,
            )
            .await,
            translator: ZbCommandTranslator {},
        }
    }
}

#[rustfmt::skip]
impl_trait!(Trade, ZbSwapWSClient, subscribe_trade, "Trade");
#[rustfmt::skip]
impl_trait!(OrderBook, ZbSwapWSClient, subscribe_orderbook, "Depth");
#[rustfmt::skip]
impl_trait!(OrderBookTopK, ZbSwapWSClient, subscribe_orderbook_topk, "DepthWhole");
#[rustfmt::skip]
impl_trait!(Ticker, ZbSwapWSClient, subscribe_ticker, "Ticker");
impl_candlestick!(ZbSwapWSClient);

panic_bbo!(ZbSwapWSClient);
panic_l3_orderbook!(ZbSwapWSClient);

impl_ws_client_trait!(ZbSwapWSClient);

struct ZbMessageHandler {}
struct ZbCommandTranslator {}

impl MessageHandler for ZbMessageHandler {
    fn handle_message(&mut self, msg: &str) -> MiscMessage {
        if msg == r#"{"action":"pong"}"# {
            return MiscMessage::Pong;
        }
        if msg.contains("error") {
            error!("Received {} from {}", msg, EXCHANGE_NAME);
            return MiscMessage::Other;
        }
        let obj = serde_json::from_str::<HashMap<String, Value>>(msg).unwrap();
        if obj.contains_key("channel") && obj.contains_key("data") {
            MiscMessage::Normal
        } else {
            warn!("Received {} from {}", msg, EXCHANGE_NAME);
            MiscMessage::Other
        }
    }

    fn get_ping_msg_and_interval(&self) -> Option<(Message, u64)> {
        // https://github.com/ZBFuture/docs/blob/main/API%20V2%20_en.md#812-ping
        Some((Message::Text(r#"{"action":"ping"}"#.to_string()), 10))
    }
}

impl ZbCommandTranslator {
    fn to_candlestick_raw_channel(&self, symbol: &str, interval: usize) -> String {
        let interval_str = match interval {
            60 => "1M",
            300 => "5M",
            900 => "15M",
            1800 => "30M",
            3600 => "1H",
            21600 => "6H",
            86400 => "1D",
            432000 => "5D",
            _ => panic!("ZB swap available intervals: 1M,5M,15M, 30M, 1H, 6H, 1D, 5D"),
        };
        format!("{}.KLine_{}", symbol, interval_str,)
    }
}

impl CommandTranslator for ZbCommandTranslator {
    fn translate_to_commands(&self, subscribe: bool, topics: &[(String, String)]) -> Vec<String> {
        let action = if subscribe {
            "subscribe"
        } else {
            "unsubscribe"
        };
        topics
            .iter()
            .map(|(channel, symbol)| match channel.as_str() {
                "Trade" => format!(
                    r#"{{"action":"{}", "channel":"{}.{}", "size":100}}"#,
                    action, symbol, channel,
                ),
                "Depth" => format!(
                    r#"{{"action":"{}", "channel":"{}.{}", "size":200}}"#,
                    action, symbol, channel,
                ),
                "DepthWhole" => format!(
                    r#"{{"action":"{}", "channel":"{}.{}", "size":10}}"#,
                    action, symbol, channel,
                ),
                "Ticker" => format!(
                    r#"{{"action":"{}", "channel":"{}.{}"}}"#,
                    action, symbol, channel,
                ),
                _ => panic!("Unknown ZB channel {}", channel),
            })
            .collect()
    }

    fn translate_to_candlestick_commands(
        &self,
        subscribe: bool,
        symbol_interval_list: &[(String, usize)],
    ) -> Vec<String> {
        let action = if subscribe {
            "subscribe"
        } else {
            "unsubscribe"
        };
        symbol_interval_list
            .iter()
            .map(|(symbol, interval)| {
                format!(
                    r#"{{"action":"{}", "channel":"{}", "size":1}}"#,
                    action,
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
            .translate_to_commands(true, &[("Trade".to_string(), "BTC_USDT".to_string())]);

        assert_eq!(1, commands.len());
        assert_eq!(
            r#"{"action":"subscribe", "channel":"BTC_USDT.Trade", "size":100}"#,
            commands[0]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_two_topic() {
        let translator = super::ZbCommandTranslator {};
        let commands = translator.translate_to_commands(
            true,
            &[("Trade".to_string(), "BTC_USDT".to_string()),
                ("Depth".to_string(), "ETH_USDT".to_string())],
        );

        assert_eq!(2, commands.len());
        assert_eq!(
            r#"{"action":"subscribe", "channel":"BTC_USDT.Trade", "size":100}"#,
            commands[0]
        );
        assert_eq!(
            r#"{"action":"subscribe", "channel":"ETH_USDT.Depth", "size":200}"#,
            commands[1]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_candlestick() {
        let translator = super::ZbCommandTranslator {};
        let commands =
            translator.translate_to_candlestick_commands(true, &[("BTC_USDT".to_string(), 60)]);

        assert_eq!(1, commands.len());
        assert_eq!(
            r#"{"action":"subscribe", "channel":"BTC_USDT.KLine_1M", "size":1}"#,
            commands[0]
        );
    }
}
