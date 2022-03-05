#[macro_use]
mod utils;

#[cfg(test)]
mod kraken_spot {
    use crypto_ws_client::{KrakenSpotWSClient, WSClient};
    use std::sync::mpsc::{Receiver, Sender};

    #[test]
    fn subscribe() {
        gen_test_code!(
            KrakenSpotWSClient,
            subscribe,
            &vec![
                "trade:XBT/USD".to_string(),
                "ticker:XBT/USD".to_string(),
                "spread:XBT/USD".to_string(),
                "book:XBT/USD".to_string()
            ]
        );
    }

    #[test]
    fn subscribe_raw_json() {
        gen_test_code!(
            KrakenSpotWSClient,
            subscribe,
            &vec![
                r#"{"event":"subscribe","pair":["XBT/USD"],"subscription":{"name":"trade"}}"#
                    .to_string()
            ]
        );
    }

    #[test]
    fn subscribe_trade() {
        gen_test_code!(
            KrakenSpotWSClient,
            subscribe_trade,
            &vec!["XBT/USD".to_string(), "ETH/USD".to_string()]
        );
    }

    #[test]
    fn subscribe_ticker() {
        gen_test_code!(
            KrakenSpotWSClient,
            subscribe_ticker,
            &vec!["XBT/USD".to_string(), "ETH/USD".to_string()]
        );
    }

    #[test]
    fn subscribe_bbo() {
        gen_test_code!(
            KrakenSpotWSClient,
            subscribe_bbo,
            &vec!["XBT/USD".to_string(), "ETH/USD".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook() {
        gen_test_code!(
            KrakenSpotWSClient,
            subscribe_orderbook,
            &vec!["XBT/USD".to_string(), "ETH/USD".to_string()]
        );
    }

    #[test]
    fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(
            KrakenSpotWSClient,
            &vec![("XBT/USD".to_string(), 60), ("ETH/USD".to_string(), 60)]
        );

        gen_test_subscribe_candlestick!(
            KrakenSpotWSClient,
            &vec![
                ("XBT/USD".to_string(), 1296000),
                ("ETH/USD".to_string(), 1296000)
            ]
        );
    }
}

#[cfg(test)]
mod kraken_inverse_swap {
    use crypto_ws_client::{KrakenFuturesWSClient, WSClient};
    use std::sync::mpsc::{Receiver, Sender};

    #[test]
    fn subscribe_raw_json() {
        gen_test_code!(
            KrakenFuturesWSClient,
            subscribe,
            &vec![
                r#"{"event":"subscribe","feed":"trade","product_ids":["PI_XBTUSD"]}"#.to_string()
            ]
        );
    }

    #[test]
    fn subscribe_trade() {
        gen_test_code!(
            KrakenFuturesWSClient,
            subscribe_trade,
            &vec!["PI_XBTUSD".to_string()]
        );
    }

    #[test]
    fn subscribe_ticker() {
        gen_test_code!(
            KrakenFuturesWSClient,
            subscribe_ticker,
            &vec!["PI_XBTUSD".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook() {
        gen_test_code!(
            KrakenFuturesWSClient,
            subscribe_orderbook,
            &vec!["PI_XBTUSD".to_string()]
        );
    }
}

#[cfg(test)]
mod kraken_inverse_future {
    use crypto_ws_client::{KrakenFuturesWSClient, WSClient};
    use std::sync::mpsc::{Receiver, Sender};

    #[test]
    fn subscribe_raw_json() {
        gen_test_code!(
            KrakenFuturesWSClient,
            subscribe,
            &vec![
                r#"{"event":"subscribe","feed":"trade","product_ids":["FI_XBTUSD_220624"]}"#
                    .to_string()
            ]
        );
    }

    #[test]
    fn subscribe_trade() {
        gen_test_code!(
            KrakenFuturesWSClient,
            subscribe_trade,
            &vec!["FI_XBTUSD_220624".to_string()]
        );
    }

    #[test]
    fn subscribe_ticker() {
        gen_test_code!(
            KrakenFuturesWSClient,
            subscribe_ticker,
            &vec!["FI_XBTUSD_220624".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook() {
        gen_test_code!(
            KrakenFuturesWSClient,
            subscribe_orderbook,
            &vec!["FI_XBTUSD_220624".to_string()]
        );
    }
}
