#[macro_use]
mod utils;

#[cfg(test)]
mod bitget_spot {
    use crypto_ws_client::{BitgetSpotWSClient, WSClient};

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_trade() {
        gen_test_code!(BitgetSpotWSClient, subscribe_trade, &["BTCUSDT".to_string()]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook_topk() {
        gen_test_code!(BitgetSpotWSClient, subscribe_orderbook_topk, &["BTCUSDT".to_string()]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(BitgetSpotWSClient, subscribe_orderbook, &["BTCUSDT".to_string()]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_ticker() {
        gen_test_code!(BitgetSpotWSClient, subscribe_ticker, &["BTCUSDT".to_string()]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(BitgetSpotWSClient, &[("BTCUSDT".to_string(), 60)]);
        gen_test_subscribe_candlestick!(BitgetSpotWSClient, &[("BTCUSDT".to_string(), 604800)]);
    }
}

#[cfg(test)]
mod bitget_inverse_swap {
    use crypto_ws_client::{BitgetSwapWSClient, WSClient};

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe() {
        gen_test_code!(
            BitgetSwapWSClient,
            subscribe,
            &[("trade".to_string(), "BTCUSD".to_string())]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_raw_json() {
        gen_test_code!(
            BitgetSwapWSClient,
            send,
            &[r#"{"op":"subscribe","args":[{"channel":"trade","instId":"BTCUSD","instType":"MC"}]}"#.to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_trade() {
        gen_test_code!(BitgetSwapWSClient, subscribe_trade, &["BTCUSD".to_string()]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook_topk() {
        gen_test_code!(BitgetSwapWSClient, subscribe_orderbook_topk, &["BTCUSD".to_string()]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(BitgetSwapWSClient, subscribe_orderbook, &["BTCUSD".to_string()]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_ticker() {
        gen_test_code!(BitgetSwapWSClient, subscribe_ticker, &["BTCUSD".to_string()]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(BitgetSwapWSClient, &[("BTCUSD".to_string(), 60)]);
        gen_test_subscribe_candlestick!(BitgetSwapWSClient, &[("BTCUSD".to_string(), 604800)]);
    }

    #[ignore]
    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_funding_rate() {
        gen_test_code!(
            BitgetSwapWSClient,
            subscribe,
            &[("funding_rate".to_string(), "BTCUSD".to_string())]
        );
    }
}

#[cfg(test)]
mod bitget_linear_swap {
    use crypto_ws_client::{BitgetSwapWSClient, WSClient};

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_trade() {
        gen_test_code!(BitgetSwapWSClient, subscribe_trade, &["BTCUSDT".to_string()]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook_topk() {
        gen_test_code!(BitgetSwapWSClient, subscribe_orderbook_topk, &["BTCUSDT".to_string()]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(BitgetSwapWSClient, subscribe_orderbook, &["BTCUSDT".to_string()]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_ticker() {
        gen_test_code!(BitgetSwapWSClient, subscribe_ticker, &["BTCUSDT".to_string()]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(BitgetSwapWSClient, &[("BTCUSDT".to_string(), 60)]);
        gen_test_subscribe_candlestick!(BitgetSwapWSClient, &[("BTCUSDT".to_string(), 604800)]);
    }

    #[ignore]
    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_funding_rate() {
        gen_test_code!(
            BitgetSwapWSClient,
            subscribe,
            &[("funding_rate".to_string(), "BTCUSDT".to_string())]
        );
    }
}
