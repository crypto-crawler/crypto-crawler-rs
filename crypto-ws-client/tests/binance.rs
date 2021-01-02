#[macro_use]
mod utils;

#[cfg(test)]
mod spot {
    use crypto_ws_client::{BinanceSpotWSClient, WSClient};

    #[test]
    fn test_subscribe() {
        gen_test_subscribe!(
            BinanceSpotWSClient,
            &vec!["btcusdt@aggTrade".to_string(), "btcusdt@ticker".to_string()]
        );
    }
}

#[cfg(test)]
mod future {
    use crypto_ws_client::{BinanceFutureWSClient, WSClient};

    #[test]
    fn test_subscribe() {
        gen_test_subscribe!(
            BinanceFutureWSClient,
            &vec!["btcusd_210625@aggTrade".to_string()]
        );
    }
}

#[cfg(test)]
mod inverse_swap {
    use crypto_ws_client::{BinanceInverseSwapWSClient, WSClient};

    #[test]
    fn test_subscribe() {
        gen_test_subscribe!(
            BinanceInverseSwapWSClient,
            &vec!["btcusd_perp@aggTrade".to_string()]
        );
    }
}

#[cfg(test)]
mod linear_swap {
    use crypto_ws_client::{BinanceLinearSwapWSClient, WSClient};

    #[test]
    fn test_subscribe() {
        gen_test_subscribe!(
            BinanceLinearSwapWSClient,
            &vec!["btcusdt@aggTrade".to_string()]
        );
    }
}
