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

const EXCHANGE_NAME: &str = "bybit";
// usize::MAX means unlimited
const MAX_SUBSCRIPTIONS_PER_CONNECTION: usize = usize::MAX;

fn extract_symbol(json: &str) -> String {
    let obj = serde_json::from_str::<HashMap<String, Value>>(&json).unwrap();
    let topic = obj.get("topic").unwrap().as_str().unwrap();
    let last_dot = topic.rfind('.').unwrap();
    (&topic[(last_dot + 1)..]).to_string()
}

#[rustfmt::skip]
gen_crawl_event!(crawl_trade_inverse_swap, BybitInverseSwapWSClient, MessageType::Trade, subscribe_trade);
#[rustfmt::skip]
gen_crawl_event!(crawl_l2_event_inverse_swap, BybitInverseSwapWSClient, MessageType::L2Event, subscribe_orderbook);
#[rustfmt::skip]
gen_crawl_event!(crawl_trade_linear_swap, BybitLinearSwapWSClient, MessageType::Trade, subscribe_trade);
#[rustfmt::skip]
gen_crawl_event!(crawl_l2_event_linear_swap, BybitLinearSwapWSClient, MessageType::L2Event, subscribe_orderbook);

pub(crate) fn crawl_trade(
    market_type: MarketType,
    symbols: Option<&[String]>,
    on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
    duration: Option<u64>,
) -> Option<std::thread::JoinHandle<()>> {
    match market_type {
        MarketType::InverseSwap => crawl_trade_inverse_swap(market_type, symbols, on_msg, duration),
        MarketType::LinearSwap => crawl_trade_linear_swap(market_type, symbols, on_msg, duration),
        _ => panic!("Bybit does NOT have the {} market type", market_type),
    }
}

pub(crate) fn crawl_l2_event(
    market_type: MarketType,
    symbols: Option<&[String]>,
    on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
    duration: Option<u64>,
) -> Option<std::thread::JoinHandle<()>> {
    match market_type {
        MarketType::InverseSwap => {
            crawl_l2_event_inverse_swap(market_type, symbols, on_msg, duration)
        }
        MarketType::LinearSwap => {
            crawl_l2_event_linear_swap(market_type, symbols, on_msg, duration)
        }
        _ => panic!("Bybit does NOT have the {} market type", market_type),
    }
}
