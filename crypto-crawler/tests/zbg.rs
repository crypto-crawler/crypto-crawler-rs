#[macro_use]
mod utils;

use test_case::test_case;

use crypto_crawler::*;
use crypto_markets::MarketType;
use std::thread_local;
use std::{
    cell::RefCell,
    sync::{Arc, Mutex},
};

const EXCHANGE_NAME: &str = "zbg";

#[test_case(MarketType::Spot, "btc_usdt")]
#[test_case(MarketType::InverseSwap, "BTC_USD-R")]
#[test_case(MarketType::LinearSwap, "BTC_USDT")]
fn test_crawl_trade(market_type: MarketType, symbol: &str) {
    gen_test_code!(
        crawl_trade,
        EXCHANGE_NAME,
        market_type,
        symbol,
        MessageType::Trade
    )
}

#[test_case(MarketType::Spot, "btc_usdt")]
#[test_case(MarketType::InverseSwap, "BTC_USD-R")]
#[test_case(MarketType::LinearSwap, "BTC_USDT")]
fn test_crawl_l2_event(market_type: MarketType, symbol: &str) {
    gen_test_code!(
        crawl_l2_event,
        EXCHANGE_NAME,
        market_type,
        symbol,
        MessageType::L2Event
    )
}

#[test_case(MarketType::Spot, "btc_usdt")]
#[test_case(MarketType::InverseSwap, "BTC_USD-R")]
#[test_case(MarketType::LinearSwap, "BTC_USDT")]
fn test_crawl_l2_snapshot(market_type: MarketType, symbol: &str) {
    gen_test_snapshot_code!(
        crawl_l2_snapshot,
        EXCHANGE_NAME,
        market_type,
        symbol,
        MessageType::L2Snapshot
    )
}
