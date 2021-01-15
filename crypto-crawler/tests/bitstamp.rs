use crypto_crawler::*;
use crypto_markets::MarketType;
use std::thread_local;
use std::{
    cell::RefCell,
    sync::{Arc, Mutex},
};

#[macro_use]
mod utils;

#[test]
fn test_crawl_trade() {
    gen_test_code!(
        crawl_trade,
        "bitstamp",
        MarketType::Spot,
        "btcusd",
        MessageType::Trade
    )
}

#[test]
fn test_crawl_l2_event() {
    gen_test_code!(
        crawl_l2_event,
        "bitstamp",
        MarketType::Spot,
        "btcusd",
        MessageType::L2Event
    )
}

#[test]
fn test_crawl_l2_snapshot() {
    gen_test_snapshot_code!(
        crawl_l2_snapshot,
        "bitstamp",
        MarketType::Spot,
        "btcusd",
        MessageType::L2Snapshot
    )
}

#[test]
fn test_crawl_l3_event() {
    gen_test_code!(
        crawl_l3_event,
        "bitstamp",
        MarketType::Spot,
        "btcusd",
        MessageType::L3Event
    )
}

#[test]
fn test_crawl_l3_snapshot() {
    gen_test_snapshot_code!(
        crawl_l3_snapshot,
        "bitstamp",
        MarketType::Spot,
        "btcusd",
        MessageType::L3Snapshot
    )
}
