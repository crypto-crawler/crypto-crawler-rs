#[macro_use]
mod utils;

#[cfg(test)]
mod mxc_spot {
    use crypto_ws_client::{MXCSpotWSClient, OrderBookSnapshot, WSClient};

    #[test]
    fn subscribe() {
        gen_test_subscribe!(MXCSpotWSClient, &vec!["symbol:BTC_USDT".to_string()]);
    }

    #[test]
    fn subscribe_raw_json() {
        gen_test_subscribe!(
            MXCSpotWSClient,
            &vec![r#"["sub.symbol",{"symbol":"BTC_USDT"}]"#.to_string()]
        );
    }

    #[test]
    fn subscribe_trade() {
        gen_test_subscribe_trade!(MXCSpotWSClient, &vec!["BTC_USDT".to_string()]);
    }

    #[test]
    fn subscribe_orderbook() {
        gen_test_subscribe_orderbook!(MXCSpotWSClient, &vec!["BTC_USDT".to_string()]);
    }

    #[test]
    fn subscribe_orderbook_snapshot() {
        gen_test_subscribe_orderbook_snapshot!(MXCSpotWSClient, &vec!["BTC_USDT".to_string()]);
    }
}

#[cfg(test)]
mod mxc_swap {
    use crypto_ws_client::{MXCSwapWSClient, OrderBookSnapshot, WSClient};

    #[test]
    fn subscribe() {
        gen_test_subscribe!(MXCSwapWSClient, &vec!["deal:BTC_USDT".to_string()]);
    }

    #[test]
    fn subscribe_raw_json() {
        gen_test_subscribe!(
            MXCSwapWSClient,
            &vec![r#"{"method":"sub.deal","param":{"symbol":"BTC_USDT"}}"#.to_string()]
        );
    }

    #[test]
    fn subscribe_trade() {
        gen_test_subscribe_trade!(MXCSwapWSClient, &vec!["BTC_USDT".to_string()]);
    }

    #[test]
    fn subscribe_ticker() {
        gen_test_subscribe_ticker!(MXCSwapWSClient, &vec!["BTC_USDT".to_string()]);
    }

    #[test]
    fn subscribe_orderbook() {
        gen_test_subscribe_orderbook!(MXCSwapWSClient, &vec!["BTC_USDT".to_string()]);
    }

    #[test]
    fn subscribe_orderbook_snapshot() {
        gen_test_subscribe_orderbook_snapshot!(MXCSwapWSClient, &vec!["BTC_USDT".to_string()]);
    }
}
