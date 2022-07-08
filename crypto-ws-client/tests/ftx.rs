#[macro_use]
mod utils;

#[cfg(test)]
mod ftx_spot {
    use crypto_ws_client::{FtxWSClient, WSClient};

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe() {
        gen_test_code!(
            FtxWSClient,
            subscribe,
            &vec![("trades".to_string(), "BTC/USD".to_string())]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_raw_json() {
        gen_test_code!(
            FtxWSClient,
            send,
            &vec![r#"{"op":"subscribe","channel":"trades","market":"BTC/USD"}"#.to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_trade() {
        gen_test_code!(FtxWSClient, subscribe_trade, &vec!["BTC/USD".to_string()]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_bbo() {
        gen_test_code!(FtxWSClient, subscribe_bbo, &vec!["BTC/USD".to_string()]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(
            FtxWSClient,
            subscribe_orderbook,
            &vec!["BTC/USD".to_string()]
        );
    }
}

#[cfg(test)]
mod ftx_linear_swap {
    use crypto_ws_client::{FtxWSClient, WSClient};

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_trade() {
        gen_test_code!(FtxWSClient, subscribe_trade, &vec!["BTC-PERP".to_string()]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_bbo() {
        gen_test_code!(FtxWSClient, subscribe_bbo, &vec!["BTC-PERP".to_string()]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(
            FtxWSClient,
            subscribe_orderbook,
            &vec!["BTC-PERP".to_string(), "ETH-PERP".to_string()]
        );
    }
}

#[cfg(test)]
mod ftx_linear_future {
    use crypto_ws_client::{FtxWSClient, WSClient};

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_trade() {
        gen_test_code!(
            FtxWSClient,
            subscribe_trade,
            &vec!["BTC-0930".to_string(), "ETH-0930".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_bbo() {
        gen_test_code!(
            FtxWSClient,
            subscribe_bbo,
            &vec!["BTC-0930".to_string(), "ETH-0930".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(
            FtxWSClient,
            subscribe_orderbook,
            &vec!["BTC-0930".to_string()]
        );
    }
}

#[cfg(test)]
mod ftx_move {
    use crypto_ws_client::{FtxWSClient, WSClient};

    #[test]
    #[ignore]
    fn subscribe_trade() {
        gen_test_code!(
            FtxWSClient,
            subscribe_trade,
            &vec!["BTC-MOVE-2022Q4".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_bbo() {
        gen_test_code!(
            FtxWSClient,
            subscribe_bbo,
            &vec!["BTC-MOVE-2022Q4".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(
            FtxWSClient,
            subscribe_orderbook,
            &vec!["BTC-MOVE-2022Q4".to_string()]
        );
    }
}

#[cfg(test)]
mod ftx_bvol {
    use crypto_ws_client::{FtxWSClient, WSClient};

    #[test]
    #[ignore]
    fn subscribe_trade() {
        gen_test_code!(FtxWSClient, subscribe_trade, &vec!["BVOL/USD".to_string()]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_bbo() {
        gen_test_code!(FtxWSClient, subscribe_bbo, &vec!["BVOL/USD".to_string()]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(
            FtxWSClient,
            subscribe_orderbook,
            &vec!["BVOL/USD".to_string()]
        );
    }
}
