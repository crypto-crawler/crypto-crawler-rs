use crate::WSClient;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use super::ws_client_internal::{MiscMessage, WSClientInternal};
use super::{Candlestick, OrderBook, OrderBookSnapshot, Ticker, Trade, BBO};

use log::*;
use serde_json::Value;

pub(super) const EXCHANGE_NAME: &str = "okex";

const WEBSOCKET_URL: &str = "wss://real.okex.com:8443/ws/v3";

/// The WebSocket client for OKEx.
///
/// OKEx has Spot, Future, Swap and Option markets.
///
/// * WebSocket API doc: <https://www.okex.com/docs/en/>
/// * Trading at:
///     * Spot <https://www.bitmex.com/app/trade/>
///     * Future <https://www.okex.com/derivatives/futures>
///     * Swap <https://www.okex.com/derivatives/swap>
///     * Option <https://www.okex.com/derivatives/options>
pub struct OkexWSClient<'a> {
    client: WSClientInternal<'a>,
}

fn channels_to_commands(channels: &[String], subscribe: bool) -> Vec<String> {
    let channels_to_parse: Vec<&String> =
        channels.iter().filter(|ch| !ch.starts_with('{')).collect();
    let mut all_commands: Vec<String> = channels
        .iter()
        .filter(|ch| ch.starts_with('{'))
        .map(|s| s.to_string())
        .collect();

    if !channels_to_parse.is_empty() {
        all_commands.append(&mut vec![format!(
            r#"{{"op":"{}","args":{}}}"#,
            if subscribe {
                "subscribe"
            } else {
                "unsubscribe"
            },
            serde_json::to_string(channels).unwrap()
        )])
    };

    all_commands
}

fn on_misc_msg(msg: &str) -> MiscMessage {
    if msg == "pong" {
        info!("Received {} from {}", msg, EXCHANGE_NAME);
        return MiscMessage::Misc;
    }
    let resp = serde_json::from_str::<HashMap<String, Value>>(&msg);
    if resp.is_err() {
        error!("{} is not a JSON string, {}", msg, EXCHANGE_NAME);
        return MiscMessage::Misc;
    }
    let obj = resp.unwrap();

    if let Some(event) = obj.get("event") {
        match event.as_str().unwrap() {
            "error" => error!("Received {} from {}", msg, EXCHANGE_NAME),
            "subscribe" => info!("Received {} from {}", msg, EXCHANGE_NAME),
            "unsubscribe" => info!("Received {} from {}", msg, EXCHANGE_NAME),
            _ => warn!("Received {} from {}", msg, EXCHANGE_NAME),
        }
        MiscMessage::Misc
    } else if !obj.contains_key("table") || !obj.contains_key("data") {
        error!("Received {} from {}", msg, EXCHANGE_NAME);
        MiscMessage::Misc
    } else {
        MiscMessage::Normal
    }
}

fn pair_to_market_type(pair: &str) -> &'static str {
    if pair.ends_with("-SWAP") {
        "swap"
    } else {
        let c = pair.matches('-').count();
        if c == 1 {
            "spot"
        } else if c == 2 {
            let date = &pair[(pair.len() - 6)..];
            debug_assert!(date.parse::<i64>().is_ok());
            "futures"
        } else {
            debug_assert!(pair.ends_with("-C") || pair.ends_with("-P"));
            "option"
        }
    }
}

fn to_raw_channel(channel: &str, pair: &str) -> String {
    format!("{}/{}:{}", pair_to_market_type(pair), channel, pair)
}

#[rustfmt::skip]
impl_trait!(Trade, OkexWSClient, subscribe_trade, "trade", to_raw_channel);
#[rustfmt::skip]
impl_trait!(Ticker, OkexWSClient, subscribe_ticker, "ticker", to_raw_channel);
#[rustfmt::skip]
impl_trait!(OrderBook, OkexWSClient, subscribe_orderbook, "depth_l2_tbt", to_raw_channel);
impl_trait!(
    OrderBookSnapshot,
    OkexWSClient,
    subscribe_orderbook_snapshot,
    "depth5",
    to_raw_channel
);

impl<'a> BBO for OkexWSClient<'a> {
    fn subscribe_bbo(&self, _pairs: &[String]) {
        panic!("OKEx WebSocket does NOT have BBO channel");
    }
}

fn to_candlestick_raw_channel(pair: &str, interval: u32) -> String {
    let valid_set: Vec<u32> = vec![
        60, 180, 300, 900, 1800, 3600, 7200, 14400, 21600, 43200, 86400, 604800,
    ];
    if !valid_set.contains(&interval) {
        let joined = valid_set
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",");
        panic!("OKEx has intervals {}", joined);
    }
    let channel = format!("candle{}s", interval);
    to_raw_channel(&channel, pair)
}

impl_candlestick!(OkexWSClient);

define_client!(
    OkexWSClient,
    EXCHANGE_NAME,
    WEBSOCKET_URL,
    channels_to_commands,
    on_misc_msg
);

#[cfg(test)]
mod tests {
    #[test]
    fn test_one_channel() {
        let commands = super::channels_to_commands(&vec!["spot/trade:BTC-USDT".to_string()], true);
        assert_eq!(1, commands.len());
        assert_eq!(
            r#"{"op":"subscribe","args":["spot/trade:BTC-USDT"]}"#,
            commands[0]
        );
    }

    #[test]
    fn test_two_channel() {
        let commands = super::channels_to_commands(
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

    #[test]
    fn test_pair_to_market_type() {
        assert_eq!("spot", super::pair_to_market_type("BTC-USDT"));
        assert_eq!("futures", super::pair_to_market_type("BTC-USDT-210625"));
        assert_eq!("swap", super::pair_to_market_type("BTC-USDT-SWAP"));
        assert_eq!(
            "option",
            super::pair_to_market_type("BTC-USD-210625-72000-C")
        );
    }
}
