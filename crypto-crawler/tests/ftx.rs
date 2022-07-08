#[macro_use]
mod utils;

use test_case::test_case;

use crypto_crawler::*;
use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;
use utils::parse;

const EXCHANGE_NAME: &str = "ftx";

#[test_case(MarketType::Spot)]
#[test_case(MarketType::LinearSwap)]
#[test_case(MarketType::LinearFuture)]
// #[test_case(MarketType::Move)]
// #[test_case(MarketType::BVOL)]
#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn test_crawl_trade_all(market_type: MarketType) {
    test_all_symbols!(crawl_trade, EXCHANGE_NAME, market_type, MessageType::Trade)
}

#[test_case(MarketType::Spot, "BTC/USD")]
#[test_case(MarketType::LinearSwap, "BTC-PERP")]
#[test_case(MarketType::LinearFuture, "BTC-0930")]
// #[test_case(MarketType::Move, "BTC-MOVE-2022Q3")]
// #[test_case(MarketType::BVOL, "BVOL/USD")]
#[tokio::test(flavor = "multi_thread")]
async fn test_crawl_trade(market_type: MarketType, symbol: &str) {
    test_one_symbol!(
        crawl_trade,
        EXCHANGE_NAME,
        market_type,
        symbol,
        MessageType::Trade
    )
}

#[test_case(MarketType::Spot, "BTC/USD")]
#[test_case(MarketType::LinearSwap, "BTC-PERP")]
#[test_case(MarketType::LinearFuture, "BTC-0930")]
#[test_case(MarketType::Move, "BTC-MOVE-2022Q3")]
#[test_case(MarketType::BVOL, "BVOL/USD")]
#[tokio::test(flavor = "multi_thread")]
async fn test_crawl_l2_event(market_type: MarketType, symbol: &str) {
    test_one_symbol!(
        crawl_l2_event,
        EXCHANGE_NAME,
        market_type,
        symbol,
        MessageType::L2Event
    )
}

#[test_case(MarketType::Spot, "BTC/USD")]
#[test_case(MarketType::LinearSwap, "BTC-PERP")]
#[test_case(MarketType::LinearFuture, "BTC-0930")]
#[test_case(MarketType::Move, "BTC-MOVE-2022Q3")]
#[test_case(MarketType::BVOL, "BVOL/USD")]
#[tokio::test(flavor = "multi_thread")]
async fn test_crawl_bbo(market_type: MarketType, symbol: &str) {
    test_one_symbol!(
        crawl_bbo,
        EXCHANGE_NAME,
        market_type,
        symbol,
        MessageType::BBO
    )
}

#[test_case(MarketType::Spot, "BTC/USD")]
#[test_case(MarketType::LinearSwap, "BTC-PERP")]
#[test_case(MarketType::LinearFuture, "BTC-0930")]
#[test_case(MarketType::Move, "BTC-MOVE-2022Q3")]
#[test_case(MarketType::BVOL, "BVOL/USD")]
fn test_crawl_l2_snapshot(market_type: MarketType, symbol: &str) {
    test_crawl_restful!(
        crawl_l2_snapshot,
        EXCHANGE_NAME,
        market_type,
        symbol,
        MessageType::L2Snapshot
    )
}

#[test_case(MarketType::Spot)]
#[test_case(MarketType::LinearSwap)]
#[test_case(MarketType::LinearFuture)]
#[test_case(MarketType::Move)]
#[test_case(MarketType::BVOL)]
fn test_crawl_l2_snapshot_without_symbol(market_type: MarketType) {
    test_crawl_restful_all_symbols!(
        crawl_l2_snapshot,
        EXCHANGE_NAME,
        market_type,
        MessageType::L2Snapshot
    )
}

// #[test_case(MarketType::Spot, "BTC/USD")]
// #[test_case(MarketType::LinearSwap, "BTC-PERP")]
// #[test_case(MarketType::LinearFuture, "BTC-0930")]
// fn test_subscribe_symbol(market_type: MarketType, symbol: &str) {
//     gen_test_subscribe_symbol!(EXCHANGE_NAME, market_type, symbol)
// }
