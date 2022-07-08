#[macro_use]
mod utils;

#[cfg(test)]
mod kraken_spot {
    use crypto_ws_client::{KrakenSpotWSClient, WSClient};

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe() {
        gen_test_code!(
            KrakenSpotWSClient,
            subscribe,
            &[("trade".to_string(), "XBT/USD".to_string()),
                ("ticker".to_string(), "XBT/USD".to_string()),
                ("spread".to_string(), "XBT/USD".to_string()),
                ("book".to_string(), "XBT/USD".to_string())]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_raw_json() {
        gen_test_code!(
            KrakenSpotWSClient,
            send,
            &[r#"{"event":"subscribe","pair":["XBT/USD"],"subscription":{"name":"trade"}}"#
                    .to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_trade() {
        gen_test_code!(
            KrakenSpotWSClient,
            subscribe_trade,
            &["XBT/USD".to_string(), "ETH/USD".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_ticker() {
        gen_test_code!(
            KrakenSpotWSClient,
            subscribe_ticker,
            &["XBT/USD".to_string(), "ETH/USD".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_bbo() {
        gen_test_code!(
            KrakenSpotWSClient,
            subscribe_bbo,
            &["XBT/USD".to_string(), "ETH/USD".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(
            KrakenSpotWSClient,
            subscribe_orderbook,
            &["XBT/USD".to_string(), "ETH/USD".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(
            KrakenSpotWSClient,
            &[("XBT/USD".to_string(), 60), ("ETH/USD".to_string(), 60)]
        );

        gen_test_subscribe_candlestick!(
            KrakenSpotWSClient,
            &[("XBT/USD".to_string(), 1296000),
                ("ETH/USD".to_string(), 1296000)]
        );
    }
}

#[cfg(test)]
mod kraken_inverse_swap {
    use crypto_ws_client::{KrakenFuturesWSClient, WSClient};

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_raw_json() {
        gen_test_code!(
            KrakenFuturesWSClient,
            send,
            &[r#"{"event":"subscribe","feed":"trade","product_ids":["PI_XBTUSD"]}"#.to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_trade() {
        gen_test_code!(
            KrakenFuturesWSClient,
            subscribe_trade,
            &["PI_XBTUSD".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_ticker() {
        gen_test_code!(
            KrakenFuturesWSClient,
            subscribe_ticker,
            &["PI_XBTUSD".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(
            KrakenFuturesWSClient,
            subscribe_orderbook,
            &["PI_XBTUSD".to_string()]
        );
    }
}

#[cfg(test)]
mod kraken_inverse_future {
    use crypto_ws_client::{KrakenFuturesWSClient, WSClient};

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_raw_json() {
        gen_test_code!(
            KrakenFuturesWSClient,
            send,
            &[r#"{"event":"subscribe","feed":"trade","product_ids":["FI_XBTUSD_220930"]}"#
                    .to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_trade() {
        gen_test_code!(
            KrakenFuturesWSClient,
            subscribe_trade,
            &["FI_XBTUSD_220930".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_ticker() {
        gen_test_code!(
            KrakenFuturesWSClient,
            subscribe_ticker,
            &["FI_XBTUSD_220930".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(
            KrakenFuturesWSClient,
            subscribe_orderbook,
            &["FI_XBTUSD_220930".to_string()]
        );
    }
}
