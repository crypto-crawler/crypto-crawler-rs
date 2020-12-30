use crypto_ws_client::{OKExWSClient, WSClient};

#[macro_use]
mod utils;

#[test]
fn okex_spot() {
    gen_test!(OKExWSClient, "spot/trade:BTC-USDT");
}

#[test]
fn okex_futures() {
    gen_test!(OKExWSClient, "futures/trade:BTC-USDT-210625");
}

#[test]
fn okex_swap() {
    gen_test!(OKExWSClient, "swap/trade:BTC-USDT-SWAP");
}

#[test]
fn okex_option() {
    gen_test!(OKExWSClient, "option/summary:BTC-USD");
}

#[test]
fn okex_index() {
    gen_test!(OKExWSClient, "index/ticker:BTC-USDT");
}
