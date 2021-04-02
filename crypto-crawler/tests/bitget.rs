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

const EXCHANGE_NAME: &str = "bitget";

#[test_case(MarketType::InverseSwap, "btcusd")]
#[test_case(MarketType::LinearSwap, "cmt_btcusdt")]
fn test_crawl_trade(market_type: MarketType, symbol: &str) {
    gen_test_code!(
        crawl_trade,
        EXCHANGE_NAME,
        market_type,
        symbol,
        MessageType::Trade
    )
}

#[test_case(MarketType::InverseSwap, "btcusd")]
#[test_case(MarketType::LinearSwap, "cmt_btcusdt")]
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
#[test_case(MarketType::InverseSwap, "btcusd")]
#[test_case(MarketType::LinearSwap, "cmt_btcusdt")]
fn test_crawl_l2_snapshot(market_type: MarketType, symbol: &str) {
    gen_test_snapshot_code!(
        crawl_l2_snapshot,
        EXCHANGE_NAME,
        market_type,
        symbol,
        MessageType::L2Snapshot
    )
}

#[test_case(MarketType::InverseSwap, "btcusd")]
#[test_case(MarketType::LinearSwap, "cmt_btcusdt")]
fn test_crawl_funding_rate(market_type: MarketType, symbol: &str) {
    gen_test_code!(
        crawl_funding_rate,
        EXCHANGE_NAME,
        market_type,
        symbol,
        MessageType::FundingRate
    )
}
