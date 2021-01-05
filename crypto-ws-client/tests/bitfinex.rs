#[macro_use]
mod utils;

#[cfg(test)]
mod bitfinex_spot {
    use crypto_ws_client::{BitfinexWSClient, WSClient};

    #[test]
    fn subscribe() {
        gen_test_subscribe!(BitfinexWSClient, &vec!["trades:tBTCUST".to_string()]);
    }

    #[test]
    fn subscribe_trade() {
        gen_test_subscribe_trade!(BitfinexWSClient, &vec!["BTCUST".to_string()]);
    }

    #[test]
    fn subscribe_ticker() {
        gen_test_subscribe_ticker!(BitfinexWSClient, &vec!["BTCUST".to_string()]);
    }

    #[test]
    fn subscribe_bbo() {
        gen_test_subscribe_bbo!(BitfinexWSClient, &vec!["BTCUST".to_string()]);
    }

    #[test]
    fn subscribe_orderbook() {
        gen_test_subscribe_orderbook!(BitfinexWSClient, &vec!["BTCUST".to_string()]);
    }

    #[test]
    fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(BitfinexWSClient, &vec!["BTCUST".to_string()], 60);
        gen_test_subscribe_candlestick!(BitfinexWSClient, &vec!["BTCUST".to_string()], 2592000);
    }
}

#[cfg(test)]
mod bitfinex_swap {
    use crypto_ws_client::{BitfinexWSClient, WSClient};

    #[test]
    fn subscribe() {
        gen_test_subscribe!(BitfinexWSClient, &vec!["trades:tBTCF0:USTF0".to_string()]);
    }

    #[test]
    fn subscribe_trade() {
        gen_test_subscribe_trade!(BitfinexWSClient, &vec!["BTCF0:USTF0".to_string()]);
    }

    #[test]
    fn subscribe_ticker() {
        gen_test_subscribe_ticker!(BitfinexWSClient, &vec!["BTCF0:USTF0".to_string()]);
    }

    #[test]
    fn subscribe_bbo() {
        gen_test_subscribe_bbo!(BitfinexWSClient, &vec!["BTCF0:USTF0".to_string()]);
    }

    #[test]
    fn subscribe_orderbook() {
        gen_test_subscribe_orderbook!(BitfinexWSClient, &vec!["BTCUST".to_string()]);
    }

    #[test]
    fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(BitfinexWSClient, &vec!["BTCUST".to_string()], 60);
        gen_test_subscribe_candlestick!(BitfinexWSClient, &vec!["BTCUST".to_string()], 2592000);
    }
}
