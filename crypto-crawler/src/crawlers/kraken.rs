use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use crate::{msg::Message, MarketType, MessageType};
use crypto_rest_client::*;
use crypto_ws_client::*;
use log::*;
use serde_json::Value;

const EXCHANGE_NAME: &str = "Kraken";

fn extract_symbol(json: &str) -> String {
    let arr = serde_json::from_str::<Vec<Value>>(&json).unwrap();
    arr[3].as_str().unwrap().to_string()
}

fn check_args(market_type: MarketType, _symbols: &[String]) {
    if market_type != MarketType::Spot {
        error!("Kraken has only Spot market");
        panic!("Kraken has only Spot market");
    }
}

#[rustfmt::skip]
gen_crawl_event!(crawl_trade, market_type, symbols, on_msg, duration, KrakenWSClient, MessageType::Trade, subscribe_trade);
#[rustfmt::skip]
gen_crawl_event!(crawl_l2_event, market_type, symbols, on_msg, duration, KrakenWSClient, MessageType::L2Event, subscribe_orderbook);
#[rustfmt::skip]
gen_crawl_snapshot!(crawl_l2_snapshot, market_type, symbols, on_msg, duration, MessageType::L2Snapshot, KrakenRestClient::fetch_l2_snapshot);
