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
            &vec![
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
            &vec![r#"{"id":9527,"method":"SUBSCRIBE","params":["!bookTicker"]}"#.to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_raw_json() {
        gen_test_code!(
            BinanceSpotWSClient,
            send,
            &vec![r#"{"id":9527,"method":"SUBSCRIBE","params":["btcusdt@aggTrade","btcusdt@ticker"]}"#.to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_trade() {
        gen_test_code!(
            BinanceSpotWSClient,
            subscribe_trade,
            &vec!["BTCUSDT".to_string(), "ETHUSDT".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_ticker() {
        gen_test_code!(
            BinanceSpotWSClient,
            subscribe_ticker,
            &vec!["BTCUSDT".to_string(), "ETHUSDT".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_tickers_all() {
        gen_test_code!(
            BinanceSpotWSClient,
            send,
            &vec![r#"{"id":9527,"method":"SUBSCRIBE","params":["!ticker@arr"]}"#.to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_bbo() {
        gen_test_code!(
            BinanceSpotWSClient,
            subscribe_bbo,
            &vec!["BTCUSDT".to_string(), "ETHUSDT".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(
            BinanceSpotWSClient,
            subscribe_orderbook,
            &vec!["BTCUSDT".to_string(), "ETHUSDT".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook_topk() {
        gen_test_code!(
            BinanceSpotWSClient,
            subscribe_orderbook_topk,
            &vec!["BTCUSDT".to_string(), "ETHUSDT".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(
            BinanceSpotWSClient,
            &vec![("BTCUSDT".to_string(), 60), ("ETHUSDT".to_string(), 60)]
        );
        gen_test_subscribe_candlestick!(
            BinanceSpotWSClient,
            &vec![
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
            &vec![
                ("aggTrade".to_string(), "BTCUSD_220930".to_string()),
                ("aggTrade".to_string(), "ETHUSD_220930".to_string()),
                ("aggTrade".to_string(), "BNBUSD_220930".to_string())
            ]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_all_bbo() {
        gen_test_code!(
            BinanceInverseWSClient,
            send,
            &vec![r#"{"id":9527,"method":"SUBSCRIBE","params":["!bookTicker"]}"#.to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_trade() {
        gen_test_code!(
            BinanceInverseWSClient,
            subscribe_trade,
            &vec![
                "BTCUSD_220930".to_string(),
                "ETHUSD_220930".to_string(),
                "BNBUSD_220930".to_string(),
            ]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_ticker() {
        gen_test_code!(
            BinanceInverseWSClient,
            subscribe_ticker,
            &vec![
                "BTCUSD_220930".to_string(),
                "ETHUSD_220930".to_string(),
                "BNBUSD_220930".to_string(),
            ]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_tickers_all() {
        gen_test_code!(
            BinanceInverseWSClient,
            send,
            &vec![r#"{"id":9527,"method":"SUBSCRIBE","params":["!ticker@arr"]}"#.to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_bbo() {
        gen_test_code!(
            BinanceInverseWSClient,
            subscribe_bbo,
            &vec![
                "BTCUSD_220930".to_string(),
                "ETHUSD_220930".to_string(),
                "BNBUSD_220930".to_string(),
            ]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(
            BinanceInverseWSClient,
            subscribe_orderbook,
            &vec![
                "BTCUSD_220930".to_string(),
                "ETHUSD_220930".to_string(),
                "BNBUSD_220930".to_string(),
            ]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook_topk() {
        gen_test_code!(
            BinanceInverseWSClient,
            subscribe_orderbook_topk,
            &vec![
                "BTCUSD_220930".to_string(),
                "ETHUSD_220930".to_string(),
                "BNBUSD_220930".to_string(),
            ]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(
            BinanceInverseWSClient,
            &vec![
                ("BTCUSD_220930".to_string(), 60),
                ("ETHUSD_220930".to_string(), 60),
                ("BNBUSD_220930".to_string(), 60)
            ]
        );
        gen_test_subscribe_candlestick!(
            BinanceInverseWSClient,
            &vec![
                ("BTCUSD_220930".to_string(), 2592000),
                ("ETHUSD_220930".to_string(), 2592000),
                ("BNBUSD_220930".to_string(), 2592000)
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
            &vec![
                ("aggTrade".to_string(), "BTCUSDT_220930".to_string()),
                ("aggTrade".to_string(), "ETHUSDT_220930".to_string()),
            ]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_all_bbo() {
        gen_test_code!(
            BinanceLinearWSClient,
            send,
            &vec![r#"{"id":9527,"method":"SUBSCRIBE","params":["!bookTicker"]}"#.to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_trade() {
        gen_test_code!(
            BinanceLinearWSClient,
            subscribe_trade,
            &vec!["BTCUSDT_220930".to_string(), "ETHUSDT_220930".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_ticker() {
        gen_test_code!(
            BinanceLinearWSClient,
            subscribe_ticker,
            &vec!["BTCUSDT_220930".to_string(), "ETHUSDT_220930".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_tickers_all() {
        gen_test_code!(
            BinanceLinearWSClient,
            send,
            &vec![r#"{"id":9527,"method":"SUBSCRIBE","params":["!ticker@arr"]}"#.to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_bbo() {
        gen_test_code!(
            BinanceLinearWSClient,
            subscribe_bbo,
            &vec!["BTCUSDT_220930".to_string(), "ETHUSDT_220930".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(
            BinanceLinearWSClient,
            subscribe_orderbook,
            &vec!["BTCUSDT_220930".to_string(), "ETHUSDT_220930".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook_topk() {
        gen_test_code!(
            BinanceLinearWSClient,
            subscribe_orderbook_topk,
            &vec!["BTCUSDT_220930".to_string(), "ETHUSDT_220930".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(
            BinanceLinearWSClient,
            &vec![
                ("BTCUSDT_220930".to_string(), 60),
                ("ETHUSDT_220930".to_string(), 60)
            ]
        );
        gen_test_subscribe_candlestick!(
            BinanceLinearWSClient,
            &vec![
                ("BTCUSDT_220930".to_string(), 2592000),
                ("ETHUSDT_220930".to_string(), 2592000)
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
            &vec![("aggTrade".to_string(), "btcusd_perp".to_string())]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_all_bbo() {
        gen_test_code!(
            BinanceInverseWSClient,
            send,
            &vec![r#"{"id":9527,"method":"SUBSCRIBE","params":["!bookTicker"]}"#.to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_trade() {
        gen_test_code!(
            BinanceInverseWSClient,
            subscribe_trade,
            &vec!["btcusd_perp".to_string(), "ethusd_perp".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_ticker() {
        gen_test_code!(
            BinanceInverseWSClient,
            subscribe_ticker,
            &vec!["btcusd_perp".to_string(), "ethusd_perp".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_tickers_all() {
        gen_test_code!(
            BinanceInverseWSClient,
            send,
            &vec![r#"{"id":9527,"method":"SUBSCRIBE","params":["!ticker@arr"]}"#.to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_bbo() {
        gen_test_code!(
            BinanceInverseWSClient,
            subscribe_bbo,
            &vec!["btcusd_perp".to_string(), "ethusd_perp".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(
            BinanceInverseWSClient,
            subscribe_orderbook,
            &vec!["btcusd_perp".to_string(), "ethusd_perp".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook_topk() {
        gen_test_code!(
            BinanceInverseWSClient,
            subscribe_orderbook_topk,
            &vec!["btcusd_perp".to_string(), "ethusd_perp".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(
            BinanceInverseWSClient,
            &vec![
                ("btcusd_perp".to_string(), 60),
                ("ethusd_perp".to_string(), 60)
            ]
        );
        gen_test_subscribe_candlestick!(
            BinanceInverseWSClient,
            &vec![
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
            &vec![("markPrice".to_string(), "btcusd_perp".to_string())]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_funding_rate_all() {
        gen_test_code!(
            BinanceInverseWSClient,
            send,
            &vec![r#"{"id":9527,"method":"SUBSCRIBE","params":["!markPrice@arr"]}"#.to_string()]
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
            &vec![("aggTrade".to_string(), "BTCUSDT".to_string())]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_all_bbo() {
        gen_test_code!(
            BinanceLinearWSClient,
            send,
            &vec![r#"{"id":9527,"method":"SUBSCRIBE","params":["!bookTicker"]}"#.to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_trade() {
        gen_test_code!(
            BinanceLinearWSClient,
            subscribe_trade,
            &vec!["BTCUSDT".to_string(), "ETHUSDT".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_ticker() {
        gen_test_code!(
            BinanceLinearWSClient,
            subscribe_ticker,
            &vec!["BTCUSDT".to_string(), "ETHUSDT".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_tickers_all() {
        gen_test_code!(
            BinanceLinearWSClient,
            send,
            &vec![r#"{"id":9527,"method":"SUBSCRIBE","params":["!ticker@arr"]}"#.to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_bbo() {
        gen_test_code!(
            BinanceLinearWSClient,
            subscribe_bbo,
            &vec!["BTCUSDT".to_string(), "ETHUSDT".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook() {
        gen_test_code!(
            BinanceLinearWSClient,
            subscribe_orderbook,
            &vec!["BTCUSDT".to_string(), "ETHUSDT".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_orderbook_topk() {
        gen_test_code!(
            BinanceLinearWSClient,
            subscribe_orderbook_topk,
            &vec!["BTCUSDT".to_string(), "ETHUSDT".to_string()]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(
            BinanceLinearWSClient,
            &vec![("BTCUSDT".to_string(), 60), ("ETHUSDT".to_string(), 60)]
        );
        gen_test_subscribe_candlestick!(
            BinanceLinearWSClient,
            &vec![
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
            &vec![("markPrice".to_string(), "BTCUSDT".to_string())]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn subscribe_funding_rate_all() {
        gen_test_code!(
            BinanceLinearWSClient,
            send,
            &vec![r#"{"id":9527,"method":"SUBSCRIBE","params":["!markPrice@arr"]}"#.to_string()]
        );
    }
}
