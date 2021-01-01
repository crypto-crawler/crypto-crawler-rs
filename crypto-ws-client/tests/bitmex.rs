use crypto_ws_client::{BitMEXWSClient, WSClient};

#[macro_use]
mod utils;

#[test]
fn bitmex_swap() {
    gen_test!(
        BitMEXWSClient,
        &vec!["trade:XBTUSD".to_string(), "quote:XBTUSD".to_string()]
    );
}

#[test]
fn bitmex_futures() {
    gen_test!(
        BitMEXWSClient,
        &vec!["trade:XBTM21".to_string(), "quote:XBTM21".to_string()]
    );
}

#[test]
fn bitmex_instrument() {
    gen_test!(BitMEXWSClient, &vec!["instrument".to_string()]);
}
