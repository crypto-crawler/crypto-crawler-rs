use crate::WSClient;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use super::{
    utils::CHANNEL_PAIR_DELIMITER,
    ws_client_internal::{MiscMessage, WSClientInternal},
};
use super::{Candlestick, OrderBook, OrderBookTopK, Ticker, Trade, BBO};

use log::*;
use serde_json::Value;

pub(super) const EXCHANGE_NAME: &str = "ftx";

const WEBSOCKET_URL: &str = "wss://ftx.com/ws/";

const CLIENT_PING_INTERVAL_AND_MSG: (u64, &str) = (15, r#"{"op":"ping"}"#);

/// The WebSocket client for FTX.
///
/// FTX has Spot, LinearFuture, LinearSwap, Option, Move and BVOL markets.
///
/// * WebSocket API doc: <https://docs.ftx.com/#websocket-api>
/// * Trading at <https://ftx.com/markets>
pub struct FtxWSClient<'a> {
    client: WSClientInternal<'a>,
}

fn channels_to_commands(channels: &[String], subscribe: bool) -> Vec<String> {
    let mut all_commands: Vec<String> = channels
        .iter()
        .filter(|ch| ch.starts_with('{'))
        .map(|s| s.to_string())
        .collect();

    for s in channels.iter().filter(|ch| !ch.starts_with('{')) {
        let v: Vec<&str> = s.split(CHANNEL_PAIR_DELIMITER).collect();
        let channel = v[0];
        let pair = v[1];
        all_commands.append(&mut vec![format!(
            r#"{{"op":"{}","channel":"{}","market":"{}"}}"#,
            if subscribe {
                "subscribe"
            } else {
                "unsubscribe"
            },
            channel,
            pair
        )])
    }

    all_commands
}

fn on_misc_msg(msg: &str) -> MiscMessage {
    let obj = serde_json::from_str::<HashMap<String, Value>>(msg).unwrap();
    let msg_type = obj.get("type").unwrap().as_str().unwrap();

    match msg_type {
        // see https://docs.ftx.com/#response-format
        "pong" => MiscMessage::Pong,
        "subscribed" | "unsubscribed" | "info" => {
            info!("Received {} from {}", msg, EXCHANGE_NAME);
            MiscMessage::Misc
        }
        "partial" | "update" => MiscMessage::Normal,
        "error" => {
            error!("Received {} from {}", msg, EXCHANGE_NAME);
            panic!("Received {} from {}", msg, EXCHANGE_NAME);
        }
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
impl_trait!(Trade, FtxWSClient, subscribe_trade, "trades", to_raw_channel);
#[rustfmt::skip]
impl_trait!(BBO, FtxWSClient, subscribe_bbo, "ticker", to_raw_channel);
#[rustfmt::skip]
impl_trait!(OrderBook, FtxWSClient, subscribe_orderbook, "orderbook", to_raw_channel);

impl<'a> OrderBookTopK for FtxWSClient<'a> {
    fn subscribe_orderbook_topk(&self, _pairs: &[String]) {
        panic!("FTX does NOT have orderbook snapshot channel");
    }
}

impl<'a> Ticker for FtxWSClient<'a> {
    fn subscribe_ticker(&self, _pairs: &[String]) {
        panic!("FTX does NOT have ticker channel");
    }
}

impl<'a> Candlestick for FtxWSClient<'a> {
    fn subscribe_candlestick(&self, _symbol_interval_list: &[(String, usize)]) {
        panic!("FTX does NOT have candlestick channel");
    }
}

define_client!(
    FtxWSClient,
    EXCHANGE_NAME,
    WEBSOCKET_URL,
    channels_to_commands,
    on_misc_msg,
    Some(CLIENT_PING_INTERVAL_AND_MSG),
    None
);
