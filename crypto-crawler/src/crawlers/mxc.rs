use std::sync::{
    atomic::{AtomicBool, AtomicUsize, Ordering},
    Arc, Mutex,
};

use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use super::utils::fetch_symbols_retry;
use crate::{msg::Message, MessageType};
use crypto_markets::MarketType;
use crypto_rest_client::*;
use crypto_ws_client::*;
use log::*;
use serde_json::Value;

const EXCHANGE_NAME: &str = "mxc";
// usize::MAX means unlimited
const MAX_SUBSCRIPTIONS_PER_CONNECTION: usize = usize::MAX;

fn extract_symbol(json: &str) -> String {
    if json.starts_with('[') {
        let arr = serde_json::from_str::<Vec<Value>>(&json).unwrap();
        arr[1].get("symbol").unwrap().as_str().unwrap().to_string()
    } else {
        let obj = serde_json::from_str::<HashMap<String, Value>>(&json).unwrap();
        obj.get("symbol").unwrap().as_str().unwrap().to_string()
    }
}

gen_check_args!(EXCHANGE_NAME);

#[rustfmt::skip]
gen_crawl_event!(crawl_trade_spot, MxcSpotWSClient, MessageType::Trade, subscribe_trade);
#[rustfmt::skip]
gen_crawl_event!(crawl_trade_swap, MxcSwapWSClient, MessageType::Trade, subscribe_trade);

#[rustfmt::skip]
gen_crawl_event!(crawl_l2_event_spot, MxcSpotWSClient, MessageType::L2Event, subscribe_orderbook);
#[rustfmt::skip]
gen_crawl_event!(crawl_l2_event_swap, MxcSwapWSClient, MessageType::L2Event, subscribe_orderbook);

#[rustfmt::skip]
gen_crawl_snapshot!(crawl_l2_snapshot_spot, MessageType::L2Snapshot, MxcSpotRestClient::fetch_l2_snapshot);
#[rustfmt::skip]
gen_crawl_snapshot!(crawl_l2_snapshot_swap, MessageType::L2Snapshot, MxcSwapRestClient::fetch_l2_snapshot);

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

pub(crate) fn crawl_l2_snapshot(
    market_type: MarketType,
    symbols: Option<&[String]>,
    on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
    interval: Option<u64>,
    duration: Option<u64>,
) {
    let func = match market_type {
        MarketType::Spot => crawl_l2_snapshot_spot,
        MarketType::LinearSwap | MarketType::InverseSwap => crawl_l2_snapshot_swap,
        _ => panic!("Binance does NOT have the {} market type", market_type),
    };
    func(market_type, symbols, on_msg, interval, duration);
}
