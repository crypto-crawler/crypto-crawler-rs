use crypto_ws_client::{BitfinexWSClient, WSClient};

#[macro_use]
mod utils;

#[test]
fn bitfinex_spot() {
    gen_test_subscribe!(BitfinexWSClient, &vec!["trades:tBTCUST".to_string()]);
}

#[test]
fn bitfinex_swap() {
    gen_test_subscribe!(BitfinexWSClient, &vec!["trades:tBTCF0:USTF0".to_string()]);
}
