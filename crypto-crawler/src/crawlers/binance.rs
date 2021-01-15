use core::panic;
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    time::Duration,
};

use crate::{msg::Message, MessageType};
use crypto_markets::{fetch_symbols, get_market_types, MarketType};
use crypto_rest_client::*;
use crypto_ws_client::*;
use log::*;
use serde_json::Value;

const EXCHANGE_NAME: &str = "binance";

fn extract_symbol(json: &str) -> String {
    let obj = serde_json::from_str::<HashMap<String, Value>>(&json).unwrap();
    obj.get("data")
        .unwrap()
        .as_object()
        .unwrap()
        .get("s")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string()
}

fn check_args(market_type: MarketType, symbols: &[String]) {
    let market_types = get_market_types(EXCHANGE_NAME);
    if !market_types.contains(&market_type) {
        panic!(
            "{} does NOT have the {} market type",
            EXCHANGE_NAME, market_type
        );
    }

    let symbols = symbols
        .iter()
        .map(|symbol| symbol.to_uppercase())
        .collect::<Vec<String>>();

    let valid_symbols = fetch_symbols(EXCHANGE_NAME, market_type).unwrap();
    let invalid_symbols: Vec<String> = symbols
        .iter()
        .filter(|symbol| !valid_symbols.contains(symbol))
        .cloned()
        .collect();
    if !invalid_symbols.is_empty() {
        panic!(
            "Invalid symbols for {} {} market: {}, available trading symbols are {}",
            EXCHANGE_NAME,
            market_type,
            invalid_symbols.join(","),
            valid_symbols.join(",")
        );
    }
}

#[rustfmt::skip]
gen_crawl_event!(crawl_trade_spot, market_type, symbols, on_msg, duration, BinanceSpotWSClient, MessageType::Trade, subscribe_trade, true);
#[rustfmt::skip]
gen_crawl_event!(crawl_trade_inverse_future, market_type, symbols, on_msg, duration, BinanceFutureWSClient, MessageType::Trade, subscribe_trade, true);
#[rustfmt::skip]
gen_crawl_event!(crawl_trade_linear_swap, market_type, symbols, on_msg, duration, BinanceLinearSwapWSClient, MessageType::Trade, subscribe_trade, true);
#[rustfmt::skip]
gen_crawl_event!(crawl_trade_inverse_swap, market_type, symbols, on_msg, duration, BinanceInverseSwapWSClient, MessageType::Trade, subscribe_trade, true);
#[rustfmt::skip]
gen_crawl_event!(crawl_trade_linear_option, market_type, symbols, on_msg, duration, BinanceOptionWSClient, MessageType::Trade, subscribe_trade, true);

#[rustfmt::skip]
gen_crawl_event!(crawl_l2_event_spot, market_type, symbols, on_msg, duration, BinanceSpotWSClient, MessageType::L2Event, subscribe_orderbook, true);
#[rustfmt::skip]
gen_crawl_event!(crawl_l2_event_inverse_future, market_type, symbols, on_msg, duration, BinanceFutureWSClient, MessageType::L2Event, subscribe_orderbook, true);
#[rustfmt::skip]
gen_crawl_event!(crawl_l2_event_linear_swap, market_type, symbols, on_msg, duration, BinanceLinearSwapWSClient, MessageType::L2Event, subscribe_orderbook, true);
#[rustfmt::skip]
gen_crawl_event!(crawl_l2_event_inverse_swap, market_type, symbols, on_msg, duration, BinanceInverseSwapWSClient, MessageType::L2Event, subscribe_orderbook, true);
#[rustfmt::skip]
gen_crawl_event!(crawl_l2_event_linear_option, market_type, symbols, on_msg, duration, BinanceOptionWSClient, MessageType::L2Event, subscribe_orderbook, true);

