#[macro_use]
mod utils;

#[cfg(test)]
mod bitfinex_spot {
    use crypto_ws_client::{BitfinexWSClient, Level3OrderBook, WSClient};

    #[test]
    fn subscribe() {
        gen_test_subscribe!(BitfinexWSClient, &vec!["trades:tBTCUST".to_string()]);
    }

    #[test]
    #[should_panic]
    fn subscribe_illegal_symbol() {
        gen_test_subscribe!(BitfinexWSClient, &vec!["trades:tXXXYYY".to_string()]);
    }

    #[test]
    fn subscribe_trade() {
        gen_test_subscribe_trade!(BitfinexWSClient, &vec!["tBTCUST".to_string()]);
    }

    #[test]
    fn subscribe_ticker() {
        gen_test_subscribe_ticker!(BitfinexWSClient, &vec!["tBTCUST".to_string()]);
    }

    #[test]
    fn subscribe_bbo() {
        gen_test_subscribe_bbo!(BitfinexWSClient, &vec!["tBTCUST".to_string()]);
    }

    #[test]
    fn subscribe_orderbook() {
        gen_test_subscribe_orderbook!(BitfinexWSClient, &vec!["tBTCUST".to_string()]);
    }

    #[test]
    fn subscribe_l3_orderbook() {
        gen_test_subscribe_l3_orderbook!(BitfinexWSClient, &vec!["tBTCUST".to_string()]);
    }

    #[test]
    fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(BitfinexWSClient, &vec!["tBTCUST".to_string()], 60);
        gen_test_subscribe_candlestick!(BitfinexWSClient, &vec!["tBTCUST".to_string()], 2592000);
    }
}

#[cfg(test)]
mod bitfinex_swap {
    use crypto_ws_client::{BitfinexWSClient, Level3OrderBook, WSClient};

    #[test]
    fn subscribe() {
        gen_test_subscribe!(BitfinexWSClient, &vec!["trades:tBTCF0:USTF0".to_string()]);
    }

    #[test]
    fn subscribe_trade() {
        gen_test_subscribe_trade!(BitfinexWSClient, &vec!["tBTCF0:USTF0".to_string()]);
    }

    #[test]
    fn subscribe_ticker() {
        gen_test_subscribe_ticker!(BitfinexWSClient, &vec!["tBTCF0:USTF0".to_string()]);
    }

    #[test]
    fn subscribe_bbo() {
        gen_test_subscribe_bbo!(BitfinexWSClient, &vec!["tBTCF0:USTF0".to_string()]);
    }

    #[test]
    fn subscribe_orderbook() {
        gen_test_subscribe_orderbook!(BitfinexWSClient, &vec!["tBTCUST".to_string()]);
    }

    #[test]
    fn subscribe_l3_orderbook() {
        gen_test_subscribe_l3_orderbook!(BitfinexWSClient, &vec!["tBTCUST".to_string()]);
    }

    #[test]
    fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(BitfinexWSClient, &vec!["tBTCUST".to_string()], 60);
        gen_test_subscribe_candlestick!(BitfinexWSClient, &vec!["tBTCUST".to_string()], 2592000);
    }
}
