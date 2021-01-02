use crypto_ws_client::{BitstampWSClient, Trade, WSClient};

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
fn subscribe_trade() {
    gen_test_subscribe_trade!(
        BitstampWSClient,
        &vec!["btcusd".to_string(), "ethusd".to_string()]
    );
}
