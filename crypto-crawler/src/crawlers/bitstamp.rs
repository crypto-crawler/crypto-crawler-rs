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

const EXCHANGE_NAME: &str = "Bitstamp";

fn extract_symbol(json: &str) -> String {
    let obj = serde_json::from_str::<HashMap<String, Value>>(&json).unwrap();
    let channel = obj.get("channel").unwrap().as_str().unwrap();
    let pos = channel.rfind('_').unwrap();
    (&channel[(pos + 1)..]).to_string()
}

fn check_args(market_type: MarketType, _symbols: &[String]) {
    if market_type != MarketType::Spot {
        error!("Bitstamp has only Spot market");
        panic!("Bitstamp has only Spot market");
    }
}

#[rustfmt::skip]
gen_crawl_event!(crawl_trade, market_type, symbols, on_msg, duration, BitstampWSClient, MessageType::Trade, subscribe_trade);
#[rustfmt::skip]
gen_crawl_event!(crawl_l2_event, market_type, symbols, on_msg, duration, BitstampWSClient, MessageType::L2Event, subscribe_orderbook);
#[rustfmt::skip]
gen_crawl_event!(crawl_l3_event, market_type, symbols, on_msg, duration, BitstampWSClient, MessageType::L3Event, subscribe_l3_orderbook);

#[rustfmt::skip]
gen_crawl_snapshot!(crawl_l2_snapshot, market_type, symbols, on_msg, duration, MessageType::L2Snapshot, BitstampRestClient::fetch_l2_snapshot);
#[rustfmt::skip]
gen_crawl_snapshot!(crawl_l3_snapshot, market_type, symbols, on_msg, duration, MessageType::L3Snapshot, BitstampRestClient::fetch_l3_snapshot);
