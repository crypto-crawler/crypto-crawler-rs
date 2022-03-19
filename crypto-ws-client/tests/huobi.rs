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
            &vec![("trade.detail".to_string(), "btcusdt".to_string())]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_raw_json() {
        gen_test_code!(
            HuobiSpotWSClient,
            send,
            &vec![r#"{"sub":"market.btcusdt.trade.detail","id":"crypto-ws-client"}"#.to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_trade() {
        gen_test_code!(
            HuobiSpotWSClient,
            subscribe_trade,
            &vec!["btcusdt".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_ticker() {
        gen_test_code!(
            HuobiSpotWSClient,
            subscribe_ticker,
            &vec!["btcusdt".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_bbo() {
        gen_test_code!(
            HuobiSpotWSClient,
            subscribe_bbo,
            &vec!["btcusdt".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        let (tx, rx) = std::sync::mpsc::channel();
        tokio::task::spawn(async move {
            let ws_client = HuobiSpotWSClient::new(tx, Some("wss://api.huobi.pro/feed")).await;
            ws_client
                .subscribe_orderbook(&vec!["btcusdt".to_string()])
                .await;
            // run for 60 seconds at most
            let _ = tokio::time::timeout(std::time::Duration::from_secs(60), ws_client.run()).await;
            ws_client.close();
        });

        let mut messages = Vec::<String>::new();
        for msg in rx {
            messages.push(msg);
            break;
        }
        assert!(!messages.is_empty());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook_topk() {
        gen_test_code!(
            HuobiSpotWSClient,
            subscribe_orderbook_topk,
            &vec!["btcusdt".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(HuobiSpotWSClient, &vec![("btcusdt".to_string(), 60)]);
        gen_test_subscribe_candlestick!(HuobiSpotWSClient, &vec![("btcusdt".to_string(), 2592000)]);
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
            &vec![("trade.detail".to_string(), "BTC_CQ".to_string())]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_trade() {
        gen_test_code!(
            HuobiFutureWSClient,
            subscribe_trade,
            &vec!["BTC_CQ".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_ticker() {
        gen_test_code!(
            HuobiFutureWSClient,
            subscribe_ticker,
            &vec!["BTC_CQ".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_bbo() {
        gen_test_code!(
            HuobiFutureWSClient,
            subscribe_bbo,
            &vec!["BTC_CQ".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(
            HuobiFutureWSClient,
            subscribe_orderbook,
            &vec!["BTC_CQ".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook_topk() {
        gen_test_code!(
            HuobiFutureWSClient,
            subscribe_orderbook_topk,
            &vec!["BTC_CQ".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(HuobiFutureWSClient, &vec![("BTC_CQ".to_string(), 60)]);
        gen_test_subscribe_candlestick!(
            HuobiFutureWSClient,
            &vec![("BTC_CQ".to_string(), 2592000)]
        );
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
            &vec![("trade.detail".to_string(), "BTC-USDT".to_string())]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_trade() {
        gen_test_code!(
            HuobiLinearSwapWSClient,
            subscribe_trade,
            &vec!["BTC-USDT".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_ticker() {
        gen_test_code!(
            HuobiLinearSwapWSClient,
            subscribe_ticker,
            &vec!["BTC-USDT".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_bbo() {
        gen_test_code!(
            HuobiLinearSwapWSClient,
            subscribe_bbo,
            &vec!["BTC-USDT".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(
            HuobiLinearSwapWSClient,
            subscribe_orderbook,
            &vec!["BTC-USDT".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook_topk() {
        gen_test_code!(
            HuobiLinearSwapWSClient,
            subscribe_orderbook_topk,
            &vec!["BTC-USDT".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(
            HuobiLinearSwapWSClient,
            &vec![("BTC-USDT".to_string(), 60)]
        );
        gen_test_subscribe_candlestick!(
            HuobiLinearSwapWSClient,
            &vec![("BTC-USDT".to_string(), 2592000)]
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
                .send(&vec![
                    r#"{"topic":"public.BTC-USDT.funding_rate","op":"sub"}"#.to_string(),
                ])
                .await;
            // run for 60 seconds at most
            let _ = tokio::time::timeout(std::time::Duration::from_secs(60), ws_client.run()).await;
            ws_client.close();
        });

        let mut messages = Vec::<String>::new();
        for msg in rx {
            messages.push(msg);
            break;
        }
        assert!(!messages.is_empty());
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
                .send(&vec![
                    r#"{"topic":"public.*.funding_rate","op":"sub"}"#.to_string()
                ])
                .await;
            // run for 60 seconds at most
            let _ = tokio::time::timeout(std::time::Duration::from_secs(60), ws_client.run()).await;
            ws_client.close();
        });

        let mut messages = Vec::<String>::new();
        for msg in rx {
            messages.push(msg);
            break;
        }
        assert!(!messages.is_empty());
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
            &vec![("trade.detail".to_string(), "BTC-USD".to_string())]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_trade() {
        gen_test_code!(
            HuobiInverseSwapWSClient,
            subscribe_trade,
            &vec!["BTC-USD".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_ticker() {
        gen_test_code!(
            HuobiInverseSwapWSClient,
            subscribe_ticker,
            &vec!["BTC-USD".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_bbo() {
        gen_test_code!(
            HuobiInverseSwapWSClient,
            subscribe_bbo,
            &vec!["BTC-USD".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(
            HuobiInverseSwapWSClient,
            subscribe_orderbook,
            &vec!["BTC-USD".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook_topk() {
        gen_test_code!(
            HuobiInverseSwapWSClient,
            subscribe_orderbook_topk,
            &vec!["BTC-USD".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(
            HuobiInverseSwapWSClient,
            &vec![("BTC-USD".to_string(), 60)]
        );
        gen_test_subscribe_candlestick!(
            HuobiInverseSwapWSClient,
            &vec![("BTC-USD".to_string(), 2592000)]
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
                .send(&vec![
                    r#"{"topic":"public.BTC-USD.funding_rate","op":"sub"}"#.to_string(),
                ])
                .await;
            // run for 60 seconds at most
            let _ = tokio::time::timeout(std::time::Duration::from_secs(60), ws_client.run()).await;
            ws_client.close();
        });

        let mut messages = Vec::<String>::new();
        for msg in rx {
            messages.push(msg);
            break;
        }
        assert!(!messages.is_empty());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_funding_rate_all() {
        let (tx, rx) = std::sync::mpsc::channel();
        tokio::task::spawn(async move {
            let ws_client =
                HuobiInverseSwapWSClient::new(tx, Some("wss://api.hbdm.com/swap-notification"))
                    .await;
            ws_client
                .send(&vec![
                    r#"{"topic":"public.*.funding_rate","op":"sub"}"#.to_string()
                ])
                .await;
            // run for 60 seconds at most
            let _ = tokio::time::timeout(std::time::Duration::from_secs(60), ws_client.run()).await;
            ws_client.close();
        });

        let mut messages = Vec::<String>::new();
        for msg in rx {
            messages.push(msg);
            break;
        }
        assert!(!messages.is_empty());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_overview() {
        gen_test_code!(
            HuobiInverseSwapWSClient,
            send,
            &vec![r#"{"sub":"market.overview","id":"crypto-ws-client"}"#.to_string()]
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
            &vec![(
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
            &vec!["BTC-USDT-210625-P-27000".to_string()]
        );
    }

    #[test]
    #[ignore]
    fn subscribe_ticker() {
        gen_test_code!(
            HuobiOptionWSClient,
            subscribe_ticker,
            &vec!["BTC-USDT-210625-P-27000".to_string()]
        );
    }

    #[test]
    #[ignore]
    fn subscribe_bbo() {
        gen_test_code!(
            HuobiOptionWSClient,
            subscribe_bbo,
            &vec!["BTC-USDT-210625-P-27000".to_string()]
        );
    }

    #[test]
    #[ignore]
    fn subscribe_orderbook() {
        gen_test_code!(
            HuobiOptionWSClient,
            subscribe_orderbook,
            &vec!["BTC-USDT-210625-P-27000".to_string()]
        );
    }

    #[test]
    #[ignore]
    fn subscribe_orderbook_topk() {
        gen_test_code!(
            HuobiOptionWSClient,
            subscribe_orderbook_topk,
            &vec!["BTC-USDT-210625-P-27000".to_string()]
        );
    }

    #[test]
    #[ignore]
    fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(
            HuobiOptionWSClient,
            &vec![("BTC-USDT-210625-P-27000".to_string(), 60)]
        );
        gen_test_subscribe_candlestick!(
            HuobiOptionWSClient,
            &vec![("BTC-USDT-210625-P-27000".to_string(), 2592000)]
        );
    }

    #[test]
    #[ignore]
    fn subscribe_overview() {
        gen_test_code!(
            HuobiOptionWSClient,
            send,
            &vec![r#"{"sub":"market.overview","id":"crypto-ws-client"}"#.to_string()]
        );
    }
}
