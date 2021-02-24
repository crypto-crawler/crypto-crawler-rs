use std::collections::HashMap;

use log::*;
use serde_json::Value;

use crate::clients::ws_client_internal::MiscMessage;

pub(super) const EXCHANGE_NAME: &str = "bybit";

/// See:
/// - https://bybit-exchange.github.io/docs/inverse/#t-heartbeat
/// - https://bybit-exchange.github.io/docs/linear/#t-heartbeat
pub(super) const PING_INTERVAL_AND_MSG: (u64, &str) = (30, r#"{"op":"ping"}"#);

pub(super) fn channels_to_commands(channels: &[String], subscribe: bool) -> Vec<String> {
    let mut all_commands: Vec<String> = channels
        .iter()
        .filter(|ch| ch.starts_with('{'))
        .map(|s| s.to_string())
        .collect();

    let channels_to_parse: Vec<&String> =
        channels.iter().filter(|ch| !ch.starts_with('{')).collect();
    if !channels_to_parse.is_empty() {
        all_commands.append(&mut vec![format!(
            r#"{{"op":"{}","args":{}}}"#,
            if subscribe {
                "subscribe"
            } else {
                "unsubscribe"
            },
            serde_json::to_string(&channels_to_parse).unwrap()
        )])
    };

    all_commands
}

pub(super) fn on_misc_msg(msg: &str) -> MiscMessage {
    let obj = serde_json::from_str::<HashMap<String, Value>>(&msg).unwrap();

    if obj.contains_key("topic") && obj.contains_key("data") {
        MiscMessage::Normal
    } else {
        if obj.contains_key("success") {
            if obj.get("success").unwrap().as_bool().unwrap() {
                info!("Received {} from {}", msg, EXCHANGE_NAME);
            } else {
                error!("Received {} from {}", msg, EXCHANGE_NAME);
                panic!("Received {} from {}", msg, EXCHANGE_NAME);
            }
        } else {
            warn!("Received {} from {}", msg, EXCHANGE_NAME);
        }
        MiscMessage::Misc
    }
}

pub(super) fn to_raw_channel(channel: &str, pair: &str) -> String {
    format!("{}.{}", channel, pair)
}
