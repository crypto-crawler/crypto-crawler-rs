use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc, Mutex,
    },
};

use std::time::Duration;

use super::utils::{check_args, fetch_symbols_retry};
use crate::{msg::Message, MessageType};
use crypto_markets::MarketType;
use crypto_ws_client::*;
use log::*;
use serde_json::Value;

const EXCHANGE_NAME: &str = "gate";
// usize::MAX means unlimited
const MAX_SUBSCRIPTIONS_PER_CONNECTION: usize = usize::MAX;

fn extract_symbol(json: &str) -> String {
    let obj = serde_json::from_str::<HashMap<String, Value>>(&json).unwrap();
    if obj.contains_key("method") {
        // spot
        let method = obj.get("method").unwrap().as_str().unwrap();
        let params = obj.get("params").unwrap().as_array().unwrap();
        match method {
            "trades.update" | "ticker.update" => {
                if !params.is_empty() {
                    params[0].as_str().unwrap().to_string()
                } else {
                    "".to_string()
                }
            }
            "depth.update" => {
                if params.len() >= 3 {
                    params[2].as_str().unwrap().to_string()
                } else {
                    "".to_string()
                }
            }
            "kline.update" => {
                if !params.is_empty() {
                    params[0].as_array().unwrap()[7]
                        .as_str()
                        .unwrap()
                        .to_string()
                } else {
                    "".to_string()
                }
            }
            _ => panic!("Unsupported method {}", method),
        }
    } else if obj.contains_key("channel") {
        // future and swap
        let channel = obj.get("channel").unwrap().as_str().unwrap();
        match channel {
            "futures.trades" | "futures.tickers" => {
                let result = obj.get("result").unwrap().as_array().unwrap();
                if !result.is_empty() {
                    result[0]
                        .get("contract")
                        .unwrap()
                        .as_str()
                        .unwrap()
                        .to_string()
                } else {
                    "".to_string()
                }
            }
            "futures.order_book" => {
                let result = obj.get("result").unwrap().as_object().unwrap();
                result
                    .get("contract")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .to_string()
            }
            "futures.candlesticks" => {
                let result = obj.get("result").unwrap().as_array().unwrap();
                if !result.is_empty() {
                    let x = result[0].as_object().unwrap();
                    let n = x.get("n").unwrap().as_str().unwrap();
                    let pos = n.find('_').unwrap();
                    (&n[(pos + 1)..]).to_string()
                } else {
                    "".to_string()
                }
            }
            _ => panic!("Unsupported channel {}", channel),
        }
    } else {
        "".to_string()
    }
}

#[rustfmt::skip]
gen_crawl_event!(crawl_trade_spot, GateSpotWSClient, MessageType::Trade, subscribe_trade);
#[rustfmt::skip]
gen_crawl_event!(crawl_trade_inverse_swap, GateInverseSwapWSClient, MessageType::Trade, subscribe_trade);
#[rustfmt::skip]
gen_crawl_event!(crawl_trade_linear_swap, GateLinearSwapWSClient, MessageType::Trade, subscribe_trade);
#[rustfmt::skip]
gen_crawl_event!(crawl_trade_linear_future, GateLinearFutureWSClient, MessageType::Trade, subscribe_trade);
#[rustfmt::skip]
gen_crawl_event!(crawl_l2_event_spot, GateSpotWSClient, MessageType::L2Event, subscribe_orderbook);
#[rustfmt::skip]
gen_crawl_event!(crawl_l2_event_inverse_swap, GateInverseSwapWSClient, MessageType::L2Event, subscribe_orderbook);
#[rustfmt::skip]
gen_crawl_event!(crawl_l2_event_linear_swap, GateLinearSwapWSClient, MessageType::L2Event, subscribe_orderbook);
#[rustfmt::skip]
gen_crawl_event!(crawl_l2_event_linear_future, GateLinearFutureWSClient, MessageType::L2Event, subscribe_orderbook);

pub(crate) fn crawl_trade(
    market_type: MarketType,
    symbols: Option<&[String]>,
    on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
    duration: Option<u64>,
) -> Option<std::thread::JoinHandle<()>> {
    match market_type {
        MarketType::Spot => crawl_trade_spot(market_type, symbols, on_msg, duration),
        MarketType::InverseSwap => crawl_trade_inverse_swap(market_type, symbols, on_msg, duration),
        MarketType::LinearSwap => crawl_trade_linear_swap(market_type, symbols, on_msg, duration),
        MarketType::LinearFuture => {
            crawl_trade_linear_future(market_type, symbols, on_msg, duration)
        }
        _ => panic!("Gate does NOT have the {} market type", market_type),
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
        MarketType::InverseSwap => {
            crawl_l2_event_inverse_swap(market_type, symbols, on_msg, duration)
        }
        MarketType::LinearSwap => {
            crawl_l2_event_linear_swap(market_type, symbols, on_msg, duration)
        }
        MarketType::LinearFuture => {
            crawl_l2_event_linear_future(market_type, symbols, on_msg, duration)
        }
        _ => panic!("Gate does NOT have the {} market type", market_type),
    }
}
