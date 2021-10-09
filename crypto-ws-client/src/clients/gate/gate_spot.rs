use crate::WSClient;
use std::sync::mpsc::Sender;

use super::super::ws_client_internal::WSClientInternal;
use super::super::{Candlestick, Level3OrderBook, OrderBook, OrderBookTopK, Ticker, Trade, BBO};
use super::utils::{
    channels_to_commands, on_misc_msg, to_candlestick_raw_channel_shared, to_raw_channel,
    EXCHANGE_NAME,
};

const WEBSOCKET_URL: &str = "wss://api.gateio.ws/ws/v4/";

// https://www.gate.io/docs/apiv4/ws/en/#application-ping-pong
const CLIENT_PING_INTERVAL_AND_MSG: (u64, &str) = (60, r#"{"channel":"spot.ping"}"#);

/// The WebSocket client for Gate spot market.
///
/// * WebSocket API doc: <https://www.gate.io/docs/apiv4/ws/en/index.html>
/// * Trading at <https://www.gate.io/en/trade/BTC_USDT>
pub struct GateSpotWSClient {
    client: WSClientInternal,
}

#[rustfmt::skip]
impl_trait!(Trade, GateSpotWSClient, subscribe_trade, "spot.trades", to_raw_channel);
#[rustfmt::skip]
impl_trait!(OrderBook, GateSpotWSClient, subscribe_orderbook, "spot.order_book_update", to_raw_channel);
#[rustfmt::skip]
impl_trait!(OrderBookTopK, GateSpotWSClient, subscribe_orderbook_topk, "spot.order_book", to_raw_channel);
#[rustfmt::skip]
impl_trait!(BBO, GateSpotWSClient, subscribe_bbo, "spot.book_ticker", to_raw_channel);
#[rustfmt::skip]
impl_trait!(Ticker, GateSpotWSClient, subscribe_ticker, "spot.tickers", to_raw_channel);

fn to_candlestick_raw_channel(pair: &str, interval: usize) -> String {
    to_candlestick_raw_channel_shared("spot", pair, interval)
}

impl_candlestick!(GateSpotWSClient);

panic_l3_orderbook!(GateSpotWSClient);

impl_new_constructor!(
    GateSpotWSClient,
    EXCHANGE_NAME,
    WEBSOCKET_URL,
    channels_to_commands,
    on_misc_msg,
    Some(CLIENT_PING_INTERVAL_AND_MSG),
    None
);
impl_ws_client_trait!(GateSpotWSClient);
