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

const EXCHANGE_NAME: &str = "mxc";
// usize::MAX means unlimited
const MAX_SUBSCRIPTIONS_PER_CONNECTION: usize = usize::MAX;

#[rustfmt::skip]
gen_crawl_event!(crawl_trade_spot, MxcSpotWSClient, MessageType::Trade, subscribe_trade);
#[rustfmt::skip]
gen_crawl_event!(crawl_trade_swap, MxcSwapWSClient, MessageType::Trade, subscribe_trade);

#[rustfmt::skip]
gen_crawl_event!(crawl_l2_event_spot, MxcSpotWSClient, MessageType::L2Event, subscribe_orderbook);
#[rustfmt::skip]
gen_crawl_event!(crawl_l2_event_swap, MxcSwapWSClient, MessageType::L2Event, subscribe_orderbook);

#[rustfmt::skip]
gen_crawl_event!(crawl_ticker_swap, MxcSwapWSClient, MessageType::Ticker, subscribe_ticker);

pub(crate) fn crawl_trade(
    market_type: MarketType,
    symbols: Option<&[String]>,
    on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
    duration: Option<u64>,
) -> Option<std::thread::JoinHandle<()>> {
    match market_type {
        MarketType::Spot => crawl_trade_spot(market_type, symbols, on_msg, duration),
        MarketType::LinearSwap | MarketType::InverseSwap => {
            crawl_trade_swap(market_type, symbols, on_msg, duration)
        }
        _ => {
            error!("Unknown market type {} of {}", market_type, EXCHANGE_NAME);
            panic!("Unknown market type {} of {}", market_type, EXCHANGE_NAME);
        }
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
        MarketType::LinearSwap | MarketType::InverseSwap => {
            crawl_l2_event_swap(market_type, symbols, on_msg, duration)
        }
        _ => {
            error!("Unknown market type {} of {}", market_type, EXCHANGE_NAME);
            panic!("Unknown market type {} of {}", market_type, EXCHANGE_NAME);
        }
    }
}

pub(crate) fn crawl_ticker(
    market_type: MarketType,
    symbols: Option<&[String]>,
    on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
    duration: Option<u64>,
) -> Option<std::thread::JoinHandle<()>> {
    match market_type {
        MarketType::LinearSwap | MarketType::InverseSwap => {
            crawl_ticker_swap(market_type, symbols, on_msg, duration)
        }
        _ => {
            panic!("Unknown market type {} of {}", market_type, EXCHANGE_NAME);
        }
    }
}
