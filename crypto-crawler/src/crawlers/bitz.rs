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

const EXCHANGE_NAME: &str = "bitz";
// usize::MAX means unlimited
const MAX_SUBSCRIPTIONS_PER_CONNECTION: usize = usize::MAX;

#[rustfmt::skip]
gen_crawl_event!(crawl_trade_spot, BitzSpotWSClient, MessageType::Trade, subscribe_trade);
#[rustfmt::skip]
gen_crawl_event!(crawl_l2_event_spot, BitzSpotWSClient, MessageType::L2Event, subscribe_orderbook);
#[rustfmt::skip]
gen_crawl_event!(crawl_ticker_spot, BitzSpotWSClient, MessageType::Ticker, subscribe_ticker);
#[rustfmt::skip]
gen_crawl_candlestick!(crawl_candlestick_spot, BitzSpotWSClient);

pub(crate) fn crawl_trade(
    market_type: MarketType,
    symbols: Option<&[String]>,
    on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
    duration: Option<u64>,
) -> Option<std::thread::JoinHandle<()>> {
    match market_type {
        MarketType::Spot => crawl_trade_spot(market_type, symbols, on_msg, duration),
        _ => panic!("Bitz does NOT have the {} market type", market_type),
    }
}

pub(crate) fn crawl_l2_event(
    market_type: MarketType,
    symbols: Option<&[String]>,
    on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
    duration: Option<u64>,
) -> Option<std::thread::JoinHandle<()>> {
    match market_type {
        MarketType::Spot => crawl_l2_event_spot(market_type, symbols, on_msg, duration),
        _ => panic!("Bitz does NOT have the {} market type", market_type),
    }
}

pub(crate) fn crawl_ticker(
    market_type: MarketType,
    symbols: Option<&[String]>,
    on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
    duration: Option<u64>,
) -> Option<std::thread::JoinHandle<()>> {
    match market_type {
        MarketType::Spot => crawl_ticker_spot(market_type, symbols, on_msg, duration),
        _ => panic!("Bitz does NOT have the {} market type", market_type),
    }
}

pub(crate) fn crawl_candlestick(
    market_type: MarketType,
    symbol_interval_list: Option<&[(String, usize)]>,
    on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
    duration: Option<u64>,
) -> Option<std::thread::JoinHandle<()>> {
    match market_type {
        MarketType::Spot => {
            crawl_candlestick_spot(market_type, symbol_interval_list, on_msg, duration)
        }
        _ => panic!("Bitz does NOT have the {} market type", market_type),
    }
}