#[rustfmt::skip]
gen_crawl_snapshot!(crawl_l2_snapshot_spot, market_type, symbols, on_msg, MessageType::L2Snapshot, BinanceSpotRestClient::fetch_l2_snapshot);
#[rustfmt::skip]
gen_crawl_snapshot!(crawl_l2_snapshot_inverse_future, market_type, symbols, on_msg, MessageType::L2Snapshot, BinanceFutureRestClient::fetch_l2_snapshot);
#[rustfmt::skip]
gen_crawl_snapshot!(crawl_l2_snapshot_linear_swap, market_type, symbols, on_msg, MessageType::L2Snapshot, BinanceLinearSwapRestClient::fetch_l2_snapshot);
#[rustfmt::skip]
gen_crawl_snapshot!(crawl_l2_snapshot_inverse_swap, market_type, symbols, on_msg, MessageType::L2Snapshot, BinanceInverseSwapRestClient::fetch_l2_snapshot);
#[rustfmt::skip]
gen_crawl_snapshot!(crawl_l2_snapshot_linear_option, market_type, symbols, on_msg, MessageType::L2Snapshot, BinanceOptionRestClient::fetch_l2_snapshot);

pub(crate) fn crawl_trade(
    market_type: MarketType,
    symbols: Option<&[String]>,
    on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
    duration: Option<u64>,
) -> Option<std::thread::JoinHandle<()>> {
    let lowercase = match symbols {
        Some(list) => list
            .iter()
            .map(|symbol| symbol.to_lowercase())
            .collect::<Vec<String>>(),
        None => Vec::new(),
    };
    let symbols = match symbols {
        Some(list) => {
            if list.is_empty() {
                None
            } else {
                let tmp: &[String] = &lowercase;
                Some(tmp)
            }
        }
        None => None,
    };

    match market_type {
        MarketType::Spot => crawl_trade_spot(market_type, symbols, on_msg, duration),
        MarketType::InverseFuture => {
            crawl_trade_inverse_future(market_type, symbols, on_msg, duration)
        }
        MarketType::LinearSwap => crawl_trade_linear_swap(market_type, symbols, on_msg, duration),
        MarketType::InverseSwap => crawl_trade_inverse_swap(market_type, symbols, on_msg, duration),
        MarketType::Option => crawl_trade_linear_option(market_type, symbols, on_msg, duration),
        _ => panic!("Binance does NOT have the {} market type", market_type),
    }
}

pub(crate) fn crawl_l2_event(
    market_type: MarketType,
    symbols: Option<&[String]>,
    on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
    duration: Option<u64>,
) -> Option<std::thread::JoinHandle<()>> {
    let lowercase = match symbols {
        Some(list) => list
            .iter()
            .map(|symbol| symbol.to_lowercase())
            .collect::<Vec<String>>(),
        None => Vec::new(),
    };
    let symbols = match symbols {
        Some(list) => {
            if list.is_empty() {
                None
            } else {
                let tmp: &[String] = &lowercase;
                Some(tmp)
            }
        }
        None => None,
    };

    match market_type {
        MarketType::Spot => {
            println!("##### {} #####", market_type);
            crawl_l2_event_spot(market_type, symbols, on_msg, duration)
        }
        MarketType::InverseFuture => {
            crawl_l2_event_inverse_future(market_type, symbols, on_msg, duration)
        }
        MarketType::LinearSwap => {
            crawl_l2_event_linear_swap(market_type, symbols, on_msg, duration)
        }
        MarketType::InverseSwap => {
            crawl_l2_event_inverse_swap(market_type, symbols, on_msg, duration)
        }
        MarketType::Option => crawl_l2_event_linear_option(market_type, symbols, on_msg, duration),
        _ => panic!("Binance does NOT have the {} market type", market_type),
    }
}

pub(crate) fn crawl_l2_snapshot(
    market_type: MarketType,
    symbols: Option<&[String]>,
    on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
) {
    let uppercase = match symbols {
        Some(list) => list
            .iter()
            .map(|symbol| symbol.to_uppercase())
            .collect::<Vec<String>>(),
        None => Vec::new(),
    };
    let symbols = match symbols {
        Some(list) => {
            if list.is_empty() {
                None
            } else {
                let tmp: &[String] = &uppercase;
                Some(tmp)
            }
        }
        None => None,
    };

    match market_type {
        MarketType::Spot => crawl_l2_snapshot_spot(market_type, symbols, on_msg),
        MarketType::InverseFuture => crawl_l2_snapshot_inverse_future(market_type, symbols, on_msg),
        MarketType::LinearSwap => crawl_l2_snapshot_linear_swap(market_type, symbols, on_msg),
        MarketType::InverseSwap => crawl_l2_snapshot_inverse_swap(market_type, symbols, on_msg),
        MarketType::Option => crawl_l2_snapshot_linear_option(market_type, symbols, on_msg),
        _ => panic!("Binance does NOT have the {} market type", market_type),
    }
}
