use crypto_ws_client::{BitmexWSClient, WSClient};

#[macro_use]
mod utils;

#[tokio::test(flavor = "multi_thread")]
async fn bitmex_instrument() {
    gen_test_code!(
        BitmexWSClient,
        send,
        &vec![r#"{"op":"subscribe","args":["instrument"]}"#.to_string()]
    );
}

#[cfg(test)]
mod bitmex_inverse_swap {
    use crypto_ws_client::{BitmexWSClient, WSClient};

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe() {
        gen_test_code!(
            BitmexWSClient,
            subscribe,
            &vec![
                ("trade".to_string(), "XBTUSD".to_string()),
                ("quote".to_string(), "XBTUSD".to_string())
            ]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_raw_json() {
        gen_test_code!(
            BitmexWSClient,
            send,
            &vec![r#"{"op":"subscribe","args":["trade:XBTUSD","quote:XBTUSD"]}"#.to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_trade() {
        gen_test_code!(BitmexWSClient, subscribe_trade, &vec!["XBTUSD".to_string()]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_bbo() {
        gen_test_code!(BitmexWSClient, subscribe_bbo, &vec!["XBTUSD".to_string()]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(
            BitmexWSClient,
            subscribe_orderbook,
            &vec!["XBTUSD".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook_topk() {
        gen_test_code!(
            BitmexWSClient,
            subscribe_orderbook_topk,
            &vec!["XBTUSD".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(BitmexWSClient, &vec![("XBTUSD".to_string(), 60)]);
        gen_test_subscribe_candlestick!(BitmexWSClient, &vec![("XBTUSD".to_string(), 86400)]);
    }

    #[test]
    #[ignore]
    fn subscribe_funding_rate() {
        gen_test_code!(
            BitmexWSClient,
            subscribe,
            &vec![("funding".to_string(), "XBTUSD".to_string())]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_funding_rate_all() {
        gen_test_code!(
            BitmexWSClient,
            send,
            &vec![r#"{"op":"subscribe","args":["funding"]}"#.to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_instrument() {
        gen_test_code!(
            BitmexWSClient,
            subscribe,
            &vec![("instrument".to_string(), "XBTUSD".to_string())]
        );
    }
}

#[cfg(test)]
mod bitmex_inverse_future {
    use crypto_ws_client::{BitmexWSClient, WSClient};

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe() {
        gen_test_code!(
            BitmexWSClient,
            subscribe,
            &vec![
                ("trade".to_string(), "XBTU22".to_string()),
                ("quote".to_string(), "XBTU22".to_string())
            ]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_trade() {
        gen_test_code!(
            BitmexWSClient,
            subscribe_trade,
            &vec!["XBTU22".to_string(), "XBTU22".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_bbo() {
        gen_test_code!(
            BitmexWSClient,
            subscribe_bbo,
            &vec!["XBTU22".to_string(), "XBTU22".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(
            BitmexWSClient,
            subscribe_orderbook,
            &vec!["XBTU22".to_string(), "XBTU22".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook_topk() {
        gen_test_code!(
            BitmexWSClient,
            subscribe_orderbook_topk,
            &vec!["XBTU22".to_string(), "XBTU22".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(
            BitmexWSClient,
            &vec![("XBTU22".to_string(), 60), ("XBTU22".to_string(), 60)]
        );
        gen_test_subscribe_candlestick!(
            BitmexWSClient,
            &vec![("XBTU22".to_string(), 86400), ("XBTU22".to_string(), 86400)]
        );
    }
}

#[cfg(test)]
mod bitmex_quanto_swap {
    use crypto_ws_client::{BitmexWSClient, WSClient};

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_trade() {
        gen_test_code!(BitmexWSClient, subscribe_trade, &vec!["ETHUSD".to_string()]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_bbo() {
        gen_test_code!(BitmexWSClient, subscribe_bbo, &vec!["ETHUSD".to_string()]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(
            BitmexWSClient,
            subscribe_orderbook,
            &vec!["ETHUSD".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook_topk() {
        gen_test_code!(
            BitmexWSClient,
            subscribe_orderbook_topk,
            &vec!["ETHUSD".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(BitmexWSClient, &vec![("ETHUSD".to_string(), 60)]);
        gen_test_subscribe_candlestick!(BitmexWSClient, &vec![("ETHUSD".to_string(), 86400)]);
    }

    #[test]
    #[ignore]
    fn subscribe_funding_rate() {
        gen_test_code!(
            BitmexWSClient,
            subscribe,
            &vec![("funding".to_string(), "ETHUSD".to_string())]
        );
    }
}

#[cfg(test)]
mod bitmex_linear_future {
    use crypto_ws_client::{BitmexWSClient, WSClient};

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_trade() {
        gen_test_code!(
            BitmexWSClient,
            subscribe_trade,
            &vec![
                "XBTUSDTU22".to_string(),
                "ETHU22".to_string(),
                "ETHUSDTU22".to_string()
            ]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_bbo() {
        gen_test_code!(BitmexWSClient, subscribe_bbo, &vec!["ETHU22".to_string()]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(
            BitmexWSClient,
            subscribe_orderbook,
            &vec!["ETHU22".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook_topk() {
        gen_test_code!(
            BitmexWSClient,
            subscribe_orderbook_topk,
            &vec!["ETHU22".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(BitmexWSClient, &vec![("ETHU22".to_string(), 60)]);
        gen_test_subscribe_candlestick!(BitmexWSClient, &vec![("ETHU22".to_string(), 86400)]);
    }
}
