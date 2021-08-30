use crate::WSClient;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex, RwLock},
};

use super::super::ws_client_internal::{MiscMessage, WSClientInternal};
use super::super::{Candlestick, OrderBook, OrderBookSnapshot, Ticker, Trade, BBO};
use super::utils::fetch_symbol_contract_id_map_swap;

use lazy_static::lazy_static;
use log::*;

const EXCHANGE_NAME: &str = "zbg";

const WEBSOCKET_URL: &str = "wss://kline.zbg.com/exchange/v1/futurews";

const CLIENT_PING_INTERVAL_AND_MSG: (u64, &str) = (25, "PING");

lazy_static! {
    static ref SYMBOL_CONTRACT_ID_MAP: RwLock<HashMap<String, i64>> =
        RwLock::new(fetch_symbol_contract_id_map_swap());
}

/// The WebSocket client for ZBG swap market.
///
/// * WebSocket API doc: <https://www.zbgpro.com/docs/future/v1/cn/#300f34d976>, there is no English doc
/// * Trading at: <https://futures.zbg.com/>
pub struct ZbgSwapWSClient<'a> {
    client: WSClientInternal<'a>,
}

fn reload_contract_ids() {
    let mut write_guard = SYMBOL_CONTRACT_ID_MAP.write().unwrap();
    let symbol_id_map = fetch_symbol_contract_id_map_swap();

    for (symbol, id) in symbol_id_map.iter() {
        write_guard.insert(symbol.clone(), *id);
    }
}

fn channel_to_command(channel: &str, subscribe: bool) -> String {
    if channel.starts_with('{') {
        channel.to_string()
    } else {
        format!(
            r#"{{"action":"{}", "topic":"{}"}}"#,
            if subscribe { "sub" } else { "unsub" },
            channel,
        )
    }
}

fn channels_to_commands(channels: &[String], subscribe: bool) -> Vec<String> {
    channels
        .iter()
        .map(|ch| channel_to_command(ch, subscribe))
        .collect()
}

fn on_misc_msg(msg: &str) -> MiscMessage {
    if msg == "Pong" {
        return MiscMessage::Pong;
    }
    if msg.starts_with('[') {
        MiscMessage::Normal
    } else {
        warn!("Received {} from {}", msg, EXCHANGE_NAME);
        MiscMessage::Misc
    }
}

fn to_raw_channel(channel: &str, pair: &str) -> String {
    if !SYMBOL_CONTRACT_ID_MAP.read().unwrap().contains_key(pair) {
        // found new symbols
        reload_contract_ids();
    }
    let contract_id = *SYMBOL_CONTRACT_ID_MAP
        .read()
        .unwrap()
        .get(pair)
        .unwrap_or_else(|| panic!("Failed to find contract_id for {}", pair));
    format!("{}-{}", channel, contract_id)
}

#[rustfmt::skip]
impl_trait!(Trade, ZbgSwapWSClient, subscribe_trade, "future_tick", to_raw_channel);
#[rustfmt::skip]
impl_trait!(OrderBook, ZbgSwapWSClient, subscribe_orderbook, "future_snapshot_depth", to_raw_channel);
#[rustfmt::skip]
impl_trait!(Ticker, ZbgSwapWSClient, subscribe_ticker, "future_snapshot_indicator", to_raw_channel);

impl<'a> BBO for ZbgSwapWSClient<'a> {
    fn subscribe_bbo(&self, _pairs: &[String]) {
        panic!("ZBG does NOT have BBO channel");
    }
}

impl<'a> OrderBookSnapshot for ZbgSwapWSClient<'a> {
    fn subscribe_orderbook_snapshot(&self, _pairs: &[String]) {
        panic!("Bitget does NOT have orderbook snapshot channel");
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
        panic!("ZBG Swap available intervals {}", joined);
    }

    let contract_id = *SYMBOL_CONTRACT_ID_MAP
        .read()
        .unwrap()
        .get(pair)
        .unwrap_or_else(|| panic!("Failed to find contract_id for {}", pair));

    format!("future_kline-{}-{}", contract_id, interval * 1000)
}

impl_candlestick!(ZbgSwapWSClient);

define_client!(
    ZbgSwapWSClient,
    EXCHANGE_NAME,
    WEBSOCKET_URL,
    channels_to_commands,
    on_misc_msg,
    Some(CLIENT_PING_INTERVAL_AND_MSG),
    None
);
