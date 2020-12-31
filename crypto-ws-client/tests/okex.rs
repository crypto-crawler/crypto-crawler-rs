use crypto_ws_client::{OKExWSClient, WSClient};

#[macro_use]
mod utils;

#[test]
fn okex_spot() {
    gen_test!(OKExWSClient, &vec!["spot/trade:BTC-USDT".to_string()]);
}

#[test]
fn okex_futures() {
    gen_test!(
        OKExWSClient,
        &vec!["futures/trade:BTC-USDT-210625".to_string()]
    );
}

#[test]
fn okex_swap() {
    gen_test!(OKExWSClient, &vec!["swap/trade:BTC-USDT-SWAP".to_string()]);
}

#[test]
fn okex_option() {
    gen_test!(OKExWSClient, &vec!["option/summary:BTC-USD".to_string()]);
}

#[test]
fn okex_index() {
    gen_test!(OKExWSClient, &vec!["index/ticker:BTC-USDT".to_string()]);
}
