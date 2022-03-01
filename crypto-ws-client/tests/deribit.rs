use crypto_ws_client::{DeribitWSClient, WSClient};
use std::sync::mpsc::{Receiver, Sender};

#[macro_use]
mod utils;

#[test]
fn deribit_all_trades() {
    gen_test_code!(
        DeribitWSClient,
        subscribe,
        // https://docs.deribit.com/?javascript#trades-kind-currency-interval
        &vec![
            "trades.future.any.100ms".to_string(),
            "trades.option.any.100ms".to_string(),
        ]
    );
}

#[cfg(test)]
mod deribit_inverse_future {
    use crypto_ws_client::{DeribitWSClient, WSClient};
    use std::sync::mpsc::{Receiver, Sender};

    #[test]
    fn subscribe() {
        gen_test_code!(
            DeribitWSClient,
            subscribe,
            &vec!["trades.future.BTC.100ms".to_string()]
        );
    }

    #[test]
    fn subscribe_trade() {
        gen_test_code!(
            DeribitWSClient,
            subscribe_trade,
            &vec!["BTC-25MAR22".to_string(), "BTC-24JUN22".to_string()]
        );
    }

    #[test]
    fn subscribe_ticker() {
        gen_test_code!(
            DeribitWSClient,
            subscribe_ticker,
            &vec!["BTC-24JUN22".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook() {
        gen_test_code!(
            DeribitWSClient,
            subscribe_orderbook,
            &vec!["BTC-24JUN22".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook_topk() {
        gen_test_code!(
            DeribitWSClient,
            subscribe_orderbook_topk,
            &vec!["BTC-24JUN22".to_string()]
        );
    }

    #[test]
    fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(DeribitWSClient, &vec![("BTC-24JUN22".to_string(), 60)]);
        gen_test_subscribe_candlestick!(DeribitWSClient, &vec![("BTC-24JUN22".to_string(), 86400)]);
    }
}

#[cfg(test)]
mod deribit_inverse_swap {
    use crypto_ws_client::{DeribitWSClient, WSClient};
    use std::sync::mpsc::{Receiver, Sender};

    #[test]
    fn subscribe() {
        gen_test_code!(
            DeribitWSClient,
            subscribe,
            &vec!["trades.BTC-PERPETUAL.100ms".to_string()]
        );
    }

    #[test]
    fn subscribe_trade() {
        gen_test_code!(
            DeribitWSClient,
            subscribe_trade,
            &vec!["BTC-PERPETUAL".to_string()]
        );
    }

    #[test]
    fn subscribe_ticker() {
        gen_test_code!(
            DeribitWSClient,
            subscribe_ticker,
            &vec!["BTC-PERPETUAL".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook() {
        gen_test_code!(
            DeribitWSClient,
            subscribe_orderbook,
            &vec!["BTC-PERPETUAL".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook_topk() {
        gen_test_code!(
            DeribitWSClient,
            subscribe_orderbook_topk,
            &vec!["BTC-PERPETUAL".to_string()]
        );
    }

    #[test]
    fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(DeribitWSClient, &vec![("BTC-PERPETUAL".to_string(), 60)]);
        gen_test_subscribe_candlestick!(
            DeribitWSClient,
            &vec![("BTC-PERPETUAL".to_string(), 86400)]
        );
    }
}

#[cfg(test)]
mod deribit_option {
    use crypto_ws_client::{DeribitWSClient, WSClient};
    use std::sync::mpsc::{Receiver, Sender};

    const SYMBOLS: &'static [&str] = &[
        "BTC-25MAR22-50000-C",
        "BTC-25MAR22-60000-C",
        "BTC-24JUN22-40000-C",
        "BTC-24JUN22-60000-C",
    ];

    #[test]
    fn subscribe() {
        gen_test_code!(
            DeribitWSClient,
            subscribe,
            &vec!["trades.option.any.100ms".to_string()]
        );
    }

    #[test]
    #[ignore]
    fn subscribe_trade() {
        gen_test_code!(
            DeribitWSClient,
            subscribe_trade,
            &SYMBOLS
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        );
    }

    #[test]
    fn subscribe_ticker() {
        gen_test_code!(
            DeribitWSClient,
            subscribe_ticker,
            &SYMBOLS
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        );
    }

    #[test]
    #[ignore]
    fn subscribe_orderbook() {
        gen_test_code!(
            DeribitWSClient,
            subscribe_orderbook,
            &SYMBOLS
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        );
    }

    #[test]
    #[ignore]
    fn subscribe_orderbook_topk() {
        gen_test_code!(
            DeribitWSClient,
            subscribe_orderbook_topk,
            &SYMBOLS
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        );
    }

    #[test]
    fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(
            DeribitWSClient,
            SYMBOLS
                .iter()
                .map(|s| (s.to_string(), 60))
                .collect::<Vec<(String, usize)>>()
                .as_slice()
        );
        gen_test_subscribe_candlestick!(
            DeribitWSClient,
            SYMBOLS
                .iter()
                .map(|s| (s.to_string(), 86400))
                .collect::<Vec<(String, usize)>>()
                .as_slice()
        );
    }
}
