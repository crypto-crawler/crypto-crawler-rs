use crypto_ws_client::{OKExWSClient, WSClient};

#[macro_use]
mod utils;

#[test]
fn okex_index() {
    gen_test_subscribe!(OKExWSClient, &vec!["index/ticker:BTC-USDT".to_string()]);
}

#[cfg(test)]
mod okex_spot {
    use crypto_ws_client::{OKExWSClient, WSClient};

    #[test]
    fn subscribe() {
        gen_test_subscribe!(OKExWSClient, &vec!["spot/trade:BTC-USDT".to_string()]);
    }

    #[test]
    fn subscribe_raw_json() {
        gen_test_subscribe!(
            OKExWSClient,
            &vec![r#"{"op":"subscribe","args":["spot/trade:BTC-USDT"]}"#.to_string()]
        );
    }

    #[test]
    fn subscribe_trade() {
        gen_test_subscribe_trade!(OKExWSClient, &vec!["BTC-USDT".to_string()]);
    }

    #[test]
    fn subscribe_ticker() {
        gen_test_subscribe_ticker!(OKExWSClient, &vec!["BTC-USDT".to_string()]);
    }

    #[test]
    fn subscribe_orderbook() {
        gen_test_subscribe_orderbook!(OKExWSClient, &vec!["BTC-USDT".to_string()]);
    }
}

#[cfg(test)]
mod okex_future {
    use crypto_ws_client::{OKExWSClient, WSClient};

    #[test]
    fn subscribe() {
        gen_test_subscribe!(
            OKExWSClient,
            &vec!["futures/trade:BTC-USDT-210625".to_string()]
        );
    }

    #[test]
    fn subscribe_trade() {
        gen_test_subscribe_trade!(OKExWSClient, &vec!["BTC-USDT-210625".to_string()]);
    }

    #[test]
    fn subscribe_ticker() {
        gen_test_subscribe_ticker!(OKExWSClient, &vec!["BTC-USDT-210625".to_string()]);
    }

    #[test]
    fn subscribe_orderbook() {
        gen_test_subscribe_orderbook!(OKExWSClient, &vec!["BTC-USDT-210625".to_string()]);
    }
}

#[cfg(test)]
mod okex_swap {
    use crypto_ws_client::{OKExWSClient, WSClient};

    #[test]
    fn subscribe() {
        gen_test_subscribe!(OKExWSClient, &vec!["swap/trade:BTC-USDT-SWAP".to_string()]);
    }

    #[test]
    fn subscribe_trade() {
        gen_test_subscribe_trade!(OKExWSClient, &vec!["BTC-USDT-SWAP".to_string()]);
    }

    #[test]
    fn subscribe_ticker() {
        gen_test_subscribe_ticker!(OKExWSClient, &vec!["BTC-USDT-SWAP".to_string()]);
    }

    #[test]
    fn subscribe_orderbook() {
        gen_test_subscribe_orderbook!(OKExWSClient, &vec!["BTC-USDT-SWAP".to_string()]);
    }
}

#[cfg(test)]
mod okex_option {
    use crypto_ws_client::{OKExWSClient, WSClient};

    #[test]
    fn subscribe() {
        gen_test_subscribe!(
            OKExWSClient,
            &vec!["option/trade:BTC-USD-210625-72000-C".to_string()]
        );
    }

    #[test]
    fn subscribe_trade() {
        gen_test_subscribe_trade!(OKExWSClient, &vec!["BTC-USD-210625-72000-C".to_string()]);
    }

    #[test]
    fn subscribe_ticker() {
        gen_test_subscribe_ticker!(OKExWSClient, &vec!["BTC-USD-210625-72000-C".to_string()]);
    }

    #[test]
    fn subscribe_orderbook() {
        gen_test_subscribe_orderbook!(OKExWSClient, &vec!["BTC-USD-210625-72000-C".to_string()]);
    }
}
