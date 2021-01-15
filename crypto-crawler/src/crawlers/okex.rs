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

const EXCHANGE_NAME: &str = "okex";

fn extract_symbol(json: &str) -> String {
    let obj = serde_json::from_str::<HashMap<String, Value>>(&json).unwrap();
    let arr = obj.get("data").unwrap().as_array().unwrap();
    let symbol = arr[0]
        .as_object()
        .unwrap()
        .get("instrument_id")
        .unwrap()
        .as_str()
        .unwrap();
    symbol.to_string()
}

gen_check_args!(EXCHANGE_NAME);

#[rustfmt::skip]
gen_crawl_event!(crawl_trade, market_type, symbols, on_msg, duration, OkexWSClient, MessageType::Trade, subscribe_trade, true);
#[rustfmt::skip]
gen_crawl_event!(crawl_l2_event, market_type, symbols, on_msg, duration, OkexWSClient, MessageType::L2Event, subscribe_orderbook, true);
#[rustfmt::skip]
gen_crawl_snapshot!(crawl_l2_snapshot, market_type, symbols, on_msg, MessageType::L2Snapshot, OkexRestClient::fetch_l2_snapshot);
