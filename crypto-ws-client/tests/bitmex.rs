use crypto_ws_client::{BitmexWSClient, WSClient};

#[macro_use]
mod utils;

#[tokio::test(flavor = "multi_thread")]
async fn bitmex_instrument() {
    gen_test_code!(
        BitmexWSClient,
        send,
        &[r#"{"op":"subscribe","args":["instrument"]}"#.to_string()]
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
            &[
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
            &[r#"{"op":"subscribe","args":["trade:XBTUSD","quote:XBTUSD"]}"#.to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_trade() {
        gen_test_code!(BitmexWSClient, subscribe_trade, &["XBTUSD".to_string()]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_bbo() {
        gen_test_code!(BitmexWSClient, subscribe_bbo, &["XBTUSD".to_string()]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(BitmexWSClient, subscribe_orderbook, &["XBTUSD".to_string()]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook_topk() {
        gen_test_code!(
            BitmexWSClient,
            subscribe_orderbook_topk,
            &["XBTUSD".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(BitmexWSClient, &[("XBTUSD".to_string(), 60)]);
        gen_test_subscribe_candlestick!(BitmexWSClient, &[("XBTUSD".to_string(), 86400)]);
    }

    #[test]
    #[ignore]
    fn subscribe_funding_rate() {
        gen_test_code!(
            BitmexWSClient,
            subscribe,
            &[("funding".to_string(), "XBTUSD".to_string())]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_funding_rate_all() {
        gen_test_code!(
            BitmexWSClient,
            send,
            &[r#"{"op":"subscribe","args":["funding"]}"#.to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_instrument() {
        gen_test_code!(
            BitmexWSClient,
            subscribe,
            &[("instrument".to_string(), "XBTUSD".to_string())]
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
            &[
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
            &["XBTU22".to_string(), "XBTU22".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_bbo() {
        gen_test_code!(
            BitmexWSClient,
            subscribe_bbo,
            &["XBTU22".to_string(), "XBTU22".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(
            BitmexWSClient,
            subscribe_orderbook,
            &["XBTU22".to_string(), "XBTU22".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook_topk() {
        gen_test_code!(
            BitmexWSClient,
            subscribe_orderbook_topk,
            &["XBTU22".to_string(), "XBTU22".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(
            BitmexWSClient,
            &[("XBTU22".to_string(), 60), ("XBTU22".to_string(), 60)]
        );
        gen_test_subscribe_candlestick!(
            BitmexWSClient,
            &[("XBTU22".to_string(), 86400), ("XBTU22".to_string(), 86400)]
        );
    }
}

#[cfg(test)]
mod bitmex_quanto_swap {
    use crypto_ws_client::{BitmexWSClient, WSClient};

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_trade() {
        gen_test_code!(BitmexWSClient, subscribe_trade, &["ETHUSD".to_string()]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_bbo() {
        gen_test_code!(BitmexWSClient, subscribe_bbo, &["ETHUSD".to_string()]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(BitmexWSClient, subscribe_orderbook, &["ETHUSD".to_string()]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook_topk() {
        gen_test_code!(
            BitmexWSClient,
            subscribe_orderbook_topk,
            &["ETHUSD".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(BitmexWSClient, &[("ETHUSD".to_string(), 60)]);
        gen_test_subscribe_candlestick!(BitmexWSClient, &[("ETHUSD".to_string(), 86400)]);
    }

    #[test]
    #[ignore]
    fn subscribe_funding_rate() {
        gen_test_code!(
            BitmexWSClient,
            subscribe,
            &[("funding".to_string(), "ETHUSD".to_string())]
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
            &[
                "XBTUSDTU22".to_string(),
                "ETHU22".to_string(),
                "ETHUSDTU22".to_string()
            ]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_bbo() {
        gen_test_code!(BitmexWSClient, subscribe_bbo, &["ETHU22".to_string()]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(BitmexWSClient, subscribe_orderbook, &["ETHU22".to_string()]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook_topk() {
        gen_test_code!(
            BitmexWSClient,
            subscribe_orderbook_topk,
            &["ETHU22".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(BitmexWSClient, &[("ETHU22".to_string(), 60)]);
        gen_test_subscribe_candlestick!(BitmexWSClient, &[("ETHU22".to_string(), 86400)]);
    }
}
