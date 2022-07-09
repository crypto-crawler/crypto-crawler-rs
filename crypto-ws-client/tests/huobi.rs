#[macro_use]
mod utils;

#[cfg(test)]
mod huobi_spot {
    use crypto_ws_client::{HuobiSpotWSClient, WSClient};

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe() {
        gen_test_code!(
            HuobiSpotWSClient,
            subscribe,
            &[("trade.detail".to_string(), "btcusdt".to_string())]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_raw_json() {
        gen_test_code!(
            HuobiSpotWSClient,
            send,
            &[r#"{"sub":"market.btcusdt.trade.detail","id":"crypto-ws-client"}"#.to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_trade() {
        gen_test_code!(HuobiSpotWSClient, subscribe_trade, &["btcusdt".to_string()]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_ticker() {
        gen_test_code!(
            HuobiSpotWSClient,
            subscribe_ticker,
            &["btcusdt".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_bbo() {
        gen_test_code!(HuobiSpotWSClient, subscribe_bbo, &["btcusdt".to_string()]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        let (tx, rx) = std::sync::mpsc::channel();
        tokio::task::spawn(async move {
            let ws_client = HuobiSpotWSClient::new(tx, Some("wss://api.huobi.pro/feed")).await;
            ws_client
                .subscribe_orderbook(&["btcusdt".to_string()])
                .await;
            // run for 60 seconds at most
            let _ = tokio::time::timeout(std::time::Duration::from_secs(60), ws_client.run()).await;
            ws_client.close();
        });

        rx.into_iter()
            .next()
            .expect("should has at least 1 element");
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook_topk() {
        gen_test_code!(
            HuobiSpotWSClient,
            subscribe_orderbook_topk,
            &["btcusdt".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(HuobiSpotWSClient, &[("btcusdt".to_string(), 60)]);
        gen_test_subscribe_candlestick!(HuobiSpotWSClient, &[("btcusdt".to_string(), 2592000)]);
    }
}

#[cfg(test)]
mod huobi_inverse_future {
    use crypto_ws_client::{HuobiFutureWSClient, WSClient};

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe() {
        gen_test_code!(
            HuobiFutureWSClient,
            subscribe,
            &[("trade.detail".to_string(), "BTC_CQ".to_string())]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_trade() {
        gen_test_code!(
            HuobiFutureWSClient,
            subscribe_trade,
            &["BTC_CQ".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_ticker() {
        gen_test_code!(
            HuobiFutureWSClient,
            subscribe_ticker,
            &["BTC_CQ".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_bbo() {
        gen_test_code!(HuobiFutureWSClient, subscribe_bbo, &["BTC_CQ".to_string()]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(
            HuobiFutureWSClient,
            subscribe_orderbook,
            &["BTC_CQ".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook_topk() {
        gen_test_code!(
            HuobiFutureWSClient,
            subscribe_orderbook_topk,
            &["BTC_CQ".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(HuobiFutureWSClient, &[("BTC_CQ".to_string(), 60)]);
        gen_test_subscribe_candlestick!(HuobiFutureWSClient, &[("BTC_CQ".to_string(), 2592000)]);
    }
}

#[cfg(test)]
mod huobi_linear_swap {
    use crypto_ws_client::{HuobiLinearSwapWSClient, WSClient};

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe() {
        gen_test_code!(
            HuobiLinearSwapWSClient,
            subscribe,
            &[("trade.detail".to_string(), "BTC-USDT".to_string())]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_trade() {
        gen_test_code!(
            HuobiLinearSwapWSClient,
            subscribe_trade,
            &["BTC-USDT".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_ticker() {
        gen_test_code!(
            HuobiLinearSwapWSClient,
            subscribe_ticker,
            &["BTC-USDT".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_bbo() {
        gen_test_code!(
            HuobiLinearSwapWSClient,
            subscribe_bbo,
            &["BTC-USDT".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(
            HuobiLinearSwapWSClient,
            subscribe_orderbook,
            &["BTC-USDT".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook_topk() {
        gen_test_code!(
            HuobiLinearSwapWSClient,
            subscribe_orderbook_topk,
            &["BTC-USDT".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(HuobiLinearSwapWSClient, &[("BTC-USDT".to_string(), 60)]);
        gen_test_subscribe_candlestick!(
            HuobiLinearSwapWSClient,
            &[("BTC-USDT".to_string(), 2592000)]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_funding_rate() {
        let (tx, rx) = std::sync::mpsc::channel();
        tokio::task::spawn(async move {
            let ws_client = HuobiLinearSwapWSClient::new(
                tx,
                Some("wss://api.hbdm.com/linear-swap-notification"),
            )
            .await;
            ws_client
                .send(&[r#"{"topic":"public.BTC-USDT.funding_rate","op":"sub"}"#.to_string()])
                .await;
            // run for 60 seconds at most
            let _ = tokio::time::timeout(std::time::Duration::from_secs(60), ws_client.run()).await;
            ws_client.close();
        });

        rx.into_iter()
            .next()
            .expect("should has at least 1 element");
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_funding_rate_all() {
        let (tx, rx) = std::sync::mpsc::channel();
        tokio::task::spawn(async move {
            let ws_client = HuobiLinearSwapWSClient::new(
                tx,
                Some("wss://api.hbdm.com/linear-swap-notification"),
            )
            .await;
            ws_client
                .send(&[r#"{"topic":"public.*.funding_rate","op":"sub"}"#.to_string()])
                .await;
            // run for 60 seconds at most
            let _ = tokio::time::timeout(std::time::Duration::from_secs(60), ws_client.run()).await;
            ws_client.close();
        });

        rx.into_iter()
            .next()
            .expect("should has at least 1 element");
    }
}

#[cfg(test)]
mod huobi_inverse_swap {
    use crypto_ws_client::{HuobiInverseSwapWSClient, WSClient};

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe() {
        gen_test_code!(
            HuobiInverseSwapWSClient,
            subscribe,
            &[("trade.detail".to_string(), "BTC-USD".to_string())]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_trade() {
        gen_test_code!(
            HuobiInverseSwapWSClient,
            subscribe_trade,
            &["BTC-USD".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_ticker() {
        gen_test_code!(
            HuobiInverseSwapWSClient,
            subscribe_ticker,
            &["BTC-USD".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_bbo() {
        gen_test_code!(
            HuobiInverseSwapWSClient,
            subscribe_bbo,
            &["BTC-USD".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(
            HuobiInverseSwapWSClient,
            subscribe_orderbook,
            &["BTC-USD".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook_topk() {
        gen_test_code!(
            HuobiInverseSwapWSClient,
            subscribe_orderbook_topk,
            &["BTC-USD".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(HuobiInverseSwapWSClient, &[("BTC-USD".to_string(), 60)]);
        gen_test_subscribe_candlestick!(
            HuobiInverseSwapWSClient,
            &[("BTC-USD".to_string(), 2592000)]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_funding_rate() {
        let (tx, rx) = std::sync::mpsc::channel();
        tokio::task::spawn(async move {
            let ws_client =
                HuobiInverseSwapWSClient::new(tx, Some("wss://api.hbdm.com/swap-notification"))
                    .await;
            ws_client
                .send(&[r#"{"topic":"public.BTC-USD.funding_rate","op":"sub"}"#.to_string()])
                .await;
            // run for 60 seconds at most
            let _ = tokio::time::timeout(std::time::Duration::from_secs(60), ws_client.run()).await;
            ws_client.close();
        });

        rx.into_iter()
            .next()
            .expect("should has at least 1 element");
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_funding_rate_all() {
        let (tx, rx) = std::sync::mpsc::channel();
        tokio::task::spawn(async move {
            let ws_client =
                HuobiInverseSwapWSClient::new(tx, Some("wss://api.hbdm.com/swap-notification"))
                    .await;
            ws_client
                .send(&[r#"{"topic":"public.*.funding_rate","op":"sub"}"#.to_string()])
                .await;
            // run for 60 seconds at most
            let _ = tokio::time::timeout(std::time::Duration::from_secs(60), ws_client.run()).await;
            ws_client.close();
        });

        rx.into_iter()
            .next()
            .expect("should has at least 1 element");
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_overview() {
        gen_test_code!(
            HuobiInverseSwapWSClient,
            send,
            &[r#"{"sub":"market.overview","id":"crypto-ws-client"}"#.to_string()]
        );
    }
}

#[cfg(test)]
mod huobi_option {
    use crypto_ws_client::{HuobiOptionWSClient, WSClient};

    #[test]
    #[ignore]
    fn subscribe() {
        gen_test_code!(
            HuobiOptionWSClient,
            subscribe,
            &[(
                "trade.detail".to_string(),
                "BTC-USDT-210625-P-27000".to_string()
            )]
        );
    }

    #[test]
    #[ignore]
    fn subscribe_trade() {
        gen_test_code!(
            HuobiOptionWSClient,
            subscribe_trade,
            &["BTC-USDT-210625-P-27000".to_string()]
        );
    }

    #[test]
    #[ignore]
    fn subscribe_ticker() {
        gen_test_code!(
            HuobiOptionWSClient,
            subscribe_ticker,
            &["BTC-USDT-210625-P-27000".to_string()]
        );
    }

    #[test]
    #[ignore]
    fn subscribe_bbo() {
        gen_test_code!(
            HuobiOptionWSClient,
            subscribe_bbo,
            &["BTC-USDT-210625-P-27000".to_string()]
        );
    }

    #[test]
    #[ignore]
    fn subscribe_orderbook() {
        gen_test_code!(
            HuobiOptionWSClient,
            subscribe_orderbook,
            &["BTC-USDT-210625-P-27000".to_string()]
        );
    }

    #[test]
    #[ignore]
    fn subscribe_orderbook_topk() {
        gen_test_code!(
            HuobiOptionWSClient,
            subscribe_orderbook_topk,
            &["BTC-USDT-210625-P-27000".to_string()]
        );
    }

    #[test]
    #[ignore]
    fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(
            HuobiOptionWSClient,
            &[("BTC-USDT-210625-P-27000".to_string(), 60)]
        );
        gen_test_subscribe_candlestick!(
            HuobiOptionWSClient,
            &[("BTC-USDT-210625-P-27000".to_string(), 2592000)]
        );
    }

    #[test]
    #[ignore]
    fn subscribe_overview() {
        gen_test_code!(
            HuobiOptionWSClient,
            send,
            &[r#"{"sub":"market.overview","id":"crypto-ws-client"}"#.to_string()]
        );
    }
}
