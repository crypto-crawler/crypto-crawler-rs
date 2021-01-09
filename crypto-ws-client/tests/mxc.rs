#[macro_use]
mod utils;

#[cfg(test)]
mod mxc_spot {
    use crypto_ws_client::{MXCSpotWSClient, WSClient};
    use std::{cell::RefCell, rc::Rc};

    #[test]
    fn subscribe() {
        gen_test_code!(
            MXCSpotWSClient,
            subscribe,
            &vec!["symbol:BTC_USDT".to_string()]
        );
    }

    #[test]
    fn subscribe_raw_json() {
        gen_test_code!(
            MXCSpotWSClient,
            subscribe,
            &vec![r#"["sub.symbol",{"symbol":"BTC_USDT"}]"#.to_string()]
        );
    }

    #[test]
    fn subscribe_trade() {
        gen_test_code!(
            MXCSpotWSClient,
            subscribe_trade,
            &vec!["BTC_USDT".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook() {
        gen_test_code!(
            MXCSpotWSClient,
            subscribe_orderbook,
            &vec!["BTC_USDT".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook_snapshot() {
        gen_test_code!(
            MXCSpotWSClient,
            subscribe_orderbook_snapshot,
            &vec!["BTC_USDT".to_string()]
        );
    }

    #[test]
    fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(MXCSpotWSClient, &vec!["BTC_USDT".to_string()], 60);
        gen_test_subscribe_candlestick!(MXCSpotWSClient, &vec!["BTC_USDT".to_string()], 2592000);
    }
}

#[cfg(test)]
mod mxc_swap {
    use crypto_ws_client::{MXCSwapWSClient, WSClient};
    use std::{cell::RefCell, rc::Rc};

    #[test]
    fn subscribe() {
        gen_test_code!(
            MXCSwapWSClient,
            subscribe,
            &vec!["deal:BTC_USDT".to_string()]
        );
    }

    #[test]
    fn subscribe_raw_json() {
        gen_test_code!(
            MXCSwapWSClient,
            subscribe,
            &vec![r#"{"method":"sub.deal","param":{"symbol":"BTC_USDT"}}"#.to_string()]
        );
    }

    #[test]
    fn subscribe_trade() {
        gen_test_code!(
            MXCSwapWSClient,
            subscribe_trade,
            &vec!["BTC_USDT".to_string()]
        );
    }

    #[test]
    fn subscribe_ticker() {
        gen_test_code!(
            MXCSwapWSClient,
            subscribe_ticker,
            &vec!["BTC_USDT".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook() {
        gen_test_code!(
            MXCSwapWSClient,
            subscribe_orderbook,
            &vec!["BTC_USDT".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook_snapshot() {
        gen_test_code!(
            MXCSwapWSClient,
            subscribe_orderbook_snapshot,
            &vec!["BTC_USDT".to_string()]
        );
    }

    #[test]
    fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(MXCSwapWSClient, &vec!["BTC_USDT".to_string()], 60);
        gen_test_subscribe_candlestick!(MXCSwapWSClient, &vec!["BTC_USDT".to_string()], 2592000);
    }
}
