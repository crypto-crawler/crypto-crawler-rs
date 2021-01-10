use crypto_ws_client::{BinanceOptionWSClient, WSClient};
use std::sync::{Arc, Mutex};

#[macro_use]
mod utils;

#[test]
#[ignore]
fn subscribe() {
    gen_test_code!(
        BinanceOptionWSClient,
        subscribe,
        &vec![
            "BTC-210108-38000-C@trade".to_string(),
            "BTC-210129-40000-C".to_string()
        ]
    );
}

#[test]
#[ignore]
fn subscribe_trade() {
    gen_test_code!(
        BinanceOptionWSClient,
        subscribe_trade,
        &vec![
            "BTC-210108-38000-C".to_string(),
            "BTC-210129-40000-C".to_string()
        ]
    );
}

#[test]
#[ignore]
fn subscribe_ticker() {
    gen_test_code!(
        BinanceOptionWSClient,
        subscribe_ticker,
        &vec![
            "BTC-210108-38000-C".to_string(),
            "BTC-210129-40000-C".to_string()
        ]
    );
}

#[test]
#[ignore]
fn subscribe_bbo() {
    gen_test_code!(
        BinanceOptionWSClient,
        subscribe_bbo,
        &vec![
            "BTC-210108-38000-C".to_string(),
            "BTC-210129-40000-C".to_string()
        ]
    );
}

#[test]
#[ignore]
fn subscribe_orderbook() {
    gen_test_code!(
        BinanceOptionWSClient,
        subscribe_orderbook,
        &vec![
            "BTC-210108-38000-C".to_string(),
            "BTC-210129-40000-C".to_string()
        ]
    );
}

#[test]
#[ignore]
fn subscribe_orderbook_snapshot() {
    gen_test_code!(
        BinanceOptionWSClient,
        subscribe_orderbook_snapshot,
        &vec![
            "BTC-210108-38000-C".to_string(),
            "BTC-210129-40000-C".to_string()
        ]
    );
}

#[test]
#[ignore]
fn subscribe_candlestick() {
    gen_test_subscribe_candlestick!(
        BinanceOptionWSClient,
        &vec![
            "BTC-210108-38000-C".to_string(),
            "BTC-210129-40000-C".to_string()
        ],
        60
    );
    gen_test_subscribe_candlestick!(
        BinanceOptionWSClient,
        &vec![
            "BTC-210108-38000-C".to_string(),
            "BTC-210129-40000-C".to_string()
        ],
        2592000
    );
}
