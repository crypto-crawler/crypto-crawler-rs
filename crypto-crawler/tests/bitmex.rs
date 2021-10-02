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

fn crawl_all(msg_type: MessageType) {
    thread_local! {
        static MESSAGES: RefCell<Vec<Message>> = RefCell::new(Vec::new());
    }

    let on_msg = Arc::new(Mutex::new(|msg: Message| {
        MESSAGES.with(|messages| messages.borrow_mut().push(msg))
    }));
    let crawl_func = match msg_type {
        MessageType::Trade => crawl_trade,
        MessageType::L2Event => crawl_l2_event,
        MessageType::L2Snapshot => crawl_l2_snapshot,
        MessageType::BBO => crawl_bbo,
        MessageType::FundingRate => crawl_funding_rate,
        _ => panic!("unsupported message type {}", msg_type),
    };
    crawl_func(EXCHANGE_NAME, MarketType::Unknown, None, on_msg, Some(0));

    MESSAGES.with(|slf| {
        let messages = slf.borrow();

        assert!(!messages.is_empty());
        assert_eq!(messages[0].exchange, EXCHANGE_NAME.to_string());
        assert_eq!(messages[0].market_type, MarketType::Unknown);
        assert_eq!(messages[0].msg_type, msg_type);
    });
}

#[test]
fn test_crawl_trade_all() {
    crawl_all(MessageType::Trade);
}

#[test]
fn test_crawl_l2_event_all() {
    crawl_all(MessageType::L2Event);
}

#[test]
fn test_crawl_bbo_all() {
    crawl_all(MessageType::BBO);
}

#[test]
fn test_crawl_l2_snapshot_all() {
    crawl_all(MessageType::L2Snapshot);
}

#[test]
fn test_crawl_funding_rate_all() {
    crawl_all(MessageType::FundingRate);
}

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
