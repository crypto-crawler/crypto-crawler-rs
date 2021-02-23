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

const EXCHANGE_NAME: &str = "bitmex";

// see <https://www.bitmex.com/app/wsAPI#Rate-Limits>
const MAX_SUBSCRIPTIONS_PER_CONNECTION: usize = 40;

fn extract_symbol(json: &str) -> String {
    let obj = serde_json::from_str::<HashMap<String, Value>>(&json).unwrap();

    let symbol_obj = if obj.contains_key("filter") {
        obj.get("filter").unwrap().as_object().unwrap()
    } else {
        let arr = obj.get("data").unwrap().as_array().unwrap();
        if arr.is_empty() {
            println!("{}", json);
        }
        arr[0].as_object().unwrap()
    };

    symbol_obj
        .get("symbol")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string()
}

#[rustfmt::skip]
gen_crawl_event!(crawl_trade, BitmexWSClient, MessageType::Trade, subscribe_trade);
#[rustfmt::skip]
gen_crawl_event!(crawl_l2_event, BitmexWSClient, MessageType::L2Event, subscribe_orderbook);
