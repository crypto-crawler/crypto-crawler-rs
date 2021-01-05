use crate::WSClient;
use std::collections::HashMap;

use super::{
    utils::CHANNEL_PAIR_DELIMITER,
    ws_client_internal::{MiscMessage, WSClientInternal},
    Candlestick, OrderBook, OrderBookSnapshot, Ticker, Trade, BBO,
};

use log::*;
use serde_json::Value;
use tungstenite::Message;

pub(super) const EXCHANGE_NAME: &str = "Kraken";

const WEBSOCKET_URL: &str = "wss://ws.kraken.com";

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

    let mut commands = Vec::<String>::new();

    for (name, pairs) in name_pairs.iter() {
        commands.push(name_pairs_to_command(name, pairs, subscribe));
    }

    all_commands.append(&mut commands);
    all_commands
}

fn on_misc_msg(msg: &str) -> MiscMessage {
    let resp = serde_json::from_str::<Value>(&msg);
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
            "pong" => {
                debug!("Received {} from {}", msg, EXCHANGE_NAME);
                MiscMessage::Misc
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
    fn subscribe_orderbook(&mut self, pairs: &[String]) {
        let command = format!(
            r#"{{"event":"subscribe","pair":{},"subscription":{{"name":"book", "depth":25}}}}"#,
            serde_json::to_string(pairs).unwrap(),
        );
        let channels = vec![command];

        self.client.subscribe(&channels);
    }
}

impl<'a> OrderBookSnapshot for KrakenWSClient<'a> {
    fn subscribe_orderbook_snapshot(&mut self, _pairs: &[String]) {
        panic!("Kraken does NOT have orderbook snapshot channel");
    }
}

impl<'a> Candlestick for KrakenWSClient<'a> {
    fn subscribe_candlestick(&mut self, pairs: &[String], interval: u32) {
        let valid_set: Vec<u32> = vec![1, 5, 15, 30, 60, 240, 1440, 10080, 21600]
            .into_iter()
            .map(|x| x * 60)
            .collect();
        if !valid_set.contains(&interval) {
            let joined = valid_set
                .into_iter()
                .map(|x| (x / 60).to_string())
                .collect::<Vec<String>>()
                .join(",");
            panic!("Kraken has intervals {}", joined);
        }

        let command = format!(
            r#"{{"event":"subscribe","pair":{},"subscription":{{"name":"ohlc", "interval":{}}}}}"#,
            serde_json::to_string(pairs).unwrap(),
            interval / 60
        );
        let channels = vec![command];

        self.client.subscribe(&channels);
    }
}

define_client!(
    KrakenWSClient,
    EXCHANGE_NAME,
    WEBSOCKET_URL,
    channels_to_commands,
    on_misc_msg
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
