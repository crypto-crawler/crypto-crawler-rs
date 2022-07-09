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
            &[("trade".to_string(), "BTCUSDU22".to_string())]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_raw_json() {
        gen_test_code!(
            BybitInverseWSClient,
            send,
            &[r#"{"op":"subscribe","args":["trade.BTCUSDU22"]}"#.to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_trade() {
        gen_test_code!(
            BybitInverseWSClient,
            subscribe_trade,
            &["BTCUSDU22".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(
            BybitInverseWSClient,
            subscribe_orderbook,
            &["BTCUSDU22".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_ticker() {
        gen_test_code!(
            BybitInverseWSClient,
            subscribe_ticker,
            &["BTCUSDU22".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(BybitInverseWSClient, &[("BTCUSDU22".to_string(), 60)]);
        gen_test_subscribe_candlestick!(
            BybitInverseWSClient,
            &[("BTCUSDU22".to_string(), 2592000)]
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
            &[("trade".to_string(), "BTCUSD".to_string())]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_raw_json() {
        gen_test_code!(
            BybitInverseWSClient,
            send,
            &[r#"{"op":"subscribe","args":["trade.BTCUSD"]}"#.to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_trade() {
        gen_test_code!(
            BybitInverseWSClient,
            subscribe_trade,
            &["BTCUSD".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(
            BybitInverseWSClient,
            subscribe_orderbook,
            &["BTCUSD".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_ticker() {
        gen_test_code!(
            BybitInverseWSClient,
            subscribe_ticker,
            &["BTCUSD".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(BybitInverseWSClient, &[("BTCUSD".to_string(), 60)]);
        gen_test_subscribe_candlestick!(BybitInverseWSClient, &[("BTCUSD".to_string(), 2592000)]);
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
            &["BTCUSDT".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(
            BybitLinearSwapWSClient,
            subscribe_orderbook,
            &["BTCUSDT".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_ticker() {
        gen_test_code!(
            BybitLinearSwapWSClient,
            subscribe_ticker,
            &["BTCUSDT".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(BybitLinearSwapWSClient, &[("BTCUSDT".to_string(), 60)]);
        gen_test_subscribe_candlestick!(
            BybitLinearSwapWSClient,
            &[("BTCUSDT".to_string(), 2592000)]
        );
    }
}
