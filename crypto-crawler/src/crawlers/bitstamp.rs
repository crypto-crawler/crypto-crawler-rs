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

const EXCHANGE_NAME: &str = "bitstamp";

fn extract_symbol(json: &str) -> String {
    let obj = serde_json::from_str::<HashMap<String, Value>>(&json).unwrap();
    let channel = obj.get("channel").unwrap().as_str().unwrap();
    let pos = channel.rfind('_').unwrap();
    (&channel[(pos + 1)..]).to_string()
}

gen_check_args!(EXCHANGE_NAME);

#[rustfmt::skip]
gen_crawl_event!(crawl_trade, market_type, symbols, on_msg, duration, BitstampWSClient, MessageType::Trade, subscribe_trade, true);
#[rustfmt::skip]
gen_crawl_event!(crawl_l2_event, market_type, symbols, on_msg, duration, BitstampWSClient, MessageType::L2Event, subscribe_orderbook, true);
#[rustfmt::skip]
gen_crawl_event!(crawl_l3_event, market_type, symbols, on_msg, duration, BitstampWSClient, MessageType::L3Event, subscribe_l3_orderbook, true);

#[rustfmt::skip]
gen_crawl_snapshot!(crawl_l2_snapshot, market_type, symbols, on_msg, MessageType::L2Snapshot, BitstampRestClient::fetch_l2_snapshot);
#[rustfmt::skip]
gen_crawl_snapshot!(crawl_l3_snapshot, market_type, symbols, on_msg, MessageType::L3Snapshot, BitstampRestClient::fetch_l3_snapshot);
