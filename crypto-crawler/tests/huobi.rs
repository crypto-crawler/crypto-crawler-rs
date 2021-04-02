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

const EXCHANGE_NAME: &str = "huobi";

#[test_case(MarketType::Spot, "btcusdt")]
#[test_case(MarketType::InverseFuture, "BTC_CQ")]
#[test_case(MarketType::InverseSwap, "BTC-USD")]
#[test_case(MarketType::LinearSwap, "BTC-USDT")]
#[test_case(MarketType::Option, "BTC-USDT-210409-C-66000")]
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
#[test_case(MarketType::InverseFuture, "BTC_CQ")]
#[test_case(MarketType::InverseSwap, "BTC-USD")]
#[test_case(MarketType::LinearSwap, "BTC-USDT")]
#[test_case(MarketType::Option, "BTC-USDT-210409-C-66000")]
fn test_crawl_l2_event(market_type: MarketType, symbol: &str) {
    gen_test_code!(
        crawl_l2_event,
        EXCHANGE_NAME,
        market_type,
        symbol,
        MessageType::L2Event
    )
}

#[test_case(MarketType::Spot, "btcusdt")]
#[test_case(MarketType::InverseFuture, "BTC_CQ")]
#[test_case(MarketType::InverseSwap, "BTC-USD")]
#[test_case(MarketType::LinearSwap, "BTC-USDT")]
#[test_case(MarketType::Option, "BTC-USDT-210409-C-66000")]
fn test_crawl_l2_snapshot(market_type: MarketType, symbol: &str) {
    gen_test_snapshot_code!(
        crawl_l2_snapshot,
        EXCHANGE_NAME,
        market_type,
        symbol,
        MessageType::L2Snapshot
    )
}

#[test_case(MarketType::InverseSwap, "BTC-USD")]
#[test_case(MarketType::LinearSwap, "BTC-USD")]
fn test_crawl_funding_rate(market_type: MarketType, symbol: &str) {
    gen_test_code!(
        crawl_funding_rate,
        EXCHANGE_NAME,
        market_type,
        symbol,
        MessageType::FundingRate
    )
}
