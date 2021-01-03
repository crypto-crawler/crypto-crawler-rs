use crypto_ws_client::{CoinbaseProWSClient, OrderBook, WSClient};

#[macro_use]
mod utils;

#[test]
fn subscribe() {
    gen_test_subscribe!(
        CoinbaseProWSClient,
        &vec![
            "matches:BTC-USD".to_string(),
            "heartbeat:BTC-USD".to_string()
        ]
    );
}

#[test]
fn subscribe_raw_json() {
    gen_test_subscribe!(
        CoinbaseProWSClient,
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
    gen_test_subscribe_trade!(
        CoinbaseProWSClient,
        &vec!["BTC-USD".to_string(), "ETH-USD".to_string()]
    );
}

#[test]
fn subscribe_ticker() {
    gen_test_subscribe_ticker!(
        CoinbaseProWSClient,
        &vec!["BTC-USD".to_string(), "ETH-USD".to_string()]
    );
}

#[test]
fn subscribe_orderbook() {
    gen_test_subscribe_orderbook!(CoinbaseProWSClient, &vec!["BTC-USD".to_string()]);
}
