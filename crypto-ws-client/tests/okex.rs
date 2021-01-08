use crypto_ws_client::{OKExWSClient, WSClient};

#[macro_use]
mod utils;

#[test]
fn okex_index() {
    gen_test_code!(
        OKExWSClient,
        subscribe,
        &vec!["index/ticker:BTC-USDT".to_string()]
    );
}

#[cfg(test)]
mod okex_spot {
    use crypto_ws_client::{OKExWSClient, WSClient};

    #[test]
    fn subscribe() {
        gen_test_code!(
            OKExWSClient,
            subscribe,
            &vec!["spot/trade:BTC-USDT".to_string()]
        );
    }

    #[test]
    fn subscribe_raw_json() {
        gen_test_code!(
            OKExWSClient,
            subscribe,
            &vec![r#"{"op":"subscribe","args":["spot/trade:BTC-USDT"]}"#.to_string()]
        );
    }

    #[test]
    fn subscribe_trade() {
        gen_test_code!(OKExWSClient, subscribe_trade, &vec!["BTC-USDT".to_string()]);
    }

    #[test]
    fn subscribe_ticker() {
        gen_test_code!(
            OKExWSClient,
            subscribe_ticker,
            &vec!["BTC-USDT".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook() {
        gen_test_code!(
            OKExWSClient,
            subscribe_orderbook,
            &vec!["BTC-USDT".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook_snapshot() {
        gen_test_code!(
            OKExWSClient,
            subscribe_orderbook_snapshot,
            &vec!["BTC-USDT".to_string()]
        );
    }

    #[test]
    fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(OKExWSClient, &vec!["BTC-USDT".to_string()], 60);
        gen_test_subscribe_candlestick!(OKExWSClient, &vec!["BTC-USDT".to_string()], 604800);
    }
}

#[cfg(test)]
mod okex_future {
    use crypto_ws_client::{OKExWSClient, WSClient};

    #[test]
    fn subscribe() {
        gen_test_code!(
            OKExWSClient,
            subscribe,
            &vec!["futures/trade:BTC-USDT-210625".to_string()]
        );
    }

    #[test]
    fn subscribe_trade() {
        gen_test_code!(
            OKExWSClient,
            subscribe_trade,
            &vec!["BTC-USDT-210625".to_string()]
        );
    }

    #[test]
    fn subscribe_ticker() {
        gen_test_code!(
            OKExWSClient,
            subscribe_ticker,
            &vec!["BTC-USDT-210625".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook() {
        gen_test_code!(
            OKExWSClient,
            subscribe_orderbook,
            &vec!["BTC-USDT-210625".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook_snapshot() {
        gen_test_code!(
            OKExWSClient,
            subscribe_orderbook_snapshot,
            &vec!["BTC-USDT-210625".to_string()]
        );
    }

    #[test]
    fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(OKExWSClient, &vec!["BTC-USDT-210625".to_string()], 60);
        gen_test_subscribe_candlestick!(OKExWSClient, &vec!["BTC-USDT-210625".to_string()], 604800);
    }
}

#[cfg(test)]
mod okex_swap {
    use crypto_ws_client::{OKExWSClient, WSClient};

    #[test]
    fn subscribe() {
        gen_test_code!(
            OKExWSClient,
            subscribe,
            &vec!["swap/trade:BTC-USDT-SWAP".to_string()]
        );
    }

    #[test]
    fn subscribe_trade() {
        gen_test_code!(
            OKExWSClient,
            subscribe_trade,
            &vec!["BTC-USDT-SWAP".to_string()]
        );
    }

    #[test]
    fn subscribe_ticker() {
        gen_test_code!(
            OKExWSClient,
            subscribe_ticker,
            &vec!["BTC-USDT-SWAP".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook() {
        gen_test_code!(
            OKExWSClient,
            subscribe_orderbook,
            &vec!["BTC-USDT-SWAP".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook_snapshot() {
        gen_test_code!(
            OKExWSClient,
            subscribe_orderbook_snapshot,
            &vec!["BTC-USDT-SWAP".to_string()]
        );
    }

    #[test]
    fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(OKExWSClient, &vec!["BTC-USDT-SWAP".to_string()], 60);
        gen_test_subscribe_candlestick!(OKExWSClient, &vec!["BTC-USDT-SWAP".to_string()], 604800);
    }
}

#[cfg(test)]
mod okex_option {
    use crypto_ws_client::{OKExWSClient, WSClient};

    #[test]
    fn subscribe() {
        gen_test_code!(
            OKExWSClient,
            subscribe,
            &vec!["option/trade:BTC-USD-210625-72000-C".to_string()]
        );
    }

    #[test]
    fn subscribe_trade() {
        gen_test_code!(
            OKExWSClient,
            subscribe_trade,
            &vec!["BTC-USD-210625-72000-C".to_string()]
        );
    }

    #[test]
    fn subscribe_ticker() {
        gen_test_code!(
            OKExWSClient,
            subscribe_ticker,
            &vec!["BTC-USD-210625-72000-C".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook() {
        gen_test_code!(
            OKExWSClient,
            subscribe_orderbook,
            &vec!["BTC-USD-210625-72000-C".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook_snapshot() {
        gen_test_code!(
            OKExWSClient,
            subscribe_orderbook_snapshot,
            &vec!["BTC-USD-210625-72000-C".to_string()]
        );
    }

    #[test]
    fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(
            OKExWSClient,
            &vec!["BTC-USD-210625-72000-C".to_string()],
            60
        );
        gen_test_subscribe_candlestick!(
            OKExWSClient,
            &vec!["BTC-USD-210625-72000-C".to_string()],
            604800
        );
    }
}
