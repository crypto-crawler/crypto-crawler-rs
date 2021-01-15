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
        "kraken",
        MarketType::Spot,
        "XBT/USD",
        MessageType::Trade
    )
}

#[test]
fn test_crawl_l2_event() {
    gen_test_code!(
        crawl_l2_event,
        "kraken",
        MarketType::Spot,
        "XBT/USD",
        MessageType::L2Event
    )
}

#[test]
fn test_crawl_l2_snapshot() {
    gen_test_snapshot_code!(
        crawl_l2_snapshot,
        "kraken",
        MarketType::Spot,
        "XBT/USD",
        MessageType::L2Snapshot
    )
}
