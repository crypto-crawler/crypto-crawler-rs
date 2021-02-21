#[macro_use]
mod utils;

#[cfg(test)]
mod bitget_inverse_swap {
    use crypto_ws_client::{BitgetSwapWSClient, WSClient};
    use std::sync::{Arc, Mutex};

    #[test]
    fn subscribe() {
        gen_test_code!(
            BitgetSwapWSClient,
            subscribe,
            &vec!["swap/trade:btcusd".to_string()]
        );
    }

    #[test]
    fn subscribe_raw_json() {
        gen_test_code!(
            BitgetSwapWSClient,
            subscribe,
            &vec![r#"{"op":"subscribe","args":["swap/trade:btcusd"]}"#.to_string()]
        );
    }

    #[test]
    fn subscribe_trade() {
        gen_test_code!(
            BitgetSwapWSClient,
            subscribe_trade,
            &vec!["btcusd".to_string()]
        );
    }

    #[test]
    fn subscribe_bbo() {
        gen_test_code!(
            BitgetSwapWSClient,
            subscribe_bbo,
            &vec!["btcusd".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook() {
        gen_test_code!(
            BitgetSwapWSClient,
            subscribe_orderbook,
            &vec!["btcusd".to_string()]
        );
    }

    #[test]
    fn subscribe_ticker() {
        gen_test_code!(
            BitgetSwapWSClient,
            subscribe_ticker,
            &vec!["btcusd".to_string()]
        );
    }

    #[test]
    fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(BitgetSwapWSClient, &vec!["btcusd".to_string()], 60);
        gen_test_subscribe_candlestick!(BitgetSwapWSClient, &vec!["btcusd".to_string()], 604800);
    }
}

#[cfg(test)]
mod bitget_linear_swap {
    use crypto_ws_client::{BitgetSwapWSClient, WSClient};
    use std::sync::{Arc, Mutex};

    #[test]
    fn subscribe_trade() {
        gen_test_code!(
            BitgetSwapWSClient,
            subscribe_trade,
            &vec!["cmt_btcusdt".to_string()]
        );
    }

    #[test]
    fn subscribe_bbo() {
        gen_test_code!(
            BitgetSwapWSClient,
            subscribe_bbo,
            &vec!["cmt_btcusdt".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook() {
        gen_test_code!(
            BitgetSwapWSClient,
            subscribe_orderbook,
            &vec!["cmt_btcusdt".to_string()]
        );
    }

    #[test]
    fn subscribe_ticker() {
        gen_test_code!(
            BitgetSwapWSClient,
            subscribe_ticker,
            &vec!["cmt_btcusdt".to_string()]
        );
    }

    #[test]
    fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(BitgetSwapWSClient, &vec!["cmt_btcusdt".to_string()], 60);
        gen_test_subscribe_candlestick!(
            BitgetSwapWSClient,
            &vec!["cmt_btcusdt".to_string()],
            604800
        );
    }
}
