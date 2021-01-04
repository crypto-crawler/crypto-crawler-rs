use crypto_ws_client::{BitstampWSClient, WSClient};

#[macro_use]
mod utils;

#[test]
fn subscribe() {
    gen_test_subscribe!(
        BitstampWSClient,
        &vec![
            "live_trades_btcusd".to_string(),
            "diff_order_book_btcusd".to_string()
        ]
    );
}

#[test]
fn subscribe_raw_json() {
    gen_test_subscribe!(
        BitstampWSClient,
        &vec![r#"{"event":"bts:subscribe","data":{"channel":"live_trades_btcusd"}}"#.to_string()]
    );
}

#[test]
fn subscribe_trade() {
    gen_test_subscribe_trade!(
        BitstampWSClient,
        &vec!["btcusd".to_string(), "ethusd".to_string()]
    );
}

#[test]
fn subscribe_orderbook() {
    gen_test_subscribe_orderbook!(
        BitstampWSClient,
        &vec!["btcusd".to_string(), "ethusd".to_string()]
    );
}
