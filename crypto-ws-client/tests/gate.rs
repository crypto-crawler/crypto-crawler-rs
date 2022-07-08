#[macro_use]
mod utils;

#[cfg(test)]
mod gate_spot {
    use crypto_ws_client::{GateSpotWSClient, WSClient};

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe() {
        gen_test_code!(
            GateSpotWSClient,
            subscribe,
            &[
                ("trades".to_string(), "BTC_USDT".to_string()),
                ("trades".to_string(), "ETH_USDT".to_string())
            ]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_raw_json() {
        gen_test_code!(
            GateSpotWSClient,
            send,
            &[r#"{"channel":"spot.trades", "event":"subscribe", "payload":["BTC_USDT","ETH_USDT"]}"#.to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_trade() {
        gen_test_code!(GateSpotWSClient, subscribe_trade, &["BTC_USDT".to_string()]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(
            GateSpotWSClient,
            subscribe_orderbook,
            &["BTC_USDT".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook_topk() {
        gen_test_code!(
            GateSpotWSClient,
            subscribe_orderbook_topk,
            &["BTC_USDT".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_bbo() {
        gen_test_code!(GateSpotWSClient, subscribe_bbo, &["BTC_USDT".to_string()]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_ticker() {
        gen_test_code!(
            GateSpotWSClient,
            subscribe_ticker,
            &["BTC_USDT".to_string()]
        );
    }

    #[ignore = "too slow"]
    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(GateSpotWSClient, &[("BTC_USDT".to_string(), 10)]);
        gen_test_subscribe_candlestick!(GateSpotWSClient, &[("BTC_USDT".to_string(), 604800)]);
    }
}

#[cfg(test)]
mod gate_inverse_swap {
    use crypto_ws_client::{GateInverseSwapWSClient, WSClient};

    #[ignore = "lack of liquidity"]
    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe() {
        gen_test_code!(
            GateInverseSwapWSClient,
            subscribe,
            &[
                ("trades".to_string(), "BTC_USD".to_string()),
                ("trades".to_string(), "ETH_USD".to_string()),
                ("trades".to_string(), "BNB_USD".to_string()),
                ("trades".to_string(), "XRP_USD".to_string())
            ]
        );
    }

    #[ignore = "lack of liquidity"]
    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_raw_json() {
        gen_test_code!(
            GateInverseSwapWSClient,
            send,
            &[r#"{"channel":"futures.trades", "event":"subscribe", "payload":["BTC_USD","ETH_USD","BNB_USD","XRP_USD"]}"#
                    .to_string()]
        );
    }

    #[ignore = "lack of liquidity"]
    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_trade() {
        gen_test_code!(
            GateInverseSwapWSClient,
            subscribe_trade,
            &["BTC_USD".to_string(), "ETH_USD".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(
            GateInverseSwapWSClient,
            subscribe_orderbook,
            &["BTC_USD".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook_topk() {
        gen_test_code!(
            GateInverseSwapWSClient,
            subscribe_orderbook_topk,
            &["BTC_USD".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_bbo() {
        gen_test_code!(
            GateInverseSwapWSClient,
            subscribe_bbo,
            &["BTC_USD".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_ticker() {
        gen_test_code!(
            GateInverseSwapWSClient,
            subscribe_ticker,
            &["BTC_USD".to_string()]
        );
    }

    #[ignore]
    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(GateInverseSwapWSClient, &[("BTC_USD".to_string(), 10)]);
        gen_test_subscribe_candlestick!(
            GateInverseSwapWSClient,
            &[("BTC_USD".to_string(), 604800)]
        );
    }
}

#[cfg(test)]
mod gate_linear_swap {
    use crypto_ws_client::{GateLinearSwapWSClient, WSClient};

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_trade() {
        gen_test_code!(
            GateLinearSwapWSClient,
            subscribe_trade,
            &["BTC_USDT".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(
            GateLinearSwapWSClient,
            subscribe_orderbook,
            &["BTC_USDT".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook_topk() {
        gen_test_code!(
            GateLinearSwapWSClient,
            subscribe_orderbook_topk,
            &["BTC_USDT".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_bbo() {
        gen_test_code!(
            GateLinearSwapWSClient,
            subscribe_bbo,
            &["BTC_USDT".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_ticker() {
        gen_test_code!(
            GateLinearSwapWSClient,
            subscribe_ticker,
            &["BTC_USDT".to_string()]
        );
    }

    #[ignore]
    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(GateLinearSwapWSClient, &[("BTC_USDT".to_string(), 10)]);
        gen_test_subscribe_candlestick!(
            GateLinearSwapWSClient,
            &[("BTC_USDT".to_string(), 604800)]
        );
    }
}

#[cfg(test)]
mod gate_inverse_future {
    use crypto_ws_client::{GateInverseFutureWSClient, WSClient};

    #[test]
    #[ignore]
    fn subscribe_trade() {
        gen_test_code!(
            GateInverseFutureWSClient,
            subscribe_trade,
            &["BTC_USD_20220930".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(
            GateInverseFutureWSClient,
            subscribe_orderbook,
            &["BTC_USD_20220930".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_ticker() {
        gen_test_code!(
            GateInverseFutureWSClient,
            subscribe_ticker,
            &["BTC_USD_20220930".to_string()]
        );
    }

    #[ignore]
    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(
            GateInverseFutureWSClient,
            &[("BTC_USD_20220930".to_string(), 10)]
        );
        gen_test_subscribe_candlestick!(
            GateInverseFutureWSClient,
            &[("BTC_USD_20220930".to_string(), 604800)]
        );
    }
}

#[cfg(test)]
mod gate_linear_future {
    use crypto_ws_client::{GateLinearFutureWSClient, WSClient};

    #[test]
    #[ignore]
    fn subscribe_trade() {
        gen_test_code!(
            GateLinearFutureWSClient,
            subscribe_trade,
            &["BTC_USDT_20220930".to_string()]
        );
    }

    #[test]
    #[ignore]
    fn subscribe_orderbook() {
        gen_test_code!(
            GateLinearFutureWSClient,
            subscribe_orderbook,
            &["BTC_USDT_20220930".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_ticker() {
        gen_test_code!(
            GateLinearFutureWSClient,
            subscribe_ticker,
            &["BTC_USDT_20220930".to_string()]
        );
    }

    #[ignore]
    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(
            GateLinearFutureWSClient,
            &[("BTC_USDT_20220930".to_string(), 10)]
        );
        gen_test_subscribe_candlestick!(
            GateLinearFutureWSClient,
            &[("BTC_USDT_20220930".to_string(), 604800)]
        );
    }
}
