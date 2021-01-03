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
}
