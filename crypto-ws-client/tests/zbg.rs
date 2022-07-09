#[macro_use]
mod utils;

#[cfg(test)]
mod zbg_spot {
    use crypto_ws_client::{WSClient, ZbgSpotWSClient};

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe() {
        gen_test_code!(
            ZbgSpotWSClient,
            subscribe,
            &[("TRADE".to_string(), "btc_usdt".to_string())]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_raw_json() {
        gen_test_code!(
            ZbgSpotWSClient,
            send,
            &[r#"{"action":"ADD", "dataType":329_TRADE_BTC_USDT}"#.to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_trade() {
        gen_test_code!(ZbgSpotWSClient, subscribe_trade, &["btc_usdt".to_string()]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(
            ZbgSpotWSClient,
            subscribe_orderbook,
            &["btc_usdt".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_ticker() {
        gen_test_code!(ZbgSpotWSClient, subscribe_ticker, &["btc_usdt".to_string()]);
    }

    #[ignore]
    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_ticker_all() {
        gen_test_code!(
            ZbgSpotWSClient,
            send,
            &[r#"{"action":"ADD", "dataType":"ALL_TRADE_STATISTIC_24H"}"#.to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(ZbgSpotWSClient, &[("btc_usdt".to_string(), 60)]);
        gen_test_subscribe_candlestick!(ZbgSpotWSClient, &[("btc_usdt".to_string(), 604800)]);
    }
}

#[cfg(test)]
mod zbg_inverse_swap {
    use crypto_ws_client::{WSClient, ZbgSwapWSClient};

    #[test]
    #[ignore]
    fn subscribe() {
        gen_test_code!(
            ZbgSwapWSClient,
            subscribe,
            &[
                ("future_tick".to_string(), "1000001".to_string()),
                ("future_tick".to_string(), "1000003".to_string())
            ]
        );
    }

    #[test]
    #[ignore]
    fn subscribe_raw_json() {
        gen_test_code!(
            ZbgSwapWSClient,
            send,
            &[
                r#"{"action":"sub", "topic":"future_tick-1000001"}"#.to_string(),
                r#"{"action":"sub", "topic":"future_tick-1000003"}"#.to_string()
            ]
        );
    }

    #[test]
    #[ignore]
    fn subscribe_trade() {
        gen_test_code!(
            ZbgSwapWSClient,
            subscribe_trade,
            &["BTC_USD-R".to_string(), "ETH_USD-R".to_string()]
        );
    }

    #[test]
    #[ignore]
    fn subscribe_orderbook() {
        gen_test_code!(
            ZbgSwapWSClient,
            subscribe_orderbook,
            &["BTC_USD-R".to_string(), "ETH_USD-R".to_string()]
        );
    }

    #[ignore]
    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_ticker() {
        gen_test_code!(
            ZbgSwapWSClient,
            subscribe_ticker,
            &["BTC_USD-R".to_string(), "ETH_USD-R".to_string()]
        );
    }

    #[ignore]
    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_ticker_all() {
        gen_test_code!(
            ZbgSwapWSClient,
            send,
            &[r#"{"action":"sub", "topic":"future_all_indicator"}"#.to_string()]
        );
    }

    #[ignore]
    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(
            ZbgSwapWSClient,
            &[("BTC_USD-R".to_string(), 60), ("ETH_USD-R".to_string(), 60)]
        );
        gen_test_subscribe_candlestick!(
            ZbgSwapWSClient,
            &[
                ("BTC_USD-R".to_string(), 604800),
                ("ETH_USD-R".to_string(), 604800)
            ]
        );
    }
}

#[cfg(test)]
mod zbg_linear_swap {
    use crypto_ws_client::{WSClient, ZbgSwapWSClient};

    #[test]
    #[ignore]
    fn subscribe() {
        gen_test_code!(
            ZbgSwapWSClient,
            subscribe,
            &[
                ("future_tick".to_string(), "1000000".to_string()),
                ("future_tick".to_string(), "1000002".to_string())
            ]
        );
    }

    #[test]
    #[ignore]
    fn subscribe_raw_json() {
        gen_test_code!(
            ZbgSwapWSClient,
            send,
            &[
                r#"{"action":"sub", "topic":"future_tick-1000000"}"#.to_string(),
                r#"{"action":"sub", "topic":"future_tick-1000002"}"#.to_string()
            ]
        );
    }

    #[test]
    #[ignore]
    fn subscribe_trade() {
        gen_test_code!(
            ZbgSwapWSClient,
            subscribe_trade,
            &["BTC_USDT".to_string(), "ETH_USDT".to_string()]
        );
    }

    #[test]
    #[ignore]
    fn subscribe_orderbook() {
        gen_test_code!(
            ZbgSwapWSClient,
            subscribe_orderbook,
            &["BTC_USDT".to_string(), "ETH_USDT".to_string()]
        );
    }

    #[ignore]
    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_ticker() {
        gen_test_code!(
            ZbgSwapWSClient,
            subscribe_ticker,
            &["BTC_USDT".to_string(), "ETH_USDT".to_string()]
        );
    }

    #[ignore]
    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_ticker_all() {
        gen_test_code!(
            ZbgSwapWSClient,
            send,
            &[r#"{"action":"sub", "topic":"future_all_indicator"}"#.to_string()]
        );
    }

    #[ignore]
    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(
            ZbgSwapWSClient,
            &[("BTC_USDT".to_string(), 60), ("ETH_USDT".to_string(), 60)]
        );
        gen_test_subscribe_candlestick!(
            ZbgSwapWSClient,
            &[
                ("BTC_USDT".to_string(), 604800),
                ("ETH_USDT".to_string(), 604800)
            ]
        );
    }
}
