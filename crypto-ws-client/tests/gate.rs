#[macro_use]
mod utils;

#[cfg(test)]
mod gate_spot {
    use crypto_ws_client::{GateSpotWSClient, WSClient};
    use std::sync::{Arc, Mutex};

    #[test]
    fn subscribe() {
        gen_test_code!(
            GateSpotWSClient,
            subscribe,
            &vec![
                "spot.trades:BTC_USDT".to_string(),
                "spot.trades:ETH_USDT".to_string()
            ]
        );
    }

    #[test]
    fn subscribe_raw_json() {
        gen_test_code!(
            GateSpotWSClient,
            subscribe,
            &vec![
                r#"{"channel":"spot.trades", "event":"subscribe", "payload":["BTC_USDT","ETH_USDT"]}"#.to_string()
            ]
        );
    }

    #[test]
    fn subscribe_trade() {
        gen_test_code!(
            GateSpotWSClient,
            subscribe_trade,
            &vec!["BTC_USDT".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook() {
        gen_test_code!(
            GateSpotWSClient,
            subscribe_orderbook,
            &vec!["BTC_USDT".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook_snapshot() {
        gen_test_code!(
            GateSpotWSClient,
            subscribe_orderbook_snapshot,
            &vec!["BTC_USDT".to_string()]
        );
    }

    #[test]
    fn subscribe_bbo() {
        gen_test_code!(
            GateSpotWSClient,
            subscribe_bbo,
            &vec!["BTC_USDT".to_string()]
        );
    }

    #[test]
    fn subscribe_ticker() {
        gen_test_code!(
            GateSpotWSClient,
            subscribe_ticker,
            &vec!["BTC_USDT".to_string()]
        );
    }

    #[test]
    fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(GateSpotWSClient, &vec!["BTC_USDT".to_string()], 10);
        gen_test_subscribe_candlestick!(GateSpotWSClient, &vec!["BTC_USDT".to_string()], 604800);
    }
}

#[cfg(test)]
mod gate_inverse_swap {
    use crypto_ws_client::{GateInverseSwapWSClient, WSClient};
    use std::sync::{Arc, Mutex};

    #[test]
    fn subscribe() {
        gen_test_code!(
            GateInverseSwapWSClient,
            subscribe,
            &vec![
                "futures.trades:BTC_USD".to_string(),
                "futures.trades:ETH_USD".to_string()
            ]
        );
    }

    #[test]
    fn subscribe_raw_json() {
        gen_test_code!(
            GateInverseSwapWSClient,
            subscribe,
            &vec![
                r#"{"channel":"futures.trades", "event":"subscribe", "payload":["BTC_USD","ETH_USD"]}"#
                    .to_string()
            ]
        );
    }

    #[test]
    fn subscribe_trade() {
        gen_test_code!(
            GateInverseSwapWSClient,
            subscribe_trade,
            &vec!["BTC_USD".to_string(), "ETH_USD".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook() {
        gen_test_code!(
            GateInverseSwapWSClient,
            subscribe_orderbook,
            &vec!["BTC_USD".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook_snapshot() {
        gen_test_code!(
            GateInverseSwapWSClient,
            subscribe_orderbook_snapshot,
            &vec!["BTC_USD".to_string()]
        );
    }

    #[test]
    fn subscribe_bbo() {
        gen_test_code!(
            GateInverseSwapWSClient,
            subscribe_bbo,
            &vec!["BTC_USD".to_string()]
        );
    }

    #[test]
    fn subscribe_ticker() {
        gen_test_code!(
            GateInverseSwapWSClient,
            subscribe_ticker,
            &vec!["BTC_USD".to_string()]
        );
    }

    #[test]
    #[ignore]
    fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(GateInverseSwapWSClient, &vec!["BTC_USD".to_string()], 10);
        gen_test_subscribe_candlestick!(
            GateInverseSwapWSClient,
            &vec!["BTC_USD".to_string()],
            604800
        );
    }
}

#[cfg(test)]
mod gate_linear_swap {
    use crypto_ws_client::{GateLinearSwapWSClient, WSClient};
    use std::sync::{Arc, Mutex};

    #[test]
    fn subscribe_trade() {
        gen_test_code!(
            GateLinearSwapWSClient,
            subscribe_trade,
            &vec!["BTC_USDT".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook() {
        gen_test_code!(
            GateLinearSwapWSClient,
            subscribe_orderbook,
            &vec!["BTC_USDT".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook_snapshot() {
        gen_test_code!(
            GateLinearSwapWSClient,
            subscribe_orderbook_snapshot,
            &vec!["BTC_USDT".to_string()]
        );
    }

    #[test]
    fn subscribe_bbo() {
        gen_test_code!(
            GateLinearSwapWSClient,
            subscribe_bbo,
            &vec!["BTC_USDT".to_string()]
        );
    }

    #[test]
    fn subscribe_ticker() {
        gen_test_code!(
            GateLinearSwapWSClient,
            subscribe_ticker,
            &vec!["BTC_USDT".to_string()]
        );
    }

    #[test]
    #[ignore]
    fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(GateLinearSwapWSClient, &vec!["BTC_USDT".to_string()], 10);
        gen_test_subscribe_candlestick!(
            GateLinearSwapWSClient,
            &vec!["BTC_USDT".to_string()],
            604800
        );
    }
}

#[cfg(test)]
mod gate_inverse_future {
    use crypto_ws_client::{GateInverseFutureWSClient, WSClient};
    use std::sync::{Arc, Mutex};

    #[test]
    #[ignore]
    fn subscribe_trade() {
        gen_test_code!(
            GateInverseFutureWSClient,
            subscribe_trade,
            &vec!["BTC_USD_20211231".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook() {
        gen_test_code!(
            GateInverseFutureWSClient,
            subscribe_orderbook,
            &vec!["BTC_USD_20211231".to_string()]
        );
    }

    #[test]
    fn subscribe_ticker() {
        gen_test_code!(
            GateInverseFutureWSClient,
            subscribe_ticker,
            &vec!["BTC_USD_20211231".to_string()]
        );
    }

    #[test]
    #[ignore]
    fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(
            GateInverseFutureWSClient,
            &vec!["BTC_USD_20211231".to_string()],
            10
        );
        gen_test_subscribe_candlestick!(
            GateInverseFutureWSClient,
            &vec!["BTC_USD_20211231".to_string()],
            604800
        );
    }
}

#[cfg(test)]
mod gate_linear_future {
    use crypto_ws_client::{GateLinearFutureWSClient, WSClient};
    use std::sync::{Arc, Mutex};

    #[test]
    #[ignore]
    fn subscribe_trade() {
        gen_test_code!(
            GateLinearFutureWSClient,
            subscribe_trade,
            &vec!["BTC_USDT_20211008".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook() {
        gen_test_code!(
            GateLinearFutureWSClient,
            subscribe_orderbook,
            &vec!["BTC_USDT_20211008".to_string()]
        );
    }

    #[test]
    fn subscribe_ticker() {
        gen_test_code!(
            GateLinearFutureWSClient,
            subscribe_ticker,
            &vec!["BTC_USDT_20211008".to_string()]
        );
    }

    #[test]
    #[ignore]
    fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(
            GateLinearFutureWSClient,
            &vec!["BTC_USDT_20211008".to_string()],
            10
        );
        gen_test_subscribe_candlestick!(
            GateLinearFutureWSClient,
            &vec!["BTC_USDT_20211008".to_string()],
            604800
        );
    }
}
