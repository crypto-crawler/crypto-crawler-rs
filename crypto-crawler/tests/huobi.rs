#[macro_use]
mod utils;

use test_case::test_case;

use crypto_crawler::*;
use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;
use utils::parse;

const EXCHANGE_NAME: &str = "huobi";

#[test_case(MarketType::Spot)]
#[test_case(MarketType::InverseFuture)]
#[test_case(MarketType::InverseSwap)]
#[test_case(MarketType::LinearSwap)]
#[tokio::test(flavor = "multi_thread")]
async fn test_crawl_trade_all(market_type: MarketType) {
    test_all_symbols!(crawl_trade, EXCHANGE_NAME, market_type, MessageType::Trade)
}

#[test_case(MarketType::Spot, "btcusdt")]
#[test_case(MarketType::InverseFuture, "BTC_CQ")]
#[test_case(MarketType::InverseSwap, "BTC-USD")]
#[test_case(MarketType::LinearSwap, "BTC-USDT")]
#[test_case(MarketType::EuropeanOption, "BTC-USDT-210625-P-27000"; "inconclusive")]
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

#[test_case(MarketType::Spot, "btcusdt")]
#[test_case(MarketType::InverseFuture, "BTC_CQ")]
#[test_case(MarketType::InverseSwap, "BTC-USD")]
#[test_case(MarketType::LinearSwap, "BTC-USDT")]
#[test_case(MarketType::EuropeanOption, "BTC-USDT-210625-P-27000"; "inconclusive")]
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

#[test_case(MarketType::Spot, "btcusdt")]
#[test_case(MarketType::InverseFuture, "BTC_CQ")]
#[test_case(MarketType::InverseSwap, "BTC-USD")]
#[test_case(MarketType::LinearSwap, "BTC-USDT")]
#[test_case(MarketType::EuropeanOption, "BTC-USDT-210625-P-27000"; "inconclusive")]
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

#[test_case(MarketType::Spot, "btcusdt")]
#[test_case(MarketType::InverseFuture, "BTC_CQ")]
#[test_case(MarketType::InverseSwap, "BTC-USD")]
#[test_case(MarketType::LinearSwap, "BTC-USDT")]
#[test_case(MarketType::EuropeanOption, "BTC-USDT-210625-P-27000"; "inconclusive")]
#[tokio::test(flavor = "multi_thread")]
async fn test_crawl_l2_topk(market_type: MarketType, symbol: &str) {
    test_one_symbol!(
        crawl_l2_topk,
        EXCHANGE_NAME,
        market_type,
        symbol,
        MessageType::L2TopK
    )
}

#[test_case(MarketType::Spot, "btcusdt")]
#[test_case(MarketType::InverseFuture, "BTC_CQ")]
#[test_case(MarketType::InverseSwap, "BTC-USD")]
#[test_case(MarketType::LinearSwap, "BTC-USDT")]
#[test_case(MarketType::EuropeanOption, "BTC-USDT-210625-P-27000"; "inconclusive")]
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
#[test_case(MarketType::InverseFuture)]
#[test_case(MarketType::InverseSwap)]
#[test_case(MarketType::LinearSwap)]
#[test_case(MarketType::EuropeanOption; "inconclusive")]
fn test_crawl_l2_snapshot_without_symbol(market_type: MarketType) {
    test_crawl_restful_all_symbols!(
        crawl_l2_snapshot,
        EXCHANGE_NAME,
        market_type,
        MessageType::L2Snapshot
    )
}

#[test_case(MarketType::InverseSwap, "BTC-USD")]
#[test_case(MarketType::LinearSwap, "BTC-USDT")]
#[tokio::test(flavor = "multi_thread")]
async fn test_crawl_funding_rate(market_type: MarketType, symbol: &str) {
    test_one_symbol!(
        crawl_funding_rate,
        EXCHANGE_NAME,
        market_type,
        symbol,
        MessageType::FundingRate
    )
}

#[test_case(MarketType::Spot, "btcusdt")]
#[test_case(MarketType::InverseFuture, "BTC_CQ")]
#[test_case(MarketType::InverseSwap, "BTC-USD")]
#[test_case(MarketType::LinearSwap, "BTC-USDT")]
#[test_case(MarketType::EuropeanOption, "BTC-USDT-210625-P-27000"; "inconclusive")]
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

#[test_case(MarketType::Spot)]
#[test_case(MarketType::InverseFuture)]
#[test_case(MarketType::InverseSwap)]
#[test_case(MarketType::LinearSwap)]
#[test_case(MarketType::EuropeanOption; "inconclusive")]
#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn test_crawl_candlestick(market_type: MarketType) {
    gen_test_crawl_candlestick!(EXCHANGE_NAME, market_type)
}

// #[test_case(MarketType::Spot, "btcusdt")]
// #[test_case(MarketType::InverseFuture, "BTC_CQ")]
// #[test_case(MarketType::InverseSwap, "BTC-USD")]
// #[test_case(MarketType::LinearSwap, "BTC-USDT")]
// fn test_subscribe_symbol(market_type: MarketType, symbol: &str) {
//     gen_test_subscribe_symbol!(EXCHANGE_NAME, market_type, symbol)
// }
