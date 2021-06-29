use crate::WSClient;
use std::sync::{Arc, Mutex};

use super::super::ws_client_internal::WSClientInternal;
use super::super::{Candlestick, OrderBook, OrderBookSnapshot, Ticker, Trade, BBO};
use super::utils::{
    channels_to_commands, on_misc_msg, to_candlestick_raw_channel, to_raw_channel,
    CLIENT_PING_INTERVAL_AND_MSG, EXCHANGE_NAME,
};

const LINEAR_FUTURE_WEBSOCKET_URL: &str = "wss://fx-ws.gateio.ws/v4/ws/delivery/usdt";

/// The WebSocket client for Gate LinearFuture market.
///
/// * WebSocket API doc: <https://www.gate.io/docs/delivery/ws/index.html>
/// * Trading at <https://www.gateio.pro/cn/futures-delivery/usdt>
pub struct GateLinearFutureWSClient<'a> {
    client: WSClientInternal<'a>,
}

#[rustfmt::skip]
impl_trait!(Trade, GateLinearFutureWSClient, subscribe_trade, "trades", to_raw_channel);
#[rustfmt::skip]
impl_trait!(OrderBook, GateLinearFutureWSClient, subscribe_orderbook, "order_book", to_raw_channel);
#[rustfmt::skip]
impl_trait!(Ticker, GateLinearFutureWSClient, subscribe_ticker, "tickers", to_raw_channel);

impl<'a> BBO for GateLinearFutureWSClient<'a> {
    fn subscribe_bbo(&self, _pairs: &[String]) {
        panic!("Gate does NOT have BBO channel");
    }
}

impl<'a> OrderBookSnapshot for GateLinearFutureWSClient<'a> {
    fn subscribe_orderbook_snapshot(&self, _pairs: &[String]) {
        panic!("Gate does NOT have orderbook snapshot channel");
    }
}

impl_candlestick!(GateLinearFutureWSClient);

define_client!(
    GateLinearFutureWSClient,
    EXCHANGE_NAME,
    LINEAR_FUTURE_WEBSOCKET_URL,
    channels_to_commands,
    on_misc_msg,
    Some(CLIENT_PING_INTERVAL_AND_MSG)
);
