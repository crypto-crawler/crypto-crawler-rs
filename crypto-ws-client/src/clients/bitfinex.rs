use crate::WSClient;
use core::panic;
use std::collections::HashMap;

use super::{
    utils::CHANNEL_PAIR_DELIMITER,
    ws_client_internal::{MiscMessage, WSClientInternal},
    Candlestick, OrderBook, OrderBookSnapshot, Ticker, Trade, BBO,
};
use log::*;
use serde_json::Value;

pub(super) const EXCHANGE_NAME: &str = "Bitfinex";

const WEBSOCKET_URL: &str = "wss://api-pub.bitfinex.com/ws/2";

/// The WebSocket client for Bitfinex, including all markets.
///
/// * WebSocket API doc: <https://docs.bitfinex.com/docs/ws-general>
/// * Spot: <https://trading.bitfinex.com/trading>
/// * Swap: <https://trading.bitfinex.com/t/BTCF0:USTF0>
/// * Funding: <https://trading.bitfinex.com/funding>
pub struct BitfinexWSClient<'a> {
    client: WSClientInternal<'a>,
}

fn channel_to_command(channel: &str, subscribe: bool) -> String {
    if channel.starts_with('{') {
        return channel.to_string();
    }
    let delim = channel.find(CHANNEL_PAIR_DELIMITER).unwrap();
    let ch = &channel[..delim];
    let symbol = &channel[(delim + 1)..];

    format!(
        r#"{{"event": "{}", "channel": "{}", "symbol": "{}"}}"#,
        if subscribe {
            "subscribe"
        } else {
            "unsubscribe"
        },
        ch,
        symbol
    )
}

fn channels_to_commands(channels: &[String], subscribe: bool) -> Vec<String> {
    channels
        .iter()
        .map(|s| channel_to_command(s, subscribe))
        .collect()
}

fn on_misc_msg(msg: &str) -> MiscMessage {
    let resp = serde_json::from_str::<HashMap<String, Value>>(&msg);
    if resp.is_err() {
        debug_assert!(msg.starts_with('['));
        return MiscMessage::Normal;
    }
    let obj = resp.unwrap();

    let event = obj.get("event").unwrap().as_str().unwrap();
    match event {
        "error" => error!("{} from {}", msg, EXCHANGE_NAME),
        "info" => info!("{} from {}", msg, EXCHANGE_NAME),
        "pong" => debug!("{} from {}", msg, EXCHANGE_NAME),
        "conf" => warn!("{} from {}", msg, EXCHANGE_NAME),
        "subscribed" => {
            // TODO: store channel_id in {"event":"subscribed","channel":"trades","chanId":676277,"symbol":"tBTCF0:USTF0","pair":"BTCF0:USTF0"}
            info!("{} from {}", msg, EXCHANGE_NAME);
        }
        "unsubscribed" => info!("{} from {}", msg, EXCHANGE_NAME),
        _ => (),
    }

    MiscMessage::Misc
}

fn to_raw_channel(channel: &str, pair: &str) -> String {
    format!("{}:t{}", channel, pair)
}

#[rustfmt::skip]
impl_trait!(Trade, BitfinexWSClient, subscribe_trade, "trades", to_raw_channel);
#[rustfmt::skip]
impl_trait!(Ticker, BitfinexWSClient, subscribe_ticker, "ticker", to_raw_channel);

impl<'a> BBO for BitfinexWSClient<'a> {
    fn subscribe_bbo(&mut self, pairs: &[String]) {
        let raw_channels = pairs
            .iter()
            .map(|pair| {
                format!(
                    r#"{{
                "event": "subscribe",
                "channel": "book",
                "symbol": "t{}",
                "prec": "R0",
                "len": 1
              }}"#,
                    pair
                )
            })
            .collect::<Vec<String>>();

        self.client.subscribe(&raw_channels);
    }
}

impl<'a> OrderBook for BitfinexWSClient<'a> {
    fn subscribe_orderbook(&mut self, pairs: &[String]) {
        let raw_channels = pairs
            .iter()
            .map(|pair| {
                format!(
                    r#"{{
                "event": "subscribe",
                "channel": "book",
                "symbol": "t{}",
                "prec": "P0",
                "frec": "F0",
                "len": 25
              }}"#,
                    pair
                )
            })
            .collect::<Vec<String>>();

        self.client.subscribe(&raw_channels);
    }
}

impl<'a> OrderBookSnapshot for BitfinexWSClient<'a> {
    fn subscribe_orderbook_snapshot(&mut self, _pairs: &[String]) {
        panic!("Bitfinex does NOT have orderbook snapshot channel");
    }
}

fn to_candlestick_raw_channel(pair: &str, interval: u32) -> String {
    let interval_str = match interval {
        60 => "1m",
        300 => "5m",
        900 => "15m",
        1800 => "30m",
        3600 => "1h",
        10800 => "3h",
        21600 => "6h",
        43200 => "12h",
        86400 => "1D",
        604800 => "7D",
        1209600 => "14D",
        2592000 => "1M",
        _ => panic!("Bitfinex has intervals 1m,5m,15m,30m,1h,3h,6h,12h,1D,7D,14D,1M"),
    };

    format!(
        r#"{{
            "event": "subscribe",
            "channel": "candles",
            "key": "trade:{}:t{}"
        }}"#,
        interval_str, pair
    )
}

impl_candlestick!(BitfinexWSClient);

define_client!(
    BitfinexWSClient,
    EXCHANGE_NAME,
    WEBSOCKET_URL,
    channels_to_commands,
    on_misc_msg
);

#[cfg(test)]
mod tests {
    #[test]
    fn test_spot_command() {
        assert_eq!(
            r#"{"event": "subscribe", "channel": "trades", "symbol": "tBTCUSD"}"#,
            super::channel_to_command("trades:tBTCUSD", true)
        );

        assert_eq!(
            r#"{"event": "unsubscribe", "channel": "trades", "symbol": "tBTCUSD"}"#,
            super::channel_to_command("trades:tBTCUSD", false)
        );
    }

    #[test]
    fn test_swap_command() {
        assert_eq!(
            r#"{"event": "subscribe", "channel": "trades", "symbol": "tBTCF0:USTF0"}"#,
            super::channel_to_command("trades:tBTCF0:USTF0", true)
        );

        assert_eq!(
            r#"{"event": "unsubscribe", "channel": "trades", "symbol": "tBTCF0:USTF0"}"#,
            super::channel_to_command("trades:tBTCF0:USTF0", false)
        );
    }
}
