#[macro_use]
mod utils;

use test_case::test_case;

use crypto_crawler::*;
use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;
use utils::parse;

const EXCHANGE_NAME: &str = "bitmex";

async fn crawl_all(msg_type: MessageType) {
    let (tx, rx) = std::sync::mpsc::channel();
    tokio::task::spawn(async move {
        match msg_type {
            MessageType::Trade => {
                crawl_trade(EXCHANGE_NAME, MarketType::Unknown, None, tx).await;
            }
            MessageType::L2Event => {
                crawl_l2_event(EXCHANGE_NAME, MarketType::Unknown, None, tx).await;
            }
            MessageType::L2Snapshot => {
                tokio::task::block_in_place(move || {
                    crawl_l2_snapshot(EXCHANGE_NAME, MarketType::Unknown, None, tx);
                });
            }
            MessageType::BBO => {
                crawl_bbo(EXCHANGE_NAME, MarketType::Unknown, None, tx).await;
            }
            MessageType::L2TopK => {
                crawl_l2_topk(EXCHANGE_NAME, MarketType::Unknown, None, tx).await;
            }
            MessageType::FundingRate => {
                crawl_funding_rate(EXCHANGE_NAME, MarketType::Unknown, None, tx).await;
            }
            _ => panic!("unsupported message type {}", msg_type),
        };
    });

    let msg = rx.recv().unwrap();

    assert_eq!(msg.exchange, EXCHANGE_NAME.to_string());
    assert_eq!(msg.market_type, MarketType::Unknown);
    assert_eq!(msg.msg_type, msg_type);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_crawl_trade_all() {
    crawl_all(MessageType::Trade).await;
}

#[tokio::test(flavor = "multi_thread")]
async fn test_crawl_l2_event_all() {
    crawl_all(MessageType::L2Event).await;
}

#[tokio::test(flavor = "multi_thread")]
async fn test_crawl_bbo_all() {
    crawl_all(MessageType::BBO).await;
}

#[tokio::test(flavor = "multi_thread")]
async fn test_crawl_l2_topk_all() {
    crawl_all(MessageType::L2TopK).await;
}

#[tokio::test(flavor = "multi_thread")]
async fn test_crawl_l2_snapshot_all() {
    crawl_all(MessageType::L2Snapshot).await;
}

#[tokio::test(flavor = "multi_thread")]
async fn test_crawl_funding_rate_all() {
    crawl_all(MessageType::FundingRate).await;
}

#[tokio::test(flavor = "multi_thread")]
async fn test_crawl_candlestick_rate_all() {
    let (tx, rx) = std::sync::mpsc::channel();
    tokio::task::spawn(async move {
        crawl_candlestick(EXCHANGE_NAME, MarketType::Unknown, None, tx).await;
    });

    let msg = rx.recv().unwrap();

    assert_eq!(msg.exchange, EXCHANGE_NAME.to_string());
    assert_eq!(msg.market_type, MarketType::Unknown);
    assert_eq!(msg.msg_type, MessageType::Candlestick);
}

#[test_case(MarketType::InverseSwap, "XBTUSD")]
#[test_case(MarketType::QuantoSwap, "ETHUSD")]
#[tokio::test(flavor = "multi_thread")]
async fn test_crawl_l2_event(market_type: MarketType, symbol: &str) {
    test_one_symbol!(crawl_l2_event, EXCHANGE_NAME, market_type, symbol, MessageType::L2Event)
}

// #[test_case(MarketType::InverseSwap, "XBTUSD")]
// fn test_subscribe_symbol(market_type: MarketType, symbol: &str) {
//     gen_test_subscribe_symbol!(EXCHANGE_NAME, market_type, symbol)
// }
