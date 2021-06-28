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

#[test_case(MarketType::Spot, "BTCUSDT")]
#[test_case(MarketType::InverseFuture, "BTCUSD_210924")]
#[test_case(MarketType::LinearFuture, "BTCUSDT_210924")]
#[test_case(MarketType::InverseSwap, "BTCUSD_PERP")]
#[test_case(MarketType::LinearSwap, "BTCUSDT")]
#[test_case(MarketType::EuropeanOption, "BTC-210129-40000-C"; "inconclusive")]
fn test_crawl_trade(market_type: MarketType, symbol: &str) {
    gen_test_code!(
        crawl_trade,
        EXCHANGE_NAME,
        market_type,
        symbol,
        MessageType::Trade
    )
}

#[test_case(MarketType::Spot, "BTCUSDT")]
#[test_case(MarketType::InverseFuture, "BTCUSD_210924")]
#[test_case(MarketType::LinearFuture, "BTCUSDT_210924")]
#[test_case(MarketType::InverseSwap, "BTCUSD_PERP")]
#[test_case(MarketType::LinearSwap, "BTCUSDT")]
#[test_case(MarketType::EuropeanOption, "BTC-210129-40000-C"; "inconclusive")]
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
#[test_case(MarketType::InverseFuture, "BTCUSD_210924")]
#[test_case(MarketType::LinearFuture, "BTCUSDT_210924")]
#[test_case(MarketType::InverseSwap, "BTCUSD_PERP")]
#[test_case(MarketType::LinearSwap, "BTCUSDT")]
#[test_case(MarketType::EuropeanOption, "BTC-210129-40000-C"; "inconclusive")]
fn test_crawl_l2_snapshot(market_type: MarketType, symbol: &str) {
    gen_test_snapshot_code!(
        crawl_l2_snapshot,
        EXCHANGE_NAME,
        market_type,
        symbol,
        MessageType::L2Snapshot
    )
}

#[test_case(MarketType::Spot, "BTCUSDT")]
#[test_case(MarketType::InverseFuture, "BTCUSD_210924")]
#[test_case(MarketType::LinearFuture, "BTCUSDT_210924")]
#[test_case(MarketType::InverseSwap, "BTCUSD_PERP")]
#[test_case(MarketType::LinearSwap, "BTCUSDT")]
#[test_case(MarketType::EuropeanOption, "BTC-210129-40000-C"; "inconclusive")]
fn test_crawl_ticker(market_type: MarketType, symbol: &str) {
    gen_test_code!(
        crawl_ticker,
        EXCHANGE_NAME,
        market_type,
        symbol,
        MessageType::Ticker
    )
}

#[test_case(MarketType::InverseSwap, "BTCUSD_PERP")]
#[test_case(MarketType::LinearSwap, "BTCUSDT")]
fn test_crawl_funding_rate(market_type: MarketType, symbol: &str) {
    gen_test_code!(
        crawl_funding_rate,
        EXCHANGE_NAME,
        market_type,
        symbol,
        MessageType::FundingRate
    )
}
