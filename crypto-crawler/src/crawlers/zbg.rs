use std::sync::{
    atomic::{AtomicBool, AtomicUsize, Ordering},
    Arc, Mutex,
};

use std::time::Duration;

use super::utils::{check_args, fetch_symbols_retry};
use crate::{msg::Message, MessageType};
use crypto_markets::MarketType;
use crypto_ws_client::*;

use log::*;

const EXCHANGE_NAME: &str = "zbg";
// usize::MAX means unlimited
const MAX_SUBSCRIPTIONS_PER_CONNECTION: usize = usize::MAX;

#[rustfmt::skip]
gen_crawl_event!(crawl_trade_spot, ZbgSpotWSClient, MessageType::Trade, subscribe_trade);
#[rustfmt::skip]
gen_crawl_event!(crawl_trade_swap, ZbgSwapWSClient, MessageType::Trade, subscribe_trade);
#[rustfmt::skip]
gen_crawl_event!(crawl_l2_event_spot, ZbgSpotWSClient, MessageType::L2Event, subscribe_orderbook);
#[rustfmt::skip]
gen_crawl_event!(crawl_l2_event_swap, ZbgSwapWSClient, MessageType::L2Event, subscribe_orderbook);
#[rustfmt::skip]
gen_crawl_event!(crawl_ticker_spot, ZbgSpotWSClient, MessageType::Ticker, subscribe_ticker);
#[rustfmt::skip]
gen_crawl_event!(crawl_ticker_swap, ZbgSwapWSClient, MessageType::Ticker, subscribe_ticker);

pub(crate) fn crawl_trade(
    market_type: MarketType,
    symbols: Option<&[String]>,
    on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
    duration: Option<u64>,
) -> Option<std::thread::JoinHandle<()>> {
    match market_type {
        MarketType::Spot => crawl_trade_spot(market_type, symbols, on_msg, duration),
        MarketType::InverseSwap | MarketType::LinearSwap => {
            crawl_trade_swap(market_type, symbols, on_msg, duration)
        }
        _ => panic!("ZBG does NOT have the {} market type", market_type),
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
        MarketType::InverseSwap | MarketType::LinearSwap => {
            crawl_l2_event_swap(market_type, symbols, on_msg, duration)
        }
        _ => panic!("ZBG does NOT have the {} market type", market_type),
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
        MarketType::InverseSwap | MarketType::LinearSwap => {
            crawl_ticker_swap(market_type, symbols, on_msg, duration)
        }
        _ => panic!("ZBG does NOT have the {} market type", market_type),
    }
}
