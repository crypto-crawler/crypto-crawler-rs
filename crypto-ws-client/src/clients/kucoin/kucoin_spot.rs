use crate::{Level3OrderBook, WSClient};
use std::sync::{Arc, Mutex};

use super::super::ws_client_internal::WSClientInternal;
use super::super::{Candlestick, OrderBook, OrderBookSnapshot, Ticker, Trade, BBO};
use super::utils::{
    channels_to_commands, fetch_ws_token, on_misc_msg, to_raw_channel, WebsocketToken,
};

use lazy_static::lazy_static;

const EXCHANGE_NAME: &str = "kucoin";

// See https://docs.kucoin.com/#ping
const PING_INTERVAL_AND_MSG: (u64, &str) = (18, r#"{"type":"ping", "id": "crypto-ws-client"}"#);

lazy_static! {
    static ref WS_TOKEN: WebsocketToken = fetch_ws_token();
    static ref WEBSOCKET_URL: String = format!("{}?token={}", WS_TOKEN.endpoint, WS_TOKEN.token);
}

/// The WebSocket client for KuCoin Spot market.
///
/// * WebSocket API doc: <https://docs.kucoin.com/#websocket-feed>
/// * Trading at: <https://trade.kucoin.com/>
pub struct KuCoinSpotWSClient<'a> {
    client: WSClientInternal<'a>,
}

#[rustfmt::skip]
impl_trait!(Trade, KuCoinSpotWSClient, subscribe_trade, "/market/match", to_raw_channel);
#[rustfmt::skip]
impl_trait!(BBO, KuCoinSpotWSClient, subscribe_bbo, "/market/ticker", to_raw_channel);
#[rustfmt::skip]
impl_trait!(OrderBook, KuCoinSpotWSClient, subscribe_orderbook, "/market/level2", to_raw_channel);
#[rustfmt::skip]
impl_trait!(OrderBookSnapshot, KuCoinSpotWSClient, subscribe_orderbook_snapshot, "/spotMarket/level2Depth50", to_raw_channel);
#[rustfmt::skip]
impl_trait!(Ticker, KuCoinSpotWSClient, subscribe_ticker, "/market/snapshot", to_raw_channel);

fn to_candlestick_raw_channel(pair: &str, interval: u32) -> String {
    let interval_str = match interval {
        60 => "1min",
        180 => "3min",
        300 => "5min",
        900 => "15min",
        1800 => "30min",
        3600 => "1hour",
        7200 => "2hour",
        14400 => "4hour",
        21600 => "6hour",
        28800 => "8hour",
        43200 => "12hour",
        86400 => "1day",
        604800 => "1week",
        _ => panic!(
            "KuCoin available intervals 1min,3min,5min,15min,30min,1hour,2hour,4hour,6hour,8hour,12hour,1day,1week"
        ),
    };
    format!(
        r#"{{"id":"crypto-ws-client","type":"subscribe","topic":"/market/candles:{}_{}","privateChannel":false,"response":true}}"#,
        pair, interval_str,
    )
}

impl_candlestick!(KuCoinSpotWSClient);

impl<'a> Level3OrderBook for KuCoinSpotWSClient<'a> {
    fn subscribe_l3_orderbook(&self, symbols: &[String]) {
        let raw_channels: Vec<String> = symbols
            .iter()
            .map(|symbol| to_raw_channel("/spotMarket/level3", symbol))
            .collect();
        self.client.subscribe(&raw_channels);
    }
}

define_client!(
    KuCoinSpotWSClient,
    EXCHANGE_NAME,
    WEBSOCKET_URL.as_str(),
    channels_to_commands,
    on_misc_msg,
    Some(PING_INTERVAL_AND_MSG)
);
