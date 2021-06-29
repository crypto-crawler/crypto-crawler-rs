use crate::WSClient;
use std::sync::{Arc, Mutex};

use super::super::ws_client_internal::WSClientInternal;
use super::super::{Candlestick, OrderBook, OrderBookSnapshot, Ticker, Trade, BBO};
use super::utils::{
    channels_to_commands, on_misc_msg, to_candlestick_raw_channel, to_raw_channel,
    CLIENT_PING_INTERVAL_AND_MSG, EXCHANGE_NAME,
};

const INVERSE_SWAP_WEBSOCKET_URL: &str = "wss://fx-ws.gateio.ws/v4/ws/btc";
const LINEAR_SWAP_WEBSOCKET_URL: &str = "wss://fx-ws.gateio.ws/v4/ws/usdt";

/// The WebSocket client for Gate InverseSwap market.
///
/// * WebSocket API doc: <https://www.gate.io/docs/futures/ws/index.html>
/// * Trading at <https://www.gateio.pro/cn/futures_trade/BTC/BTC_USD>
pub struct GateInverseSwapWSClient<'a> {
    client: WSClientInternal<'a>,
}

/// The WebSocket client for Gate LinearSwap market.
///
/// * WebSocket API doc: <https://www.gate.io/docs/futures/ws/index.html>
/// * Trading at <https://www.gateio.pro/cn/futures_trade/USDT/BTC_USDT>
pub struct GateLinearSwapWSClient<'a> {
    client: WSClientInternal<'a>,
}

#[rustfmt::skip]
impl_trait!(Trade, GateInverseSwapWSClient, subscribe_trade, "trades", to_raw_channel);
#[rustfmt::skip]
impl_trait!(OrderBook, GateInverseSwapWSClient, subscribe_orderbook, "order_book", to_raw_channel);
#[rustfmt::skip]
impl_trait!(Ticker, GateInverseSwapWSClient, subscribe_ticker, "tickers", to_raw_channel);
#[rustfmt::skip]
impl_trait!(Trade, GateLinearSwapWSClient, subscribe_trade, "trades", to_raw_channel);
#[rustfmt::skip]
impl_trait!(OrderBook, GateLinearSwapWSClient, subscribe_orderbook, "order_book", to_raw_channel);
#[rustfmt::skip]
impl_trait!(Ticker, GateLinearSwapWSClient, subscribe_ticker, "tickers", to_raw_channel);

impl<'a> BBO for GateInverseSwapWSClient<'a> {
    fn subscribe_bbo(&self, _pairs: &[String]) {
        panic!("Gate does NOT have BBO channel");
    }
}
impl<'a> BBO for GateLinearSwapWSClient<'a> {
    fn subscribe_bbo(&self, _pairs: &[String]) {
        panic!("Gate does NOT have BBO channel");
    }
}

impl<'a> OrderBookSnapshot for GateInverseSwapWSClient<'a> {
    fn subscribe_orderbook_snapshot(&self, _pairs: &[String]) {
        panic!("Gate does NOT have orderbook snapshot channel");
    }
}
impl<'a> OrderBookSnapshot for GateLinearSwapWSClient<'a> {
    fn subscribe_orderbook_snapshot(&self, _pairs: &[String]) {
        panic!("Gate does NOT have orderbook snapshot channel");
    }
}

impl_candlestick!(GateInverseSwapWSClient);
impl_candlestick!(GateLinearSwapWSClient);

define_client!(
    GateInverseSwapWSClient,
    EXCHANGE_NAME,
    INVERSE_SWAP_WEBSOCKET_URL,
    channels_to_commands,
    on_misc_msg,
    Some(CLIENT_PING_INTERVAL_AND_MSG)
);

define_client!(
    GateLinearSwapWSClient,
    EXCHANGE_NAME,
    LINEAR_SWAP_WEBSOCKET_URL,
    channels_to_commands,
    on_misc_msg,
    Some(CLIENT_PING_INTERVAL_AND_MSG)
);
