use crate::WSClient;
use std::collections::HashMap;

use super::ws_client_internal::{MiscMessage, WSClientInternal};
use super::{Ticker, Trade};

use log::*;
use serde_json::Value;

pub(super) const EXCHANGE_NAME: &str = "OKEx";

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
pub struct OKExWSClient<'a> {
    client: WSClientInternal<'a>,
}

fn channels_to_commands(channels: &[String], subscribe: bool) -> Vec<String> {
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

impl<'a> Trade for OKExWSClient<'a> {
    fn subscribe_trade(&mut self, pairs: &[String]) {
        let pair_to_raw_channel =
            |pair: &String| format!("{}/trade:{}", pair_to_market_type(pair), pair);

        let channels = pairs
            .iter()
            .map(pair_to_raw_channel)
            .collect::<Vec<String>>();
        self.client.subscribe(&channels);
    }
}

impl<'a> Ticker for OKExWSClient<'a> {
    fn subscribe_ticker(&mut self, pairs: &[String]) {
        let pair_to_raw_channel =
            |pair: &String| format!("{}/ticker:{}", pair_to_market_type(pair), pair);

        let channels = pairs
            .iter()
            .map(pair_to_raw_channel)
            .collect::<Vec<String>>();
        self.client.subscribe(&channels);
    }
}

define_client!(
    OKExWSClient,
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
