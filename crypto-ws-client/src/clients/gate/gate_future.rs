use crate::WSClient;
use std::sync::{Arc, Mutex};

use super::super::ws_client_internal::WSClientInternal;
use super::super::{Candlestick, Level3OrderBook, OrderBook, OrderBookTopK, Ticker, Trade, BBO};
use super::utils::{
    channels_to_commands, on_misc_msg, to_candlestick_raw_channel_shared, to_raw_channel,
    CLIENT_PING_INTERVAL_AND_MSG, EXCHANGE_NAME,
};

const INVERSE_FUTURE_WEBSOCKET_URL: &str = "wss://fx-ws.gateio.ws/v4/ws/delivery/btc";
const LINEAR_FUTURE_WEBSOCKET_URL: &str = "wss://fx-ws.gateio.ws/v4/ws/delivery/usdt";

/// The WebSocket client for Gate InverseFuture market.
///
/// * WebSocket API doc: <https://www.gate.io/docs/delivery/ws/en/index.html>
/// * Trading at <https://www.gate.io/cn/futures-delivery/btc>
pub struct GateInverseFutureWSClient<'a> {
    client: WSClientInternal<'a>,
}

/// The WebSocket client for Gate LinearFuture market.
///
/// * WebSocket API doc: <https://www.gate.io/docs/delivery/ws/en/index.html>
/// * Trading at <https://www.gate.io/cn/futures-delivery/usdt>
pub struct GateLinearFutureWSClient<'a> {
    client: WSClientInternal<'a>,
}

#[rustfmt::skip]
impl_trait!(Trade, GateInverseFutureWSClient, subscribe_trade, "futures.trades", to_raw_channel);
#[rustfmt::skip]
impl_trait!(OrderBook, GateInverseFutureWSClient, subscribe_orderbook, "futures.order_book", to_raw_channel);
#[rustfmt::skip]
impl_trait!(Ticker, GateInverseFutureWSClient, subscribe_ticker, "futures.tickers", to_raw_channel);

impl<'a> BBO for GateInverseFutureWSClient<'a> {
    fn subscribe_bbo(&self, _pairs: &[String]) {
        panic!("Gate does NOT have BBO channel");
    }
}
impl<'a> OrderBookTopK for GateInverseFutureWSClient<'a> {
    fn subscribe_orderbook_topk(&self, _pairs: &[String]) {
        panic!("Gate does NOT have orderbook snapshot channel");
    }
}

#[rustfmt::skip]
impl_trait!(Trade, GateLinearFutureWSClient, subscribe_trade, "futures.trades", to_raw_channel);
#[rustfmt::skip]
impl_trait!(OrderBook, GateLinearFutureWSClient, subscribe_orderbook, "futures.order_book", to_raw_channel);
#[rustfmt::skip]
impl_trait!(Ticker, GateLinearFutureWSClient, subscribe_ticker, "futures.tickers", to_raw_channel);

impl<'a> BBO for GateLinearFutureWSClient<'a> {
    fn subscribe_bbo(&self, _pairs: &[String]) {
        panic!("Gate does NOT have BBO channel");
    }
}
impl<'a> OrderBookTopK for GateLinearFutureWSClient<'a> {
    fn subscribe_orderbook_topk(&self, _pairs: &[String]) {
        panic!("Gate does NOT have orderbook snapshot channel");
    }
}

fn to_candlestick_raw_channel(pair: &str, interval: usize) -> String {
    to_candlestick_raw_channel_shared("futures", pair, interval)
}

impl_candlestick!(GateInverseFutureWSClient);
impl_candlestick!(GateLinearFutureWSClient);

panic_l3_orderbook!(GateInverseFutureWSClient);
panic_l3_orderbook!(GateLinearFutureWSClient);

define_client!(
    GateInverseFutureWSClient,
    EXCHANGE_NAME,
    INVERSE_FUTURE_WEBSOCKET_URL,
    channels_to_commands,
    on_misc_msg,
    Some(CLIENT_PING_INTERVAL_AND_MSG),
    None
);

define_client!(
    GateLinearFutureWSClient,
    EXCHANGE_NAME,
    LINEAR_FUTURE_WEBSOCKET_URL,
    channels_to_commands,
    on_misc_msg,
    Some(CLIENT_PING_INTERVAL_AND_MSG),
    None
);
