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

use lazy_static::lazy_static;
use log::*;
use reqwest::{header, Result};
use serde_json::Value;

const EXCHANGE_NAME: &str = "zbg";
// usize::MAX means unlimited
const MAX_SUBSCRIPTIONS_PER_CONNECTION: usize = usize::MAX;

lazy_static! {
    static ref CONTRACT_ID_SYMBOL_MAP: HashMap<i64, String> = fetch_contract_id_symbol_map_swap();
}

fn http_get(url: &str) -> Result<String> {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/json"),
    );

    let client = reqwest::blocking::Client::builder()
         .default_headers(headers)
         .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/87.0.4280.88 Safari/537.36")
         .gzip(true)
         .build()?;
    let response = client.get(url).send()?;

    match response.error_for_status() {
        Ok(resp) => Ok(resp.text()?),
        Err(error) => Err(error),
    }
}

// See https://zbgapi.github.io/docs/future/v1/en/#public-get-contracts
fn fetch_contract_id_symbol_map_swap() -> HashMap<i64, String> {
    let mut contract_id_symbol_map: HashMap<i64, String> = HashMap::new();

    if let Ok(txt) = http_get("https://www.zbg.com/exchange/api/v1/future/common/contracts") {
        if let Ok(obj) = serde_json::from_str::<HashMap<String, Value>>(&txt) {
            if obj
                .get("resMsg")
                .unwrap()
                .as_object()
                .unwrap()
                .get("code")
                .unwrap()
                .as_str()
                .unwrap()
                == "1"
            {
                let arr = obj.get("datas").unwrap().as_array().unwrap();
                for v in arr.iter() {
                    let obj = v.as_object().unwrap();
                    let symbol = obj.get("symbol").unwrap().as_str().unwrap();
                    let contract_id = obj.get("contractId").unwrap().as_i64().unwrap();

                    contract_id_symbol_map.insert(contract_id, symbol.to_string());
                }
            }
        }
    }

    contract_id_symbol_map
}

fn extract_spot_symbol(json: &[Value]) -> String {
    let channel = json[0].as_str().unwrap();
    match channel {
        "T" | "E" => json[3].as_str().unwrap().to_string(),
        "AE" => json[2].as_str().unwrap().to_string(),
        _ => panic!("Unknown channel {}", channel),
    }
}

fn extract_symbol(json: &str) -> String {
    if json.starts_with(r#"["future_"#) {
        let arr = serde_json::from_str::<Vec<Value>>(&json).unwrap();
        if arr.len() == 2 {
            let contract_id = arr[1]
                .as_object()
                .unwrap()
                .get("contractId")
                .unwrap()
                .as_i64()
                .unwrap();
            CONTRACT_ID_SYMBOL_MAP
                .get(&contract_id)
                .unwrap_or_else(|| panic!("Failed to find symbol for {}", contract_id))
                .clone()
        } else {
            "".to_string()
        }
    } else if json.starts_with("[[") {
        let arr = serde_json::from_str::<Vec<Value>>(&json).unwrap();
        let arr = arr[0].as_array().unwrap();
        extract_spot_symbol(&arr)
    } else if json.starts_with('[') {
        let arr = serde_json::from_str::<Vec<Value>>(&json).unwrap();
        extract_spot_symbol(&arr)
    } else {
        panic!("Unknown format: {}", json)
    }
}

gen_check_args!(EXCHANGE_NAME);

#[rustfmt::skip]
gen_crawl_event!(crawl_trade_spot, ZbgSpotWSClient, MessageType::Trade, subscribe_trade);
#[rustfmt::skip]
gen_crawl_event!(crawl_trade_swap, ZbgSwapWSClient, MessageType::Trade, subscribe_trade);
#[rustfmt::skip]
gen_crawl_event!(crawl_l2_event_spot, ZbgSpotWSClient, MessageType::L2Event, subscribe_orderbook);
#[rustfmt::skip]
gen_crawl_event!(crawl_l2_event_swap, ZbgSwapWSClient, MessageType::L2Event, subscribe_orderbook);

#[rustfmt::skip]
gen_crawl_snapshot!(crawl_l2_snapshot_spot, MessageType::L2Snapshot, ZbgSpotRestClient::fetch_l2_snapshot);
#[rustfmt::skip]
gen_crawl_snapshot!(crawl_l2_snapshot_swap, MessageType::L2Snapshot, ZbgSwapRestClient::fetch_l2_snapshot);

pub(crate) fn crawl_trade(
    market_type: MarketType,
    symbols: Option<&[String]>,
    on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
    duration: Option<u64>,
) -> Option<std::thread::JoinHandle<()>> {
    match market_type {
        MarketType::Spot => crawl_trade_spot(market_type, symbols, on_msg, duration),
        MarketType::InverseSwap | MarketType::LinearSwap => {
            crawl_trade_swap(market_type, symbols, on_msg, duration)
        }
        _ => panic!("ZBG does NOT have the {} market type", market_type),
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
        MarketType::InverseSwap | MarketType::LinearSwap => {
            crawl_l2_event_swap(market_type, symbols, on_msg, duration)
        }
        _ => panic!("ZBG does NOT have the {} market type", market_type),
    }
}

pub(crate) fn crawl_l2_snapshot(
    market_type: MarketType,
    symbols: Option<&[String]>,
    on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
    interval: Option<u64>,
    duration: Option<u64>,
) {
    let func = match market_type {
        MarketType::Spot => crawl_l2_snapshot_spot,
        MarketType::InverseSwap | MarketType::LinearSwap => crawl_l2_snapshot_swap,
        _ => panic!("ZBG does NOT have the {} market type", market_type),
    };
    func(market_type, symbols, on_msg, interval, duration);
}
