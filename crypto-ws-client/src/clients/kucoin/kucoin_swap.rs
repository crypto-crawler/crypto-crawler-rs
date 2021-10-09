use crate::WSClient;
use std::sync::mpsc::Sender;

use super::super::ws_client_internal::WSClientInternal;
use super::super::{Candlestick, Level3OrderBook, OrderBook, OrderBookTopK, Ticker, Trade, BBO};
use super::utils::{
    channels_to_commands, fetch_ws_token, on_misc_msg, to_raw_channel,
    CLIENT_PING_INTERVAL_AND_MSG, EXCHANGE_NAME,
};

/// The WebSocket client for KuCoin Swap markets.
///
/// * WebSocket API doc: <https://docs.kucoin.cc/futures/#websocket-2>
/// * Trading at: <https://futures.kucoin.com/>
pub struct KuCoinSwapWSClient {
    client: WSClientInternal,
}

impl KuCoinSwapWSClient {
    /// Creates a KuCoinSwapWSClient websocket client.
    ///
    /// # Arguments
    ///
    /// * `tx` - The sending part of a channel
    /// * `url` - Optional server url, usually you don't need specify it
    pub fn new(tx: Sender<String>, url: Option<&str>) -> Self {
        let real_url = match url {
            Some(endpoint) => endpoint.to_string(),
            None => {
                let ws_token = fetch_ws_token();
                let ws_url = format!("{}?token={}", ws_token.endpoint, ws_token.token);
                ws_url
            }
        };
        KuCoinSwapWSClient {
            client: WSClientInternal::new(
                EXCHANGE_NAME,
                &real_url,
                tx,
                on_misc_msg,
                channels_to_commands,
                Some(CLIENT_PING_INTERVAL_AND_MSG),
                None,
            ),
        }
    }
}

#[rustfmt::skip]
impl_trait!(Trade, KuCoinSwapWSClient, subscribe_trade, "/contractMarket/execution", to_raw_channel);
#[rustfmt::skip]
impl_trait!(BBO, KuCoinSwapWSClient, subscribe_bbo, "/contractMarket/tickerV2", to_raw_channel);
#[rustfmt::skip]
impl_trait!(OrderBook, KuCoinSwapWSClient, subscribe_orderbook, "/contractMarket/level2", to_raw_channel);
#[rustfmt::skip]
impl_trait!(OrderBookTopK, KuCoinSwapWSClient, subscribe_orderbook_topk, "/contractMarket/level2Depth5", to_raw_channel);
#[rustfmt::skip]
impl_trait!(Ticker, KuCoinSwapWSClient, subscribe_ticker, "/contractMarket/snapshot", to_raw_channel);

fn to_candlestick_raw_channel(pair: &str, interval: usize) -> String {
    let valid_set: Vec<usize> = vec![
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

impl Level3OrderBook for KuCoinSwapWSClient {
    fn subscribe_l3_orderbook(&self, symbols: &[String]) {
        let raw_channels: Vec<String> = symbols
            .iter()
            .map(|symbol| to_raw_channel("/contractMarket/level3v2", symbol))
            .collect();
        self.client.subscribe(&raw_channels);
    }
}

impl_ws_client_trait!(KuCoinSwapWSClient);
