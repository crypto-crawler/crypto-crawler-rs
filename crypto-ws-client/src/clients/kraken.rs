use crate::WSClient;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use super::{
    utils::CHANNEL_PAIR_DELIMITER,
    ws_client_internal::{MiscMessage, WSClientInternal},
    Candlestick, Level3OrderBook, OrderBook, OrderBookTopK, Ticker, Trade, BBO,
};

use log::*;
use serde_json::Value;
use tungstenite::Message;

pub(super) const EXCHANGE_NAME: &str = "kraken";

const WEBSOCKET_URL: &str = "wss://ws.kraken.com";

// Client can ping server to determine whether connection is alive
// https://docs.kraken.com/websockets/#message-ping
const CLIENT_PING_INTERVAL_AND_MSG: (u64, &str) = (10, r#"{"event":"ping"}"#);

/// The WebSocket client for Kraken.
///
/// Kraken has only Spot market.
///
///   * WebSocket API doc: <https://docs.kraken.com/websockets/>
///   * Trading at: <https://trade.kraken.com/>

pub struct KrakenWSClient<'a> {
    client: WSClientInternal<'a>,
}

fn name_pairs_to_command(name: &str, pairs: &[String], subscribe: bool) -> String {
    format!(
        r#"{{"event":"{}","pair":{},"subscription":{{"name":"{}"}}}}"#,
        if subscribe {
            "subscribe"
        } else {
            "unsubscribe"
        },
        serde_json::to_string(pairs).unwrap(),
        name
    )
}

fn channels_to_commands(channels: &[String], subscribe: bool) -> Vec<String> {
    let mut all_commands: Vec<String> = channels
        .iter()
        .filter(|ch| ch.starts_with('{'))
        .map(|s| s.to_string())
        .collect();

    let mut name_pairs = HashMap::<String, Vec<String>>::new();
    for s in channels.iter().filter(|ch| !ch.starts_with('{')) {
        let v: Vec<&str> = s.split(CHANNEL_PAIR_DELIMITER).collect();
        let name = v[0];
        let pair = v[1];
        match name_pairs.get_mut(name) {
            Some(pairs) => pairs.push(pair.to_string()),
            None => {
                name_pairs.insert(name.to_string(), vec![pair.to_string()]);
            }
        }
    }

    for (name, pairs) in name_pairs.iter() {
        all_commands.push(name_pairs_to_command(name, pairs, subscribe));
    }

    all_commands
}

fn on_misc_msg(msg: &str) -> MiscMessage {
    let resp = serde_json::from_str::<Value>(msg);
    if resp.is_err() {
        error!("{} is not a JSON string, {}", msg, EXCHANGE_NAME);
        return MiscMessage::Misc;
    }
    let value = resp.unwrap();

    if value.is_object() {
        let obj = value.as_object().unwrap();
        let event = obj.get("event").unwrap().as_str().unwrap();
        match event {
            "heartbeat" => {
                debug!("Received {} from {}", msg, EXCHANGE_NAME);
                let ping = r#"{
                    "event": "ping",
                    "reqid": 9527
                }"#;
                MiscMessage::WebSocket(Message::Text(ping.to_string()))
            }
            "pong" => MiscMessage::Pong,
            "subscriptionStatus" => {
                let status = obj.get("status").unwrap().as_str().unwrap();
                match status {
                    "subscribed" | "unsubscribed" => {
                        info!("Received {} from {}", msg, EXCHANGE_NAME)
                    }
                    "error" => {
                        error!("Received {} from {}", msg, EXCHANGE_NAME);
                        let error_msg = obj.get("errorMessage").unwrap().as_str().unwrap();
                        if error_msg.starts_with("Currency pair not supported") {
                            panic!("Received {} from {}", msg, EXCHANGE_NAME)
                        }
                    }
                    _ => warn!("Received {} from {}", msg, EXCHANGE_NAME),
                }

                MiscMessage::Misc
            }
            "systemStatus" => {
                let status = obj.get("status").unwrap().as_str().unwrap();
                match status {
                    "maintenance" | "cancel_only" => {
                        warn!(
                            "Received {}, which means Kraken is in maintenance mode",
                            msg
                        );
                        std::thread::sleep(std::time::Duration::from_secs(20));
                        MiscMessage::Reconnect
                    }
                    _ => {
                        info!("Received {} from {}", msg, EXCHANGE_NAME);
                        MiscMessage::Misc
                    }
                }
            }
            _ => {
                warn!("Received {} from {}", msg, EXCHANGE_NAME);
                MiscMessage::Misc
            }
        }
    } else {
        MiscMessage::Normal
    }
}

