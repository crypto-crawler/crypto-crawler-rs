use crypto_ws_client::{
    BinanceFutureWSClient, BinanceInverseSwapWSClient, BinanceLinearSwapWSClient,
    BinanceSpotWSClient, WSClient,
};

#[macro_use]
mod utils;

#[test]
fn binance_spot() {
    gen_test!(
        BinanceSpotWSClient,
        &vec!["btcusdt@aggTrade".to_string(), "btcusdt@ticker".to_string()]
    );
}

#[test]
fn binance_futures() {
    gen_test!(
        BinanceFutureWSClient,
        &vec!["btcusd_210625@aggTrade".to_string()]
    );
}

#[test]
fn binance_inverse_swap() {
    gen_test!(
        BinanceInverseSwapWSClient,
        &vec!["btcusd_perp@aggTrade".to_string()]
    );
}

#[test]
fn binance_linear_swap() {
    gen_test!(
        BinanceLinearSwapWSClient,
        &vec!["btcusdt@aggTrade".to_string()]
    );
}
