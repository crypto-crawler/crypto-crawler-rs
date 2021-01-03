use crate::WSClient;
use std::collections::HashMap;

use super::ws_client_internal::{MiscMessage, WSClientInternal};
use super::{BBO, Ticker, Trade};
use log::*;
use serde_json::Value;

pub(super) const EXCHANGE_NAME: &str = "Bitstamp";

const WEBSOCKET_URL: &str = "wss://ws.bitstamp.net";

/// The WebSocket client for Bitstamp Spot market.
///
/// Bitstamp has only Spot market.
///
///   * WebSocket API doc: <https://www.bitstamp.net/websocket/v2/>
///   * Trading at: <https://www.bitstamp.net/market/tradeview/>
pub struct BitstampWSClient<'a> {
    client: WSClientInternal<'a>,
}

fn channel_to_command(channel: &str, subscribe: bool) -> String {
    if channel.starts_with('{') {
        return channel.to_string();
    }
    format!(
        r#"{{"event":"bts:{}","data":{{"channel":"{}"}}}}"#,
        if subscribe {
            "subscribe"
        } else {
            "unsubscribe"
        },
        channel
    )
}

fn channels_to_commands(channels: &[String], subscribe: bool) -> Vec<String> {
    channels
        .iter()
        .map(|ch| channel_to_command(ch, subscribe))
        .collect()
}

fn on_misc_msg(msg: &str) -> MiscMessage {
    let resp = serde_json::from_str::<HashMap<String, Value>>(&msg);
    if resp.is_err() {
        error!("{} is not a JSON string, {}", msg, EXCHANGE_NAME);
        return MiscMessage::Misc;
    }
    let obj = resp.unwrap();

    let event = obj.get("event").unwrap().as_str().unwrap();
    match event {
        "bts:subscription_succeeded" | "bts:unsubscription_succeeded" => {
            debug!("Received {} from {}", msg, EXCHANGE_NAME);
            MiscMessage::Misc
        }
        "bts:error" => {
            error!("Received {} from {}", msg, EXCHANGE_NAME);
            MiscMessage::Misc
        }
        "bts:request_reconnect" => {
            warn!("Received {} from {}", msg, EXCHANGE_NAME);
            MiscMessage::Reconnect
        }
        _ => MiscMessage::Normal,
    }
}

fn to_raw_channel(channel: &str, pair: &str) -> String {
    format!("{}_{}", channel, pair)
}

#[rustfmt::skip]
impl_trait!(Trade, BitstampWSClient, subscribe_trade, "live_trades", to_raw_channel);

impl<'a> Ticker for BitstampWSClient<'a> {
    fn subscribe_ticker(&mut self, _pairs: &[String]) {
        panic!("Bitstamp WebSocket does NOT have ticker channel");
    }
}

impl<'a> BBO for BitstampWSClient<'a> {
    fn subscribe_bbo(&mut self, _pairs: &[String]) {
        panic!("Bitstamp WebSocket does NOT have BBO channel");
    }
}

define_client!(
    BitstampWSClient,
    EXCHANGE_NAME,
    WEBSOCKET_URL,
    channels_to_commands,
    on_misc_msg
);

#[cfg(test)]
mod tests {
    #[test]
    fn test_channel_to_command() {
        assert_eq!(
            r#"{"event":"bts:subscribe","data":{"channel":"live_trades_btcusd"}}"#,
            super::channel_to_command("live_trades_btcusd", true)
        );

        assert_eq!(
            r#"{"event":"bts:unsubscribe","data":{"channel":"live_trades_btcusd"}}"#,
            super::channel_to_command("live_trades_btcusd", false)
        );
    }
}
