#[macro_use]
mod utils;

#[cfg(test)]
mod kucoin_spot {
    use crypto_ws_client::{KuCoinSpotWSClient, Level3OrderBook, WSClient};
    use std::sync::{Arc, Mutex};

    #[test]
    fn subscribe() {
        gen_test_code!(
            KuCoinSpotWSClient,
            subscribe,
            &vec!["/market/match:BTC-USDT".to_string()]
        );
    }

    #[test]
    fn subscribe_raw_json() {
        gen_test_code!(
            KuCoinSpotWSClient,
            subscribe,
            &vec![r#"{"id":"crypto-ws-client","type":"subscribe","topic":"/market/match:BTC-USDT","privateChannel":false,"response":true}"#.to_string()]
        );
    }

    #[test]
    fn subscribe_trade() {
        gen_test_code!(
            KuCoinSpotWSClient,
            subscribe_trade,
            &vec!["BTC-USDT".to_string(), "ETH-USDT".to_string()]
        );
    }

    #[test]
    fn subscribe_bbo() {
        gen_test_code!(
            KuCoinSpotWSClient,
            subscribe_bbo,
            &vec!["BTC-USDT".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook() {
        gen_test_code!(
            KuCoinSpotWSClient,
            subscribe_orderbook,
            &vec!["BTC-USDT".to_string()]
        );
    }

    #[test]
    fn subscribe_l3_orderbook() {
        gen_test_code!(
            KuCoinSpotWSClient,
            subscribe_l3_orderbook,
            &vec!["BTC-USDT".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook_snapshot() {
        gen_test_code!(
            KuCoinSpotWSClient,
            subscribe_orderbook_snapshot,
            &vec!["BTC-USDT".to_string()]
        );
    }

    #[test]
    fn subscribe_ticker() {
        gen_test_code!(
            KuCoinSpotWSClient,
            subscribe_ticker,
            &vec!["BTC-USDT".to_string()]
        );
    }

    #[test]
    fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(KuCoinSpotWSClient, &vec!["BTC-USDT".to_string()], 60);
        gen_test_subscribe_candlestick!(KuCoinSpotWSClient, &vec!["BTC-USDT".to_string()], 604800);
    }
}

#[cfg(test)]
mod kucoin_inverse_swap {
    use crypto_ws_client::{KuCoinSwapWSClient, Level3OrderBook, WSClient};
    use std::sync::{Arc, Mutex};

    #[test]
    fn subscribe() {
        gen_test_code!(
            KuCoinSwapWSClient,
            subscribe,
            &vec!["/contractMarket/execution:XBTUSDM".to_string()]
        );
    }

    #[test]
    fn subscribe_raw_json() {
        gen_test_code!(
            KuCoinSwapWSClient,
            subscribe,
            &vec![r#"{"id":"crypto-ws-client","type":"subscribe","topic":"/contractMarket/execution:XBTUSDM","privateChannel":false,"response":true}"#.to_string()]
        );
    }

    #[test]
    fn subscribe_trade() {
        gen_test_code!(
            KuCoinSwapWSClient,
            subscribe_trade,
            &vec!["XBTUSDM".to_string()]
        );
    }

    #[test]
    fn subscribe_bbo() {
        gen_test_code!(
            KuCoinSwapWSClient,
            subscribe_bbo,
            &vec!["XBTUSDM".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook() {
        gen_test_code!(
            KuCoinSwapWSClient,
            subscribe_orderbook,
            &vec!["XBTUSDM".to_string()]
        );
    }

    #[test]
    fn subscribe_l3_orderbook() {
        gen_test_code!(
            KuCoinSwapWSClient,
            subscribe_l3_orderbook,
            &vec!["XBTUSDM".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook_snapshot() {
        gen_test_code!(
            KuCoinSwapWSClient,
            subscribe_orderbook_snapshot,
            &vec!["XBTUSDM".to_string()]
        );
    }

    #[test]
    fn subscribe_ticker() {
        gen_test_code!(
            KuCoinSwapWSClient,
            subscribe_ticker,
            &vec!["XBTUSDM".to_string()]
        );
    }

    #[test]
    #[ignore]
    fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(
            KuCoinSwapWSClient,
            &vec!["XBTUSDM".to_string(), "ETHUSDM".to_string()],
            60
        );
        gen_test_subscribe_candlestick!(
            KuCoinSwapWSClient,
            &vec!["XBTUSDM".to_string(), "ETHUSDM".to_string()],
            604800
        );
    }
}

#[cfg(test)]
mod kucoin_linear_swap {
    use crypto_ws_client::{KuCoinSwapWSClient, Level3OrderBook, WSClient};
    use std::sync::{Arc, Mutex};

    #[test]
    fn subscribe() {
        gen_test_code!(
            KuCoinSwapWSClient,
            subscribe,
            &vec!["/contractMarket/execution:XBTUSDTM".to_string()]
        );
    }

    #[test]
    fn subscribe_raw_json() {
        gen_test_code!(
            KuCoinSwapWSClient,
            subscribe,
            &vec![r#"{"id":"crypto-ws-client","type":"subscribe","topic":"/contractMarket/execution:XBTUSDTM","privateChannel":false,"response":true}"#.to_string()]
        );
    }

    #[test]
    fn subscribe_trade() {
        gen_test_code!(
            KuCoinSwapWSClient,
            subscribe_trade,
            &vec!["XBTUSDTM".to_string()]
        );
    }

    #[test]
    fn subscribe_bbo() {
        gen_test_code!(
            KuCoinSwapWSClient,
            subscribe_bbo,
            &vec!["XBTUSDTM".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook() {
        gen_test_code!(
            KuCoinSwapWSClient,
            subscribe_orderbook,
            &vec!["XBTUSDTM".to_string()]
        );
    }

    #[test]
    fn subscribe_l3_orderbook() {
        gen_test_code!(
            KuCoinSwapWSClient,
            subscribe_l3_orderbook,
            &vec!["XBTUSDTM".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook_snapshot() {
        gen_test_code!(
            KuCoinSwapWSClient,
            subscribe_orderbook_snapshot,
            &vec!["XBTUSDTM".to_string()]
        );
    }

    #[test]
    fn subscribe_ticker() {
        gen_test_code!(
            KuCoinSwapWSClient,
            subscribe_ticker,
            &vec!["XBTUSDTM".to_string()]
        );
    }

    #[test]
    #[ignore]
    fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(
            KuCoinSwapWSClient,
            &vec!["XBTUSDTM".to_string(), "ETHUSDTM".to_string()],
            60
        );
        gen_test_subscribe_candlestick!(
            KuCoinSwapWSClient,
            &vec!["XBTUSDTM".to_string(), "ETHUSDTM".to_string()],
            604800
        );
    }
}

#[cfg(test)]
mod kucoin_inverse_future {
    use crypto_ws_client::{KuCoinSwapWSClient, Level3OrderBook, WSClient};
    use std::sync::{Arc, Mutex};

    #[test]
    #[ignore]
    fn subscribe() {
        gen_test_code!(
            KuCoinSwapWSClient,
            subscribe,
            &vec!["/contractMarket/execution:XBTMH21".to_string()]
        );
    }

    #[test]
    #[ignore]
    fn subscribe_raw_json() {
        gen_test_code!(
            KuCoinSwapWSClient,
            subscribe,
            &vec![r#"{"id":"crypto-ws-client","type":"subscribe","topic":"/contractMarket/execution:XBTMH21","privateChannel":false,"response":true}"#.to_string()]
        );
    }

    #[test]
    #[ignore]
    fn subscribe_trade() {
        gen_test_code!(
            KuCoinSwapWSClient,
            subscribe_trade,
            &vec!["XBTMH21".to_string()]
        );
    }

    #[test]
    #[ignore]
    fn subscribe_bbo() {
        gen_test_code!(
            KuCoinSwapWSClient,
            subscribe_bbo,
            &vec!["XBTMH21".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook() {
        gen_test_code!(
            KuCoinSwapWSClient,
            subscribe_orderbook,
            &vec!["XBTMH21".to_string()]
        );
    }

    #[test]
    fn subscribe_l3_orderbook() {
        gen_test_code!(
            KuCoinSwapWSClient,
            subscribe_l3_orderbook,
            &vec!["XBTMH21".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook_snapshot() {
        gen_test_code!(
            KuCoinSwapWSClient,
            subscribe_orderbook_snapshot,
            &vec!["XBTMH21".to_string()]
        );
    }

    #[test]
    fn subscribe_ticker() {
        gen_test_code!(
            KuCoinSwapWSClient,
            subscribe_ticker,
            &vec!["XBTMH21".to_string()]
        );
    }

    #[test]
    #[ignore]
    fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(KuCoinSwapWSClient, &vec!["XBTMH21".to_string()], 60);
        gen_test_subscribe_candlestick!(KuCoinSwapWSClient, &vec!["XBTMH21".to_string()], 604800);
    }
}
