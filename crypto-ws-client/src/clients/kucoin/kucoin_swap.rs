use crate::{Level3OrderBook, WSClient};
use std::sync::{Arc, Mutex};

use super::super::ws_client_internal::WSClientInternal;
use super::super::{Candlestick, OrderBook, OrderBookSnapshot, Ticker, Trade, BBO};
use super::utils::{
    channels_to_commands, fetch_ws_token, on_misc_msg, to_raw_channel, WebsocketToken,
    CLIENT_PING_INTERVAL_AND_MSG, EXCHANGE_NAME,
};

use lazy_static::lazy_static;

lazy_static! {
    static ref WS_TOKEN: WebsocketToken = fetch_ws_token();
    static ref WEBSOCKET_URL: String = format!("{}?token={}", WS_TOKEN.endpoint, WS_TOKEN.token);
}

/// The WebSocket client for KuCoin Swap markets.
///
/// * WebSocket API doc: <https://docs.kucoin.cc/futures/#websocket-2>
/// * Trading at: <https://futures.kucoin.com/>
pub struct KuCoinSwapWSClient<'a> {
    client: WSClientInternal<'a>,
}

#[rustfmt::skip]
impl_trait!(Trade, KuCoinSwapWSClient, subscribe_trade, "/contractMarket/execution", to_raw_channel);
#[rustfmt::skip]
impl_trait!(BBO, KuCoinSwapWSClient, subscribe_bbo, "/contractMarket/ticker", to_raw_channel);
#[rustfmt::skip]
impl_trait!(OrderBook, KuCoinSwapWSClient, subscribe_orderbook, "/contractMarket/level2", to_raw_channel);
#[rustfmt::skip]
impl_trait!(OrderBookSnapshot, KuCoinSwapWSClient, subscribe_orderbook_snapshot, "/contractMarket/level2Depth50", to_raw_channel);
#[rustfmt::skip]
impl_trait!(Ticker, KuCoinSwapWSClient, subscribe_ticker, "/contractMarket/snapshot", to_raw_channel);

fn to_candlestick_raw_channel(pair: &str, interval: u32) -> String {
    let valid_set: Vec<u32> = vec![
        60, 300, 900, 1800, 3600, 7200, 14400, 28800, 43200, 86400, 604800,
    ];
    if !valid_set.contains(&interval) {
        let joined = valid_set
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",");
        panic!("KuCoin Swap available intervals {}", joined);
    }
    format!(
        r#"{{"id":"crypto-ws-client","type":"subscribe","topic":"/contractMarket/candle:{}_{}","privateChannel":false,"response":true}}"#,
        pair,
        interval / 60,
    )
}

impl_candlestick!(KuCoinSwapWSClient);

impl<'a> Level3OrderBook for KuCoinSwapWSClient<'a> {
    fn subscribe_l3_orderbook(&self, symbols: &[String]) {
        let raw_channels: Vec<String> = symbols
            .iter()
            .map(|symbol| to_raw_channel("/contractMarket/level3v2", symbol))
            .collect();
        self.client.subscribe(&raw_channels);
    }
}

define_client!(
    KuCoinSwapWSClient,
    EXCHANGE_NAME,
    WEBSOCKET_URL.as_str(),
    channels_to_commands,
    on_misc_msg,
    Some(CLIENT_PING_INTERVAL_AND_MSG)
);
