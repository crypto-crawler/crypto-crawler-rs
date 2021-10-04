#[macro_use]
mod utils;

#[cfg(test)]
mod bitfinex_spot {
    use crypto_ws_client::{BitfinexWSClient, Level3OrderBook, WSClient};
    use std::sync::{Arc, Mutex};

    #[test]
    fn subscribe() {
        gen_test_code!(
            BitfinexWSClient,
            subscribe,
            &vec!["trades:tBTCUST".to_string(), "trades:tETHUST".to_string()]
        );
    }

    #[test]
    #[should_panic]
    fn subscribe_illegal_symbol() {
        gen_test_code!(
            BitfinexWSClient,
            subscribe,
            &vec!["trades:tXXXYYY".to_string()]
        );
    }

    #[test]
    fn subscribe_trade() {
        gen_test_code!(
            BitfinexWSClient,
            subscribe_trade,
            &vec!["tBTCUST".to_string(), "tETHUST".to_string()]
        );
    }

    #[test]
    fn subscribe_ticker() {
        gen_test_code!(
            BitfinexWSClient,
            subscribe_ticker,
            &vec!["tBTCUST".to_string()]
        );
    }

    #[test]
    fn subscribe_bbo() {
        gen_test_code!(
            BitfinexWSClient,
            subscribe_bbo,
            &vec!["tBTCUST".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook() {
        gen_test_code!(
            BitfinexWSClient,
            subscribe_orderbook,
            &vec!["tBTCUST".to_string()]
        );
    }

    #[test]
    fn subscribe_l3_orderbook() {
        gen_test_code!(
            BitfinexWSClient,
            subscribe_l3_orderbook,
            &vec!["tBTCUST".to_string()]
        );
    }

    #[test]
    fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(BitfinexWSClient, &vec![("tBTCUST".to_string(), 60)]);
        gen_test_subscribe_candlestick!(BitfinexWSClient, &vec![("tBTCUST".to_string(), 2592000)]);
    }
}

#[cfg(test)]
mod bitfinex_swap {
    use crypto_ws_client::{BitfinexWSClient, Level3OrderBook, WSClient};
    use std::sync::{Arc, Mutex};

    #[test]
    fn subscribe() {
        gen_test_code!(
            BitfinexWSClient,
            subscribe,
            &vec!["trades:tBTCF0:USTF0".to_string()]
        );
    }

    #[test]
    fn subscribe_trade() {
        gen_test_code!(
            BitfinexWSClient,
            subscribe_trade,
            &vec!["tBTCF0:USTF0".to_string(), "tETHF0:USTF0".to_string()]
        );
    }

    #[test]
    fn subscribe_ticker() {
        gen_test_code!(
            BitfinexWSClient,
            subscribe_ticker,
            &vec!["tBTCF0:USTF0".to_string()]
        );
    }

    #[test]
    fn subscribe_bbo() {
        gen_test_code!(
            BitfinexWSClient,
            subscribe_bbo,
            &vec!["tBTCF0:USTF0".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook() {
        gen_test_code!(
            BitfinexWSClient,
            subscribe_orderbook,
            &vec!["tBTCF0:USTF0".to_string()]
        );
    }

    #[test]
    fn subscribe_l3_orderbook() {
        gen_test_code!(
            BitfinexWSClient,
            subscribe_l3_orderbook,
            &vec!["tBTCF0:USTF0".to_string()]
        );
    }

    #[test]
    fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(BitfinexWSClient, &vec![("tBTCF0:USTF0".to_string(), 60)]);
        gen_test_subscribe_candlestick!(
            BitfinexWSClient,
            &vec![("tBTCF0:USTF0".to_string(), 2592000)]
        );
    }
}
