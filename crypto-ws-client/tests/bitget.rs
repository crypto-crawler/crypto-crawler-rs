#[macro_use]
mod utils;

#[cfg(test)]
mod bitget_inverse_swap {
    use crypto_ws_client::{BitgetSwapWSClient, WSClient};

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe() {
        gen_test_code!(
            BitgetSwapWSClient,
            subscribe,
            &vec![("trade".to_string(), "btcusd".to_string())]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_raw_json() {
        gen_test_code!(
            BitgetSwapWSClient,
            send,
            &vec![r#"{"op":"subscribe","args":["swap/trade:btcusd"]}"#.to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_trade() {
        gen_test_code!(
            BitgetSwapWSClient,
            subscribe_trade,
            &vec!["btcusd".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook_topk() {
        gen_test_code!(
            BitgetSwapWSClient,
            subscribe_orderbook_topk,
            &vec!["btcusd".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(
            BitgetSwapWSClient,
            subscribe_orderbook,
            &vec!["btcusd".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_ticker() {
        gen_test_code!(
            BitgetSwapWSClient,
            subscribe_ticker,
            &vec!["btcusd".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(BitgetSwapWSClient, &vec![("btcusd".to_string(), 60)]);
        gen_test_subscribe_candlestick!(BitgetSwapWSClient, &vec![("btcusd".to_string(), 604800)]);
    }

    #[ignore]
    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_funding_rate() {
        gen_test_code!(
            BitgetSwapWSClient,
            subscribe,
            &vec![("funding_rate".to_string(), "btcusd".to_string())]
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
            &vec!["cmt_btcusdt".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook_topk() {
        gen_test_code!(
            BitgetSwapWSClient,
            subscribe_orderbook_topk,
            &vec!["cmt_btcusdt".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(
            BitgetSwapWSClient,
            subscribe_orderbook,
            &vec!["cmt_btcusdt".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_ticker() {
        gen_test_code!(
            BitgetSwapWSClient,
            subscribe_ticker,
            &vec!["cmt_btcusdt".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(BitgetSwapWSClient, &vec![("cmt_btcusdt".to_string(), 60)]);
        gen_test_subscribe_candlestick!(
            BitgetSwapWSClient,
            &vec![("cmt_btcusdt".to_string(), 604800)]
        );
    }

    #[ignore]
    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_funding_rate() {
        gen_test_code!(
            BitgetSwapWSClient,
            subscribe,
            &vec![("funding_rate".to_string(), "cmt_btcusdt".to_string())]
        );
    }
}
