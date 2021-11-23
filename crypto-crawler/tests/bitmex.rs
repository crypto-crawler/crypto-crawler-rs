#[macro_use]
mod utils;

use test_case::test_case;

use crypto_crawler::*;
use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;
use utils::parse;

const EXCHANGE_NAME: &str = "bitmex";

fn crawl_all(msg_type: MessageType) {
    let (tx, rx) = std::sync::mpsc::channel();
    let mut messages = Vec::new();
    let crawl_func = match msg_type {
        MessageType::Trade => crawl_trade,
        MessageType::L2Event => crawl_l2_event,
        MessageType::L2Snapshot => crawl_l2_snapshot,
        MessageType::BBO => crawl_bbo,
        MessageType::L2TopK => crawl_l2_topk,
        MessageType::FundingRate => crawl_funding_rate,
        _ => panic!("unsupported message type {}", msg_type),
    };
    crawl_func(EXCHANGE_NAME, MarketType::Unknown, None, tx, Some(0));

    for msg in rx {
        messages.push(msg);
    }

    assert!(!messages.is_empty());
    assert_eq!(messages[0].exchange, EXCHANGE_NAME.to_string());
    assert_eq!(messages[0].market_type, MarketType::Unknown);
    assert_eq!(messages[0].msg_type, msg_type);
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
fn test_crawl_l2_topk_all() {
    crawl_all(MessageType::L2TopK);
}

#[test]
fn test_crawl_l2_snapshot_all() {
    crawl_all(MessageType::L2Snapshot);
}

#[test]
fn test_crawl_funding_rate_all() {
    crawl_all(MessageType::FundingRate);
}

#[test]
fn test_crawl_candlestick_rate_all() {
    let (tx, rx) = std::sync::mpsc::channel();
    let mut messages = Vec::new();
    crawl_candlestick(EXCHANGE_NAME, MarketType::Unknown, None, tx, Some(0));

    for msg in rx {
        messages.push(msg);
    }

    assert!(!messages.is_empty());
    assert_eq!(messages[0].exchange, EXCHANGE_NAME.to_string());
    assert_eq!(messages[0].market_type, MarketType::Unknown);
    assert_eq!(messages[0].msg_type, MessageType::Candlestick);
}

#[test_case(MarketType::InverseSwap, "XBTUSD")]
#[test_case(MarketType::QuantoSwap, "ETHUSD")]
fn test_crawl_l2_event(market_type: MarketType, symbol: &str) {
    test_one_symbol!(
        crawl_l2_event,
        EXCHANGE_NAME,
        market_type,
        symbol,
        MessageType::L2Event
    )
}

#[test_case(MarketType::InverseSwap, "XBTUSD")]
fn test_subscribe_symbol(market_type: MarketType, symbol: &str) {
    gen_test_subscribe_symbol!(EXCHANGE_NAME, market_type, symbol)
}
