use crypto_ws_client::{CoinbaseProWSClient, WSClient};

#[macro_use]
mod utils;

#[test]
fn coinbase_pro_spot() {
    gen_test_subscribe!(
        CoinbaseProWSClient,
        &vec![
            "matches:BTC-USD".to_string(),
            "heartbeat:BTC-USD".to_string()
        ]
    );
}
