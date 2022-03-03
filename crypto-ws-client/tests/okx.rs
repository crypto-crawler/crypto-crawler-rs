use crypto_ws_client::{OkxWSClient, WSClient};
use std::sync::mpsc::{Receiver, Sender};

#[macro_use]
mod utils;

#[test]
fn okex_index() {
    gen_test_code!(
        OkxWSClient,
        subscribe,
        &vec!["index-tickers:BTC-USDT".to_string()]
    );
}

#[cfg(test)]
mod okex_spot {
    use crypto_ws_client::{OkxWSClient, WSClient};
    use std::sync::mpsc::{Receiver, Sender};

    #[test]
    fn subscribe() {
        gen_test_code!(OkxWSClient, subscribe, &vec!["trades:BTC-USDT".to_string()]);
    }

    #[test]
    fn subscribe_raw_json() {
        gen_test_code!(
            OkxWSClient,
            subscribe,
            &vec![
                r#"{"op":"subscribe","args":[{"channel":"trades","instId":"BTC-USDT"}]}"#
                    .to_string()
            ]
        );
    }

    #[test]
    fn subscribe_trade() {
        gen_test_code!(OkxWSClient, subscribe_trade, &vec!["BTC-USDT".to_string()]);
    }

    #[test]
    fn subscribe_ticker() {
        gen_test_code!(OkxWSClient, subscribe_ticker, &vec!["BTC-USDT".to_string()]);
    }

    #[test]
    fn subscribe_orderbook() {
        gen_test_code!(
            OkxWSClient,
            subscribe_orderbook,
            &vec!["BTC-USDT".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook_topk() {
        gen_test_code!(
            OkxWSClient,
            subscribe_orderbook_topk,
            &vec!["BTC-USDT".to_string()]
        );
    }

    #[test]
    fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(OkxWSClient, &vec![("BTC-USDT".to_string(), 60)]);
        gen_test_subscribe_candlestick!(OkxWSClient, &vec![("BTC-USDT".to_string(), 604800)]);
    }
}

#[cfg(test)]
mod okex_future {
    use crypto_ws_client::{OkxWSClient, WSClient};
    use std::sync::mpsc::{Receiver, Sender};

    #[test]
    fn subscribe() {
        gen_test_code!(
            OkxWSClient,
            subscribe,
            &vec!["trades:BTC-USDT-220325".to_string()]
        );
    }

    #[test]
    fn subscribe_trade() {
        gen_test_code!(
            OkxWSClient,
            subscribe_trade,
            &vec!["BTC-USDT-220325".to_string()]
        );
    }

    #[test]
    fn subscribe_ticker() {
        gen_test_code!(
            OkxWSClient,
            subscribe_ticker,
            &vec!["BTC-USDT-220325".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook() {
        gen_test_code!(
            OkxWSClient,
            subscribe_orderbook,
            &vec!["BTC-USDT-220325".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook_topk() {
        gen_test_code!(
            OkxWSClient,
            subscribe_orderbook_topk,
            &vec!["BTC-USDT-220325".to_string()]
        );
    }

    #[test]
    fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(OkxWSClient, &vec![("BTC-USDT-220325".to_string(), 60)]);
        gen_test_subscribe_candlestick!(
            OkxWSClient,
            &vec![("BTC-USDT-220325".to_string(), 604800)]
        );
    }
}

#[cfg(test)]
mod okex_swap {
    use crypto_ws_client::{OkxWSClient, WSClient};
    use std::sync::mpsc::{Receiver, Sender};

    #[test]
    fn subscribe() {
        gen_test_code!(
            OkxWSClient,
            subscribe,
            &vec!["trades:BTC-USDT-SWAP".to_string()]
        );
    }

    #[test]
    fn subscribe_trade() {
        gen_test_code!(
            OkxWSClient,
            subscribe_trade,
            &vec!["BTC-USDT-SWAP".to_string()]
        );
    }

    #[test]
    fn subscribe_ticker() {
        gen_test_code!(
            OkxWSClient,
            subscribe_ticker,
            &vec!["BTC-USDT-SWAP".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook() {
        gen_test_code!(
            OkxWSClient,
            subscribe_orderbook,
            &vec!["BTC-USDT-SWAP".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook_topk() {
        gen_test_code!(
            OkxWSClient,
            subscribe_orderbook_topk,
            &vec!["BTC-USDT-SWAP".to_string()]
        );
    }

    #[test]
    fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(OkxWSClient, &vec![("BTC-USDT-SWAP".to_string(), 60)]);
        gen_test_subscribe_candlestick!(OkxWSClient, &vec![("BTC-USDT-SWAP".to_string(), 604800)]);
    }

    #[test]
    fn subscribe_funding_rate() {
        gen_test_code!(
            OkxWSClient,
            subscribe,
            &vec!["funding-rate:BTC-USDT-SWAP".to_string()]
        );
    }
}

#[cfg(test)]
mod okex_option {
    use crypto_ws_client::{OkxWSClient, WSClient};
    use std::sync::mpsc::{Receiver, Sender};

    #[test]
    #[ignore]
    fn subscribe_trade() {
        gen_test_code!(
            OkxWSClient,
            subscribe_trade,
            &vec!["BTC-USD-220304-32000-P".to_string()]
        );
    }

    #[test]
    #[ignore]
    fn subscribe_ticker() {
        gen_test_code!(
            OkxWSClient,
            subscribe_ticker,
            &vec!["BTC-USD-220304-32000-P".to_string()]
        );
    }

    #[test]
    #[ignore]
    fn subscribe_orderbook() {
        gen_test_code!(
            OkxWSClient,
            subscribe_orderbook,
            &vec!["BTC-USD-220304-32000-P".to_string()]
        );
    }

    #[test]
    #[ignore]
    fn subscribe_orderbook_topk() {
        gen_test_code!(
            OkxWSClient,
            subscribe_orderbook_topk,
            &vec!["BTC-USD-220304-32000-P".to_string()]
        );
    }

    #[test]
    #[ignore]
    fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(
            OkxWSClient,
            &vec![("BTC-USD-220304-32000-P".to_string(), 60)]
        );
        gen_test_subscribe_candlestick!(
            OkxWSClient,
            &vec![("BTC-USD-220304-32000-P".to_string(), 604800)]
        );
    }
}
