use crypto_ws_client::{OkxWSClient, WSClient};

#[macro_use]
mod utils;

#[tokio::test(flavor = "multi_thread")]
async fn okex_index() {
    gen_test_code!(
        OkxWSClient,
        subscribe,
        &[("index-tickers".to_string(), "BTC-USDT".to_string())]
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
            &[("trades".to_string(), "BTC-USDT".to_string())]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_raw_json() {
        gen_test_code!(
            OkxWSClient,
            send,
            &[
                r#"{"op":"subscribe","args":[{"channel":"trades","instId":"BTC-USDT"}]}"#
                    .to_string()
            ]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_trade() {
        gen_test_code!(OkxWSClient, subscribe_trade, &["BTC-USDT".to_string()]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_ticker() {
        gen_test_code!(OkxWSClient, subscribe_ticker, &["BTC-USDT".to_string()]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_bbo() {
        gen_test_code!(OkxWSClient, subscribe_bbo, &["BTC-USDT".to_string()]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(OkxWSClient, subscribe_orderbook, &["BTC-USDT".to_string()]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook_topk() {
        gen_test_code!(
            OkxWSClient,
            subscribe_orderbook_topk,
            &["BTC-USDT".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(OkxWSClient, &[("BTC-USDT".to_string(), 60)]);
        gen_test_subscribe_candlestick!(OkxWSClient, &[("BTC-USDT".to_string(), 604800)]);
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
            &[("trades".to_string(), "BTC-USDT-220930".to_string())]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_trade() {
        gen_test_code!(
            OkxWSClient,
            subscribe_trade,
            &["BTC-USDT-220930".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_ticker() {
        gen_test_code!(
            OkxWSClient,
            subscribe_ticker,
            &["BTC-USDT-220930".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_bbo() {
        gen_test_code!(OkxWSClient, subscribe_bbo, &["BTC-USDT-220930".to_string()]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(
            OkxWSClient,
            subscribe_orderbook,
            &["BTC-USDT-220930".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook_topk() {
        gen_test_code!(
            OkxWSClient,
            subscribe_orderbook_topk,
            &["BTC-USDT-220930".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(OkxWSClient, &[("BTC-USDT-220930".to_string(), 60)]);
        gen_test_subscribe_candlestick!(OkxWSClient, &[("BTC-USDT-220930".to_string(), 604800)]);
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
            &[("trades".to_string(), "BTC-USDT-SWAP".to_string())]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_trade() {
        gen_test_code!(OkxWSClient, subscribe_trade, &["BTC-USDT-SWAP".to_string()]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_ticker() {
        gen_test_code!(
            OkxWSClient,
            subscribe_ticker,
            &["BTC-USDT-SWAP".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_bbo() {
        gen_test_code!(OkxWSClient, subscribe_bbo, &["BTC-USDT-SWAP".to_string()]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(
            OkxWSClient,
            subscribe_orderbook,
            &["BTC-USDT-SWAP".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook_topk() {
        gen_test_code!(
            OkxWSClient,
            subscribe_orderbook_topk,
            &["BTC-USDT-SWAP".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(OkxWSClient, &[("BTC-USDT-SWAP".to_string(), 60)]);
        gen_test_subscribe_candlestick!(OkxWSClient, &[("BTC-USDT-SWAP".to_string(), 604800)]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_funding_rate() {
        gen_test_code!(
            OkxWSClient,
            subscribe,
            &[("funding-rate".to_string(), "BTC-USDT-SWAP".to_string())]
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
            &["BTC-USD-220930-50000-C".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_ticker() {
        gen_test_code!(
            OkxWSClient,
            subscribe_ticker,
            &["BTC-USD-220930-50000-C".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_bbo() {
        gen_test_code!(
            OkxWSClient,
            subscribe_bbo,
            &["BTC-USD-220930-50000-C".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(
            OkxWSClient,
            subscribe_orderbook,
            &["BTC-USD-220930-50000-C".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook_topk() {
        gen_test_code!(
            OkxWSClient,
            subscribe_orderbook_topk,
            &["BTC-USD-220930-50000-C".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    #[ignore = "lack of liquidity"]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(OkxWSClient, &[("BTC-USD-220930-50000-C".to_string(), 60)]);
        gen_test_subscribe_candlestick!(
            OkxWSClient,
            &[("BTC-USD-220930-50000-C".to_string(), 604800)]
        );
    }
}
