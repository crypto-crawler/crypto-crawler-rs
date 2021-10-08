use crate::WSClient;
use std::{collections::HashMap, sync::mpsc::Sender, sync::RwLock};

use super::super::ws_client_internal::{MiscMessage, WSClientInternal};
use super::super::{Candlestick, Level3OrderBook, OrderBook, OrderBookTopK, Ticker, Trade, BBO};
use super::utils::fetch_symbol_id_map_spot;

use lazy_static::lazy_static;

const EXCHANGE_NAME: &str = "zbg";

const WEBSOCKET_URL: &str = "wss://kline.zbg.com/websocket";

const CLIENT_PING_INTERVAL_AND_MSG: (u64, &str) = (10, r#"{"action":"PING"}"#);

lazy_static! {
    static ref SYMBOL_ID_MAP: RwLock<HashMap<String, String>> =
        RwLock::new(fetch_symbol_id_map_spot());
}

fn reload_symbol_ids() {
    let mut write_guard = SYMBOL_ID_MAP.write().unwrap();
    let symbol_id_map = fetch_symbol_id_map_spot();

    for (symbol, id) in symbol_id_map.iter() {
        write_guard.insert(symbol.clone(), id.clone());
    }
}

/// The WebSocket client for ZBG spot market.
///
/// * WebSocket API doc: <https://zbgapi.github.io/docs/spot/v1/en/#websocket-market-data>
/// * Trading at: <https://www.zbg.com/trade/>
pub struct ZbgSpotWSClient {
    client: WSClientInternal,
}

fn channel_to_command(channel: &str, subscribe: bool) -> String {
    if channel.starts_with('{') {
        channel.to_string()
    } else {
        format!(
            r#"{{"action":"{}", "dataType":{}}}"#,
            if subscribe { "ADD" } else { "DEL" },
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
    if msg.contains(r#"action":"PING"#) {
        MiscMessage::Pong
    } else {
        MiscMessage::Normal
    }
}

fn to_raw_channel(channel: &str, pair: &str) -> String {
    if !SYMBOL_ID_MAP.read().unwrap().contains_key(pair) {
        // found new symbols
        reload_symbol_ids();
    }
    let symbol_id = SYMBOL_ID_MAP
        .read()
        .unwrap()
        .get(pair)
        .unwrap_or_else(|| panic!("Failed to find symbol_id for {}", pair))
        .clone();
    if channel == "TRADE_STATISTIC_24H" {
        format!("{}_{}", symbol_id, channel)
    } else {
        format!("{}_{}_{}", symbol_id, channel, pair.to_uppercase())
    }
}

#[rustfmt::skip]
impl_trait!(Trade, ZbgSpotWSClient, subscribe_trade, "TRADE", to_raw_channel);
#[rustfmt::skip]
impl_trait!(OrderBook, ZbgSpotWSClient, subscribe_orderbook, "ENTRUST_ADD", to_raw_channel);
#[rustfmt::skip]
impl_trait!(Ticker, ZbgSpotWSClient, subscribe_ticker, "TRADE_STATISTIC_24H", to_raw_channel);

impl BBO for ZbgSpotWSClient {
    fn subscribe_bbo(&self, _pairs: &[String]) {
        panic!("ZBG does NOT have BBO channel");
    }
}

impl OrderBookTopK for ZbgSpotWSClient {
    fn subscribe_orderbook_topk(&self, _pairs: &[String]) {
        panic!("Bitget does NOT have orderbook snapshot channel");
    }
}

fn to_candlestick_raw_channel(pair: &str, interval: usize) -> String {
    let interval_str = match interval {
        60 => "1M",
        300 => "5M",
        900 => "15M",
        1800 => "30M",
        3600 => "1H",
        14400 => "4H",
        86400 => "1D",
        604800 => "1W",
        _ => panic!("ZBG spot available intervals 1M,5M,15M,30M,1H,4H,1D,1W"),
    };

    if !SYMBOL_ID_MAP.read().unwrap().contains_key(pair) {
        // found new symbols
        reload_symbol_ids();
    }
    let symbol_id = SYMBOL_ID_MAP
        .read()
        .unwrap()
        .get(pair)
        .unwrap_or_else(|| panic!("Failed to find symbol_id for {}", pair))
        .clone();

    format!(
        "{}_KLINE_{}_{}",
        symbol_id,
        interval_str,
        pair.to_uppercase()
    )
}

impl_candlestick!(ZbgSpotWSClient);

panic_l3_orderbook!(ZbgSpotWSClient);

define_client!(
    ZbgSpotWSClient,
    EXCHANGE_NAME,
    WEBSOCKET_URL,
    channels_to_commands,
    on_misc_msg,
    Some(CLIENT_PING_INTERVAL_AND_MSG),
    None
);
