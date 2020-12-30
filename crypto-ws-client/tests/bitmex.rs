use crypto_ws_client::{BitMEXWSClient, WSClient};

#[macro_use]
mod utils;

#[test]
fn bitmex_swap() {
    gen_test!(BitMEXWSClient, "trade:XBTUSD");
}

#[test]
fn bitmex_futures() {
    gen_test!(BitMEXWSClient, "trade:XBTM21");
}

#[test]
fn bitmex_instrument() {
    gen_test!(BitMEXWSClient, "instrument");
}
