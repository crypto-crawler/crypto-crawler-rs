use crypto_ws_client::{
    BinanceDeliveryWSClient, BinanceFuturesWSClient, BinanceSpotWSClient, WSClient,
};

#[macro_use]
mod utils;

#[test]
fn binance_spot() {
    gen_test!(BinanceSpotWSClient, &vec!["btcusdt@aggTrade".to_string()]);
}

#[test]
fn binance_futures() {
    gen_test!(
        BinanceFuturesWSClient,
        &vec!["btcusdt@aggTrade".to_string()]
    );
}

#[test]
fn binance_delivery() {
    gen_test!(
        BinanceDeliveryWSClient,
        &vec!["btcusd_perp@aggTrade".to_string()]
    );
}
