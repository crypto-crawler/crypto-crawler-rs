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
                ("trade".to_string(), "XBTM22".to_string()),
                ("quote".to_string(), "XBTM22".to_string())
            ]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_trade() {
        gen_test_code!(
            BitmexWSClient,
            subscribe_trade,
            &vec!["XBTM22".to_string(), "XBTM22".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_bbo() {
        gen_test_code!(
            BitmexWSClient,
            subscribe_bbo,
            &vec!["XBTM22".to_string(), "XBTM22".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(
            BitmexWSClient,
            subscribe_orderbook,
            &vec!["XBTM22".to_string(), "XBTM22".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook_topk() {
        gen_test_code!(
            BitmexWSClient,
            subscribe_orderbook_topk,
            &vec!["XBTM22".to_string(), "XBTM22".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(
            BitmexWSClient,
            &vec![("XBTM22".to_string(), 60), ("XBTM22".to_string(), 60)]
        );
        gen_test_subscribe_candlestick!(
            BitmexWSClient,
            &vec![("XBTM22".to_string(), 86400), ("XBTM22".to_string(), 86400)]
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
                "XBTUSDTH22".to_string(),
                "ETHH22".to_string(),
                "ETHUSDTH22".to_string()
            ]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_bbo() {
        gen_test_code!(BitmexWSClient, subscribe_bbo, &vec!["ETHH22".to_string()]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(
            BitmexWSClient,
            subscribe_orderbook,
            &vec!["ETHH22".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook_topk() {
        gen_test_code!(
            BitmexWSClient,
            subscribe_orderbook_topk,
            &vec!["ETHH22".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(BitmexWSClient, &vec![("ETHH22".to_string(), 60)]);
        gen_test_subscribe_candlestick!(BitmexWSClient, &vec![("ETHH22".to_string(), 86400)]);
    }
}
