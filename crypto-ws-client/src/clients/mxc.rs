use std::collections::HashMap;
use std::{cell::RefCell, rc::Rc};

use crate::WSClient;

use super::{
    utils::CHANNEL_PAIR_DELIMITER,
    ws_client_internal::{MiscMessage, WSClientInternal},
    Candlestick, OrderBook, OrderBookSnapshot, Ticker, Trade, BBO,
};

use log::*;
use serde_json::Value;

pub(super) const EXCHANGE_NAME: &str = "MXC";
pub(super) const SOCKETIO_PREFIX: &str = "42";

pub(super) const SPOT_WEBSOCKET_URL: &str =
    "wss://wbs.mxc.com/socket.io/?EIO=3&transport=websocket";
pub(super) const SWAP_WEBSOCKET_URL: &str = "wss://contract.mxc.com/ws";

/// MXC Spot market.
///
///   * WebSocket API doc: <https://github.com/mxcdevelop/APIDoc/blob/master/websocket/websocket-api.md>
///   * Trading at: <https://www.mxc.com/trade/pro>
pub struct MXCSpotWSClient<'a> {
    client: WSClientInternal<'a>,
}

/// MXC Swap market.
///
///   * WebSocket API doc: <https://github.com/mxcdevelop/APIDoc/blob/master/contract/contract-api.md>
///   * Trading at: <https://contract.mxc.com/exchange>
pub struct MXCSwapWSClient<'a> {
    client: WSClientInternal<'a>,
}

// Example: symbol:BTC_USDT -> 42["sub.symbol",{"symbol":"BTC_USDT"}]
fn spot_channel_to_command(raw_channel: &str, subscribe: bool) -> String {
    if raw_channel.starts_with('[') {
        return format!("{}{}", SOCKETIO_PREFIX, raw_channel);
    }
    let v: Vec<&str> = raw_channel.split(CHANNEL_PAIR_DELIMITER).collect();
    let channel = v[0];
    let pair = v[1];

    let (command, ch) = if channel.starts_with("get.") {
        let v: Vec<&str> = channel.split('.').collect();
        (v[0], v[1])
    } else {
        (if subscribe { "sub" } else { "unsub" }, channel)
    };
    format!(
        r#"{}["{}.{}",{{"symbol":"{}"}}]"#,
        SOCKETIO_PREFIX, command, ch, pair
    )
}

fn spot_channels_to_commands(channels: &[String], subscribe: bool) -> Vec<String> {
    let mut commands = Vec::<String>::new();

    for s in channels.iter() {
        let command = spot_channel_to_command(s, subscribe);
        commands.push(command);
    }

    commands
}

// Example: deal:BTC_USDT -> {"method":"sub.deal","param":{"symbol":"BTC_USDT"}}
fn swap_channel_to_command(ch: &str, subscribe: bool) -> String {
    if ch.starts_with('{') {
        return ch.to_string();
    }
    let v: Vec<&str> = ch.split(CHANNEL_PAIR_DELIMITER).collect();
    let channel = v[0];
    let pair = v[1];
    format!(
        r#"{{"method":"{}.{}","param":{{"symbol":"{}"}}}}"#,
        if subscribe { "sub" } else { "unsub" },
        channel,
        pair
    )
}

fn swap_channels_to_commands(channels: &[String], subscribe: bool) -> Vec<String> {
    let mut commands = Vec::<String>::new();

    for s in channels.iter() {
        let command = swap_channel_to_command(s, subscribe);
        commands.push(command);
    }

    commands
}

fn on_misc_msg(msg: &str) -> MiscMessage {
    if msg == "1" {
        return MiscMessage::Reconnect;
    }

    if !msg.starts_with('{') {
        if !msg.starts_with("42") {
            // see https://stackoverflow.com/a/65244958/381712
            if msg.starts_with("0{") {
                debug!("Connection opened {}", SPOT_WEBSOCKET_URL);
            } else if msg == "40" {
                debug!("Connected successfully {}", SPOT_WEBSOCKET_URL);
            } else if msg == "3" {
                // socket.io pong
                debug!("Received pong from {}", SPOT_WEBSOCKET_URL);
            }
            MiscMessage::Misc
        } else {
            MiscMessage::Normal
        }
    } else {
        let obj = serde_json::from_str::<HashMap<String, Value>>(&msg).unwrap();
        if obj.contains_key("channel")
            && obj.contains_key("data")
            && obj.contains_key("symbol")
            && obj.contains_key("ts")
        {
            MiscMessage::Normal
        } else {
            error!("{} is not a JSON string, {}", msg, EXCHANGE_NAME);
            MiscMessage::Misc
        }
    }
}

fn to_raw_channel(channel: &str, pair: &str) -> String {
    format!("{}{}{}", channel, CHANNEL_PAIR_DELIMITER, pair)
}

