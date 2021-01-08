use crypto_ws_client::{BitstampWSClient, Level3OrderBook, WSClient};

#[macro_use]
mod utils;

#[test]
fn subscribe() {
    gen_test_code!(
        BitstampWSClient,
        subscribe,
        &vec![
            "live_trades_btcusd".to_string(),
            "diff_order_book_btcusd".to_string()
        ]
    );
}

#[test]
fn subscribe_raw_json() {
    gen_test_code!(
        BitstampWSClient,
        subscribe,
        &vec![r#"{"event":"bts:subscribe","data":{"channel":"live_trades_btcusd"}}"#.to_string()]
    );
}

#[test]
fn subscribe_trade() {
    gen_test_code!(
        BitstampWSClient,
        subscribe_trade,
        &vec!["btcusd".to_string(), "ethusd".to_string()]
    );
}

#[test]
fn subscribe_orderbook() {
    gen_test_code!(
        BitstampWSClient,
        subscribe_orderbook,
        &vec!["btcusd".to_string(), "ethusd".to_string()]
    );
}

#[test]
fn subscribe_orderbook_snapshot() {
    gen_test_code!(
        BitstampWSClient,
        subscribe_orderbook_snapshot,
        &vec!["btcusd".to_string(), "ethusd".to_string()]
    );
}

#[test]
fn subscribe_l3_orderbook() {
    gen_test_code!(
        BitstampWSClient,
        subscribe_l3_orderbook,
        &vec!["btcusd".to_string(), "ethusd".to_string()]
    );
}
