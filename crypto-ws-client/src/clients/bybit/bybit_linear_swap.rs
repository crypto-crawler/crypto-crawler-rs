use crate::WSClient;
use std::sync::mpsc::Sender;

use super::super::ws_client_internal::WSClientInternal;
use super::super::{Candlestick, Level3OrderBook, OrderBook, OrderBookTopK, Ticker, Trade, BBO};
use super::utils::{
    channels_to_commands, on_misc_msg, to_raw_channel, CLIENT_PING_INTERVAL_AND_MSG, EXCHANGE_NAME,
};

const WEBSOCKET_URL: &str = "wss://stream.bybit.com/realtime_public";

/// Bybit LinearSwap market.
///
/// * WebSocket API doc: <https://bybit-exchange.github.io/docs/inverse/#t-websocket>
/// * Trading at: <https://www.bybit.com/trade/inverse/>
pub struct BybitLinearSwapWSClient {
    client: WSClientInternal,
}

#[rustfmt::skip]
impl_trait!(Trade, BybitLinearSwapWSClient, subscribe_trade, "trade", to_raw_channel);
#[rustfmt::skip]
impl_trait!(OrderBookTopK, BybitLinearSwapWSClient, subscribe_orderbook_topk, "orderBookL2_25", to_raw_channel);
#[rustfmt::skip]
impl_trait!(OrderBook, BybitLinearSwapWSClient, subscribe_orderbook, "orderBookL2_25", to_raw_channel);
#[rustfmt::skip]
impl_trait!(Ticker, BybitLinearSwapWSClient, subscribe_ticker, "instrument_info.100ms", to_raw_channel);

impl BBO for BybitLinearSwapWSClient {
    fn subscribe_bbo(&self, _pairs: &[String]) {
        panic!("bybit does NOT have BBO channel");
    }
}

fn to_candlestick_raw_channel(pair: &str, interval: usize) -> String {
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
    format!("candle.{}.{}", interval_str, pair)
}

impl_candlestick!(BybitLinearSwapWSClient);

panic_l3_orderbook!(BybitLinearSwapWSClient);

define_client!(
    BybitLinearSwapWSClient,
    EXCHANGE_NAME,
    WEBSOCKET_URL,
    channels_to_commands,
    on_misc_msg,
    Some(CLIENT_PING_INTERVAL_AND_MSG),
    None
);
