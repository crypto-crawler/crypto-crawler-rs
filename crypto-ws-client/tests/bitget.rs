#[macro_use]
mod utils;

#[cfg(test)]
mod bitget_spot {
    use crypto_ws_client::{BitgetSpotWSClient, WSClient};

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_trade() {
        gen_test_code!(
            BitgetSpotWSClient,
            subscribe_trade,
            &vec!["BTCUSDT".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook_topk() {
        gen_test_code!(
            BitgetSpotWSClient,
            subscribe_orderbook_topk,
            &vec!["BTCUSDT".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(
            BitgetSpotWSClient,
            subscribe_orderbook,
            &vec!["BTCUSDT".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_ticker() {
        gen_test_code!(
            BitgetSpotWSClient,
            subscribe_ticker,
            &vec!["BTCUSDT".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(BitgetSpotWSClient, &vec![("BTCUSDT".to_string(), 60)]);
        gen_test_subscribe_candlestick!(BitgetSpotWSClient, &vec![("BTCUSDT".to_string(), 604800)]);
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
            &vec![("trade".to_string(), "BTCUSD".to_string())]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_raw_json() {
        gen_test_code!(
            BitgetSwapWSClient,
            send,
            &vec![r#"{"op":"subscribe","args":[{"channel":"trade","instId":"BTCUSD","instType":"MC"}]}"#.to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_trade() {
        gen_test_code!(
            BitgetSwapWSClient,
            subscribe_trade,
            &vec!["BTCUSD".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook_topk() {
        gen_test_code!(
            BitgetSwapWSClient,
            subscribe_orderbook_topk,
            &vec!["BTCUSD".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(
            BitgetSwapWSClient,
            subscribe_orderbook,
            &vec!["BTCUSD".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_ticker() {
        gen_test_code!(
            BitgetSwapWSClient,
            subscribe_ticker,
            &vec!["BTCUSD".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(BitgetSwapWSClient, &vec![("BTCUSD".to_string(), 60)]);
        gen_test_subscribe_candlestick!(BitgetSwapWSClient, &vec![("BTCUSD".to_string(), 604800)]);
    }

    #[ignore]
    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_funding_rate() {
        gen_test_code!(
            BitgetSwapWSClient,
            subscribe,
            &vec![("funding_rate".to_string(), "BTCUSD".to_string())]
        );
    }
}

#[cfg(test)]
mod bitget_linear_swap {
    use crypto_ws_client::{BitgetSwapWSClient, WSClient};

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_trade() {
        gen_test_code!(
            BitgetSwapWSClient,
            subscribe_trade,
            &vec!["BTCUSDT".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook_topk() {
        gen_test_code!(
            BitgetSwapWSClient,
            subscribe_orderbook_topk,
            &vec!["BTCUSDT".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(
            BitgetSwapWSClient,
            subscribe_orderbook,
            &vec!["BTCUSDT".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_ticker() {
        gen_test_code!(
            BitgetSwapWSClient,
            subscribe_ticker,
            &vec!["BTCUSDT".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(BitgetSwapWSClient, &vec![("BTCUSDT".to_string(), 60)]);
        gen_test_subscribe_candlestick!(BitgetSwapWSClient, &vec![("BTCUSDT".to_string(), 604800)]);
    }

    #[ignore]
    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_funding_rate() {
        gen_test_code!(
            BitgetSwapWSClient,
            subscribe,
            &vec![("funding_rate".to_string(), "BTCUSDT".to_string())]
        );
    }
}
