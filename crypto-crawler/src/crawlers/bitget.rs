use std::sync::{
    atomic::{AtomicBool, AtomicUsize, Ordering},
    Arc, Mutex,
};

use std::{collections::HashMap, time::Duration};

use super::utils::{check_args, fetch_symbols_retry};
use crate::{msg::Message, MessageType};
use crypto_markets::MarketType;
use crypto_ws_client::*;
use log::*;
use serde_json::Value;

const EXCHANGE_NAME: &str = "bitget";
// usize::MAX means unlimited
const MAX_SUBSCRIPTIONS_PER_CONNECTION: usize = usize::MAX;

fn extract_symbol(json: &str) -> String {
    let obj = serde_json::from_str::<HashMap<String, Value>>(&json).unwrap();
    let data = obj.get("data").unwrap().as_array().unwrap();
    data[0]
        .as_object()
        .unwrap()
        .get("instrument_id")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string()
}

#[rustfmt::skip]
gen_crawl_event!(crawl_trade_swap, BitgetSwapWSClient, MessageType::Trade, subscribe_trade);
#[rustfmt::skip]
gen_crawl_event!(crawl_l2_event_swap, BitgetSwapWSClient, MessageType::L2Event, subscribe_orderbook);

pub(crate) fn crawl_trade(
    market_type: MarketType,
    symbols: Option<&[String]>,
    on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
    duration: Option<u64>,
) -> Option<std::thread::JoinHandle<()>> {
    match market_type {
        MarketType::InverseSwap | MarketType::LinearSwap => {
            crawl_trade_swap(market_type, symbols, on_msg, duration)
        }
        _ => panic!("Bitget does NOT have the {} market type", market_type),
    }
}

pub(crate) fn crawl_l2_event(
    market_type: MarketType,
    symbols: Option<&[String]>,
    on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
    duration: Option<u64>,
) -> Option<std::thread::JoinHandle<()>> {
    match market_type {
        MarketType::InverseSwap | MarketType::LinearSwap => {
            crawl_l2_event_swap(market_type, symbols, on_msg, duration)
        }
        _ => panic!("Bitget does NOT have the {} market type", market_type),
    }
}
