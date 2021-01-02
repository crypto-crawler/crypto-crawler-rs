use crypto_ws_client::{CoinbaseProWSClient, Trade, WSClient};

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
fn subscribe_trade() {
    gen_test_subscribe_trade!(
        CoinbaseProWSClient,
        &vec!["BTC-USD".to_string(), "ETH-USD".to_string()]
    );
}
