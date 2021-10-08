use crypto_ws_client::{KrakenWSClient, WSClient};
use std::sync::mpsc::{Receiver, Sender};

#[macro_use]
mod utils;

#[test]
fn subscribe() {
    gen_test_code!(
        KrakenWSClient,
        subscribe,
        &vec![
            "trade:XBT/USD".to_string(),
            "ticker:XBT/USD".to_string(),
            "spread:XBT/USD".to_string(),
            "book:XBT/USD".to_string()
        ]
    );
}

#[test]
fn subscribe_raw_json() {
    gen_test_code!(
        KrakenWSClient,
        subscribe,
        &vec![
            r#"{"event":"subscribe","pair":["XBT/USD"],"subscription":{"name":"trade"}}"#
                .to_string()
        ]
    );
}

#[test]
fn subscribe_trade() {
    gen_test_code!(
        KrakenWSClient,
        subscribe_trade,
        &vec!["XBT/USD".to_string(), "ETH/USD".to_string()]
    );
}

#[test]
fn subscribe_ticker() {
    gen_test_code!(
        KrakenWSClient,
        subscribe_ticker,
        &vec!["XBT/USD".to_string(), "ETH/USD".to_string()]
    );
}

#[test]
fn subscribe_bbo() {
    gen_test_code!(
        KrakenWSClient,
        subscribe_bbo,
        &vec!["XBT/USD".to_string(), "ETH/USD".to_string()]
    );
}

#[test]
fn subscribe_orderbook() {
    gen_test_code!(
        KrakenWSClient,
        subscribe_orderbook,
        &vec!["XBT/USD".to_string(), "ETH/USD".to_string()]
    );
}

#[test]
fn subscribe_candlestick() {
    gen_test_subscribe_candlestick!(
        KrakenWSClient,
        &vec![("XBT/USD".to_string(), 60), ("ETH/USD".to_string(), 60)]
    );

    gen_test_subscribe_candlestick!(
        KrakenWSClient,
        &vec![
            ("XBT/USD".to_string(), 1296000),
            ("ETH/USD".to_string(), 1296000)
        ]
    );
}
