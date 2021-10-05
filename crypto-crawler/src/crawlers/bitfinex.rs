use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use std::time::Duration;

use super::utils::{
    check_args, fetch_symbols_retry, get_candlestick_intervals, get_connection_interval_ms,
    get_send_interval_ms,
};
use crate::utils::WS_LOCKS;
use crate::{msg::Message, MessageType};
use crypto_markets::MarketType;
use crypto_ws_client::*;
use log::*;

const EXCHANGE_NAME: &str = "bitfinex";
// All websocket connections have a limit of 30 subscriptions to public market data feed channels
// (tickers, book, candles, trades, …). We kindly ask all users to adapt their application setup
// accordingly to split subscriptions to channels using multiple WebSocket connections.
// see https://docs.bitfinex.com/docs/ws-general
const MAX_SUBSCRIPTIONS_PER_CONNECTION: usize = 30;

#[rustfmt::skip]
gen_crawl_event!(crawl_trade, BitfinexWSClient, MessageType::Trade, subscribe_trade);
#[rustfmt::skip]
gen_crawl_event!(crawl_l2_event, BitfinexWSClient, MessageType::L2Event, subscribe_orderbook);
#[rustfmt::skip]
gen_crawl_event!(crawl_bbo, BitfinexWSClient, MessageType::BBO, subscribe_bbo);
#[rustfmt::skip]
gen_crawl_event!(crawl_l3_event, BitfinexWSClient, MessageType::L3Event, subscribe_l3_orderbook);
#[rustfmt::skip]
gen_crawl_event!(crawl_ticker, BitfinexWSClient, MessageType::Ticker, subscribe_ticker);
#[rustfmt::skip]
gen_crawl_candlestick!(crawl_candlestick, BitfinexWSClient);
