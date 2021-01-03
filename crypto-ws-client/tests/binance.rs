#[macro_use]
mod utils;

#[cfg(test)]
mod binance_spot {
    use crypto_ws_client::{BinanceSpotWSClient, WSClient};

    #[test]
    fn subscribe() {
        gen_test_subscribe!(
            BinanceSpotWSClient,
            &vec!["btcusdt@aggTrade".to_string(), "btcusdt@ticker".to_string()]
        );
    }

    #[test]
    fn subscribe_trade() {
        gen_test_subscribe_trade!(
            BinanceSpotWSClient,
            &vec!["btcusdt".to_string(), "ethusdt".to_string()]
        );
    }

    #[test]
    fn subscribe_ticker() {
        gen_test_subscribe_ticker!(
            BinanceSpotWSClient,
            &vec!["btcusdt".to_string(), "ethusdt".to_string()]
        );
    }
}

#[cfg(test)]
mod binance_future {
    use crypto_ws_client::{BinanceFutureWSClient, WSClient};

    #[test]
    fn subscribe() {
        gen_test_subscribe!(
            BinanceFutureWSClient,
            &vec!["btcusd_210625@aggTrade".to_string()]
        );
    }

    #[test]
    fn subscribe_trade() {
        gen_test_subscribe_trade!(
            BinanceFutureWSClient,
            &vec!["btcusd_210625".to_string(), "ethusd_210625".to_string()]
        );
    }

    #[test]
    fn subscribe_ticker() {
        gen_test_subscribe_ticker!(
            BinanceFutureWSClient,
            &vec!["btcusd_210625".to_string(), "ethusd_210625".to_string()]
        );
    }
}

#[cfg(test)]
mod binance_inverse_swap {
    use crypto_ws_client::{BinanceInverseSwapWSClient, WSClient};

    #[test]
    fn subscribe() {
        gen_test_subscribe!(
            BinanceInverseSwapWSClient,
            &vec!["btcusd_perp@aggTrade".to_string()]
        );
    }

    #[test]
    fn subscribe_trade() {
        gen_test_subscribe_trade!(
            BinanceInverseSwapWSClient,
            &vec!["btcusd_perp".to_string(), "ethusd_perp".to_string()]
        );
    }

    #[test]
    fn subscribe_ticker() {
        gen_test_subscribe_ticker!(
            BinanceInverseSwapWSClient,
            &vec!["btcusd_perp".to_string(), "ethusd_perp".to_string()]
        );
    }
}

#[cfg(test)]
mod binance_linear_swap {
    use crypto_ws_client::{BinanceLinearSwapWSClient, WSClient};

    #[test]
    fn subscribe() {
        gen_test_subscribe!(
            BinanceLinearSwapWSClient,
            &vec!["btcusdt@aggTrade".to_string()]
        );
    }

    #[test]
    fn subscribe_trade() {
        gen_test_subscribe_trade!(
            BinanceLinearSwapWSClient,
            &vec!["btcusdt".to_string(), "ethusdt".to_string()]
        );
    }

    #[test]
    fn subscribe_ticker() {
        gen_test_subscribe_ticker!(
            BinanceLinearSwapWSClient,
            &vec!["btcusdt".to_string(), "ethusdt".to_string()]
        );
    }
}
