use std::collections::HashMap;

use crate::WSClient;

use super::{
    utils::CHANNEL_PAIR_DELIMITER,
    ws_client_internal::{MiscMessage, WSClientInternal},
};

use log::*;
use serde_json::Value;

pub(super) const EXCHANGE_NAME: &str = "MXC";
pub(super) const SOCKETIO_PREFIX: &str = "42";

pub(super) const SPOT_WEBSOCKET_URL: &str =
    "wss://wbs.mxc.com/socket.io/?EIO=3&transport=websocket";
pub(super) const SWAP_WEBSOCKET_URL: &str = "wss://contract.mxc.com/ws";

/// The WebSocket client for MXC Spot market.
///
/// Official doc: <https://github.com/mxcdevelop/APIDoc/blob/master/websocket/websocket-api.md>.
///
/// ## Channel format
pub struct MXCSpotWSClient<'a> {
    client: WSClientInternal<'a>,
}

/// The WebSocket client for MXC Swap market(<https://github.com/mxcdevelop/APIDoc/blob/master/contract/contract-api.md>).
pub struct MXCSwapWSClient<'a> {
    client: WSClientInternal<'a>,
}

// Example: symbol:BTC_USDT -> 42["sub.symbol",{"symbol":"BTC_USDT"}]
fn spot_channel_to_command(ch: &str, subscribe: bool) -> String {
    let v: Vec<&str> = ch.split(CHANNEL_PAIR_DELIMITER).collect();
    let channel = v[0];
    let pair = v[1];

    format!(
        r#"{}["{}.{}",{{"symbol":"{}"}}]"#,
        SOCKETIO_PREFIX,
        if subscribe { "sub" } else { "unsub" },
        channel,
        pair
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
