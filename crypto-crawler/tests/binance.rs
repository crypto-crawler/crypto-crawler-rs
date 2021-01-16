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

const EXCHANGE_NAME: &str = "binance";

#[test_case(MarketType::Spot, "btcusdt")]
#[test_case(MarketType::InverseFuture, "btcusd_210625")]
#[test_case(MarketType::InverseSwap, "btcusd_perp")]
#[test_case(MarketType::LinearSwap, "btcusdt")]
#[test_case(MarketType::Option, "BTC-210129-40000-C"; "inconclusive")]
fn test_crawl_trade(market_type: MarketType, symbol: &str) {
    gen_test_code!(
        crawl_trade,
        EXCHANGE_NAME,
        market_type,
        symbol,
        MessageType::Trade
    )
}

#[test_case(MarketType::Spot, "btcusdt")]
#[test_case(MarketType::InverseFuture, "btcusd_210625")]
#[test_case(MarketType::InverseSwap, "btcusd_perp")]
#[test_case(MarketType::LinearSwap, "btcusdt")]
#[test_case(MarketType::Option, "BTC-210129-40000-C"; "inconclusive")]
fn test_crawl_l2_event(market_type: MarketType, symbol: &str) {
    gen_test_code!(
        crawl_l2_event,
        EXCHANGE_NAME,
        market_type,
        symbol,
        MessageType::L2Event
    )
}

#[test_case(MarketType::Spot, "BTCUSDT")]
#[test_case(MarketType::InverseFuture, "BTCUSD_210625")]
#[test_case(MarketType::InverseSwap, "BTCUSD_PERP")]
#[test_case(MarketType::LinearSwap, "BTCUSDT")]
#[test_case(MarketType::Option, "BTC-210129-40000-C"; "inconclusive")]
fn test_crawl_l2_snapshot(market_type: MarketType, symbol: &str) {
    gen_test_snapshot_code!(
        crawl_l2_snapshot,
        EXCHANGE_NAME,
        market_type,
        symbol,
        MessageType::L2Snapshot
    )
}
