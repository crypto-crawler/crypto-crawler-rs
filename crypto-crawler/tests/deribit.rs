#[macro_use]
mod utils;

use test_case::test_case;

use crypto_crawler::*;
use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;
use utils::parse;

const EXCHANGE_NAME: &str = "deribit";

#[test_case(MarketType::InverseSwap, "BTC-PERPETUAL")]
#[test_case(MarketType::InverseFuture, "BTC-31DEC21")]
#[test_case(MarketType::EuropeanOption, "BTC-31DEC21-400000-C"; "inconclusive")]
fn test_crawl_trade(market_type: MarketType, symbol: &str) {
    test_one_symbol!(
        crawl_trade,
        EXCHANGE_NAME,
        market_type,
        symbol,
        MessageType::Trade
    )
}

#[test_case(MarketType::InverseSwap)]
#[test_case(MarketType::InverseFuture)]
#[test_case(MarketType::EuropeanOption)]
fn test_crawl_trade_all(market_type: MarketType) {
    test_all_symbols!(crawl_trade, EXCHANGE_NAME, market_type, MessageType::Trade)
}

#[test_case(MarketType::InverseSwap, "BTC-PERPETUAL")]
#[test_case(MarketType::InverseFuture, "BTC-31DEC21")]
#[test_case(MarketType::EuropeanOption, "BTC-31DEC21-400000-C")]
fn test_crawl_l2_event(market_type: MarketType, symbol: &str) {
    test_one_symbol!(
        crawl_l2_event,
        EXCHANGE_NAME,
        market_type,
        symbol,
        MessageType::L2Event
    )
}

#[test_case(MarketType::InverseSwap, "BTC-PERPETUAL")]
#[test_case(MarketType::InverseFuture, "BTC-31DEC21")]
#[test_case(MarketType::EuropeanOption, "BTC-31DEC21-400000-C")]
fn test_crawl_bbo(market_type: MarketType, symbol: &str) {
    test_one_symbol!(
        crawl_bbo,
        EXCHANGE_NAME,
        market_type,
        symbol,
        MessageType::BBO
    )
}

#[test_case(MarketType::InverseSwap, "BTC-PERPETUAL")]
#[test_case(MarketType::InverseFuture, "BTC-31DEC21")]
#[test_case(MarketType::EuropeanOption, "BTC-31DEC21-400000-C")]
fn test_crawl_l2_topk(market_type: MarketType, symbol: &str) {
    test_one_symbol!(
        crawl_l2_topk,
        EXCHANGE_NAME,
        market_type,
        symbol,
        MessageType::L2TopK
    )
}

#[test_case(MarketType::InverseSwap, "BTC-PERPETUAL")]
#[test_case(MarketType::InverseFuture, "BTC-31DEC21")]
#[test_case(MarketType::EuropeanOption, "BTC-31DEC21-400000-C")]
fn test_crawl_l2_snapshot(market_type: MarketType, symbol: &str) {
    test_one_symbol!(
        crawl_l2_snapshot,
        EXCHANGE_NAME,
        market_type,
        symbol,
        MessageType::L2Snapshot
    )
}

#[test_case(MarketType::InverseSwap)]
#[test_case(MarketType::InverseFuture)]
#[test_case(MarketType::EuropeanOption; "inconclusive")]
fn test_crawl_l2_snapshot_without_symbol(market_type: MarketType) {
    test_all_symbols!(
        crawl_l2_snapshot,
        EXCHANGE_NAME,
        market_type,
        MessageType::L2Snapshot
    )
}

#[test_case(MarketType::InverseSwap, "BTC-PERPETUAL")]
#[test_case(MarketType::InverseFuture, "BTC-31DEC21")]
#[test_case(MarketType::EuropeanOption, "BTC-31DEC21-400000-C")]
fn test_crawl_ticker(market_type: MarketType, symbol: &str) {
    test_one_symbol!(
        crawl_ticker,
        EXCHANGE_NAME,
        market_type,
        symbol,
        MessageType::Ticker
    )
}

#[test_case(MarketType::InverseSwap)]
#[test_case(MarketType::InverseFuture)]
#[test_case(MarketType::EuropeanOption)]
fn test_crawl_candlestick(market_type: MarketType) {
    gen_test_crawl_candlestick!(EXCHANGE_NAME, market_type)
}
