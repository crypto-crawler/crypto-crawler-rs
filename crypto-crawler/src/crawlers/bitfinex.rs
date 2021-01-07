use std::time::{Duration, Instant};

use crate::{msg::Message, MarketType, MessageType};
use crypto_rest_client::*;
use crypto_ws_client::*;
use log::*;
use serde_json::Value;

const EXCHANGE_NAME: &str = "Bitfinex";

fn extract_symbol(json: &str) -> String {
    let arr = serde_json::from_str::<Vec<Value>>(&json).unwrap();
    let obj = arr[0].as_object().unwrap();
    obj.get("symbol").unwrap().as_str().unwrap().to_string()
}

fn check_args(market_type: MarketType, _symbols: &[String]) {
    if market_type != MarketType::Spot && market_type != MarketType::Swap {
        error!("Bitfinex has only Spot and Swap markets");
        panic!("Bitfinex has only Spot and Swap markets");
    }
}

#[rustfmt::skip]
gen_crawl_event!(crawl_trade, market_type, symbols, on_msg, duration, BitfinexWSClient, MessageType::Trade, subscribe_trade);
#[rustfmt::skip]
gen_crawl_event!(crawl_l2_event, market_type, symbols, on_msg, duration, BitfinexWSClient, MessageType::L2Event, subscribe_orderbook);
#[rustfmt::skip]
gen_crawl_event!(crawl_l3_event, market_type, symbols, on_msg, duration, BitfinexWSClient, MessageType::L3Event, subscribe_l3_orderbook);
#[rustfmt::skip]
gen_crawl_snapshot!(crawl_l2_snapshot, market_type, symbols, on_msg, duration, MessageType::L2Snapshot, BitfinexRestClient::fetch_l2_snapshot);
#[rustfmt::skip]
gen_crawl_snapshot!(crawl_l3_snapshot, market_type, symbols, on_msg, duration, MessageType::L3Snapshot, BitfinexRestClient::fetch_l3_snapshot);
