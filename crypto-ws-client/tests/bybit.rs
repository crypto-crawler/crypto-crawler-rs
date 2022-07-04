#[macro_use]
mod utils;

#[cfg(test)]
mod bybit_inverse_future {
    use crypto_ws_client::{BybitInverseWSClient, WSClient};

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe() {
        gen_test_code!(
            BybitInverseWSClient,
            subscribe,
            &vec![("trade".to_string(), "BTCUSDU22".to_string())]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_raw_json() {
        gen_test_code!(
            BybitInverseWSClient,
            send,
            &vec![r#"{"op":"subscribe","args":["trade.BTCUSDU22"]}"#.to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_trade() {
        gen_test_code!(
            BybitInverseWSClient,
            subscribe_trade,
            &vec!["BTCUSDU22".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(
            BybitInverseWSClient,
            subscribe_orderbook,
            &vec!["BTCUSDU22".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_ticker() {
        gen_test_code!(
            BybitInverseWSClient,
            subscribe_ticker,
            &vec!["BTCUSDU22".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(BybitInverseWSClient, &vec![("BTCUSDU22".to_string(), 60)]);
        gen_test_subscribe_candlestick!(
            BybitInverseWSClient,
            &vec![("BTCUSDU22".to_string(), 2592000)]
        );
    }
}

#[cfg(test)]
mod bybit_inverse_swap {
    use crypto_ws_client::{BybitInverseWSClient, WSClient};

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe() {
        gen_test_code!(
            BybitInverseWSClient,
            subscribe,
            &vec![("trade".to_string(), "BTCUSD".to_string())]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_raw_json() {
        gen_test_code!(
            BybitInverseWSClient,
            send,
            &vec![r#"{"op":"subscribe","args":["trade.BTCUSD"]}"#.to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_trade() {
        gen_test_code!(
            BybitInverseWSClient,
            subscribe_trade,
            &vec!["BTCUSD".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(
            BybitInverseWSClient,
            subscribe_orderbook,
            &vec!["BTCUSD".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_ticker() {
        gen_test_code!(
            BybitInverseWSClient,
            subscribe_ticker,
            &vec!["BTCUSD".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(BybitInverseWSClient, &vec![("BTCUSD".to_string(), 60)]);
        gen_test_subscribe_candlestick!(
            BybitInverseWSClient,
            &vec![("BTCUSD".to_string(), 2592000)]
        );
    }
}

#[cfg(test)]
mod bybit_linear_swap {
    use crypto_ws_client::{BybitLinearSwapWSClient, WSClient};

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_trade() {
        gen_test_code!(
            BybitLinearSwapWSClient,
            subscribe_trade,
            &vec!["BTCUSDT".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(
            BybitLinearSwapWSClient,
            subscribe_orderbook,
            &vec!["BTCUSDT".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_ticker() {
        gen_test_code!(
            BybitLinearSwapWSClient,
            subscribe_ticker,
            &vec!["BTCUSDT".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(
            BybitLinearSwapWSClient,
            &vec![("BTCUSDT".to_string(), 60)]
        );
        gen_test_subscribe_candlestick!(
            BybitLinearSwapWSClient,
            &vec![("BTCUSDT".to_string(), 2592000)]
        );
    }
}
