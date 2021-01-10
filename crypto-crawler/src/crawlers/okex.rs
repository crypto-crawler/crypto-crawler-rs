use std::sync::{Arc, Mutex};

use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use crate::{msg::Message, MarketType, MessageType};
use crypto_rest_client::*;
use crypto_ws_client::*;
use log::*;
use serde_json::Value;

const EXCHANGE_NAME: &str = "OKEx";

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

fn check_args(_market_type: MarketType, _symbols: &[String]) {
    // TODO: add more check
}

#[rustfmt::skip]
gen_crawl_event!(crawl_trade, market_type, symbols, on_msg, duration, OKExWSClient, MessageType::Trade, subscribe_trade);
#[rustfmt::skip]
gen_crawl_event!(crawl_l2_event, market_type, symbols, on_msg, duration, OKExWSClient, MessageType::L2Event, subscribe_orderbook);
#[rustfmt::skip]
gen_crawl_snapshot!(crawl_l2_snapshot, market_type, symbols, on_msg, duration, MessageType::L2Snapshot, OKExRestClient::fetch_l2_snapshot);
