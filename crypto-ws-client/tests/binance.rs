#[macro_use]
mod utils;

#[cfg(test)]
mod binance_spot {
    use crypto_ws_client::{BinanceSpotWSClient, WSClient};

    #[test]
    fn subscribe() {
        gen_test_code!(
            BinanceSpotWSClient,
            subscribe,
            &vec!["btcusdt@aggTrade".to_string(), "btcusdt@ticker".to_string()]
        );
    }

    #[test]
    fn subscribe_raw_json() {
        gen_test_code!(
            BinanceSpotWSClient,
            subscribe,
            &vec![r#"{"id":9527,"method":"SUBSCRIBE","params":["btcusdt@aggTrade","btcusdt@ticker"]}"#.to_string()]
        );
    }

    #[test]
    fn subscribe_trade() {
        gen_test_code!(
            BinanceSpotWSClient,
            subscribe_trade,
            &vec!["btcusdt".to_string(), "ethusdt".to_string()]
        );
    }

    #[test]
    fn subscribe_ticker() {
        gen_test_code!(
            BinanceSpotWSClient,
            subscribe_ticker,
            &vec!["btcusdt".to_string(), "ethusdt".to_string()]
        );
    }

    #[test]
    fn subscribe_bbo() {
        gen_test_code!(
            BinanceSpotWSClient,
            subscribe_bbo,
            &vec!["btcusdt".to_string(), "ethusdt".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook() {
        gen_test_code!(
            BinanceSpotWSClient,
            subscribe_orderbook,
            &vec!["btcusdt".to_string(), "ethusdt".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook_snapshot() {
        gen_test_code!(
            BinanceSpotWSClient,
            subscribe_orderbook_snapshot,
            &vec!["btcusdt".to_string(), "ethusdt".to_string()]
        );
    }

    #[test]
    fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(
            BinanceSpotWSClient,
            &vec!["btcusdt".to_string(), "ethusdt".to_string()],
            60
        );
        gen_test_subscribe_candlestick!(
            BinanceSpotWSClient,
            &vec!["btcusdt".to_string(), "ethusdt".to_string()],
            2592000
        );
    }
}

#[cfg(test)]
mod binance_future {
    use crypto_ws_client::{BinanceFutureWSClient, WSClient};

    #[test]
    fn subscribe() {
        gen_test_code!(
            BinanceFutureWSClient,
            subscribe,
            &vec!["btcusd_210625@aggTrade".to_string()]
        );
    }

    #[test]
    fn subscribe_trade() {
        gen_test_code!(
            BinanceFutureWSClient,
            subscribe_trade,
            &vec!["btcusd_210625".to_string(), "ethusd_210625".to_string()]
        );
    }

    #[test]
    fn subscribe_ticker() {
        gen_test_code!(
            BinanceFutureWSClient,
            subscribe_ticker,
            &vec!["btcusd_210625".to_string(), "ethusd_210625".to_string()]
        );
    }

    #[test]
    fn subscribe_bbo() {
        gen_test_code!(
            BinanceFutureWSClient,
            subscribe_bbo,
            &vec!["btcusd_210625".to_string(), "ethusd_210625".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook() {
        gen_test_code!(
            BinanceFutureWSClient,
            subscribe_orderbook,
            &vec!["btcusd_210625".to_string(), "ethusd_210625".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook_snapshot() {
        gen_test_code!(
            BinanceFutureWSClient,
            subscribe_orderbook_snapshot,
            &vec!["btcusd_210625".to_string(), "ethusd_210625".to_string()]
        );
    }

    #[test]
    fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(
            BinanceFutureWSClient,
            &vec!["btcusd_210625".to_string(), "ethusd_210625".to_string()],
            60
        );
        gen_test_subscribe_candlestick!(
            BinanceFutureWSClient,
            &vec!["btcusd_210625".to_string(), "ethusd_210625".to_string()],
            2592000
        );
    }
}

#[cfg(test)]
mod binance_inverse_swap {
    use crypto_ws_client::{BinanceInverseSwapWSClient, WSClient};

    #[test]
    fn subscribe() {
        gen_test_code!(
            BinanceInverseSwapWSClient,
            subscribe,
            &vec!["btcusd_perp@aggTrade".to_string()]
        );
    }

    #[test]
    fn subscribe_trade() {
        gen_test_code!(
            BinanceInverseSwapWSClient,
            subscribe_trade,
            &vec!["btcusd_perp".to_string(), "ethusd_perp".to_string()]
        );
    }

    #[test]
    fn subscribe_ticker() {
        gen_test_code!(
            BinanceInverseSwapWSClient,
            subscribe_ticker,
            &vec!["btcusd_perp".to_string(), "ethusd_perp".to_string()]
        );
    }

    #[test]
    fn subscribe_bbo() {
        gen_test_code!(
            BinanceInverseSwapWSClient,
            subscribe_bbo,
            &vec!["btcusd_perp".to_string(), "ethusd_perp".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook() {
        gen_test_code!(
            BinanceInverseSwapWSClient,
            subscribe_orderbook,
            &vec!["btcusd_perp".to_string(), "ethusd_perp".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook_snapshot() {
        gen_test_code!(
            BinanceInverseSwapWSClient,
            subscribe_orderbook_snapshot,
            &vec!["btcusd_perp".to_string(), "ethusd_perp".to_string()]
        );
    }

    #[test]
    fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(
            BinanceInverseSwapWSClient,
            &vec!["btcusd_perp".to_string(), "ethusd_perp".to_string()],
            60
        );
        gen_test_subscribe_candlestick!(
            BinanceInverseSwapWSClient,
            &vec!["btcusd_perp".to_string(), "ethusd_perp".to_string()],
            2592000
        );
    }
}

#[cfg(test)]
mod binance_linear_swap {
    use crypto_ws_client::{BinanceLinearSwapWSClient, WSClient};

    #[test]
    fn subscribe() {
        gen_test_code!(
            BinanceLinearSwapWSClient,
            subscribe,
            &vec!["btcusdt@aggTrade".to_string()]
        );
    }

    #[test]
    fn subscribe_trade() {
        gen_test_code!(
            BinanceLinearSwapWSClient,
            subscribe_trade,
            &vec!["btcusdt".to_string(), "ethusdt".to_string()]
        );
    }

    #[test]
    fn subscribe_ticker() {
        gen_test_code!(
            BinanceLinearSwapWSClient,
            subscribe_ticker,
            &vec!["btcusdt".to_string(), "ethusdt".to_string()]
        );
    }

    #[test]
    fn subscribe_bbo() {
        gen_test_code!(
            BinanceLinearSwapWSClient,
            subscribe_bbo,
            &vec!["btcusdt".to_string(), "ethusdt".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook() {
        gen_test_code!(
            BinanceLinearSwapWSClient,
            subscribe_orderbook,
            &vec!["btcusdt".to_string(), "ethusdt".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook_snapshot() {
        gen_test_code!(
            BinanceLinearSwapWSClient,
            subscribe_orderbook_snapshot,
            &vec!["btcusdt".to_string(), "ethusdt".to_string()]
        );
    }

    #[test]
    fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(
            BinanceLinearSwapWSClient,
            &vec!["btcusdt".to_string(), "ethusdt".to_string()],
            60
        );
        gen_test_subscribe_candlestick!(
            BinanceLinearSwapWSClient,
            &vec!["btcusdt".to_string(), "ethusdt".to_string()],
            2592000
        );
    }
}
