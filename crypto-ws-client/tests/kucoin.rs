#[macro_use]
mod utils;

#[cfg(test)]
mod kucoin_spot {
    use crypto_ws_client::{KuCoinSpotWSClient, WSClient};

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe() {
        gen_test_code!(
            KuCoinSpotWSClient,
            subscribe,
            &[("/market/match".to_string(), "BTC-USDT".to_string())]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_all_bbo() {
        gen_test_code!(
            KuCoinSpotWSClient,
            subscribe,
            &[("/market/ticker".to_string(), "all".to_string())]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_raw_json() {
        gen_test_code!(
            KuCoinSpotWSClient,
            send,
            &[r#"{"id":"crypto-ws-client","type":"subscribe","topic":"/market/match:BTC-USDT","privateChannel":false,"response":true}"#.to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_trade() {
        gen_test_code!(
            KuCoinSpotWSClient,
            subscribe_trade,
            &["BTC-USDT".to_string(), "ETH-USDT".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_bbo() {
        gen_test_code!(KuCoinSpotWSClient, subscribe_bbo, &["BTC-USDT".to_string()]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(KuCoinSpotWSClient, subscribe_orderbook, &["BTC-USDT".to_string()]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook_topk() {
        gen_test_code!(KuCoinSpotWSClient, subscribe_orderbook_topk, &["BTC-USDT".to_string()]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_ticker() {
        gen_test_code!(KuCoinSpotWSClient, subscribe_ticker, &["BTC-USDT".to_string()]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(
            KuCoinSpotWSClient,
            &[("BTC-USDT".to_string(), 60), ("BTC-USDT".to_string(), 604800)]
        );
    }
}

#[cfg(test)]
mod kucoin_inverse_swap {
    use crypto_ws_client::{KuCoinSwapWSClient, WSClient};

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe() {
        gen_test_code!(
            KuCoinSwapWSClient,
            subscribe,
            &[
                ("/contractMarket/execution".to_string(), "XBTUSDM".to_string()),
                ("/contractMarket/execution".to_string(), "ETHUSDM".to_string()),
                ("/contractMarket/execution".to_string(), "DOTUSDM".to_string()),
                ("/contractMarket/execution".to_string(), "XRPUSDM".to_string())
            ]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_raw_json() {
        gen_test_code!(
            KuCoinSwapWSClient,
            send,
            &[r#"{"id":"crypto-ws-client","type":"subscribe","topic":"/contractMarket/execution:XBTUSDM,ETHUSDM,DOTUSDM,XRPUSDM","privateChannel":false,"response":true}"#.to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_trade() {
        gen_test_code!(
            KuCoinSwapWSClient,
            subscribe_trade,
            &[
                "XBTUSDM".to_string(),
                "ETHUSDM".to_string(),
                "DOTUSDM".to_string(),
                "XRPUSDM".to_string()
            ]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_bbo() {
        gen_test_code!(KuCoinSwapWSClient, subscribe_bbo, &["XBTUSDM".to_string()]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(KuCoinSwapWSClient, subscribe_orderbook, &["XBTUSDM".to_string()]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook_topk() {
        gen_test_code!(KuCoinSwapWSClient, subscribe_orderbook_topk, &["XBTUSDM".to_string()]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_ticker() {
        gen_test_code!(KuCoinSwapWSClient, subscribe_ticker, &["XBTUSDM".to_string()]);
    }

    #[ignore]
    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(
            KuCoinSwapWSClient,
            &[
                ("XBTUSDM".to_string(), 60),
                ("ETHUSDM".to_string(), 60),
                ("XBTUSDM".to_string(), 604800),
                ("ETHUSDM".to_string(), 604800)
            ]
        );
    }
}

#[cfg(test)]
mod kucoin_linear_swap {
    use crypto_ws_client::{KuCoinSwapWSClient, WSClient};

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe() {
        gen_test_code!(
            KuCoinSwapWSClient,
            subscribe,
            &[("/contractMarket/execution".to_string(), "XBTUSDTM".to_string())]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_raw_json() {
        gen_test_code!(
            KuCoinSwapWSClient,
            send,
            &[r#"{"id":"crypto-ws-client","type":"subscribe","topic":"/contractMarket/execution:XBTUSDTM","privateChannel":false,"response":true}"#.to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_trade() {
        gen_test_code!(KuCoinSwapWSClient, subscribe_trade, &["XBTUSDTM".to_string()]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_bbo() {
        gen_test_code!(KuCoinSwapWSClient, subscribe_bbo, &["XBTUSDTM".to_string()]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(KuCoinSwapWSClient, subscribe_orderbook, &["XBTUSDTM".to_string()]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook_topk() {
        gen_test_code!(KuCoinSwapWSClient, subscribe_orderbook_topk, &["XBTUSDTM".to_string()]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_ticker() {
        gen_test_code!(KuCoinSwapWSClient, subscribe_ticker, &["XBTUSDTM".to_string()]);
    }

    #[ignore]
    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(
            KuCoinSwapWSClient,
            &[
                ("XBTUSDTM".to_string(), 60),
                ("ETHUSDTM".to_string(), 60),
                ("XBTUSDTM".to_string(), 604800),
                ("ETHUSDTM".to_string(), 604800)
            ]
        );
    }
}

#[cfg(test)]
mod kucoin_inverse_future {
    use crypto_ws_client::{KuCoinSwapWSClient, WSClient};

    #[ignore]
    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe() {
        gen_test_code!(
            KuCoinSwapWSClient,
            subscribe,
            &[("/contractMarket/execution".to_string(), "XBTMZ22".to_string())]
        );
    }

    #[ignore]
    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_raw_json() {
        gen_test_code!(
            KuCoinSwapWSClient,
            send,
            &[r#"{"id":"crypto-ws-client","type":"subscribe","topic":"/contractMarket/execution:XBTMZ22","privateChannel":false,"response":true}"#.to_string()]
        );
    }

    #[ignore]
    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_trade() {
        gen_test_code!(KuCoinSwapWSClient, subscribe_trade, &["XBTMZ22".to_string()]);
    }

    #[ignore]
    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_bbo() {
        gen_test_code!(KuCoinSwapWSClient, subscribe_bbo, &["XBTMZ22".to_string()]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(KuCoinSwapWSClient, subscribe_orderbook, &["XBTMZ22".to_string()]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook_topk() {
        gen_test_code!(KuCoinSwapWSClient, subscribe_orderbook_topk, &["XBTMZ22".to_string()]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_ticker() {
        gen_test_code!(KuCoinSwapWSClient, subscribe_ticker, &["XBTMZ22".to_string()]);
    }

    #[ignore]
    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(
            KuCoinSwapWSClient,
            &[("XBTMZ22".to_string(), 60), ("XBTMZ22".to_string(), 604800)]
        );
    }
}
