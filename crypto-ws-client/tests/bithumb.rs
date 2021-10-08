use crypto_ws_client::{BithumbWSClient, WSClient};
use std::sync::mpsc::{Receiver, Sender};

#[macro_use]
mod utils;

#[test]
fn subscribe() {
    gen_test_code!(
        BithumbWSClient,
        subscribe,
        &vec!["TRADE:BTC-USDT".to_string(), "TRADE:ETH-USDT".to_string()]
    );
}

#[test]
#[should_panic]
fn subscribe_illegal_symbol() {
    gen_test_code!(
        BithumbWSClient,
        subscribe,
        &vec!["TRADE:XXX-YYY".to_string(),]
    );
}

#[test]
fn subscribe_raw_json() {
    gen_test_code!(
        BithumbWSClient,
        subscribe,
        &vec![r#"{"cmd":"subscribe","args":["TRADE:BTC-USDT","TRADE:ETH-USDT"]}"#.to_string()]
    );
}

#[test]
fn subscribe_trade() {
    gen_test_code!(
        BithumbWSClient,
        subscribe_trade,
        &vec!["BTC-USDT".to_string(), "ETH-USDT".to_string()]
    );
}

#[test]
fn subscribe_orderbook() {
    gen_test_code!(
        BithumbWSClient,
        subscribe_orderbook,
        &vec!["BTC-USDT".to_string(), "ETH-USDT".to_string()]
    );
}

#[test]
fn subscribe_ticker() {
    gen_test_code!(
        BithumbWSClient,
        subscribe_ticker,
        &vec!["BTC-USDT".to_string(), "ETH-USDT".to_string()]
    );
}
