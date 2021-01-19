use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};

use std::{collections::HashMap, time::Duration};

use crate::{msg::Message, MessageType};
use crypto_markets::{fetch_symbols, MarketType};
use crypto_rest_client::*;
use crypto_ws_client::*;
use log::*;
use serde_json::Value;

const EXCHANGE_NAME: &str = "mxc";

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
gen_crawl_event!(crawl_trade_spot, MxcSpotWSClient, MessageType::Trade, subscribe_trade, true);
#[rustfmt::skip]
gen_crawl_event!(crawl_trade_swap, MxcSwapWSClient, MessageType::Trade, subscribe_trade, true);

#[rustfmt::skip]
gen_crawl_event!(crawl_l2_event_spot, MxcSpotWSClient, MessageType::L2Event, subscribe_orderbook, true);
#[rustfmt::skip]
gen_crawl_event!(crawl_l2_event_swap, MxcSwapWSClient, MessageType::L2Event, subscribe_orderbook, true);

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
) {
    let real_symbols = match symbols {
        Some(list) => {
            if list.is_empty() {
                fetch_symbols(EXCHANGE_NAME, market_type).unwrap()
            } else {
                check_args(market_type, &list);
                symbols.unwrap().to_vec()
            }
        }
        None => fetch_symbols(EXCHANGE_NAME, market_type).unwrap(),
    };

    for symbol in real_symbols.iter() {
        let resp = match market_type {
            MarketType::Spot => {
                let access_key = std::env::var("MXC_ACCESS_KEY").unwrap();
                let client = MxcSpotRestClient::new(access_key, None);
                client.fetch_l2_snapshot(symbol)
            }
            MarketType::LinearSwap | MarketType::InverseSwap => {
                MxcSwapRestClient::fetch_l2_snapshot(symbol)
            }
            _ => {
                error!("Unknown market type {} of {}", market_type, EXCHANGE_NAME);
                panic!("Unknown market type {} of {}", market_type, EXCHANGE_NAME);
            }
        };
        match resp {
            Ok(msg) => {
                let message = Message::new(
                    EXCHANGE_NAME.to_string(),
                    market_type,
                    symbol.to_string(),
                    MessageType::L2Snapshot,
                    msg,
                );
                (on_msg.lock().unwrap())(message);
            }
            Err(err) => error!(
                "{} {} {}, error: {}",
                EXCHANGE_NAME, market_type, symbol, err
            ),
        }
    }
}
