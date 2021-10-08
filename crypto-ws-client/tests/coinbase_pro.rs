use crypto_ws_client::{CoinbaseProWSClient, WSClient};
use std::sync::{Arc, Mutex};

#[macro_use]
mod utils;

#[test]
fn subscribe() {
    gen_test_code!(
        CoinbaseProWSClient,
        subscribe,
        &vec![
            "matches:BTC-USD".to_string(),
            "heartbeat:BTC-USD".to_string()
        ]
    );
}

#[test]
#[should_panic]
fn subscribe_illegal_symbol() {
    gen_test_code!(
        CoinbaseProWSClient,
        subscribe,
        &vec!["matches:XXX-YYY".to_string(),]
    );
}

#[test]
fn subscribe_raw_json() {
    gen_test_code!(
        CoinbaseProWSClient,
        subscribe,
        &vec![r#"{
                "type":"subscribe",
                "channels":[
                   {
                      "name":"heartbeat",
                      "product_ids":[
                         "BTC-USD"
                      ]
                   },
                   {
                      "name":"matches",
                      "product_ids":[
                         "BTC-USD"
                      ]
                   }
                ]
             }"#
        .to_string()]
    );
}

#[test]
fn subscribe_trade() {
    gen_test_code!(
        CoinbaseProWSClient,
        subscribe_trade,
        &vec!["BTC-USD".to_string(), "ETH-USD".to_string()]
    );
}

#[test]
fn subscribe_ticker() {
    gen_test_code!(
        CoinbaseProWSClient,
        subscribe_ticker,
        &vec!["BTC-USD".to_string(), "ETH-USD".to_string()]
    );
}

#[test]
fn subscribe_orderbook() {
    gen_test_code!(
        CoinbaseProWSClient,
        subscribe_orderbook,
        &vec!["BTC-USD".to_string()]
    );
}

#[test]
fn subscribe_l3_orderbook() {
    gen_test_code!(
        CoinbaseProWSClient,
        subscribe_l3_orderbook,
        &vec!["BTC-USD".to_string()]
    );
}
