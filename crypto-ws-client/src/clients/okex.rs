use crate::WSClient;
use std::collections::HashMap;

use super::ws_client_internal::{MiscMessage, WSClientInternal};

use log::*;
use serde_json::Value;

pub(super) const EXCHANGE_NAME: &str = "OKEx";

const WEBSOCKET_URL: &str = "wss://real.okex.com:8443/ws/v3";

/// The WebSocket client for OKEx, including Spot, Futures, Swap and Option(<https://www.okex.com/docs/en/>).
pub struct OKExWSClient<'a> {
    client: WSClientInternal<'a>,
}

fn serialize_command(channels: &[String], subscribe: bool) -> Vec<String> {
    vec![format!(
        r#"{{"op":"{}","args":{}}}"#,
        if subscribe {
            "subscribe"
        } else {
            "unsubscribe"
        },
        serde_json::to_string(channels).unwrap()
    )]
}

fn on_misc_msg(msg: &str) -> MiscMessage {
    let resp = serde_json::from_str::<HashMap<String, Value>>(&msg);
    if resp.is_err() {
        error!("{} is not a JSON string, {}", msg, EXCHANGE_NAME);
        return MiscMessage::Misc;
    }
    let obj = resp.unwrap();

    if let Some(event) = obj.get("event") {
        if event.as_str().unwrap() == "error" {
            error!("Received {} from {}", msg, EXCHANGE_NAME);
            return MiscMessage::Misc;
        }
    }

    if !obj.contains_key("table") || !obj.contains_key("data") {
        warn!("Received {} from {}", msg, EXCHANGE_NAME);
        return MiscMessage::Misc;
    }

    MiscMessage::Normal
}

define_client!(
    OKExWSClient,
    EXCHANGE_NAME,
    WEBSOCKET_URL,
    serialize_command,
    on_misc_msg
);

#[cfg(test)]
mod tests {
    #[test]
    fn test_one_channel() {
        let commands = super::serialize_command(&vec!["spot/trade:BTC-USDT".to_string()], true);
        assert_eq!(1, commands.len());
        assert_eq!(
            r#"{"op":"subscribe","args":["spot/trade:BTC-USDT"]}"#,
            commands[0]
        );
    }

    #[test]
    fn test_two_channel() {
        let commands = super::serialize_command(
            &vec![
                "spot/trade:BTC-USDT".to_string(),
                "ticker/trade:BTC-USDT".to_string(),
            ],
            true,
        );
        assert_eq!(1, commands.len());
        assert_eq!(
            r#"{"op":"subscribe","args":["spot/trade:BTC-USDT","ticker/trade:BTC-USDT"]}"#,
            commands[0]
        );
    }
}
