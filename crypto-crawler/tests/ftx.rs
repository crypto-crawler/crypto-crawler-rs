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

const EXCHANGE_NAME: &str = "ftx";

#[test_case(MarketType::Spot)]
#[test_case(MarketType::LinearSwap)]
#[test_case(MarketType::LinearFuture)]
// #[test_case(MarketType::Move)]
// #[test_case(MarketType::BVOL)]
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

#[test_case(MarketType::Spot, "BTC/USD")]
#[test_case(MarketType::LinearSwap, "BTC-PERP")]
#[test_case(MarketType::LinearFuture, "BTC-1231")]
// #[test_case(MarketType::Move, "BTC-MOVE-2021Q4")]
// #[test_case(MarketType::BVOL, "BVOL/USD")]
fn test_crawl_trade(market_type: MarketType, symbol: &str) {
    gen_test_code!(
        crawl_trade,
        EXCHANGE_NAME,
        market_type,
        symbol,
        MessageType::Trade
    )
}

#[test_case(MarketType::Spot, "BTC/USD")]
#[test_case(MarketType::LinearSwap, "BTC-PERP")]
#[test_case(MarketType::LinearFuture, "BTC-1231")]
#[test_case(MarketType::Move, "BTC-MOVE-2021Q4")]
#[test_case(MarketType::BVOL, "BVOL/USD")]
fn test_crawl_l2_event(market_type: MarketType, symbol: &str) {
    gen_test_code!(
        crawl_l2_event,
        EXCHANGE_NAME,
        market_type,
        symbol,
        MessageType::L2Event
    )
}

#[test_case(MarketType::Spot, "BTC/USD")]
#[test_case(MarketType::LinearSwap, "BTC-PERP")]
#[test_case(MarketType::LinearFuture, "BTC-1231")]
#[test_case(MarketType::Move, "BTC-MOVE-2021Q4")]
#[test_case(MarketType::BVOL, "BVOL/USD")]
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
#[test_case(MarketType::LinearSwap)]
#[test_case(MarketType::LinearFuture)]
#[test_case(MarketType::Move)]
#[test_case(MarketType::BVOL)]
fn test_crawl_l2_snapshot_without_symbol(market_type: MarketType) {
    gen_test_snapshot_without_symbol_code!(
        crawl_l2_snapshot,
        EXCHANGE_NAME,
        market_type,
        MessageType::L2Snapshot
    )
}
