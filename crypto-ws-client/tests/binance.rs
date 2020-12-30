use crypto_ws_client::{
    BinanceDeliveryWSClient, BinanceFuturesWSClient, BinanceSpotWSClient, WSClient,
};

#[macro_use]
mod utils;

#[test]
fn binance_spot() {
    gen_test!(BinanceSpotWSClient, "btcusdt@aggTrade");
}

#[test]
fn binance_futures() {
    gen_test!(BinanceFuturesWSClient, "btcusdt@aggTrade");
}

#[test]
fn binance_delivery() {
    gen_test!(BinanceDeliveryWSClient, "btcusd_perp@aggTrade");
}
