#[macro_use]
mod utils;

#[cfg(test)]
mod dydx_linear_swap {
    use crypto_ws_client::{DydxSwapWSClient, WSClient};

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe() {
        gen_test_code!(
            DydxSwapWSClient,
            send,
            &[
                r#"{"type": "subscribe", "channel": "v3_trades", "id": "BTC-USD"}"#.to_string(),
                r#"{"type": "subscribe", "channel": "v3_trades", "id": "ETH-USD"}"#.to_string()
            ]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_trade() {
        gen_test_code!(
            DydxSwapWSClient,
            subscribe_trade,
            &["BTC-USD".to_string(), "ETH-USD".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(
            DydxSwapWSClient,
            subscribe_orderbook,
            &["BTC-USD".to_string(), "ETH-USD".to_string()]
        );
    }
}
