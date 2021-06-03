use crate::WSClient;
use std::sync::{Arc, Mutex};

use super::super::ws_client_internal::WSClientInternal;
use super::super::{Candlestick, OrderBook, OrderBookSnapshot, Ticker, Trade, BBO};
use super::utils::{
    channels_to_commands, on_misc_msg, to_raw_channel, EXCHANGE_NAME, PING_INTERVAL_AND_MSG,
};

const WEBSOCKET_URL: &str = "wss://stream.bybit.com/realtime";

/// Bybit InverseFuture market.
///
/// * WebSocket API doc: <https://bybit-exchange.github.io/docs/inverse_futures/>
/// * Trading at: <https://www.bybit.com/trade/inverse/futures/BTCUSD_BIQ>
pub struct BybitInverseFutureWSClient<'a> {
    client: WSClientInternal<'a>,
}

#[rustfmt::skip]
impl_trait!(Trade, BybitInverseFutureWSClient, subscribe_trade, "trade", to_raw_channel);
#[rustfmt::skip]
impl_trait!(BBO, BybitInverseFutureWSClient, subscribe_bbo, "orderBookL2_25", to_raw_channel);
#[rustfmt::skip]
impl_trait!(OrderBook, BybitInverseFutureWSClient, subscribe_orderbook, "orderBookL2_25", to_raw_channel);
#[rustfmt::skip]
impl_trait!(Ticker, BybitInverseFutureWSClient, subscribe_ticker, "instrument_info.100ms", to_raw_channel);

impl<'a> OrderBookSnapshot for BybitInverseFutureWSClient<'a> {
    fn subscribe_orderbook_snapshot(&self, _pairs: &[String]) {
        panic!("FTX does NOT have orderbook snapshot channel");
    }
}

fn to_candlestick_raw_channel(pair: &str, interval: u32) -> String {
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
        _ => panic!("Huobi has intervals 1min,5min,15min,30min,60min,4hour,1day,1week,1mon"),
    };
    format!("klineV2.{}.{}", interval_str, pair)
}

impl_candlestick!(BybitInverseFutureWSClient);

define_client!(
    BybitInverseFutureWSClient,
    EXCHANGE_NAME,
    WEBSOCKET_URL,
    channels_to_commands,
    on_misc_msg,
    Some(PING_INTERVAL_AND_MSG)
);
