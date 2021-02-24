use std::collections::HashMap;

use log::*;
use reqwest::{header, Result};
use serde_json::Value;

use crate::clients::{utils::CHANNEL_PAIR_DELIMITER, ws_client_internal::MiscMessage};

// Maximum number of batch subscriptions at a time: 100 topics
// See https://docs.kucoin.cc/#request-rate-limit
const MAX_SUBSCRIPTIONS_PER_TIME: usize = 100;

pub(super) struct WebsocketToken {
    pub token: String,
    pub endpoint: String,
    pub ping_interval: i64,
}

fn http_post(url: &str) -> Result<String> {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/json"),
    );

    let client = reqwest::blocking::Client::builder()
         .default_headers(headers)
         .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/87.0.4280.88 Safari/537.36")
         .gzip(true)
         .build()?;
    let response = client.post(url).send()?;

    match response.error_for_status() {
        Ok(resp) => Ok(resp.text()?),
        Err(error) => Err(error),
    }
}

// See <https://docs.kucoin.com/#apply-connect-token>
pub(super) fn fetch_ws_token() -> WebsocketToken {
    let txt = http_post("https://openapi-v2.kucoin.com/api/v1/bullet-public").unwrap();
    let obj = serde_json::from_str::<HashMap<String, Value>>(&txt).unwrap();
    let code = obj.get("code").unwrap().as_str().unwrap();
    if code != "200000" {
        panic!("Failed to get token, code is {}", code);
    }
    let data = obj.get("data").unwrap().as_object().unwrap();
    let token = data.get("token").unwrap().as_str().unwrap();
    let servers = data.get("instanceServers").unwrap().as_array().unwrap();
    let server = servers[0].as_object().unwrap();

    WebsocketToken {
        token: token.to_string(),
        endpoint: server
            .get("endpoint")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string(),
        ping_interval: server.get("pingInterval").unwrap().as_i64().unwrap(),
    }
}

const EXCHANGE_NAME: &str = "kucoin";

pub(super) fn on_misc_msg(msg: &str) -> MiscMessage {
    let obj = serde_json::from_str::<HashMap<String, Value>>(&msg).unwrap();
    let msg_type = obj.get("type").unwrap().as_str().unwrap();
    match msg_type {
        "pong" | "welcome" | "ack" => {
            debug!("Received {} from {}", msg, EXCHANGE_NAME);
            MiscMessage::Misc
        }
        "notice" | "command" => {
            info!("Received {} from {}", msg, EXCHANGE_NAME);
            MiscMessage::Misc
        }
        "message" => MiscMessage::Normal,
        "error" => {
            error!("Received {} from {}", msg, EXCHANGE_NAME);
            panic!("Received {} from {}", msg, EXCHANGE_NAME);
        }
        _ => {
            error!("Received {} from {}", msg, EXCHANGE_NAME);
            panic!("Received {} from {}", msg, EXCHANGE_NAME);
        }
    }
}

fn channel_pairs_to_command(channel: &str, pairs: &[String], subscribe: bool) -> String {
    format!(
        r#"{{"id":"crypto-ws-client","type":"{}","topic":"{}:{}","privateChannel":false,"response":true}}"#,
        if subscribe {
            "subscribe"
        } else {
            "unsubscribe"
        },
        channel,
        pairs.join(",")
    )
}

pub(super) fn channels_to_commands(channels: &[String], subscribe: bool) -> Vec<String> {
    let mut all_commands: Vec<String> = channels
        .iter()
        .filter(|ch| ch.starts_with('{'))
        .map(|s| s.to_string())
        .collect();

    let mut channel_pairs = HashMap::<String, Vec<String>>::new();
    for s in channels.iter().filter(|ch| !ch.starts_with('{')) {
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

    for (channel, pairs) in channel_pairs.iter() {
        let mut chunk: Vec<String> = Vec::new();
        for pair in pairs.iter() {
            chunk.push(pair.clone());
            if chunk.len() >= MAX_SUBSCRIPTIONS_PER_TIME {
                all_commands.push(channel_pairs_to_command(channel, &chunk, subscribe));
                chunk.clear();
            }
        }
        if !chunk.is_empty() {
            all_commands.push(channel_pairs_to_command(channel, &chunk, subscribe));
        }
    }

    all_commands
}

pub(super) fn to_raw_channel(channel: &str, pair: &str) -> String {
    format!("{}:{}", channel, pair)
}

#[cfg(test)]
mod tests {
    #[test]
    fn fetch_ws_token() {
        let ws_token = super::fetch_ws_token();
        assert!(!ws_token.token.is_empty())
    }
}
