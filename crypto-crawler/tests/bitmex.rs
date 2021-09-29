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

const EXCHANGE_NAME: &str = "bitmex";

#[test_case(MarketType::InverseFuture, "XBTZ21")]
#[test_case(MarketType::LinearFuture, "ETHZ21")]
#[test_case(MarketType::QuantoFuture, "ETHUSDZ21")]
#[test_case(MarketType::InverseSwap, "XBTUSD")]
#[test_case(MarketType::QuantoSwap, "ETHUSD")]
fn test_crawl_trade(market_type: MarketType, symbol: &str) {
    gen_test_code!(
        crawl_trade,
        EXCHANGE_NAME,
        market_type,
        symbol,
        MessageType::Trade
    )
}

#[test_case(MarketType::InverseFuture, "XBTZ21")]
#[test_case(MarketType::LinearFuture, "ETHZ21")]
#[test_case(MarketType::QuantoFuture, "ETHUSDZ21")]
#[test_case(MarketType::InverseSwap, "XBTUSD")]
#[test_case(MarketType::QuantoSwap, "ETHUSD")]
fn test_crawl_l2_event(market_type: MarketType, symbol: &str) {
    gen_test_code!(
        crawl_l2_event,
        EXCHANGE_NAME,
        market_type,
        symbol,
        MessageType::L2Event
    )
}

#[test_case(MarketType::InverseFuture, "XBTZ21")]
#[test_case(MarketType::LinearFuture, "ETHZ21")]
#[test_case(MarketType::QuantoFuture, "ETHUSDZ21")]
#[test_case(MarketType::InverseSwap, "XBTUSD")]
#[test_case(MarketType::QuantoSwap, "ETHUSD")]
fn test_crawl_l2_snapshot(market_type: MarketType, symbol: &str) {
    gen_test_snapshot_code!(
        crawl_l2_snapshot,
        EXCHANGE_NAME,
        market_type,
        symbol,
        MessageType::L2Snapshot
    )
}

#[test_case(MarketType::InverseFuture)]
#[test_case(MarketType::LinearFuture)]
#[test_case(MarketType::QuantoFuture)]
#[test_case(MarketType::InverseSwap)]
#[test_case(MarketType::QuantoSwap)]
fn test_crawl_l2_snapshot_without_symbol(market_type: MarketType) {
    gen_test_snapshot_without_symbol_code!(
        crawl_l2_snapshot,
        EXCHANGE_NAME,
        market_type,
        MessageType::L2Snapshot
    )
}

#[test_case(MarketType::InverseSwap, "XBTUSD")]
#[test_case(MarketType::QuantoSwap, "ETHUSD")]
fn test_crawl_funding_rate(market_type: MarketType, symbol: &str) {
    gen_test_code!(
        crawl_funding_rate,
        EXCHANGE_NAME,
        market_type,
        symbol,
        MessageType::FundingRate
    )
}
