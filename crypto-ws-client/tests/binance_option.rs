use crypto_ws_client::{BinanceOptionWSClient, WSClient};

#[macro_use]
mod utils;

#[ignore]
#[tokio::test(flavor = "multi_thread")]
async fn subscribe() {
    gen_test_code!(
        BinanceOptionWSClient,
        subscribe,
        &vec![
            ("TICKER_ALL".to_string(), "BTCUSDT".to_string()),
            ("TRADE_ALL".to_string(), "BTCUSDT_C".to_string()),
            ("TRADE_ALL".to_string(), "BTCUSDT_P".to_string())
        ]
    );
}

#[tokio::test(flavor = "multi_thread")]
#[ignore]
async fn subscribe_trade() {
    gen_test_code!(
        BinanceOptionWSClient,
        subscribe_trade,
        &vec![
            "BTC-220325-40000-C".to_string(),
            "BTC-220325-35000-P".to_string()
        ]
    );
}

#[tokio::test(flavor = "multi_thread")]
#[ignore]
async fn subscribe_ticker() {
    gen_test_code!(
        BinanceOptionWSClient,
        subscribe_ticker,
        &vec![
            "BTC-220325-40000-C".to_string(),
            "BTC-220325-35000-P".to_string()
        ]
    );
}

#[ignore]
#[tokio::test(flavor = "multi_thread")]
async fn subscribe_ticker_all() {
    gen_test_code!(
        BinanceOptionWSClient,
        subscribe,
        &vec![("TICKER_ALL".to_string(), "BTCUSDT".to_string())]
    );
}

#[tokio::test(flavor = "multi_thread")]
#[ignore]
async fn subscribe_orderbook() {
    gen_test_code!(
        BinanceOptionWSClient,
        subscribe_orderbook,
        &vec![
            "BTC-220325-40000-C".to_string(),
            "BTC-220325-35000-P".to_string()
        ]
    );
}

#[tokio::test(flavor = "multi_thread")]
#[ignore]
async fn subscribe_orderbook_topk() {
    gen_test_code!(
        BinanceOptionWSClient,
        subscribe_orderbook_topk,
        &vec![
            "BTC-220325-40000-C".to_string(),
            "BTC-220325-35000-P".to_string()
        ]
    );
}

#[tokio::test(flavor = "multi_thread")]
#[ignore]
async fn subscribe_candlestick() {
    gen_test_subscribe_candlestick!(
        BinanceOptionWSClient,
        &vec![
            ("BTC-220325-40000-C".to_string(), 60),
            ("BTC-220325-35000-P".to_string(), 60),
        ]
    );
    gen_test_subscribe_candlestick!(
        BinanceOptionWSClient,
        &vec![
            ("BTC-220325-40000-C".to_string(), 60),
            ("BTC-220325-35000-P".to_string(), 60),
        ]
    );
}
