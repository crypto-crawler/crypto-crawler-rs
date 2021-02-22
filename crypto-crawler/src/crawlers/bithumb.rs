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

const EXCHANGE_NAME: &str = "bithumb";
// usize::MAX means unlimited
const MAX_SUBSCRIPTIONS_PER_CONNECTION: usize = usize::MAX;

fn extract_symbol(json: &str) -> String {
    let obj = serde_json::from_str::<HashMap<String, Value>>(&json).unwrap();
    let data = obj.get("data").unwrap();
    if data.is_array() {
        let arr = data.as_array().unwrap();
        debug_assert!(arr.len() > 0);
        let symbol = arr[0]
            .as_object()
            .unwrap()
            .get("symbol")
            .unwrap()
            .as_str()
            .unwrap();
        symbol.to_string()
    } else {
        data.get("symbol").unwrap().as_str().unwrap().to_string()
    }
}

gen_check_args!(EXCHANGE_NAME);

#[rustfmt::skip]
gen_crawl_event!(crawl_trade, BithumbWSClient, MessageType::Trade, subscribe_trade);
#[rustfmt::skip]
gen_crawl_event!(crawl_l2_event, BithumbWSClient, MessageType::L2Event, subscribe_orderbook);
#[rustfmt::skip]
gen_crawl_snapshot!(crawl_l2_snapshot, MessageType::L2Snapshot, BithumbRestClient::fetch_l2_snapshot);