#[rustfmt::skip]
impl_trait!(Trade, MXCSpotWSClient, subscribe_trade, "symbol", to_raw_channel);
#[rustfmt::skip]
impl_trait!(Trade, MXCSwapWSClient, subscribe_trade, "deal", to_raw_channel);

impl<'a> Ticker for MXCSpotWSClient<'a> {
    fn subscribe_ticker(&self, _pairs: &[String]) {
        panic!("MXC Spot WebSocket does NOT have ticker channel");
    }
}
#[rustfmt::skip]
impl_trait!(Ticker, MXCSwapWSClient, subscribe_ticker, "ticker", to_raw_channel);

#[rustfmt::skip]
impl_trait!(OrderBook, MXCSpotWSClient, subscribe_orderbook, "symbol", to_raw_channel);
#[rustfmt::skip]
impl_trait!(OrderBook, MXCSwapWSClient, subscribe_orderbook, "depth", to_raw_channel);
#[rustfmt::skip]
impl_trait!(OrderBookSnapshot, MXCSpotWSClient, subscribe_orderbook_snapshot, "get.depth", to_raw_channel);
#[rustfmt::skip]
impl_trait!(OrderBookSnapshot, MXCSwapWSClient, subscribe_orderbook_snapshot, "depth.full", to_raw_channel);

impl<'a> BBO for MXCSpotWSClient<'a> {
    fn subscribe_bbo(&self, _pairs: &[String]) {
        panic!("MXC Spot WebSocket does NOT have BBO channel");
    }
}
impl<'a> BBO for MXCSwapWSClient<'a> {
    fn subscribe_bbo(&self, _pairs: &[String]) {
        panic!("MXC Swap WebSocket does NOT have BBO channel");
    }
}

fn interval_to_string(interval: u32) -> String {
    let tmp = match interval {
        60 => "Min1",
        300 => "Min5",
        900 => "Min15",
        1800 => "Min30",
        3600 => "Min60",
        14400 => "Hour4",
        28800 => "Hour8",
        86400 => "Day1",
        604800 => "Week1",
        2592000 => "Month1",
        _ => panic!("MXC has intervals Min1,Min5,Min15,Min30,Min60,Hour4,Hour8,Day1,Week1,Month1"),
    };
    tmp.to_string()
}

impl<'a> Candlestick for MXCSpotWSClient<'a> {
    fn subscribe_candlestick(&self, pairs: &[String], interval: u32) {
        let interval_str = interval_to_string(interval);

        let channels = pairs
            .iter()
            .map(|pair| {
                format!(
                    r#"["sub.kline",{{"symbol":"{}","interval":"{}"}}]"#,
                    pair, interval_str
                )
            })
            .collect::<Vec<String>>();

        self.client.subscribe(&channels);
    }
}

impl<'a> Candlestick for MXCSwapWSClient<'a> {
    fn subscribe_candlestick(&self, pairs: &[String], interval: u32) {
        let interval_str = interval_to_string(interval);

        let channels = pairs
            .iter()
            .map(|pair| {
                format!(
                    r#"{{"method":"sub.kline","param":{{"symbol":"{}","interval":"{}"}}}}"#,
                    pair, interval_str
                )
            })
            .collect::<Vec<String>>();

        self.client.subscribe(&channels);
    }
}

define_client!(
    MXCSpotWSClient,
    EXCHANGE_NAME,
    SPOT_WEBSOCKET_URL,
    spot_channels_to_commands,
    on_misc_msg
);
define_client!(
    MXCSwapWSClient,
    EXCHANGE_NAME,
    SWAP_WEBSOCKET_URL,
    swap_channels_to_commands,
    on_misc_msg
);

#[cfg(test)]
mod tests {
    #[test]
    fn test_spot_channel_to_command() {
        let channel = "symbol:BTC_USDT";

        let subscribe_command = super::spot_channel_to_command(channel, true);
        assert_eq!(
            r#"42["sub.symbol",{"symbol":"BTC_USDT"}]"#.to_string(),
            subscribe_command
        );

        let unsubscribe_command = super::spot_channel_to_command(channel, false);
        assert_eq!(
            r#"42["unsub.symbol",{"symbol":"BTC_USDT"}]"#.to_string(),
            unsubscribe_command
        );
    }

    #[test]
    fn test_swap_channel_to_command() {
        let channel = "deal:BTC_USDT";

        let subscribe_command = super::swap_channel_to_command(channel, true);
        assert_eq!(
            r#"{"method":"sub.deal","param":{"symbol":"BTC_USDT"}}"#.to_string(),
            subscribe_command
        );

        let unsubscribe_command = super::swap_channel_to_command(channel, false);
        assert_eq!(
            r#"{"method":"unsub.deal","param":{"symbol":"BTC_USDT"}}"#.to_string(),
            unsubscribe_command
        );
    }
}
