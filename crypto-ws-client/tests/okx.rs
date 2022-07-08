use crypto_ws_client::{OkxWSClient, WSClient};

#[macro_use]
mod utils;

#[tokio::test(flavor = "multi_thread")]
async fn okex_index() {
    gen_test_code!(
        OkxWSClient,
        subscribe,
        &vec![("index-tickers".to_string(), "BTC-USDT".to_string())]
    );
}

#[cfg(test)]
mod okx_spot {
    use crypto_ws_client::{OkxWSClient, WSClient};

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe() {
        gen_test_code!(
            OkxWSClient,
            subscribe,
            &vec![("trades".to_string(), "BTC-USDT".to_string())]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_raw_json() {
        gen_test_code!(
            OkxWSClient,
            send,
            &vec![
                r#"{"op":"subscribe","args":[{"channel":"trades","instId":"BTC-USDT"}]}"#
                    .to_string()
            ]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_trade() {
        gen_test_code!(OkxWSClient, subscribe_trade, &vec!["BTC-USDT".to_string()]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_ticker() {
        gen_test_code!(OkxWSClient, subscribe_ticker, &vec!["BTC-USDT".to_string()]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_bbo() {
        gen_test_code!(OkxWSClient, subscribe_bbo, &vec!["BTC-USDT".to_string()]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(
            OkxWSClient,
            subscribe_orderbook,
            &vec!["BTC-USDT".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook_topk() {
        gen_test_code!(
            OkxWSClient,
            subscribe_orderbook_topk,
            &vec!["BTC-USDT".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(OkxWSClient, &vec![("BTC-USDT".to_string(), 60)]);
        gen_test_subscribe_candlestick!(OkxWSClient, &vec![("BTC-USDT".to_string(), 604800)]);
    }
}

#[cfg(test)]
mod okx_future {
    use crypto_ws_client::{OkxWSClient, WSClient};

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe() {
        gen_test_code!(
            OkxWSClient,
            subscribe,
            &vec![("trades".to_string(), "BTC-USDT-220930".to_string())]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_trade() {
        gen_test_code!(
            OkxWSClient,
            subscribe_trade,
            &vec!["BTC-USDT-220930".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_ticker() {
        gen_test_code!(
            OkxWSClient,
            subscribe_ticker,
            &vec!["BTC-USDT-220930".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_bbo() {
        gen_test_code!(
            OkxWSClient,
            subscribe_bbo,
            &vec!["BTC-USDT-220930".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(
            OkxWSClient,
            subscribe_orderbook,
            &vec!["BTC-USDT-220930".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook_topk() {
        gen_test_code!(
            OkxWSClient,
            subscribe_orderbook_topk,
            &vec!["BTC-USDT-220930".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(OkxWSClient, &vec![("BTC-USDT-220930".to_string(), 60)]);
        gen_test_subscribe_candlestick!(
            OkxWSClient,
            &vec![("BTC-USDT-220930".to_string(), 604800)]
        );
    }
}

#[cfg(test)]
mod okx_swap {
    use crypto_ws_client::{OkxWSClient, WSClient};

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe() {
        gen_test_code!(
            OkxWSClient,
            subscribe,
            &vec![("trades".to_string(), "BTC-USDT-SWAP".to_string())]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_trade() {
        gen_test_code!(
            OkxWSClient,
            subscribe_trade,
            &vec!["BTC-USDT-SWAP".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_ticker() {
        gen_test_code!(
            OkxWSClient,
            subscribe_ticker,
            &vec!["BTC-USDT-SWAP".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_bbo() {
        gen_test_code!(
            OkxWSClient,
            subscribe_bbo,
            &vec!["BTC-USDT-SWAP".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(
            OkxWSClient,
            subscribe_orderbook,
            &vec!["BTC-USDT-SWAP".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook_topk() {
        gen_test_code!(
            OkxWSClient,
            subscribe_orderbook_topk,
            &vec!["BTC-USDT-SWAP".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(OkxWSClient, &vec![("BTC-USDT-SWAP".to_string(), 60)]);
        gen_test_subscribe_candlestick!(OkxWSClient, &vec![("BTC-USDT-SWAP".to_string(), 604800)]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_funding_rate() {
        gen_test_code!(
            OkxWSClient,
            subscribe,
            &vec![("funding-rate".to_string(), "BTC-USDT-SWAP".to_string())]
        );
    }
}

#[cfg(test)]
mod okx_option {
    use crypto_ws_client::{OkxWSClient, WSClient};

    #[tokio::test(flavor = "multi_thread")]
    #[ignore = "lack of liquidity"]
    async fn subscribe_trade() {
        gen_test_code!(
            OkxWSClient,
            subscribe_trade,
            &vec!["BTC-USD-220930-50000-C".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_ticker() {
        gen_test_code!(
            OkxWSClient,
            subscribe_ticker,
            &vec!["BTC-USD-220930-50000-C".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_bbo() {
        gen_test_code!(
            OkxWSClient,
            subscribe_bbo,
            &vec!["BTC-USD-220930-50000-C".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(
            OkxWSClient,
            subscribe_orderbook,
            &vec!["BTC-USD-220930-50000-C".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook_topk() {
        gen_test_code!(
            OkxWSClient,
            subscribe_orderbook_topk,
            &vec!["BTC-USD-220930-50000-C".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    #[ignore = "lack of liquidity"]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(
            OkxWSClient,
            &vec![("BTC-USD-220930-50000-C".to_string(), 60)]
        );
        gen_test_subscribe_candlestick!(
            OkxWSClient,
            &vec![("BTC-USD-220930-50000-C".to_string(), 604800)]
        );
    }
}
