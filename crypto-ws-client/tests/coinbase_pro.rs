use crypto_ws_client::{CoinbaseProWSClient, WSClient};

#[macro_use]
mod utils;

#[tokio::test(flavor = "multi_thread")]
async fn subscribe() {
    gen_test_code!(
        CoinbaseProWSClient,
        subscribe,
        &vec![
            ("matches".to_string(), "BTC-USD".to_string()),
            ("heartbeat".to_string(), "BTC-USD".to_string())
        ]
    );
}

#[tokio::test(flavor = "multi_thread")]
#[should_panic]
async fn subscribe_illegal_symbol() {
    gen_test_code!(
        CoinbaseProWSClient,
        subscribe,
        &vec![("matches".to_string(), "XXX-YYY".to_string())]
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn subscribe_raw_json() {
    gen_test_code!(
        CoinbaseProWSClient,
        send,
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

#[tokio::test(flavor = "multi_thread")]
async fn subscribe_trade() {
    gen_test_code!(
        CoinbaseProWSClient,
        subscribe_trade,
        &vec!["BTC-USD".to_string(), "ETH-USD".to_string()]
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn subscribe_ticker() {
    gen_test_code!(
        CoinbaseProWSClient,
        subscribe_ticker,
        &vec!["BTC-USD".to_string(), "ETH-USD".to_string()]
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn subscribe_orderbook() {
    gen_test_code!(
        CoinbaseProWSClient,
        subscribe_orderbook,
        &vec!["BTC-USD".to_string()]
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn subscribe_l3_orderbook() {
    gen_test_code!(
        CoinbaseProWSClient,
        subscribe_l3_orderbook,
        &vec!["BTC-USD".to_string()]
    );
}
