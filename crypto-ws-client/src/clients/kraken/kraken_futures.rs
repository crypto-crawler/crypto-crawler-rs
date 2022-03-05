use crate::WSClient;
use std::collections::HashMap;
use std::sync::mpsc::Sender;

use super::super::{
    utils::CHANNEL_PAIR_DELIMITER,
    ws_client_internal::{MiscMessage, WSClientInternal},
    Candlestick, Level3OrderBook, OrderBook, OrderBookTopK, Ticker, Trade, BBO,
};

use log::*;
use serde_json::Value;
use tungstenite::Message;

pub(super) const EXCHANGE_NAME: &str = "kraken";

// https://support.kraken.com/hc/en-us/articles/360022839491-API-URLs
const WEBSOCKET_URL: &str = "wss://futures.kraken.com/ws/v1";

// In order to keep the websocket connection alive, you will need to
// make a ping request at least every 60 seconds.
// https://support.kraken.com/hc/en-us/articles/360022635632-Subscriptions-WebSockets-API-
// const CLIENT_PING_INTERVAL_AND_MSG: (u64, &str) = (60, r#"{"event":"ping"}"#);

/// The WebSocket client for Kraken Futures market.
///
///
///   * WebSocket API doc: <https://support.kraken.com/hc/en-us/sections/360003562371-Websocket-API-Public>
///   * Trading at: <https://futures.kraken.com/>
pub struct KrakenFuturesWSClient {
    client: WSClientInternal,
}

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

fn channels_to_commands(channels: &[String], subscribe: bool) -> Vec<String> {
    let mut all_commands: Vec<String> = channels
        .iter()
        .filter(|ch| ch.starts_with('{'))
        .map(|s| s.to_string())
        .collect();

    let mut channel_symbols = HashMap::<String, Vec<String>>::new();
    for s in channels.iter().filter(|ch| !ch.starts_with('{')) {
        let v: Vec<&str> = s.split(CHANNEL_PAIR_DELIMITER).collect();
        let channel = v[0];
        let symbol = v[1];
        match channel_symbols.get_mut(channel) {
            Some(symbols) => symbols.push(symbol.to_string()),
            None => {
                channel_symbols.insert(channel.to_string(), vec![symbol.to_string()]);
            }
        }
    }

    for (channel, symbols) in channel_symbols.iter() {
        all_commands.push(channel_symbols_to_command(channel, symbols, subscribe));
    }
    all_commands.push(r#"{"event":"subscribe","feed":"heartbeat"}"#.to_string());

    all_commands
}

fn on_misc_msg(msg: &str) -> MiscMessage {
    let obj = serde_json::from_str::<HashMap<String, Value>>(msg).unwrap();

    if obj.contains_key("event") {
        let event = obj.get("event").unwrap().as_str().unwrap();
        match event {
            "error" => panic!("Received {} from {}", msg, EXCHANGE_NAME),
            _ => {
                warn!("Received {} from {}", msg, EXCHANGE_NAME);
                MiscMessage::Misc
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
            MiscMessage::Misc
        }
    } else {
        MiscMessage::Misc
    }
}

fn to_raw_channel(channel: &str, symbol: &str) -> String {
    format!("{}{}{}", channel, CHANNEL_PAIR_DELIMITER, symbol)
}

#[rustfmt::skip]
impl_trait!(Trade, KrakenFuturesWSClient, subscribe_trade, "trade", to_raw_channel);
#[rustfmt::skip]
impl_trait!(OrderBook, KrakenFuturesWSClient, subscribe_orderbook, "book", to_raw_channel);
#[rustfmt::skip]
impl_trait!(Ticker, KrakenFuturesWSClient, subscribe_ticker, "ticker", to_raw_channel);

panic_bbo!(KrakenFuturesWSClient);
panic_l2_topk!(KrakenFuturesWSClient);
panic_l3_orderbook!(KrakenFuturesWSClient);
panic_candlestick!(KrakenFuturesWSClient);

impl_new_constructor!(
    KrakenFuturesWSClient,
    EXCHANGE_NAME,
    WEBSOCKET_URL,
    channels_to_commands,
    on_misc_msg,
    None, // Some(CLIENT_PING_INTERVAL_AND_MSG)
    None
);
impl_ws_client_trait!(KrakenFuturesWSClient);

#[cfg(test)]
mod tests {
    #[test]
    fn test_one_symbol() {
        assert_eq!(
            r#"{"event":"subscribe","feed":"trade","product_ids":["PI_XBTUSD"]}"#,
            super::channel_symbols_to_command("trade", &vec!["PI_XBTUSD".to_string()], true)
        );

        assert_eq!(
            r#"{"event":"unsubscribe","feed":"trade","product_ids":["PI_XBTUSD"]}"#,
            super::channel_symbols_to_command("trade", &vec!["PI_XBTUSD".to_string()], false)
        );
    }

    #[test]
    fn test_two_symbols() {
        assert_eq!(
            r#"{"event":"subscribe","feed":"trade","product_ids":["PI_XBTUSD","PI_ETHUSD"]}"#,
            super::channel_symbols_to_command(
                "trade",
                &vec!["PI_XBTUSD".to_string(), "PI_ETHUSD".to_string()],
                true
            )
        );

        assert_eq!(
            r#"{"event":"unsubscribe","feed":"trade","product_ids":["PI_XBTUSD","PI_ETHUSD"]}"#,
            super::channel_symbols_to_command(
                "trade",
                &vec!["PI_XBTUSD".to_string(), "PI_ETHUSD".to_string()],
                false
            )
        );
    }
}
