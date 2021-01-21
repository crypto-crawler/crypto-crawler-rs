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

const EXCHANGE_NAME: &str = "coinbase_pro";
// usize::MAX means unlimited
const MAX_SUBSCRIPTIONS_PER_CONNECTION: usize = usize::MAX;

fn extract_symbol(json: &str) -> String {
    let obj = serde_json::from_str::<HashMap<String, Value>>(&json).unwrap();
    obj.get("product_id").unwrap().as_str().unwrap().to_string()
}

gen_check_args!(EXCHANGE_NAME);

#[rustfmt::skip]
gen_crawl_event!(crawl_trade, CoinbaseProWSClient, MessageType::Trade, subscribe_trade);
#[rustfmt::skip]
gen_crawl_event!(crawl_l2_event, CoinbaseProWSClient, MessageType::L2Event, subscribe_orderbook);
#[rustfmt::skip]
gen_crawl_event!(crawl_l3_event, CoinbaseProWSClient, MessageType::L3Event, subscribe_l3_orderbook);
#[rustfmt::skip]
gen_crawl_snapshot!(crawl_l2_snapshot, MessageType::L2Snapshot, CoinbaseProRestClient::fetch_l2_snapshot);
#[rustfmt::skip]
gen_crawl_snapshot!(crawl_l3_snapshot, MessageType::L3Snapshot, CoinbaseProRestClient::fetch_l3_snapshot);
