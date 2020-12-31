use crypto_ws_client::{
    HuobiCoinSwapWSClient, HuobiFuturesWSClient, HuobiOptionWSClient, HuobiSpotWSClient,
    HuobiUsdtSwapWSClient, WSClient,
};

#[macro_use]
mod utils;

#[test]
fn huobi_spot() {
    gen_test!(
        HuobiSpotWSClient,
        &vec!["market.btcusdt.trade.detail".to_string()]
    );
}

#[test]
fn huobi_futures() {
    gen_test!(
        HuobiFuturesWSClient,
        &vec!["market.BTC_CQ.trade.detail".to_string()]
    );
}

#[test]
fn huobi_usdt_swap() {
    gen_test!(
        HuobiUsdtSwapWSClient,
        &vec!["market.BTC-USDT.trade.detail".to_string()]
    );
}

#[test]
fn huobi_coin_swap() {
    gen_test!(
        HuobiCoinSwapWSClient,
        &vec!["market.BTC-USD.trade.detail".to_string()]
    );
}

#[test]
fn huobi_option() {
    gen_test!(HuobiOptionWSClient, &vec!["market.overview".to_string()]);
}

#[test]
fn huobi_hb10() {
    gen_test!(
        HuobiSpotWSClient,
        &vec![
            "market.hb10usdt.trade.detail".to_string(),
            "market.huobi10.kline.1min".to_string()
        ]
    );
}
