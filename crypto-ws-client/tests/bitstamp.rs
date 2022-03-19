use crypto_ws_client::{BitstampWSClient, WSClient};
use std::sync::mpsc::{Receiver, Sender};

#[macro_use]
mod utils;

#[tokio::test(flavor = "multi_thread")]
async fn subscribe() {
    gen_test_code!(
        BitstampWSClient,
        subscribe,
        &vec![
            ("live_trades".to_string(), "btcusd".to_string()),
            ("diff_order_book".to_string(), "btcusd".to_string()),
        ]
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn subscribe_raw_json() {
    gen_test_code!(
        BitstampWSClient,
        send,
        &vec![r#"{"event":"bts:subscribe","data":{"channel":"live_trades_btcusd"}}"#.to_string()]
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn subscribe_trade() {
    gen_test_code!(
        BitstampWSClient,
        subscribe_trade,
        &vec!["btcusd".to_string(), "ethusd".to_string()]
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn subscribe_orderbook() {
    gen_test_code!(
        BitstampWSClient,
        subscribe_orderbook,
        &vec!["btcusd".to_string(), "ethusd".to_string()]
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn subscribe_orderbook_topk() {
    gen_test_code!(
        BitstampWSClient,
        subscribe_orderbook_topk,
        &vec!["btcusd".to_string(), "ethusd".to_string()]
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn subscribe_l3_orderbook() {
    gen_test_code!(
        BitstampWSClient,
        subscribe_l3_orderbook,
        &vec!["btcusd".to_string(), "ethusd".to_string()]
    );
}
