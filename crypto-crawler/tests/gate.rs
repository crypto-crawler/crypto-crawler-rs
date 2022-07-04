#[macro_use]
mod utils;

use test_case::test_case;

use crypto_crawler::*;
use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;
use utils::parse;

const EXCHANGE_NAME: &str = "gate";

#[test_case(MarketType::Spot)]
#[test_case(MarketType::InverseSwap)]
#[test_case(MarketType::LinearSwap)]
#[test_case(MarketType::InverseFuture)]
#[test_case(MarketType::LinearFuture)]
#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn test_crawl_trade_all(market_type: MarketType) {
    test_all_symbols!(crawl_trade, EXCHANGE_NAME, market_type, MessageType::Trade)
}

#[test_case(MarketType::Spot, "BTC_USDT")]
#[test_case(MarketType::InverseSwap, "BTC_USD")]
#[test_case(MarketType::LinearSwap, "BTC_USDT")]
// #[test_case(MarketType::InverseFuture, "BTC_USD_20220930"; "ignore")]
// #[test_case(MarketType::LinearFuture, "BTC_USDT_20220930"; "ignore")]
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

#[test_case(MarketType::Spot, "BTC_USDT")]
#[test_case(MarketType::InverseSwap, "BTC_USD")]
#[test_case(MarketType::LinearSwap, "BTC_USDT")]
#[test_case(MarketType::InverseFuture, "BTC_USD_20220930")]
#[test_case(MarketType::LinearFuture, "BTC_USDT_20220930")]
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

#[test_case(MarketType::Spot, "BTC_USDT")]
#[test_case(MarketType::InverseSwap, "BTC_USD")]
#[test_case(MarketType::LinearSwap, "BTC_USDT")]
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

#[test_case(MarketType::Spot, "BTC_USDT")]
#[test_case(MarketType::InverseSwap, "BTC_USD")]
#[test_case(MarketType::LinearSwap, "BTC_USDT")]
#[test_case(MarketType::InverseFuture, "BTC_USD_20220930")]
#[test_case(MarketType::LinearFuture, "BTC_USDT_20220930")]
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
#[test_case(MarketType::InverseSwap)]
#[test_case(MarketType::LinearSwap)]
#[test_case(MarketType::InverseFuture)]
#[test_case(MarketType::LinearFuture)]
fn test_crawl_l2_snapshot_without_symbol(market_type: MarketType) {
    test_crawl_restful_all_symbols!(
        crawl_l2_snapshot,
        EXCHANGE_NAME,
        market_type,
        MessageType::L2Snapshot
    )
}

#[test_case(MarketType::Spot, "BTC_USDT")]
#[test_case(MarketType::InverseSwap, "BTC_USD")]
#[test_case(MarketType::LinearSwap, "BTC_USDT")]
#[test_case(MarketType::InverseFuture, "BTC_USD_20220930")]
#[test_case(MarketType::LinearFuture, "BTC_USDT_20220930")]
#[tokio::test(flavor = "multi_thread")]
async fn test_crawl_ticker(market_type: MarketType, symbol: &str) {
    test_one_symbol!(
        crawl_ticker,
        EXCHANGE_NAME,
        market_type,
        symbol,
        MessageType::Ticker
    )
}

#[test_case(MarketType::Spot; "inconclusive due to too many symbols")]
#[test_case(MarketType::InverseSwap)]
#[test_case(MarketType::LinearSwap)]
#[test_case(MarketType::InverseFuture)]
#[test_case(MarketType::LinearFuture)]
#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn test_crawl_candlestick(market_type: MarketType) {
    gen_test_crawl_candlestick!(EXCHANGE_NAME, market_type)
}
