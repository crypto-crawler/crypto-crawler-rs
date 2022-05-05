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

// https://support.kraken.com/hc/en-us/articles/360022839491-API-URLs
const WEBSOCKET_URL: &str = "wss://futures.kraken.com/ws/v1";

/// The WebSocket client for Kraken Futures market.
///
///
///   * WebSocket API doc: <https://support.kraken.com/hc/en-us/sections/360003562371-Websocket-API-Public>
///   * Trading at: <https://futures.kraken.com/>
pub struct KrakenFuturesWSClient {
    client: WSClientInternal<KrakenMessageHandler>,
    translator: KrakenCommandTranslator,
}

impl_new_constructor!(
    KrakenFuturesWSClient,
    EXCHANGE_NAME,
    WEBSOCKET_URL,
    KrakenMessageHandler {},
    KrakenCommandTranslator {}
);

#[rustfmt::skip]
impl_trait!(Trade, KrakenFuturesWSClient, subscribe_trade, "trade");
#[rustfmt::skip]
impl_trait!(OrderBook, KrakenFuturesWSClient, subscribe_orderbook, "book");
#[rustfmt::skip]
impl_trait!(Ticker, KrakenFuturesWSClient, subscribe_ticker, "ticker");

panic_bbo!(KrakenFuturesWSClient);
panic_l2_topk!(KrakenFuturesWSClient);
panic_l3_orderbook!(KrakenFuturesWSClient);
panic_candlestick!(KrakenFuturesWSClient);

impl_ws_client_trait!(KrakenFuturesWSClient);

struct KrakenMessageHandler {}
struct KrakenCommandTranslator {}

impl KrakenCommandTranslator {
    fn channel_symbols_to_command(channel: &str, symbols: &[String], subscribe: bool) -> String {
        format!(
            r#"{{"event":"{}","feed":"{}","product_ids":{}}}"#,
            if subscribe {
                "subscribe"
            } else {
                "unsubscribe"
            },
            channel,
            serde_json::to_string(symbols).unwrap(),
        )
    }
}

impl MessageHandler for KrakenMessageHandler {
    fn handle_message(&mut self, msg: &str) -> MiscMessage {
        let obj = serde_json::from_str::<HashMap<String, Value>>(msg).unwrap();

        if obj.contains_key("event") {
            let event = obj.get("event").unwrap().as_str().unwrap();
            match event {
                "error" => panic!("Received {} from {}", msg, EXCHANGE_NAME),
                "info" | "subscribed" | "unsubscribed" => {
                    info!("Received {} from {}", msg, EXCHANGE_NAME);
                    MiscMessage::Other
                }
                _ => {
                    warn!("Received {} from {}", msg, EXCHANGE_NAME);
                    MiscMessage::Other
                }
            }
        } else if obj.contains_key("feed") {
            let feed = obj.get("feed").unwrap().as_str().unwrap();
            if feed == "heartbeat" {
                debug!("Received {} from {}", msg, EXCHANGE_NAME);
                MiscMessage::WebSocket(Message::Ping(Vec::new()))
            } else if obj.contains_key("product_id") {
                MiscMessage::Normal
            } else {
                MiscMessage::Other
            }
        } else {
            MiscMessage::Other
        }
    }

    fn get_ping_msg_and_interval(&self) -> Option<(Message, u64)> {
        // In order to keep the websocket connection alive, you will need to
        // make a ping request at least every 60 seconds.
        // https://support.kraken.com/hc/en-us/articles/360022635632-Subscriptions-WebSockets-API-
        None // TODO: lack of doc
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
            commands.push(Self::channel_symbols_to_command(
                channel, symbols, subscribe,
            ));
        }
        commands.push(r#"{"event":"subscribe","feed":"heartbeat"}"#.to_string());

        commands
    }

    fn translate_to_candlestick_commands(
        &self,
        _subscribe: bool,
        _symbol_interval_list: &[(String, usize)],
    ) -> Vec<String> {
        panic!("Kraken Futures does NOT have candlestick channel");
    }
}

#[cfg(test)]
mod tests {
    use crate::common::command_translator::CommandTranslator;

    #[test]
    fn test_one_symbol() {
        let translator = super::KrakenCommandTranslator {};
        let commands = translator
            .translate_to_commands(true, &vec![("trade".to_string(), "PI_XBTUSD".to_string())]);

        assert_eq!(2, commands.len());
        assert_eq!(
            r#"{"event":"subscribe","feed":"trade","product_ids":["PI_XBTUSD"]}"#,
            commands[0]
        );
        assert_eq!(r#"{"event":"subscribe","feed":"heartbeat"}"#, commands[1]);
    }

    #[test]
    fn test_two_symbols() {
        let translator = super::KrakenCommandTranslator {};
        let commands = translator.translate_to_commands(
            true,
            &vec![
                ("trade".to_string(), "PI_XBTUSD".to_string()),
                ("trade".to_string(), "PI_ETHUSD".to_string()),
            ],
        );

        assert_eq!(2, commands.len());

        assert_eq!(
            r#"{"event":"subscribe","feed":"trade","product_ids":["PI_XBTUSD","PI_ETHUSD"]}"#,
            commands[0]
        );
        assert_eq!(r#"{"event":"subscribe","feed":"heartbeat"}"#, commands[1]);
    }
}
