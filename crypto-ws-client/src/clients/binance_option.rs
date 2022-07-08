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

pub(crate) const EXCHANGE_NAME: &str = "binance";

pub(super) const WEBSOCKET_URL: &str = "wss://stream.opsnest.com/stream";

/// Binance Option market
///
///   * WebSocket API doc: <https://binance-docs.github.io/apidocs/voptions/en/>
///   * Trading at: <https://voptions.binance.com/en>
pub struct BinanceOptionWSClient {
    client: WSClientInternal<BinanceOptionMessageHandler>,
    translator: BinanceOptionCommandTranslator,
}

impl_new_constructor!(
    BinanceOptionWSClient,
    EXCHANGE_NAME,
    WEBSOCKET_URL,
    BinanceOptionMessageHandler {},
    BinanceOptionCommandTranslator {}
);

impl_trait!(Trade, BinanceOptionWSClient, subscribe_trade, "trade");
impl_trait!(Ticker, BinanceOptionWSClient, subscribe_ticker, "ticker");
impl_trait!(BBO, BinanceOptionWSClient, subscribe_bbo, "bookTicker");
#[rustfmt::skip]
impl_trait!(OrderBook, BinanceOptionWSClient, subscribe_orderbook, "depth@100ms");
#[rustfmt::skip]
impl_trait!(OrderBookTopK, BinanceOptionWSClient, subscribe_orderbook_topk, "depth10");
impl_candlestick!(BinanceOptionWSClient);
panic_l3_orderbook!(BinanceOptionWSClient);

impl_ws_client_trait!(BinanceOptionWSClient);

struct BinanceOptionMessageHandler {}
struct BinanceOptionCommandTranslator {}

impl BinanceOptionCommandTranslator {
    fn topics_to_command(topics: &[(String, String)], subscribe: bool) -> String {
        let raw_topics = topics
            .iter()
            .map(|(topic, symbol)| format!("{}@{}", symbol, topic))
            .collect::<Vec<String>>();
        format!(
            r#"{{"id":9527,"method":"{}","params":{}}}"#,
            if subscribe {
                "SUBSCRIBE"
            } else {
                "UNSUBSCRIBE"
            },
            serde_json::to_string(&raw_topics).unwrap()
        )
    }

    // see https://binance-docs.github.io/apidocs/voptions/en/#payload-candle
    fn to_candlestick_raw_channel(interval: usize) -> String {
        let interval_str = match interval {
            60 => "1m",
            300 => "5m",
            900 => "15m",
            1800 => "30m",
            3600 => "1h",
            14400 => "4h",
            86400 => "1d",
            604800 => "1w",
            _ => panic!("Binance Option has intervals 1m,5m,15m,30m,1h4h,1d,1w"),
        };
        format!("kline_{}", interval_str)
    }
}

impl MessageHandler for BinanceOptionMessageHandler {
    fn handle_message(&mut self, msg: &str) -> MiscMessage {
        if msg == r#"{"id":9527}"# {
            return MiscMessage::Other;
        } else if msg == r#"{"event":"pong"}"# {
            return MiscMessage::Pong;
        }

        let resp = serde_json::from_str::<HashMap<String, Value>>(msg);
        if resp.is_err() {
            error!("{} is not a JSON string, {}", msg, EXCHANGE_NAME);
            return MiscMessage::Other;
        }
        let obj = resp.unwrap();

        if obj.contains_key("code") {
            panic!("Received {} from {}", msg, EXCHANGE_NAME);
        }

        if let Some(result) = obj.get("result") {
            if serde_json::Value::Null == *result {
                return MiscMessage::Other;
            }
        }

        if !obj.contains_key("stream") || !obj.contains_key("data") {
            warn!("Received {} from {}", msg, EXCHANGE_NAME);
            return MiscMessage::Other;
        }

        MiscMessage::Normal
    }

    fn get_ping_msg_and_interval(&self) -> Option<(Message, u64)> {
        // https://binance-docs.github.io/apidocs/voptions/en/#push-websocket-account-info
        // The client will send a ping frame every 2 minutes. If the websocket server does not
        // receive a ping frame back from the connection within a 2 minute period, the
        // connection will be disconnected. Unsolicited ping frames are allowed.
        Some((Message::Text(r#"{"event":"ping"}"#.to_string()), 120))
    }
}

impl CommandTranslator for BinanceOptionCommandTranslator {
    fn translate_to_commands(&self, subscribe: bool, topics: &[(String, String)]) -> Vec<String> {
        let command = Self::topics_to_command(topics, subscribe);
        vec![command]
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
        let translator = super::BinanceOptionCommandTranslator {};
        let commands = translator.translate_to_commands(
            true,
            &[("trade".to_string(), "BTC-220429-50000-C".to_string())],
        );

        assert_eq!(1, commands.len());
        assert_eq!(
            r#"{"id":9527,"method":"SUBSCRIBE","params":["BTC-220429-50000-C@trade"]}"#,
            commands[0]
        );
    }

    #[test]
    fn test_two_topics() {
        let translator = super::BinanceOptionCommandTranslator {};
        let commands = translator.translate_to_commands(
            true,
            &[("trade".to_string(), "BTC-220429-50000-C".to_string()),
                ("ticker".to_string(), "BTC-220429-50000-C".to_string())],
        );

        assert_eq!(1, commands.len());
        assert_eq!(
            r#"{"id":9527,"method":"SUBSCRIBE","params":["BTC-220429-50000-C@trade","BTC-220429-50000-C@ticker"]}"#,
            commands[0]
        );
    }
}
