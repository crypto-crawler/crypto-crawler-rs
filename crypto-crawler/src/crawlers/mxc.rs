use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use crate::{msg::Message, MarketType, MessageType};
use crypto_rest_client::*;
use crypto_ws_client::*;
use log::*;
use serde_json::Value;

const EXCHANGE_NAME: &str = "MXC";

fn extract_symbol(json: &str) -> String {
    if json.starts_with('[') {
        let arr = serde_json::from_str::<Vec<Value>>(&json).unwrap();
        arr[1].get("symbol").unwrap().as_str().unwrap().to_string()
    } else {
        let obj = serde_json::from_str::<HashMap<String, Value>>(&json).unwrap();
        obj.get("symbol").unwrap().as_str().unwrap().to_string()
    }
}

fn check_args(market_type: MarketType, _symbols: &[String]) {
    if market_type != MarketType::Spot && market_type != MarketType::Swap {
        error!("MXC has only Spot and Swap markets");
        panic!("MXC has only Spot and Swap markets");
    }
}

#[rustfmt::skip]
gen_crawl_event!(crawl_trade_spot, market_type, symbols, on_msg, duration, MXCSpotWSClient, MessageType::Trade, subscribe_trade);
#[rustfmt::skip]
gen_crawl_event!(crawl_l2_event_spot, market_type, symbols, on_msg, duration, MXCSpotWSClient, MessageType::L2Event, subscribe_orderbook);
#[rustfmt::skip]
gen_crawl_event!(crawl_l2_event_swap, market_type, symbols, on_msg, duration, MXCSwapWSClient, MessageType::L2Event, subscribe_orderbook);
#[rustfmt::skip]
gen_crawl_event!(crawl_trade_swap, market_type, symbols, on_msg, duration, MXCSwapWSClient, MessageType::Trade, subscribe_trade);

pub(crate) fn crawl_trade<'a>(
    market_type: MarketType,
    symbols: &[String],
    on_msg: Box<dyn FnMut(Message) + 'a>,
    duration: Option<u64>,
) {
    check_args(market_type, symbols);
    match market_type {
        MarketType::Spot => crawl_trade_spot(market_type, symbols, on_msg, duration),
        MarketType::Swap => crawl_trade_swap(market_type, symbols, on_msg, duration),
        _ => {
            error!("Unknown market type {} of MXC", market_type);
            panic!("Unknown market type {} of MXC", market_type);
        }
    }
}

pub(crate) fn crawl_l2_event<'a>(
    market_type: MarketType,
    symbols: &[String],
    on_msg: Box<dyn FnMut(Message) + 'a>,
    duration: Option<u64>,
) {
    check_args(market_type, symbols);
    match market_type {
        MarketType::Spot => crawl_l2_event_spot(market_type, symbols, on_msg, duration),
        MarketType::Swap => crawl_l2_event_swap(market_type, symbols, on_msg, duration),
        _ => {
            error!("Unknown market type {} of MXC", market_type);
            panic!("Unknown market type {} of MXC", market_type);
        }
    }
}

pub(crate) fn crawl_l2_snapshot<'a>(
    market_type: MarketType,
    symbols: &[String],
    mut on_msg: Box<dyn FnMut(Message) + 'a>,
    duration: Option<u64>,
) {
    check_args(market_type, symbols);
    if market_type == MarketType::Spot {
        if let Err(_) = std::env::var("MXC_ACCESS_KEY") {
            error!("MXC Spot REST APIs require access key, please set it to the MXC_ACCESS_KEY environment variable");
            panic!("MXC Spot REST APIs require access key, please set it to the MXC_ACCESS_KEY environment variable");
        }
    }

    let mut on_msg_ext = |json: String, symbol: String| {
        let message = Message::new(
            EXCHANGE_NAME.to_string(),
            market_type,
            symbol,
            MessageType::L2Snapshot,
            json,
        );
        on_msg(message);
    };

    let now = Instant::now();
    loop {
        let mut succeeded = false;
        for symbol in symbols.iter() {
            let resp = match market_type {
                MarketType::Spot => {
                    let access_key = std::env::var("MXC_ACCESS_KEY").unwrap();
                    let client = MXCSpotRestClient::new(access_key, None);
                    client.fetch_l2_snapshot(symbol)
                }
                MarketType::Swap => MXCSwapRestClient::fetch_l2_snapshot(symbol),
                _ => {
                    error!("Unknown market type {} of MXC", market_type);
                    panic!("Unknown market type {} of MXC", market_type);
                }
            };
            match resp {
                Ok(msg) => {
                    on_msg_ext(msg, symbol.to_string());
                    succeeded = true
                }
                Err(err) => error!(
                    "{} {} {}, error: {}",
                    EXCHANGE_NAME, market_type, symbol, err
                ),
            }
        }

        if let Some(seconds) = duration {
            if now.elapsed() > Duration::from_secs(seconds) && succeeded {
                break;
            }
        }

        std::thread::sleep(Duration::from_secs(crate::SNAPSHOT_INTERVAL));
    }
}
