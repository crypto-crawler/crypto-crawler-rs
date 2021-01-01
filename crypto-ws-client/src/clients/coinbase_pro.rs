use crate::WSClient;
use std::collections::HashMap;

use super::{
    utils::CHANNEL_PAIR_DELIMITER,
    ws_client_internal::{MiscMessage, WSClientInternal},
};

use log::*;
use serde_json::Value;

pub(super) const EXCHANGE_NAME: &str = "CoinbasePro";

const WEBSOCKET_URL: &str = "wss://ws-feed.pro.coinbase.com";

/// The WebSocket client for CoinbasePro, which has only Spot market for now(<https://docs.pro.coinbase.com/>).
pub struct CoinbaseProWSClient<'a> {
    client: WSClientInternal<'a>,
}

fn channel_pairs_to_command(channel: &str, pairs: &[String]) -> String {
    format!(
        r#"{{"name":"{}","product_ids":{}}}"#,
        channel,
        serde_json::to_string(pairs).unwrap(),
    )
}

fn serialize_command(channels: &[String], subscribe: bool) -> Vec<String> {
    let mut channel_pairs = HashMap::<String, Vec<String>>::new();
    for s in channels {
        let v: Vec<&str> = s.split(CHANNEL_PAIR_DELIMITER).collect();
        let channel = v[0];
        let pair = v[1];
        match channel_pairs.get_mut(channel) {
            Some(pairs) => pairs.push(pair.to_string()),
            None => {
                channel_pairs.insert(channel.to_string(), vec![pair.to_string()]);
            }
        }
    }

    let mut command = String::new();
    command.push_str(
        format!(
            r#"{{"type":"{}","channels": ["#,
            if subscribe {
                "subscribe"
            } else {
                "unsubscribe"
            }
        )
        .as_str(),
    );
    for (channel, pairs) in channel_pairs.iter() {
        command.push_str(channel_pairs_to_command(channel, pairs).as_str());
        command.push(',')
    }
    command.pop();
    command.push_str("]}");

    vec![command]
}

fn on_misc_msg(msg: &str) -> MiscMessage {
    let resp = serde_json::from_str::<HashMap<String, Value>>(&msg);
    if resp.is_err() {
        error!("{} is not a JSON string, {}", msg, EXCHANGE_NAME);
        return MiscMessage::Misc;
    }
    let obj = resp.unwrap();

    match obj.get("type").unwrap().as_str().unwrap() {
        "error" => {
            error!("Received {} from {}", msg, EXCHANGE_NAME);
            MiscMessage::Misc
        }
        "subscriptions" => {
            info!("Received {} from {}", msg, EXCHANGE_NAME);
            MiscMessage::Misc
        }
        "heartbeat" => {
            debug!("Received {} from {}", msg, EXCHANGE_NAME);
            MiscMessage::Misc
        }
        _ => MiscMessage::Normal,
    }
}

define_client!(
    CoinbaseProWSClient,
    EXCHANGE_NAME,
    WEBSOCKET_URL,
    serialize_command,
    on_misc_msg
);

#[cfg(test)]
mod tests {
    #[test]
    fn test_two_pairs() {
        assert_eq!(
            r#"{"name":"matches","product_ids":["BTC-USD","ETH-USD"]}"#,
            super::channel_pairs_to_command(
                "matches",
                &vec!["BTC-USD".to_string(), "ETH-USD".to_string()],
            )
        );
    }
}
