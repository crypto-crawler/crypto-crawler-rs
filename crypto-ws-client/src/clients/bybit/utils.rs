use std::collections::HashMap;

use log::*;
use serde_json::Value;
use tokio_tungstenite::tungstenite::Message;

use crate::common::message_handler::{MessageHandler, MiscMessage};

pub(super) const EXCHANGE_NAME: &str = "bybit";

pub(super) fn topics_to_command(topics: &[(String, String)], subscribe: bool) -> String {
    let raw_channels = topics
        .iter()
        .map(|(channel, symbol)| format!("{}.{}", channel, symbol))
        .collect::<Vec<String>>();
    format!(
        r#"{{"op":"{}","args":{}}}"#,
        if subscribe { "subscribe" } else { "unsubscribe" },
        serde_json::to_string(&raw_channels).unwrap()
    )
}

pub(super) struct BybitMessageHandler {}

impl MessageHandler for BybitMessageHandler {
    fn handle_message(&mut self, msg: &str) -> MiscMessage {
        let obj = serde_json::from_str::<HashMap<String, Value>>(msg).unwrap();

        if obj.contains_key("topic") && obj.contains_key("data") {
            MiscMessage::Normal
        } else {
            if obj.contains_key("success") {
                if obj.get("success").unwrap().as_bool().unwrap() {
                    info!("Received {} from {}", msg, EXCHANGE_NAME);
                    if obj.contains_key("ret_msg")
                        && obj.get("ret_msg").unwrap().as_str().unwrap() == "pong"
                    {
                        return MiscMessage::Pong;
                    }
                } else {
                    error!("Received {} from {}", msg, EXCHANGE_NAME);
                    panic!("Received {} from {}", msg, EXCHANGE_NAME);
                }
            } else {
                warn!("Received {} from {}", msg, EXCHANGE_NAME);
            }
            MiscMessage::Other
        }
    }

    fn get_ping_msg_and_interval(&self) -> Option<(Message, u64)> {
        // See:
        // - https://bybit-exchange.github.io/docs/inverse/#t-heartbeat
        // - https://bybit-exchange.github.io/docs/linear/#t-heartbeat
        Some((Message::Text(r#"{"op":"ping"}"#.to_string()), 30))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_one_channel() {
        let command =
            super::topics_to_command(&[("trade".to_string(), "BTCUSD".to_string())], true);
        assert_eq!(r#"{"op":"subscribe","args":["trade.BTCUSD"]}"#, command);
    }

    #[test]
    fn test_multiple_channels() {
        let command = super::topics_to_command(
            &[
                ("trade".to_string(), "BTCUSD".to_string()),
                ("orderBookL2_25".to_string(), "BTCUSD".to_string()),
                ("instrument_info.100ms".to_string(), "BTCUSD".to_string()),
            ],
            true,
        );

        assert_eq!(
            r#"{"op":"subscribe","args":["trade.BTCUSD","orderBookL2_25.BTCUSD","instrument_info.100ms.BTCUSD"]}"#,
            command
        );
    }
}
