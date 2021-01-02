use crypto_ws_client::{MXCSpotWSClient, MXCSwapWSClient, WSClient};

#[macro_use]
mod utils;

#[test]
fn mxc_spot() {
    gen_test_subscribe!(MXCSpotWSClient, &vec!["symbol:BTC_USDT".to_string()]);
}

#[test]
fn mxc_swap() {
    gen_test_subscribe!(MXCSwapWSClient, &vec!["deal:BTC_USDT".to_string()]);
}
