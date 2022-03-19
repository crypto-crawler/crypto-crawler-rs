use std::collections::{BTreeMap, HashMap};

use log::*;
use reqwest::{header, Result};
use serde_json::Value;

use crate::common::message_handler::{MessageHandler, MiscMessage};

pub(super) const EXCHANGE_NAME: &str = "kucoin";

// Maximum number of batch subscriptions at a time: 100 topics
// See https://docs.kucoin.cc/#request-rate-limit
const MAX_TOPICS_PER_COMMAND: usize = 100;

pub(super) struct WebsocketToken {
    pub token: String,
    pub endpoint: String,
}

async fn http_post(url: &str) -> Result<String> {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/json"),
    );

    let client = reqwest::Client::builder()
         .default_headers(headers)
         .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/87.0.4280.88 Safari/537.36")
         .gzip(true)
         .build()?;
    let response = client.post(url).send().await?;

    match response.error_for_status() {
        Ok(resp) => Ok(resp.text().await?),
        Err(error) => Err(error),
    }
}

// See <https://docs.kucoin.com/#apply-connect-token>
pub(super) async fn fetch_ws_token() -> WebsocketToken {
    let txt = http_post("https://openapi-v2.kucoin.com/api/v1/bullet-public")
        .await
        .unwrap();
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
    }
}

fn channel_symbols_to_command(channel: &str, symbols: &[String], subscribe: bool) -> String {
    format!(
        r#"{{"id":"crypto-ws-client","type":"{}","topic":"{}:{}","privateChannel":false,"response":true}}"#,
        if subscribe {
            "subscribe"
        } else {
            "unsubscribe"
        },
        channel,
        symbols.join(",")
    )
}

pub(super) fn topics_to_commands(topics: &[(String, String)], subscribe: bool) -> Vec<String> {
    let mut commands: Vec<String> = Vec::new();

    let mut channel_symbols = BTreeMap::<String, Vec<String>>::new();
    for (channel, symbol) in topics {
        match channel_symbols.get_mut(channel) {
            Some(symbols) => symbols.push(symbol.to_string()),
            None => {
                channel_symbols.insert(channel.to_string(), vec![symbol.to_string()]);
            }
        }
    }

    for (channel, symbols) in channel_symbols.iter() {
        let mut chunk: Vec<String> = Vec::new();
        for symbol in symbols.iter() {
            chunk.push(symbol.clone());
            if chunk.len() >= MAX_TOPICS_PER_COMMAND {
                commands.push(channel_symbols_to_command(channel, &chunk, subscribe));
                chunk.clear();
            }
        }
        if !chunk.is_empty() {
            commands.push(channel_symbols_to_command(channel, &chunk, subscribe));
        }
    }

    commands
}

pub(super) struct KucoinMessageHandler {}

impl MessageHandler for KucoinMessageHandler {
    fn handle_message(&mut self, msg: &str) -> MiscMessage {
        let obj = serde_json::from_str::<HashMap<String, Value>>(msg).unwrap();
        let msg_type = obj.get("type").unwrap().as_str().unwrap();
        match msg_type {
            "pong" => MiscMessage::Pong,
            "welcome" | "ack" => {
                debug!("Received {} from {}", msg, EXCHANGE_NAME);
                MiscMessage::Other
            }
            "notice" | "command" => {
                info!("Received {} from {}", msg, EXCHANGE_NAME);
                MiscMessage::Other
            }
            "message" => MiscMessage::Normal,
            "error" => {
                panic!("Received {} from {}", msg, EXCHANGE_NAME);
            }
            _ => {
                panic!("Received {} from {}", msg, EXCHANGE_NAME);
            }
        }
    }

    fn get_ping_msg_and_interval(&self) -> Option<(String, u64)> {
        // See:
        // - https://docs.kucoin.com/#ping
        // - https://docs.kucoin.cc/futures/#ping
        //
        // If the server has not received the ping from the client for 60 seconds , the connection will be disconnected.
        Some((
            r#"{"type":"ping", "id": "crypto-ws-client"}"#.to_string(),
            60,
        ))
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test(flavor = "multi_thread")]
    async fn fetch_ws_token() {
        let ws_token = super::fetch_ws_token().await;
        assert!(!ws_token.token.is_empty())
    }

    #[test]
    fn test_topics_to_commands() {
        let commands = super::topics_to_commands(
            &vec![("/market/match".to_string(), "BTC-USDT".to_string())],
            true,
        );
        assert_eq!(1, commands.len());
    }
}
