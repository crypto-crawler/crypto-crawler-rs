use crate::WSClient;
use std::sync::mpsc::Sender;

use super::super::ws_client_internal::WSClientInternal;
use super::super::{Candlestick, Level3OrderBook, OrderBook, OrderBookTopK, Ticker, Trade, BBO};
use super::utils::{
    channels_to_commands, on_misc_msg, to_raw_channel, CLIENT_PING_INTERVAL_AND_MSG, EXCHANGE_NAME,
};

const WEBSOCKET_URL: &str = "wss://stream.bybit.com/realtime";

/// Bybit Inverses markets.
///
/// InverseFuture:
///   * WebSocket API doc: <https://bybit-exchange.github.io/docs/inverse_futures/>
///   * Trading at: <https://www.bybit.com/trade/inverse/futures/BTCUSD_BIQ>
///
/// InverseSwap:
///   * WebSocket API doc: <https://bybit-exchange.github.io/docs/inverse/#t-websocket>
///   * Trading at: <https://www.bybit.com/trade/inverse/>
pub struct BybitInverseWSClient {
    client: WSClientInternal,
}

#[rustfmt::skip]
impl_trait!(Trade, BybitInverseWSClient, subscribe_trade, "trade", to_raw_channel);
#[rustfmt::skip]
impl_trait!(OrderBook, BybitInverseWSClient, subscribe_orderbook, "orderBookL2_25", to_raw_channel);
#[rustfmt::skip]
impl_trait!(Ticker, BybitInverseWSClient, subscribe_ticker, "instrument_info.100ms", to_raw_channel);

// https://bybit-exchange.github.io/docs/inverse_futures/#t-websocketklinev2
// https://bybit-exchange.github.io/docs/inverse/#t-websocketklinev2
fn to_candlestick_raw_channel(symbol: &str, interval: usize) -> String {
    let interval_str = match interval {
        60 => "1",
        180 => "3",
        300 => "5",
        900 => "15",
        1800 => "30",
        3600 => "60",
        7200 => "120",
        14400 => "240",
        21600 => "360",
        86400 => "D",
        604800 => "W",
        2592000 => "M",
        _ => panic!("Invalid Bybit candlestick interval Bybit In {}", interval),
    };
    format!("klineV2.{}.{}", interval_str, symbol)
}

impl_candlestick!(BybitInverseWSClient);

panic_l2_topk!(BybitInverseWSClient);
panic_bbo!(BybitInverseWSClient);
panic_l3_orderbook!(BybitInverseWSClient);

impl_new_constructor!(
    BybitInverseWSClient,
    EXCHANGE_NAME,
    WEBSOCKET_URL,
    channels_to_commands,
    on_misc_msg,
    Some(CLIENT_PING_INTERVAL_AND_MSG),
    None
);
impl_ws_client_trait!(BybitInverseWSClient);
