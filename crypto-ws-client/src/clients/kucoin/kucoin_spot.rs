use crate::WSClient;
use std::sync::mpsc::Sender;

use super::super::ws_client_internal::WSClientInternal;
use super::super::{Candlestick, Level3OrderBook, OrderBook, OrderBookTopK, Ticker, Trade, BBO};
use super::utils::{
    channels_to_commands, fetch_ws_token, on_misc_msg, to_raw_channel,
    CLIENT_PING_INTERVAL_AND_MSG, EXCHANGE_NAME,
};

/// The WebSocket client for KuCoin Spot market.
///
/// * WebSocket API doc: <https://docs.kucoin.com/#websocket-feed>
/// * Trading at: <https://trade.kucoin.com/>
pub struct KuCoinSpotWSClient {
    client: WSClientInternal,
}

impl KuCoinSpotWSClient {
    /// Creates a KuCoinSpotWSClient websocket client.
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
        println!("{}", real_url);
        KuCoinSpotWSClient {
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
impl_trait!(Trade, KuCoinSpotWSClient, subscribe_trade, "/market/match", to_raw_channel);
#[rustfmt::skip]
impl_trait!(BBO, KuCoinSpotWSClient, subscribe_bbo, "/market/ticker", to_raw_channel);
#[rustfmt::skip]
impl_trait!(OrderBook, KuCoinSpotWSClient, subscribe_orderbook, "/market/level2", to_raw_channel);
#[rustfmt::skip]
impl_trait!(OrderBookTopK, KuCoinSpotWSClient, subscribe_orderbook_topk, "/spotMarket/level2Depth5", to_raw_channel);
#[rustfmt::skip]
impl_trait!(Ticker, KuCoinSpotWSClient, subscribe_ticker, "/market/snapshot", to_raw_channel);

fn to_candlestick_raw_channel(pair: &str, interval: usize) -> String {
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

impl Level3OrderBook for KuCoinSpotWSClient {
    fn subscribe_l3_orderbook(&self, symbols: &[String]) {
        let raw_channels: Vec<String> = symbols
            .iter()
            .map(|symbol| to_raw_channel("/spotMarket/level3", symbol))
            .collect();
        self.client.subscribe(&raw_channels);
    }
}

impl_ws_client_trait!(KuCoinSpotWSClient);
