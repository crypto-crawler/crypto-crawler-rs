use crypto_ws_client::{
    HuobiCoinSwapWSClient, HuobiFuturesWSClient, HuobiOptionWSClient, HuobiSpotWSClient,
    HuobiUsdtSwapWSClient, WSClient,
};

#[macro_use]
mod utils;

#[test]
fn huobi_spot() {
    gen_test!(HuobiSpotWSClient, "market.btcusdt.trade.detail");
}

#[test]
fn huobi_futures() {
    gen_test!(HuobiFuturesWSClient, "market.BTC_CQ.trade.detail");
}

#[test]
fn huobi_usdt_swap() {
    gen_test!(HuobiUsdtSwapWSClient, "market.BTC-USDT.trade.detail");
}

#[test]
fn huobi_coin_swap() {
    gen_test!(HuobiCoinSwapWSClient, "market.BTC-USD.trade.detail");
}

#[test]
fn huobi_option() {
    gen_test!(HuobiOptionWSClient, "market.overview");
}

#[test]
fn huobi_hb10() {
    gen_test!(HuobiSpotWSClient, "market.hb10usdt.trade.detail");
    gen_test!(HuobiSpotWSClient, "market.huobi10.kline.1min");
}
