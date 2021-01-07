use crypto_ws_client::{CoinbaseProWSClient, WSClient};

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
#[should_panic]
fn subscribe_illegal_symbol() {
    gen_test_subscribe!(CoinbaseProWSClient, &vec!["matches:XXX-YYY".to_string(),]);
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

#[test]
fn subscribe_l3_orderbook() {
    let mut messages = Vec::<String>::new();
    {
        let on_msg = |msg: String| messages.push(msg);
        let mut ws_client = CoinbaseProWSClient::new(Box::new(on_msg), None);
        ws_client.subscribe_orderbook(&vec!["BTC-USD".to_string()]);
        ws_client.run(Some(0)); // return immediately once after a normal message
    }
    assert!(!messages.is_empty());
}
