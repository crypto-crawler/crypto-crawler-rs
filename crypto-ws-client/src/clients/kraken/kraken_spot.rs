use async_trait::async_trait;
use std::collections::HashMap;
use tokio_tungstenite::tungstenite::Message;

use super::EXCHANGE_NAME;
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

const WEBSOCKET_URL: &str = "wss://ws.kraken.com";

/// The WebSocket client for Kraken Spot market.
///
///
///   * WebSocket API doc: <https://docs.kraken.com/websockets/>
///   * Trading at: <https://trade.kraken.com/>
pub struct KrakenSpotWSClient {
    client: WSClientInternal<KrakenMessageHandler>,
    translator: KrakenCommandTranslator,
}

impl_new_constructor!(
    KrakenSpotWSClient,
    EXCHANGE_NAME,
    WEBSOCKET_URL,
    KrakenMessageHandler {},
    KrakenCommandTranslator {}
);

#[rustfmt::skip]
impl_trait!(Trade, KrakenSpotWSClient, subscribe_trade, "trade");
impl_trait!(OrderBook, KrakenSpotWSClient, subscribe_orderbook, "book");
#[rustfmt::skip]
impl_trait!(Ticker, KrakenSpotWSClient, subscribe_ticker, "ticker");
#[rustfmt::skip]
impl_trait!(BBO, KrakenSpotWSClient, subscribe_bbo, "spread");
impl_candlestick!(KrakenSpotWSClient);

panic_l2_topk!(KrakenSpotWSClient);
panic_l3_orderbook!(KrakenSpotWSClient);

impl_ws_client_trait!(KrakenSpotWSClient);

struct KrakenMessageHandler {}
struct KrakenCommandTranslator {}

impl MessageHandler for KrakenMessageHandler {
    fn handle_message(&mut self, msg: &str) -> MiscMessage {
        let resp = serde_json::from_str::<Value>(msg);
        if resp.is_err() {
            error!("{} is not a JSON string, {}", msg, EXCHANGE_NAME);
            return MiscMessage::Other;
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
                            if error_msg.starts_with("Currency pair not") {
                                panic!("Received {} from {}", msg, EXCHANGE_NAME)
                            }
                        }
                        _ => warn!("Received {} from {}", msg, EXCHANGE_NAME),
                    }

                    MiscMessage::Other
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
                            MiscMessage::Other
                        }
                    }
                }
                _ => {
                    warn!("Received {} from {}", msg, EXCHANGE_NAME);
                    MiscMessage::Other
                }
            }
        } else {
            MiscMessage::Normal
        }
    }

    fn get_ping_msg_and_interval(&self) -> Option<(String, u64)> {
        // Client can ping server to determine whether connection is alive
        // https://docs.kraken.com/websockets/#message-ping
        Some((r#"{"event":"ping"}"#.to_string(), 10))
    }
}

impl KrakenCommandTranslator {
    fn name_symbols_to_command(name: &str, symbols: &[String], subscribe: bool) -> String {
        if name == "book" {
            format!(
                r#"{{"event":"{}","pair":{},"subscription":{{"name":"{}","depth":25}}}}"#,
                if subscribe {
                    "subscribe"
                } else {
                    "unsubscribe"
                },
                serde_json::to_string(symbols).unwrap(),
                name
            )
        } else {
            format!(
                r#"{{"event":"{}","pair":{},"subscription":{{"name":"{}"}}}}"#,
                if subscribe {
                    "subscribe"
                } else {
                    "unsubscribe"
                },
                serde_json::to_string(symbols).unwrap(),
                name
            )
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
}

impl CommandTranslator for KrakenCommandTranslator {
    fn translate_to_commands(&self, subscribe: bool, topics: &[(String, String)]) -> Vec<String> {
        let mut commands: Vec<String> = Vec::new();

        let mut channel_symbols = HashMap::<String, Vec<String>>::new();
        for (channel, symbol) in topics {
            match channel_symbols.get_mut(channel) {
                Some(symbols) => symbols.push(symbol.to_string()),
                None => {
                    channel_symbols.insert(channel.to_string(), vec![symbol.to_string()]);
                }
            }
        }

        for (channel, symbols) in channel_symbols.iter() {
            commands.push(Self::name_symbols_to_command(channel, symbols, subscribe));
        }

        commands
    }

    fn translate_to_candlestick_commands(
        &self,
        subscribe: bool,
        symbol_interval_list: &[(String, usize)],
    ) -> Vec<String> {
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
        let symbols_interval_list = Self::convert_symbol_interval_list(symbol_interval_list);
        let commands: Vec<String> = symbols_interval_list
            .into_iter()
            .map(|(symbols, interval)| {
                format!(
                    r#"{{"event":"{}","pair":{},"subscription":{{"name":"ohlc", "interval":{}}}}}"#,
                    if subscribe {
                        "subscribe"
                    } else {
                        "unsubscribe"
                    },
                    serde_json::to_string(&symbols).unwrap(),
                    interval / 60
                )
            })
            .collect();

        commands
    }
}

#[cfg(test)]
mod tests {
    use crate::common::command_translator::CommandTranslator;

    #[test]
    fn test_one_symbol() {
        let translator = super::KrakenCommandTranslator {};
        let commands = translator
            .translate_to_commands(true, &vec![("trade".to_string(), "XBT/USD".to_string())]);

        assert_eq!(1, commands.len());
        assert_eq!(
            r#"{"event":"subscribe","pair":["XBT/USD"],"subscription":{"name":"trade"}}"#,
            commands[0]
        );
    }

    #[test]
    fn test_two_symbols() {
        let translator = super::KrakenCommandTranslator {};
        let commands = translator.translate_to_commands(
            true,
            &vec![
                ("trade".to_string(), "XBT/USD".to_string()),
                ("trade".to_string(), "ETH/USD".to_string()),
            ],
        );

        assert_eq!(1, commands.len());
        assert_eq!(
            r#"{"event":"subscribe","pair":["XBT/USD","ETH/USD"],"subscription":{"name":"trade"}}"#,
            commands[0]
        );
    }
}
