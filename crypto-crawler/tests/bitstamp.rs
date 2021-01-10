use crypto_crawler::*;
use std::sync::{Arc, Mutex};

#[macro_use]
mod utils;

#[test]
fn test_crawl_trade() {
    gen_test_code!(
        crawl_trade,
        "Bitstamp",
        MarketType::Spot,
        "btcusd",
        MessageType::Trade
    )
}

#[test]
fn test_crawl_l2_event() {
    gen_test_code!(
        crawl_l2_event,
        "Bitstamp",
        MarketType::Spot,
        "btcusd",
        MessageType::L2Event
    )
}

#[test]
fn test_crawl_l2_snapshot() {
    gen_test_code!(
        crawl_l2_snapshot,
        "Bitstamp",
        MarketType::Spot,
        "btcusd",
        MessageType::L2Snapshot
    )
}

#[test]
fn test_crawl_l3_event() {
    gen_test_code!(
        crawl_l3_event,
        "Bitstamp",
        MarketType::Spot,
        "btcusd",
        MessageType::L3Event
    )
}

#[test]
fn test_crawl_l3_snapshot() {
    gen_test_code!(
        crawl_l3_snapshot,
        "Bitstamp",
        MarketType::Spot,
        "btcusd",
        MessageType::L3Snapshot
    )
}
