use crate::WSClient;
use std::collections::HashMap;
use std::sync::mpsc::Sender;

use super::super::ws_client_internal::{MiscMessage, WSClientInternal};
use super::super::{Candlestick, Level3OrderBook, OrderBook, OrderBookTopK, Ticker, Trade, BBO};
use crate::clients::utils::CHANNEL_PAIR_DELIMITER;

use log::*;
use serde_json::Value;

const EXCHANGE_NAME: &str = "dydx";

const WEBSOCKET_URL: &str = "wss://api.dydx.exchange/v3/ws";

// https://docs.dydx.exchange/#v3-websocket-api
// The server will send pings every 30s and expects a pong within 10s.
// The server does not expect pings, but will respond with a pong if sent one.
const CLIENT_PING_INTERVAL_AND_MSG: (u64, &str) = (30, r#"{"type":"ping"}"#);

/// The WebSocket client for dYdX perpetual markets.
///
/// * WebSocket API doc: <https://docs.dydx.exchange/#v3-websocket-api>
/// * Trading at: <https://trade.dydx.exchange/trade>
pub struct DydxSwapWSClient {
    client: WSClientInternal,
}

fn channel_to_command(channel: &str, subscribe: bool) -> String {
    if channel.starts_with('{') {
        return channel.to_string();
    }
    let delim = channel.find(CHANNEL_PAIR_DELIMITER).unwrap();
    let ch = &channel[..delim];
    let symbol = &channel[(delim + 1)..];

    format!(
        r#"{{"type": "{}", "channel": "{}", "id": "{}"}}"#,
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
    let obj = serde_json::from_str::<HashMap<String, Value>>(msg).unwrap();

    match obj.get("type").unwrap().as_str().unwrap() {
        "error" => {
            error!("Received {} from {}", msg, EXCHANGE_NAME);
            if obj.contains_key("message")
                && obj
                    .get("message")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .starts_with("Invalid subscription id for channel")
            {
                panic!("Received {} from {}", msg, EXCHANGE_NAME);
            } else {
                MiscMessage::Misc
            }
        }
        "connected" | "pong" => {
            debug!("Received {} from {}", msg, EXCHANGE_NAME);
            MiscMessage::Misc
        }
        "channel_data" | "subscribed" => MiscMessage::Normal,
        _ => {
            warn!("Received {} from {}", msg, EXCHANGE_NAME);
            MiscMessage::Misc
        }
    }
}

fn to_raw_channel(channel: &str, pair: &str) -> String {
    format!("{}{}{}", channel, CHANNEL_PAIR_DELIMITER, pair)
}

#[rustfmt::skip]
impl_trait!(Trade, DydxSwapWSClient, subscribe_trade, "v3_trades", to_raw_channel);
#[rustfmt::skip]
impl_trait!(OrderBook, DydxSwapWSClient, subscribe_orderbook, "v3_orderbook", to_raw_channel);

panic_ticker!(DydxSwapWSClient);
panic_bbo!(DydxSwapWSClient);
panic_l2_topk!(DydxSwapWSClient);
panic_l3_orderbook!(DydxSwapWSClient);
panic_candlestick!(DydxSwapWSClient);

impl_new_constructor!(
    DydxSwapWSClient,
    EXCHANGE_NAME,
    WEBSOCKET_URL,
    channels_to_commands,
    on_misc_msg,
    Some(CLIENT_PING_INTERVAL_AND_MSG),
    None
);
impl_ws_client_trait!(DydxSwapWSClient);
