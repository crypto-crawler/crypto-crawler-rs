use async_trait::async_trait;
use std::{collections::HashMap, time::Duration};
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

pub(super) const EXCHANGE_NAME: &str = "bitmex";

const WEBSOCKET_URL: &str = "wss://www.bitmex.com/realtime";

// Too many args sent. Max length is 20
const MAX_CHANNELS_PER_COMMAND: usize = 20;

/// The WebSocket client for BitMEX.
///
/// BitMEX has Swap and Future markets.
///
///   * WebSocket API doc: <https://www.bitmex.com/app/wsAPI>
///   * Trading at: <https://www.bitmex.com/app/trade/>
pub struct BitmexWSClient {
    client: WSClientInternal<BitmexMessageHandler>,
    translator: BitmexCommandTranslator,
}

impl_new_constructor!(
    BitmexWSClient,
    EXCHANGE_NAME,
    WEBSOCKET_URL,
    BitmexMessageHandler {},
    BitmexCommandTranslator {}
);

impl_trait!(Trade, BitmexWSClient, subscribe_trade, "trade");
impl_trait!(BBO, BitmexWSClient, subscribe_bbo, "quote");
#[rustfmt::skip]
impl_trait!(OrderBook, BitmexWSClient, subscribe_orderbook, "orderBookL2");
#[rustfmt::skip]
impl_trait!(OrderBookTopK, BitmexWSClient, subscribe_orderbook_topk, "orderBook10");
impl_candlestick!(BitmexWSClient);
panic_l3_orderbook!(BitmexWSClient);
panic_ticker!(BitmexWSClient);

impl_ws_client_trait!(BitmexWSClient);

struct BitmexMessageHandler {}
struct BitmexCommandTranslator {}

impl BitmexCommandTranslator {
    fn topics_to_command(topics: &[(String, String)], subscribe: bool) -> String {
        let raw_channels = topics
            .iter()
            .map(|(channel, symbol)| format!("{}:{}", channel, symbol))
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

    // see https://www.okx.com/docs-v5/en/#websocket-api-public-channel-candlesticks-channel
    fn to_candlestick_raw_channel(interval: usize) -> String {
        let interval_str = match interval {
            60 => "1m",
            300 => "5m",
            3600 => "1h",
            86400 => "1d",
            _ => panic!("BitMEX has intervals 1m,5m,1h,1d"),
        };
        format!("tradeBin{}", interval_str)
    }
}

impl MessageHandler for BitmexMessageHandler {
    fn handle_message(&mut self, msg: &str) -> MiscMessage {
        if msg == "pong" {
            return MiscMessage::Pong;
        }
        let resp = serde_json::from_str::<HashMap<String, Value>>(msg);
        if resp.is_err() {
            error!("{} is not a JSON string, {}", msg, EXCHANGE_NAME);
            return MiscMessage::Other;
        }
        let obj = resp.unwrap();

        if obj.contains_key("error") {
            let error_msg = obj.get("error").unwrap().as_str().unwrap();
            let code = obj.get("status").unwrap().as_i64().unwrap();

            match code {
                // Rate limit exceeded
                429 => {
                    error!("Received {} from {}", msg, EXCHANGE_NAME);
                    std::thread::sleep(Duration::from_secs(3));
                }
                400 => {
                    if error_msg.starts_with("Unknown") {
                        panic!("Received {} from {}", msg, EXCHANGE_NAME);
                    } else if error_msg.starts_with("You are already subscribed to this topic") {
                        info!("Received {} from {}", msg, EXCHANGE_NAME)
                    } else {
                        warn!("Received {} from {}", msg, EXCHANGE_NAME);
                    }
                }
                _ => error!("Received {} from {}", msg, EXCHANGE_NAME),
            }
            MiscMessage::Other
        } else if obj.contains_key("success") || obj.contains_key("info") {
            info!("Received {} from {}", msg, EXCHANGE_NAME);
            MiscMessage::Other
        } else if obj.contains_key("table")
            && obj.contains_key("action")
            && obj.contains_key("data")
        {
            MiscMessage::Normal
        } else {
            warn!("Received {} from {}", msg, EXCHANGE_NAME);
            MiscMessage::Other
        }
    }

    fn get_ping_msg_and_interval(&self) -> Option<(Message, u64)> {
        Some((Message::Text("ping".to_string()), 5))
    }
}

impl CommandTranslator for BitmexCommandTranslator {
    fn translate_to_commands(&self, subscribe: bool, topics: &[(String, String)]) -> Vec<String> {
        let mut commands: Vec<String> = Vec::new();

        let n = topics.len();
        for i in (0..n).step_by(MAX_CHANNELS_PER_COMMAND) {
            let chunk: Vec<(String, String)> =
                (topics[i..(std::cmp::min(i + MAX_CHANNELS_PER_COMMAND, n))]).to_vec();
            commands.push(Self::topics_to_command(&chunk, subscribe));
        }

        commands
    }

    fn translate_to_candlestick_commands(
        &self,
        subscribe: bool,
        symbol_interval_list: &[(String, usize)],
    ) -> Vec<String> {
        let topics = symbol_interval_list
            .iter()
            .map(|(symbol, interval)| {
                let channel = Self::to_candlestick_raw_channel(*interval);
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
        let translator = super::BitmexCommandTranslator {};
        let commands = translator
            .translate_to_commands(true, &vec![("trade".to_string(), "XBTUSD".to_string())]);

        assert_eq!(1, commands.len());
        assert_eq!(r#"{"op":"subscribe","args":["trade:XBTUSD"]}"#, commands[0]);
    }

    #[test]
    fn test_multiple_topics() {
        let translator = super::BitmexCommandTranslator {};
        let commands = translator.translate_to_commands(
            true,
            &vec![
                ("trade".to_string(), "XBTUSD".to_string()),
                ("quote".to_string(), "XBTUSD".to_string()),
                ("orderBookL2_25".to_string(), "XBTUSD".to_string()),
                ("tradeBin1m".to_string(), "XBTUSD".to_string()),
            ],
        );

        assert_eq!(1, commands.len());
        assert_eq!(
            r#"{"op":"subscribe","args":["trade:XBTUSD","quote:XBTUSD","orderBookL2_25:XBTUSD","tradeBin1m:XBTUSD"]}"#,
            commands[0]
        );
    }
}
