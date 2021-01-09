use crypto_crawler::*;
use std::{cell::RefCell, rc::Rc};

#[macro_use]
mod utils;

#[test]
fn test_crawl_trade() {
    gen_test_code!(
        crawl_trade,
        "Kraken",
        MarketType::Spot,
        "XBT/USD",
        MessageType::Trade
    )
}

#[test]
fn test_crawl_l2_event() {
    gen_test_code!(
        crawl_l2_event,
        "Kraken",
        MarketType::Spot,
        "XBT/USD",
        MessageType::L2Event
    )
}

#[test]
fn test_crawl_l2_snapshot() {
    gen_test_code!(
        crawl_l2_snapshot,
        "Kraken",
        MarketType::Spot,
        "XBT/USD",
        MessageType::L2Snapshot
    )
}
