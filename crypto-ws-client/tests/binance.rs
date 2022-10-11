#[macro_use]
mod utils;

#[cfg(test)]
mod binance_spot {
    use crypto_ws_client::{BinanceSpotWSClient, WSClient};

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe() {
        gen_test_code!(
            BinanceSpotWSClient,
            subscribe,
            &[
                ("aggTrade".to_string(), "BTCUSDT".to_string()),
                ("ticker".to_string(), "BTCUSDT".to_string())
            ]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_all_bbo() {
        gen_test_code!(
            BinanceSpotWSClient,
            send,
            &[r#"{"id":9527,"method":"SUBSCRIBE","params":["!bookTicker"]}"#.to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_raw_json() {
        gen_test_code!(
            BinanceSpotWSClient,
            send,
            &[r#"{"id":9527,"method":"SUBSCRIBE","params":["btcusdt@aggTrade","btcusdt@ticker"]}"#.to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_trade() {
        gen_test_code!(
            BinanceSpotWSClient,
            subscribe_trade,
            &["BTCUSDT".to_string(), "ETHUSDT".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_ticker() {
        gen_test_code!(
            BinanceSpotWSClient,
            subscribe_ticker,
            &["BTCUSDT".to_string(), "ETHUSDT".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_tickers_all() {
        gen_test_code!(
            BinanceSpotWSClient,
            send,
            &[r#"{"id":9527,"method":"SUBSCRIBE","params":["!ticker@arr"]}"#.to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_bbo() {
        gen_test_code!(
            BinanceSpotWSClient,
            subscribe_bbo,
            &["BTCUSDT".to_string(), "ETHUSDT".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(
            BinanceSpotWSClient,
            subscribe_orderbook,
            &["BTCUSDT".to_string(), "ETHUSDT".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook_topk() {
        gen_test_code!(
            BinanceSpotWSClient,
            subscribe_orderbook_topk,
            &["BTCUSDT".to_string(), "ETHUSDT".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(
            BinanceSpotWSClient,
            &[("BTCUSDT".to_string(), 60), ("ETHUSDT".to_string(), 60)]
        );
        gen_test_subscribe_candlestick!(
            BinanceSpotWSClient,
            &[
                ("BTCUSDT".to_string(), 2592000),
                ("ETHUSDT".to_string(), 2592000)
            ]
        );
    }
}

#[cfg(test)]
mod binance_inverse_future {
    use crypto_ws_client::{BinanceInverseWSClient, WSClient};

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe() {
        gen_test_code!(
            BinanceInverseWSClient,
            subscribe,
            &[
                ("aggTrade".to_string(), "BTCUSD_221230".to_string()),
                ("aggTrade".to_string(), "ETHUSD_221230".to_string()),
                ("aggTrade".to_string(), "BNBUSD_221230".to_string())
            ]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_all_bbo() {
        gen_test_code!(
            BinanceInverseWSClient,
            send,
            &[r#"{"id":9527,"method":"SUBSCRIBE","params":["!bookTicker"]}"#.to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_trade() {
        gen_test_code!(
            BinanceInverseWSClient,
            subscribe_trade,
            &[
                "BTCUSD_221230".to_string(),
                "ETHUSD_221230".to_string(),
                "BNBUSD_221230".to_string()
            ]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_ticker() {
        gen_test_code!(
            BinanceInverseWSClient,
            subscribe_ticker,
            &[
                "BTCUSD_221230".to_string(),
                "ETHUSD_221230".to_string(),
                "BNBUSD_221230".to_string()
            ]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_tickers_all() {
        gen_test_code!(
            BinanceInverseWSClient,
            send,
            &[r#"{"id":9527,"method":"SUBSCRIBE","params":["!ticker@arr"]}"#.to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_bbo() {
        gen_test_code!(
            BinanceInverseWSClient,
            subscribe_bbo,
            &[
                "BTCUSD_221230".to_string(),
                "ETHUSD_221230".to_string(),
                "BNBUSD_221230".to_string()
            ]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(
            BinanceInverseWSClient,
            subscribe_orderbook,
            &[
                "BTCUSD_221230".to_string(),
                "ETHUSD_221230".to_string(),
                "BNBUSD_221230".to_string()
            ]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook_topk() {
        gen_test_code!(
            BinanceInverseWSClient,
            subscribe_orderbook_topk,
            &[
                "BTCUSD_221230".to_string(),
                "ETHUSD_221230".to_string(),
                "BNBUSD_221230".to_string()
            ]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(
            BinanceInverseWSClient,
            &[
                ("BTCUSD_221230".to_string(), 60),
                ("ETHUSD_221230".to_string(), 60),
                ("BNBUSD_221230".to_string(), 60)
            ]
        );
        gen_test_subscribe_candlestick!(
            BinanceInverseWSClient,
            &[
                ("BTCUSD_221230".to_string(), 2592000),
                ("ETHUSD_221230".to_string(), 2592000),
                ("BNBUSD_221230".to_string(), 2592000)
            ]
        );
    }
}

#[cfg(test)]
mod binance_linear_future {
    use crypto_ws_client::{BinanceLinearWSClient, WSClient};

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe() {
        gen_test_code!(
            BinanceLinearWSClient,
            subscribe,
            &[
                ("aggTrade".to_string(), "BTCUSDT_221230".to_string()),
                ("aggTrade".to_string(), "ETHUSDT_221230".to_string())
            ]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_all_bbo() {
        gen_test_code!(
            BinanceLinearWSClient,
            send,
            &[r#"{"id":9527,"method":"SUBSCRIBE","params":["!bookTicker"]}"#.to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_trade() {
        gen_test_code!(
            BinanceLinearWSClient,
            subscribe_trade,
            &["BTCUSDT_221230".to_string(), "ETHUSDT_221230".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_ticker() {
        gen_test_code!(
            BinanceLinearWSClient,
            subscribe_ticker,
            &["BTCUSDT_221230".to_string(), "ETHUSDT_221230".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_tickers_all() {
        gen_test_code!(
            BinanceLinearWSClient,
            send,
            &[r#"{"id":9527,"method":"SUBSCRIBE","params":["!ticker@arr"]}"#.to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_bbo() {
        gen_test_code!(
            BinanceLinearWSClient,
            subscribe_bbo,
            &["BTCUSDT_221230".to_string(), "ETHUSDT_221230".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(
            BinanceLinearWSClient,
            subscribe_orderbook,
            &["BTCUSDT_221230".to_string(), "ETHUSDT_221230".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook_topk() {
        gen_test_code!(
            BinanceLinearWSClient,
            subscribe_orderbook_topk,
            &["BTCUSDT_221230".to_string(), "ETHUSDT_221230".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(
            BinanceLinearWSClient,
            &[
                ("BTCUSDT_221230".to_string(), 60),
                ("ETHUSDT_221230".to_string(), 60)
            ]
        );
        gen_test_subscribe_candlestick!(
            BinanceLinearWSClient,
            &[
                ("BTCUSDT_221230".to_string(), 2592000),
                ("ETHUSDT_221230".to_string(), 2592000)
            ]
        );
    }
}

#[cfg(test)]
mod binance_inverse_swap {
    use crypto_ws_client::{BinanceInverseWSClient, WSClient};

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe() {
        gen_test_code!(
            BinanceInverseWSClient,
            subscribe,
            &[("aggTrade".to_string(), "btcusd_perp".to_string())]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_all_bbo() {
        gen_test_code!(
            BinanceInverseWSClient,
            send,
            &[r#"{"id":9527,"method":"SUBSCRIBE","params":["!bookTicker"]}"#.to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_trade() {
        gen_test_code!(
            BinanceInverseWSClient,
            subscribe_trade,
            &["btcusd_perp".to_string(), "ethusd_perp".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_ticker() {
        gen_test_code!(
            BinanceInverseWSClient,
            subscribe_ticker,
            &["btcusd_perp".to_string(), "ethusd_perp".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_tickers_all() {
        gen_test_code!(
            BinanceInverseWSClient,
            send,
            &[r#"{"id":9527,"method":"SUBSCRIBE","params":["!ticker@arr"]}"#.to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_bbo() {
        gen_test_code!(
            BinanceInverseWSClient,
            subscribe_bbo,
            &["btcusd_perp".to_string(), "ethusd_perp".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(
            BinanceInverseWSClient,
            subscribe_orderbook,
            &["btcusd_perp".to_string(), "ethusd_perp".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook_topk() {
        gen_test_code!(
            BinanceInverseWSClient,
            subscribe_orderbook_topk,
            &["btcusd_perp".to_string(), "ethusd_perp".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(
            BinanceInverseWSClient,
            &[
                ("btcusd_perp".to_string(), 60),
                ("ethusd_perp".to_string(), 60)
            ]
        );
        gen_test_subscribe_candlestick!(
            BinanceInverseWSClient,
            &[
                ("btcusd_perp".to_string(), 2592000),
                ("ethusd_perp".to_string(), 2592000)
            ]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_funding_rate() {
        gen_test_code!(
            BinanceInverseWSClient,
            subscribe,
            &[("markPrice".to_string(), "btcusd_perp".to_string())]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_funding_rate_all() {
        gen_test_code!(
            BinanceInverseWSClient,
            send,
            &[r#"{"id":9527,"method":"SUBSCRIBE","params":["!markPrice@arr"]}"#.to_string()]
        );
    }
}

#[cfg(test)]
mod binance_linear_swap {
    use crypto_ws_client::{BinanceLinearWSClient, WSClient};

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe() {
        gen_test_code!(
            BinanceLinearWSClient,
            subscribe,
            &[("aggTrade".to_string(), "BTCUSDT".to_string())]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_all_bbo() {
        gen_test_code!(
            BinanceLinearWSClient,
            send,
            &[r#"{"id":9527,"method":"SUBSCRIBE","params":["!bookTicker"]}"#.to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_trade() {
        gen_test_code!(
            BinanceLinearWSClient,
            subscribe_trade,
            &["BTCUSDT".to_string(), "ETHUSDT".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_ticker() {
        gen_test_code!(
            BinanceLinearWSClient,
            subscribe_ticker,
            &["BTCUSDT".to_string(), "ETHUSDT".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_tickers_all() {
        gen_test_code!(
            BinanceLinearWSClient,
            send,
            &[r#"{"id":9527,"method":"SUBSCRIBE","params":["!ticker@arr"]}"#.to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_bbo() {
        gen_test_code!(
            BinanceLinearWSClient,
            subscribe_bbo,
            &["BTCUSDT".to_string(), "ETHUSDT".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(
            BinanceLinearWSClient,
            subscribe_orderbook,
            &["BTCUSDT".to_string(), "ETHUSDT".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook_topk() {
        gen_test_code!(
            BinanceLinearWSClient,
            subscribe_orderbook_topk,
            &["BTCUSDT".to_string(), "ETHUSDT".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(
            BinanceLinearWSClient,
            &[("BTCUSDT".to_string(), 60), ("ETHUSDT".to_string(), 60)]
        );
        gen_test_subscribe_candlestick!(
            BinanceLinearWSClient,
            &[
                ("BTCUSDT".to_string(), 2592000),
                ("ETHUSDT".to_string(), 2592000)
            ]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_funding_rate() {
        gen_test_code!(
            BinanceLinearWSClient,
            subscribe,
            &[("markPrice".to_string(), "BTCUSDT".to_string())]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_funding_rate_all() {
        gen_test_code!(
            BinanceLinearWSClient,
            send,
            &[r#"{"id":9527,"method":"SUBSCRIBE","params":["!markPrice@arr"]}"#.to_string()]
        );
    }
}