fn to_raw_channel(channel: &str, pair: &str) -> String {
    format!("{}{}{}", channel, CHANNEL_PAIR_DELIMITER, pair)
}

#[rustfmt::skip]
impl_trait!(Trade, KrakenWSClient, subscribe_trade, "trade", to_raw_channel);
#[rustfmt::skip]
impl_trait!(Ticker, KrakenWSClient, subscribe_ticker, "ticker", to_raw_channel);
#[rustfmt::skip]
impl_trait!(BBO, KrakenWSClient, subscribe_bbo, "spread", to_raw_channel);

impl<'a> OrderBook for KrakenWSClient<'a> {
    fn subscribe_orderbook(&self, pairs: &[String]) {
        let command = format!(
            r#"{{"event":"subscribe","pair":{},"subscription":{{"name":"book", "depth":25}}}}"#,
            serde_json::to_string(pairs).unwrap(),
        );
        let channels = vec![command];

        self.client.subscribe(&channels);
    }
}

impl<'a> OrderBookTopK for KrakenWSClient<'a> {
    fn subscribe_orderbook_topk(&self, _pairs: &[String]) {
        panic!("Kraken does NOT have orderbook snapshot channel");
    }
}

fn convert_symbol_interval_list(
    symbol_interval_list: &[(String, usize)],
) -> Vec<(Vec<String>, usize)> {
    let mut map = HashMap::<usize, Vec<String>>::new();
    for task in symbol_interval_list {
        let v = map.entry(task.1).or_insert_with(Vec::new);
        v.push(task.0.clone());
    }
    let mut result = Vec::new();
    for (k, v) in map {
        result.push((v, k));
    }
    result
}

impl<'a> Candlestick for KrakenWSClient<'a> {
    fn subscribe_candlestick(&self, symbol_interval_list: &[(String, usize)]) {
        let valid_set: Vec<usize> = vec![1, 5, 15, 30, 60, 240, 1440, 10080, 21600]
            .into_iter()
            .map(|x| x * 60)
            .collect();
        let invalid_intervals = symbol_interval_list
            .iter()
            .map(|(_, interval)| *interval)
            .filter(|x| !valid_set.contains(x))
            .collect::<Vec<usize>>();
        if !invalid_intervals.is_empty() {
            panic!(
                "Invalid intervals: {}, available intervals: {}",
                invalid_intervals
                    .into_iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(","),
                valid_set
                    .into_iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
            );
        }
        let symbols_interval_list = convert_symbol_interval_list(symbol_interval_list);

        let commands: Vec<String> = symbols_interval_list.into_iter().map(|(symbols, interval)| format!(
            r#"{{"event":"subscribe","pair":{},"subscription":{{"name":"ohlc", "interval":{}}}}}"#,
            serde_json::to_string(&symbols).unwrap(),
            interval / 60
        )).collect();

        self.client.subscribe(&commands);
    }
}

panic_l3_orderbook!(KrakenWSClient);

define_client!(
    KrakenWSClient,
    EXCHANGE_NAME,
    WEBSOCKET_URL,
    channels_to_commands,
    on_misc_msg,
    Some(CLIENT_PING_INTERVAL_AND_MSG),
    None
);

#[cfg(test)]
mod tests {
    #[test]
    fn test_one_pair() {
        assert_eq!(
            r#"{"event":"subscribe","pair":["XBT/USD"],"subscription":{"name":"trade"}}"#,
            super::name_pairs_to_command("trade", &vec!["XBT/USD".to_string()], true)
        );

        assert_eq!(
            r#"{"event":"unsubscribe","pair":["XBT/USD"],"subscription":{"name":"trade"}}"#,
            super::name_pairs_to_command("trade", &vec!["XBT/USD".to_string()], false)
        );
    }

    #[test]
    fn test_two_pairs() {
        assert_eq!(
            r#"{"event":"subscribe","pair":["XBT/USD","ETH/USD"],"subscription":{"name":"trade"}}"#,
            super::name_pairs_to_command(
                "trade",
                &vec!["XBT/USD".to_string(), "ETH/USD".to_string()],
                true
            )
        );

        assert_eq!(
            r#"{"event":"unsubscribe","pair":["XBT/USD","ETH/USD"],"subscription":{"name":"trade"}}"#,
            super::name_pairs_to_command(
                "trade",
                &vec!["XBT/USD".to_string(), "ETH/USD".to_string()],
                false
            )
        );
    }
}
