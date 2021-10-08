#[macro_use]
mod utils;

#[cfg(test)]
mod mxc_spot {
    use crypto_ws_client::{MxcSpotWSClient, WSClient};
    use std::sync::mpsc::{Receiver, Sender};

    #[test]
    #[ignore]
    fn subscribe() {
        gen_test_code!(
            MxcSpotWSClient,
            subscribe,
            &vec![
                "symbol:BTC_USDT".to_string(),
                "symbol:ETH_USDT".to_string(),
                "symbol:MX_USDT".to_string()
            ]
        );
    }

    #[test]
    #[ignore]
    fn subscribe_raw_json() {
        gen_test_code!(
            MxcSpotWSClient,
            subscribe,
            &vec![
                r#"["sub.symbol",{"symbol":"BTC_USDT"}]"#.to_string(),
                r#"["sub.symbol",{"symbol":"ETH_USDT"}]"#.to_string(),
                r#"["sub.symbol",{"symbol":"MX_USDT"}]"#.to_string()
            ]
        );
    }

    #[test]
    fn subscribe_trade() {
        gen_test_code!(
            MxcSpotWSClient,
            subscribe_trade,
            &vec![
                "BTC_USDT".to_string(),
                "ETH_USDT".to_string(),
                "MX_USDT".to_string()
            ]
        );
    }

    #[test]
    fn subscribe_orderbook() {
        gen_test_code!(
            MxcSpotWSClient,
            subscribe_orderbook,
            &vec![
                "BTC_USDT".to_string(),
                "ETH_USDT".to_string(),
                "MX_USDT".to_string()
            ]
        );
    }

    #[test]
    #[ignore]
    fn subscribe_orderbook_topk() {
        gen_test_code!(
            MxcSpotWSClient,
            subscribe_orderbook_topk,
            &vec![
                "BTC_USDT".to_string(),
                "ETH_USDT".to_string(),
                "MX_USDT".to_string()
            ]
        );
    }

    #[test]
    #[ignore]
    fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(
            MxcSpotWSClient,
            &vec![
                ("BTC_USDT".to_string(), 60),
                ("ETH_USDT".to_string(), 60),
                ("MX_USDT".to_string(), 60)
            ]
        );
        gen_test_subscribe_candlestick!(
            MxcSpotWSClient,
            &vec![
                ("BTC_USDT".to_string(), 2592000),
                ("ETH_USDT".to_string(), 2592000),
                ("MX_USDT".to_string(), 2592000)
            ]
        );
    }
}

#[cfg(test)]
mod mxc_linear_swap {
    use crypto_ws_client::{MxcSwapWSClient, WSClient};
    use std::sync::mpsc::{Receiver, Sender};

    #[test]
    fn subscribe() {
        gen_test_code!(
            MxcSwapWSClient,
            subscribe,
            &vec!["deal:BTC_USDT".to_string()]
        );
    }

    #[test]
    fn subscribe_raw_json() {
        gen_test_code!(
            MxcSwapWSClient,
            subscribe,
            &vec![r#"{"method":"sub.deal","param":{"symbol":"BTC_USDT"}}"#.to_string()]
        );
    }

    #[test]
    fn subscribe_trade() {
        gen_test_code!(
            MxcSwapWSClient,
            subscribe_trade,
            &vec!["BTC_USDT".to_string()]
        );
    }

    #[test]
    fn subscribe_ticker() {
        gen_test_code!(
            MxcSwapWSClient,
            subscribe_ticker,
            &vec!["BTC_USDT".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook() {
        gen_test_code!(
            MxcSwapWSClient,
            subscribe_orderbook,
            &vec!["BTC_USDT".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook_topk() {
        gen_test_code!(
            MxcSwapWSClient,
            subscribe_orderbook_topk,
            &vec!["BTC_USDT".to_string()]
        );
    }

    #[test]
    fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(MxcSwapWSClient, &vec![("BTC_USDT".to_string(), 60)]);
        gen_test_subscribe_candlestick!(MxcSwapWSClient, &vec![("BTC_USDT".to_string(), 2592000)]);
    }
}

#[cfg(test)]
mod mxc_inverse_swap {
    use crypto_ws_client::{MxcSwapWSClient, WSClient};
    use std::sync::mpsc::{Receiver, Sender};

    #[test]
    fn subscribe_trade() {
        gen_test_code!(
            MxcSwapWSClient,
            subscribe_trade,
            &vec!["BTC_USD".to_string()]
        );
    }

    #[test]
    fn subscribe_ticker() {
        gen_test_code!(
            MxcSwapWSClient,
            subscribe_ticker,
            &vec!["BTC_USD".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook() {
        gen_test_code!(
            MxcSwapWSClient,
            subscribe_orderbook,
            &vec!["BTC_USD".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook_topk() {
        gen_test_code!(
            MxcSwapWSClient,
            subscribe_orderbook_topk,
            &vec!["BTC_USD".to_string()]
        );
    }

    #[test]
    fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(MxcSwapWSClient, &vec![("BTC_USD".to_string(), 60)]);
        gen_test_subscribe_candlestick!(MxcSwapWSClient, &vec![("BTC_USD".to_string(), 2592000)]);
    }
}
