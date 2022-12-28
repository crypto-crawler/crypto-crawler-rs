use nonzero_ext::nonzero;
use std::{
    collections::{BTreeMap, HashMap},
    num::NonZeroU32,
};
use tokio_tungstenite::tungstenite::Message;

use log::*;
use serde_json::Value;

use crate::common::{
    command_translator::CommandTranslator,
    message_handler::{MessageHandler, MiscMessage},
    utils::ensure_frame_size,
};

pub(crate) const EXCHANGE_NAME: &str = "bitget";

// The total length of multiple channel can not exceeds 4096 bytes, see:
// * https://bitgetlimited.github.io/apidoc/en/mix/#subscribe
// * https://bitgetlimited.github.io/apidoc/en/spot/#subscribe
const WS_FRAME_SIZE: usize = 4096;

// Subscription limit: 240 times per hour, see:
// * https://bitgetlimited.github.io/apidoc/en/mix/#connect
// * https://bitgetlimited.github.io/apidoc/en/spot/#connect
pub(super) const UPLINK_LIMIT: (NonZeroU32, std::time::Duration) =
    (nonzero!(240u32), std::time::Duration::from_secs(3600));

// MARKET_TYPE: S for SP, M for MC
pub(super) struct BitgetMessageHandler {}
pub(super) struct BitgetCommandTranslator<const MARKET_TYPE: char> {}

impl<const MARKET_TYPE: char> BitgetCommandTranslator<MARKET_TYPE> {
    // doc: https://bitgetlimited.github.io/apidoc/en/spot/#subscribe
    fn topics_to_command(topics: &[(String, String)], subscribe: bool) -> String {
        let arr = topics
            .iter()
            .map(|t| {
                let mut map = BTreeMap::new();
                let (channel, symbol) = t;
                // websocket doesn't recognize SPBL, DMCBL and UMCBL suffixes
                let symbol = if let Some(x) = symbol.strip_suffix("_SPBL") {
                    x
                } else if let Some(x) = symbol.strip_suffix("_DMCBL") {
                    x
                } else if let Some(x) = symbol.strip_suffix("_UMCBL") {
                    x
                } else {
                    symbol
                };
                map.insert(
                    "instType".to_string(),
                    (if MARKET_TYPE == 'S' { "SP" } else { "MC" }).to_string(),
                );
                map.insert("channel".to_string(), channel.to_string());
                map.insert("instId".to_string(), symbol.to_string());
                map
            })
            .collect::<Vec<BTreeMap<String, String>>>();
        format!(
            r#"{{"op":"{}","args":{}}}"#,
            if subscribe { "subscribe" } else { "unsubscribe" },
            serde_json::to_string(&arr).unwrap(),
        )
    }

    // https://bitgetlimited.github.io/apidoc/en/spot/#candlesticks-channel
    // https://bitgetlimited.github.io/apidoc/en/mix/#candlesticks-channel
    fn to_candlestick_raw_channel(interval: usize) -> &'static str {
        match interval {
            60 => "candle1m",
            300 => "candle5m",
            900 => "candle15m",
            1800 => "candle30m",
            3600 => "candle1H",
            14400 => "candle4H",
            43200 => "candle12H",
            86400 => "candle1D",
            604800 => "candle1W",
            _ => panic!("Invalid Bitget candlestick interval {}", interval),
        }
    }
}

impl MessageHandler for BitgetMessageHandler {
    // the logic is almost the same with OKX
    fn handle_message(&mut self, msg: &str) -> MiscMessage {
        if msg == "pong" {
            // see https://bitgetlimited.github.io/apidoc/en/spot/#connect
            return MiscMessage::Pong;
        }
        let resp = serde_json::from_str::<HashMap<String, Value>>(msg);
        if resp.is_err() {
            error!("{} is not a JSON string, {}", msg, EXCHANGE_NAME);
            return MiscMessage::Other;
        }
        let obj = resp.unwrap();

        if let Some(event) = obj.get("event") {
            match event.as_str().unwrap() {
                "error" => error!("Received {} from {}", msg, EXCHANGE_NAME),
                "subscribe" => info!("Received {} from {}", msg, EXCHANGE_NAME),
                "unsubscribe" => info!("Received {} from {}", msg, EXCHANGE_NAME),
                _ => warn!("Received {} from {}", msg, EXCHANGE_NAME),
            }
            MiscMessage::Other
        } else if !obj.contains_key("arg") || !obj.contains_key("data") {
            error!("Received {} from {}", msg, EXCHANGE_NAME);
            MiscMessage::Other
        } else {
            MiscMessage::Normal
        }
    }

    fn get_ping_msg_and_interval(&self) -> Option<(Message, u64)> {
        // https://bitgetlimited.github.io/apidoc/en/spot/#connect
        Some((Message::Text("ping".to_string()), 30))
    }
}

impl<const MARKET_TYPE: char> CommandTranslator for BitgetCommandTranslator<MARKET_TYPE> {
    fn translate_to_commands(&self, subscribe: bool, topics: &[(String, String)]) -> Vec<String> {
        ensure_frame_size(topics, subscribe, Self::topics_to_command, WS_FRAME_SIZE, None)
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
                (channel.to_string(), symbol.to_string())
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
        let translator = super::BitgetCommandTranslator::<'S'> {};
        let commands =
            translator.translate_to_commands(true, &[("trade".to_string(), "BTCUSDT".to_string())]);

        assert_eq!(1, commands.len());
        assert_eq!(
            r#"{"op":"subscribe","args":[{"channel":"trade","instId":"BTCUSDT","instType":"SP"}]}"#,
            commands[0]
        );
    }

    #[test]
    fn test_two_topics() {
        let translator = super::BitgetCommandTranslator::<'S'> {};
        let commands = translator.translate_to_commands(
            true,
            &[
                ("trade".to_string(), "BTCUSDT".to_string()),
                ("books".to_string(), "ETHUSDT".to_string()),
            ],
        );

        assert_eq!(1, commands.len());
        assert_eq!(
            r#"{"op":"subscribe","args":[{"channel":"trade","instId":"BTCUSDT","instType":"SP"},{"channel":"books","instId":"ETHUSDT","instType":"SP"}]}"#,
            commands[0]
        );
    }

    #[test]
    fn test_candlestick() {
        let translator = super::BitgetCommandTranslator::<'S'> {};
        let commands = translator.translate_to_candlestick_commands(
            true,
            &[("BTCUSDT".to_string(), 60), ("ETHUSDT".to_string(), 300)],
        );

        assert_eq!(1, commands.len());
        assert_eq!(
            r#"{"op":"subscribe","args":[{"channel":"candle1m","instId":"BTCUSDT","instType":"SP"},{"channel":"candle5m","instId":"ETHUSDT","instType":"SP"}]}"#,
            commands[0]
        );
    }
}
