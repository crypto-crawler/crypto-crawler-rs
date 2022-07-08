#[macro_use]
mod utils;

#[cfg(test)]
mod zb_spot {
    use crypto_ws_client::{WSClient, ZbSpotWSClient};

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe() {
        gen_test_code!(
            ZbSpotWSClient,
            subscribe,
            &[("trades".to_string(), "btc_usdt".to_string())]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_raw_json() {
        gen_test_code!(
            ZbSpotWSClient,
            send,
            &[r#"{"event":"addChannel","channel":"btcusdt_trades"}"#.to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_trade() {
        gen_test_code!(ZbSpotWSClient, subscribe_trade, &["btc_usdt".to_string()]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook_topk() {
        gen_test_code!(
            ZbSpotWSClient,
            subscribe_orderbook_topk,
            &["btc_usdt".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_ticker() {
        gen_test_code!(ZbSpotWSClient, subscribe_ticker, &["btc_usdt".to_string()]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(ZbSpotWSClient, &[("btc_usdt".to_string(), 60)]);
        gen_test_subscribe_candlestick!(ZbSpotWSClient, &[("btc_usdt".to_string(), 604800)]);
    }
}

#[cfg(test)]
mod zb_linear_swap {
    use crypto_ws_client::{WSClient, ZbSwapWSClient};

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe() {
        gen_test_code!(
            ZbSwapWSClient,
            subscribe,
            &[
                ("Trade".to_string(), "BTC_USDT".to_string()),
                ("Depth".to_string(), "BTC_USDT".to_string())
            ]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_raw_json() {
        gen_test_code!(
            ZbSwapWSClient,
            send,
            &[
                r#"{"action":"subscribe", "channel":"BTC_USDT.Trade", "size":100}"#.to_string(),
                r#"{"action":"subscribe", "channel":"BTC_USDT.Depth", "size":200}"#.to_string()
            ]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_trade() {
        gen_test_code!(
            ZbSwapWSClient,
            subscribe_trade,
            &["BTC_USDT".to_string(), "ETH_USDT".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(
            ZbSwapWSClient,
            subscribe_orderbook,
            &["BTC_USDT".to_string(), "ETH_USDT".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook_topk() {
        gen_test_code!(
            ZbSwapWSClient,
            subscribe_orderbook_topk,
            &["BTC_USDT".to_string(), "ETH_USDT".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_ticker() {
        gen_test_code!(
            ZbSwapWSClient,
            subscribe_ticker,
            &["BTC_USDT".to_string(), "ETH_USDT".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(
            ZbSwapWSClient,
            &[("BTC_USDT".to_string(), 60), ("ETH_USDT".to_string(), 60)]
        );
    }
}
