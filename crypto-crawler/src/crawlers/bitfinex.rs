use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use std::time::Duration;

use crate::{msg::Message, MessageType};
use crypto_markets::{fetch_symbols, get_market_types, MarketType};
use crypto_rest_client::*;
use crypto_ws_client::*;
use log::*;
use serde_json::Value;

const EXCHANGE_NAME: &str = "bitfinex";
// Each websocket connection has a limit of 30 subscriptions to public market
// data feed channels, see https://docs.bitfinex.com/docs/ws-general
const MAX_CHANNELS: usize = 30;

fn extract_symbol(json: &str) -> String {
    let arr = serde_json::from_str::<Vec<Value>>(&json).unwrap();
    let obj = arr[0].as_object().unwrap();
    obj.get("symbol").unwrap().as_str().unwrap().to_string()
}

fn check_args(market_type: MarketType, symbols: &[String]) {
    let market_types = get_market_types(EXCHANGE_NAME);
    if !market_types.contains(&market_type) {
        panic!(
            "{} does NOT have the {} market type",
            EXCHANGE_NAME, market_type
        );
    }

    if symbols.len() > MAX_CHANNELS {
        error!("Each websocket connection has a limit of 30 subscriptions");
        panic!("Each websocket connection has a limit of 30 subscriptions");
    }

    let valid_symbols = fetch_symbols(EXCHANGE_NAME, market_type).unwrap();
    let invalid_symbols: Vec<String> = symbols
        .iter()
        .filter(|symbol| !valid_symbols.contains(symbol))
        .cloned()
        .collect();
    if !invalid_symbols.is_empty() {
        panic!(
            "Invalid symbols for {} {} market: {}, available trading symbols are {}",
            EXCHANGE_NAME,
            market_type,
            invalid_symbols.join(","),
            valid_symbols.join(",")
        );
    }
}

#[rustfmt::skip]
gen_crawl_event!(crawl_trade, BitfinexWSClient, MessageType::Trade, subscribe_trade, true);
#[rustfmt::skip]
gen_crawl_event!(crawl_l2_event, BitfinexWSClient, MessageType::L2Event, subscribe_orderbook, true);
#[rustfmt::skip]
gen_crawl_event!(crawl_l3_event, BitfinexWSClient, MessageType::L3Event, subscribe_l3_orderbook, true);
#[rustfmt::skip]
gen_crawl_snapshot!(crawl_l2_snapshot, MessageType::L2Snapshot, BitfinexRestClient::fetch_l2_snapshot);
#[rustfmt::skip]
gen_crawl_snapshot!(crawl_l3_snapshot, MessageType::L3Snapshot, BitfinexRestClient::fetch_l3_snapshot);
