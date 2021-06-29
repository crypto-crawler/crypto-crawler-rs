use crate::WSClient;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use super::{
    utils::CHANNEL_PAIR_DELIMITER,
    ws_client_internal::{MiscMessage, WSClientInternal},
    Candlestick, OrderBook, OrderBookSnapshot, Ticker, Trade, BBO,
};

use log::*;
use serde_json::Value;

pub(super) const EXCHANGE_NAME: &str = "bithumb";

const WEBSOCKET_URL: &str = "wss://global-api.bithumb.pro/message/realtime";

const CLIENT_PING_INTERVAL_AND_MSG: (u64, &str) = (60, r#"{"cmd":"ping"}"#);

/// The WebSocket client for Bithumb.
///
/// Bithumb has only Spot market.
///
///   * WebSocket API doc: <https://github.com/bithumb-pro/bithumb.pro-official-api-docs/blob/master/ws-api.md>
///   * Trading at: <https://en.bithumb.com/trade/order/BTC_KRW>
pub struct BithumbWSClient<'a> {
    client: WSClientInternal<'a>,
}

fn channels_to_commands(channels: &[String], subscribe: bool) -> Vec<String> {
    let mut all_commands: Vec<String> = channels
        .iter()
        .filter(|ch| ch.starts_with('{'))
        .map(|s| s.to_string())
        .collect();

    let channels_to_parse: Vec<&String> =
        channels.iter().filter(|ch| !ch.starts_with('{')).collect();
    if !channels_to_parse.is_empty() {
        all_commands.append(&mut vec![format!(
            r#"{{"cmd":"{}","args":{}}}"#,
            if subscribe {
                "subscribe"
            } else {
                "unsubscribe"
            },
            serde_json::to_string(&channels_to_parse).unwrap()
        )])
    };

    all_commands
}

fn on_misc_msg(msg: &str) -> MiscMessage {
    let obj = serde_json::from_str::<HashMap<String, Value>>(&msg).unwrap();
    let code = obj.get("code").unwrap().as_str().unwrap();
    let code = code.parse::<i64>().unwrap();
    if code < 10000 {
        match code {
            0 => MiscMessage::Pong,
            6 => {
                let arr = obj.get("data").unwrap().as_array();
                if arr != None && arr.unwrap().is_empty() {
                    // ignore empty data
                    MiscMessage::Misc
                } else {
                    MiscMessage::Normal
                }
            }
            7 => MiscMessage::Normal,
            _ => {
                debug!("Received {} from {}", msg, EXCHANGE_NAME);
                MiscMessage::Misc
            }
        }
    } else {
        error!("Received {} from {}", msg, EXCHANGE_NAME);
        panic!("Received {} from {}", msg, EXCHANGE_NAME);
    }
}

fn to_raw_channel(channel: &str, pair: &str) -> String {
    format!("{}{}{}", channel, CHANNEL_PAIR_DELIMITER, pair)
}

#[rustfmt::skip]
impl_trait!(Trade, BithumbWSClient, subscribe_trade, "TRADE", to_raw_channel);
#[rustfmt::skip]
impl_trait!(Ticker, BithumbWSClient, subscribe_ticker, "TICKER", to_raw_channel);
#[rustfmt::skip]
impl_trait!(OrderBook, BithumbWSClient, subscribe_orderbook, "ORDERBOOK", to_raw_channel);

impl<'a> BBO for BithumbWSClient<'a> {
    fn subscribe_bbo(&self, _pairs: &[String]) {
        panic!("CoinbasePro WebSocket does NOT have BBO channel");
    }
}

impl<'a> OrderBookSnapshot for BithumbWSClient<'a> {
    fn subscribe_orderbook_snapshot(&self, _pairs: &[String]) {
        panic!("CoinbasePro does NOT have orderbook snapshot channel");
    }
}

impl<'a> Candlestick for BithumbWSClient<'a> {
    fn subscribe_candlestick(&self, _pairs: &[String], _interval: u32) {
        panic!("CoinbasePro does NOT have candlestick channel");
    }
}

define_client!(
    BithumbWSClient,
    EXCHANGE_NAME,
    WEBSOCKET_URL,
    channels_to_commands,
    on_misc_msg,
    Some(CLIENT_PING_INTERVAL_AND_MSG)
);
