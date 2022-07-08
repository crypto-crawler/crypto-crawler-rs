#[macro_use]
mod utils;

#[cfg(test)]
mod mexc_spot {
    use crypto_ws_client::{MexcSpotWSClient, WSClient};

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe() {
        gen_test_code!(
            MexcSpotWSClient,
            subscribe,
            &[("deal".to_string(), "BTC_USDT".to_string()),
                ("deal".to_string(), "ETH_USDT".to_string()),
                ("deal".to_string(), "MX_USDT".to_string())]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_raw_json() {
        gen_test_code!(
            MexcSpotWSClient,
            send,
            &[r#"{"op":"sub.deal","symbol":"BTC_USDT"}"#.to_string(),
                r#"{"op":"sub.deal","symbol":"ETH_USDT"}"#.to_string(),
                r#"{"op":"sub.deal","symbol":"MX_USDT"}"#.to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_trade() {
        gen_test_code!(
            MexcSpotWSClient,
            subscribe_trade,
            &["BTC_USDT".to_string(),
                "ETH_USDT".to_string(),
                "MX_USDT".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(
            MexcSpotWSClient,
            subscribe_orderbook,
            &["BTC_USDT".to_string(),
                "ETH_USDT".to_string(),
                "MX_USDT".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook_topk() {
        gen_test_code!(
            MexcSpotWSClient,
            subscribe_orderbook_topk,
            &["BTC_USDT".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(
            MexcSpotWSClient,
            &[("BTC_USDT".to_string(), 60),
                ("ETH_USDT".to_string(), 60),
                ("MX_USDT".to_string(), 60)]
        );
        gen_test_subscribe_candlestick!(
            MexcSpotWSClient,
            &[("BTC_USDT".to_string(), 2592000),
                ("ETH_USDT".to_string(), 2592000),
                ("MX_USDT".to_string(), 2592000)]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_overview() {
        gen_test_code!(
            MexcSpotWSClient,
            send,
            &[r#"{"op":"sub.overview"}"#.to_string()]
        );
    }
}

#[cfg(test)]
mod mexc_linear_swap {
    use crypto_ws_client::{MexcSwapWSClient, WSClient};

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe() {
        gen_test_code!(
            MexcSwapWSClient,
            subscribe,
            &[("deal".to_string(), "BTC_USDT".to_string()),
                ("deal".to_string(), "ETH_USDT".to_string())]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_raw_json() {
        gen_test_code!(
            MexcSwapWSClient,
            send,
            &[r#"{"method":"sub.deal","param":{"symbol":"BTC_USDT"}}"#.to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_trade() {
        gen_test_code!(
            MexcSwapWSClient,
            subscribe_trade,
            &["BTC_USDT".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_ticker() {
        gen_test_code!(
            MexcSwapWSClient,
            subscribe_ticker,
            &["BTC_USDT".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(
            MexcSwapWSClient,
            subscribe_orderbook,
            &["BTC_USDT".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook_topk() {
        gen_test_code!(
            MexcSwapWSClient,
            subscribe_orderbook_topk,
            &["BTC_USDT".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(MexcSwapWSClient, &[("BTC_USDT".to_string(), 60)]);
        gen_test_subscribe_candlestick!(MexcSwapWSClient, &[("BTC_USDT".to_string(), 2592000)]);
    }
}

#[cfg(test)]
mod mexc_inverse_swap {
    use crypto_ws_client::{MexcSwapWSClient, WSClient};

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_trade() {
        gen_test_code!(
            MexcSwapWSClient,
            subscribe_trade,
            &["BTC_USD".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_ticker() {
        gen_test_code!(
            MexcSwapWSClient,
            subscribe_ticker,
            &["BTC_USD".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(
            MexcSwapWSClient,
            subscribe_orderbook,
            &["BTC_USD".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook_topk() {
        gen_test_code!(
            MexcSwapWSClient,
            subscribe_orderbook_topk,
            &["BTC_USD".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(MexcSwapWSClient, &[("BTC_USD".to_string(), 60)]);
        gen_test_subscribe_candlestick!(MexcSwapWSClient, &[("BTC_USD".to_string(), 2592000)]);
    }
}
