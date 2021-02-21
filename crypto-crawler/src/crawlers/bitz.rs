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

const EXCHANGE_NAME: &str = "bitz";
// usize::MAX means unlimited
const MAX_SUBSCRIPTIONS_PER_CONNECTION: usize = usize::MAX;

fn extract_symbol(json: &str) -> String {
    let obj = serde_json::from_str::<HashMap<String, Value>>(&json).unwrap();
    obj.get("params")
        .unwrap()
        .as_object()
        .unwrap()
        .get("symbol")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string()
}

gen_check_args!(EXCHANGE_NAME);

#[rustfmt::skip]
gen_crawl_event!(crawl_trade_spot, BitzSpotWSClient, MessageType::Trade, subscribe_trade);
#[rustfmt::skip]
gen_crawl_event!(crawl_l2_event_spot, BitzSpotWSClient, MessageType::L2Event, subscribe_orderbook);

#[rustfmt::skip]
gen_crawl_snapshot!(crawl_l2_snapshot_spot, MessageType::L2Snapshot, BitzSpotRestClient::fetch_l2_snapshot);
#[rustfmt::skip]
gen_crawl_snapshot!(crawl_l2_snapshot_swap, MessageType::L2Snapshot, BitzSwapRestClient::fetch_l2_snapshot);

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

pub(crate) fn crawl_l2_snapshot(
    market_type: MarketType,
    symbols: Option<&[String]>,
    on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
    interval: Option<u64>,
    duration: Option<u64>,
) {
    let func = match market_type {
        MarketType::Spot => crawl_l2_snapshot_spot,
        MarketType::InverseSwap | MarketType::LinearSwap => crawl_l2_snapshot_swap,
        _ => panic!("Bitz does NOT have the {} market type", market_type),
    };
    func(market_type, symbols, on_msg, interval, duration);
}
