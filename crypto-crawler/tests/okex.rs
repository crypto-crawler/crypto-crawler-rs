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

const EXCHANGE_NAME: &str = "okex";

#[test_case(MarketType::Spot; "inconclusive")] // TODO
#[test_case(MarketType::InverseFuture)]
#[test_case(MarketType::LinearFuture)]
#[test_case(MarketType::InverseSwap)]
#[test_case(MarketType::LinearSwap)]
#[test_case(MarketType::EuropeanOption)]
fn test_crawl_trade_all(market_type: MarketType) {
    thread_local! {
        static MESSAGES: RefCell<Vec<Message>> = RefCell::new(Vec::new());
    }

    let on_msg = Arc::new(Mutex::new(|msg: Message| {
        MESSAGES.with(|messages| messages.borrow_mut().push(msg))
    }));
    crawl_trade(EXCHANGE_NAME, market_type, None, on_msg, Some(0));

    MESSAGES.with(|slf| {
        let messages = slf.borrow();

        assert!(!messages.is_empty());
        assert_eq!(messages[0].exchange, EXCHANGE_NAME.to_string());
        assert_eq!(messages[0].market_type, market_type);
        assert_eq!(messages[0].msg_type, MessageType::Trade);
    });
}

#[test_case(MarketType::Spot, "BTC-USDT")]
#[test_case(MarketType::InverseFuture, "BTC-USD-211231")]
#[test_case(MarketType::LinearFuture, "BTC-USDT-211231")]
#[test_case(MarketType::InverseSwap, "BTC-USD-SWAP")]
#[test_case(MarketType::LinearSwap, "BTC-USDT-SWAP")]
#[test_case(MarketType::EuropeanOption, "BTC-USD-211231-18000-P"; "inconclusive")]
fn test_crawl_trade(market_type: MarketType, symbol: &str) {
    gen_test_code!(
        crawl_trade,
        EXCHANGE_NAME,
        market_type,
        symbol,
        MessageType::Trade
    )
}

#[test_case(MarketType::Spot, "BTC-USDT")]
#[test_case(MarketType::InverseFuture, "BTC-USD-211231")]
#[test_case(MarketType::LinearFuture, "BTC-USDT-211231")]
#[test_case(MarketType::InverseSwap, "BTC-USD-SWAP")]
#[test_case(MarketType::LinearSwap, "BTC-USDT-SWAP")]
#[test_case(MarketType::EuropeanOption, "BTC-USD-211231-18000-P")]
fn test_crawl_l2_event(market_type: MarketType, symbol: &str) {
    gen_test_code!(
        crawl_l2_event,
        EXCHANGE_NAME,
        market_type,
        symbol,
        MessageType::L2Event
    )
}

#[test_case(MarketType::Spot, "BTC-USDT")]
#[test_case(MarketType::InverseFuture, "BTC-USD-211231")]
#[test_case(MarketType::LinearFuture, "BTC-USDT-211231")]
#[test_case(MarketType::InverseSwap, "BTC-USD-SWAP")]
#[test_case(MarketType::LinearSwap, "BTC-USDT-SWAP")]
#[test_case(MarketType::EuropeanOption, "BTC-USD-211231-18000-P")]
fn test_crawl_bbo(market_type: MarketType, symbol: &str) {
    gen_test_code!(
        crawl_bbo,
        EXCHANGE_NAME,
        market_type,
        symbol,
        MessageType::BBO
    )
}

#[test_case(MarketType::Spot, "BTC-USDT")]
#[test_case(MarketType::InverseFuture, "BTC-USD-211231")]
#[test_case(MarketType::LinearFuture, "BTC-USDT-211231")]
#[test_case(MarketType::InverseSwap, "BTC-USD-SWAP")]
#[test_case(MarketType::LinearSwap, "BTC-USDT-SWAP")]
#[test_case(MarketType::EuropeanOption, "BTC-USD-211231-18000-P")]
fn test_crawl_l2_topk(market_type: MarketType, symbol: &str) {
    gen_test_code!(
        crawl_l2_topk,
        EXCHANGE_NAME,
        market_type,
        symbol,
        MessageType::L2TopK
    )
}

#[test_case(MarketType::Spot, "BTC-USDT")]
#[test_case(MarketType::InverseFuture, "BTC-USD-211231")]
#[test_case(MarketType::LinearFuture, "BTC-USDT-211231")]
#[test_case(MarketType::InverseSwap, "BTC-USD-SWAP")]
#[test_case(MarketType::LinearSwap, "BTC-USDT-SWAP")]
#[test_case(MarketType::EuropeanOption, "BTC-USD-211231-18000-P")]
fn test_crawl_l2_snapshot(market_type: MarketType, symbol: &str) {
    gen_test_snapshot_code!(
        crawl_l2_snapshot,
        EXCHANGE_NAME,
        market_type,
        symbol,
        MessageType::L2Snapshot
    )
}

#[test_case(MarketType::Spot)]
#[test_case(MarketType::InverseFuture)]
#[test_case(MarketType::LinearFuture)]
#[test_case(MarketType::InverseSwap)]
#[test_case(MarketType::LinearSwap)]
#[test_case(MarketType::EuropeanOption)]
fn test_crawl_l2_snapshot_without_symbol(market_type: MarketType) {
    gen_test_snapshot_without_symbol_code!(
        crawl_l2_snapshot,
        EXCHANGE_NAME,
        market_type,
        MessageType::L2Snapshot
    )
}

#[test_case(MarketType::InverseSwap, "BTC-USD-SWAP")]
#[test_case(MarketType::LinearSwap, "BTC-USDT-SWAP")]
fn test_crawl_funding_rate(market_type: MarketType, symbol: &str) {
    gen_test_code!(
        crawl_funding_rate,
        EXCHANGE_NAME,
        market_type,
        symbol,
        MessageType::FundingRate
    )
}

#[test_case(MarketType::Spot, "BTC-USDT")]
#[test_case(MarketType::InverseFuture, "BTC-USD-211231")]
#[test_case(MarketType::LinearFuture, "BTC-USDT-211231")]
#[test_case(MarketType::InverseSwap, "BTC-USD-SWAP")]
#[test_case(MarketType::LinearSwap, "BTC-USDT-SWAP")]
#[test_case(MarketType::EuropeanOption, "BTC-USD-211231-18000-P")]
fn test_crawl_ticker(market_type: MarketType, symbol: &str) {
    gen_test_code!(
        crawl_ticker,
        EXCHANGE_NAME,
        market_type,
        symbol,
        MessageType::Ticker
    )
}
