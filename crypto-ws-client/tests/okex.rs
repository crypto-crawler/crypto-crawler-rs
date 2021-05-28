use crypto_ws_client::{OkexWSClient, WSClient};
use std::sync::{Arc, Mutex};

#[macro_use]
mod utils;

#[test]
fn okex_index() {
    gen_test_code!(
        OkexWSClient,
        subscribe,
        &vec!["index/ticker:BTC-USDT".to_string()]
    );
}

#[cfg(test)]
mod okex_spot {
    use crypto_ws_client::{OkexWSClient, WSClient};
    use std::sync::{Arc, Mutex};

    #[test]
    fn subscribe() {
        gen_test_code!(
            OkexWSClient,
            subscribe,
            &vec!["spot/trade:BTC-USDT".to_string()]
        );
    }

    #[test]
    fn subscribe_raw_json() {
        gen_test_code!(
            OkexWSClient,
            subscribe,
            &vec![r#"{"op":"subscribe","args":["spot/trade:BTC-USDT"]}"#.to_string()]
        );
    }

    #[test]
    fn subscribe_trade() {
        gen_test_code!(OkexWSClient, subscribe_trade, &vec!["BTC-USDT".to_string()]);
    }

    #[test]
    fn subscribe_ticker() {
        gen_test_code!(
            OkexWSClient,
            subscribe_ticker,
            &vec!["BTC-USDT".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook() {
        gen_test_code!(
            OkexWSClient,
            subscribe_orderbook,
            &vec!["BTC-USDT".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook_snapshot() {
        gen_test_code!(
            OkexWSClient,
            subscribe_orderbook_snapshot,
            &vec!["BTC-USDT".to_string()]
        );
    }

    #[test]
    fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(OkexWSClient, &vec!["BTC-USDT".to_string()], 60);
        gen_test_subscribe_candlestick!(OkexWSClient, &vec!["BTC-USDT".to_string()], 604800);
    }
}

#[cfg(test)]
mod okex_future {
    use crypto_ws_client::{OkexWSClient, WSClient};
    use std::sync::{Arc, Mutex};

    #[test]
    fn subscribe() {
        gen_test_code!(
            OkexWSClient,
            subscribe,
            &vec!["futures/trade:BTC-USDT-210625".to_string()]
        );
    }

    #[test]
    fn subscribe_trade() {
        gen_test_code!(
            OkexWSClient,
            subscribe_trade,
            &vec!["BTC-USDT-210625".to_string()]
        );
    }

    #[test]
    fn subscribe_ticker() {
        gen_test_code!(
            OkexWSClient,
            subscribe_ticker,
            &vec!["BTC-USDT-210625".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook() {
        gen_test_code!(
            OkexWSClient,
            subscribe_orderbook,
            &vec!["BTC-USDT-210625".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook_snapshot() {
        gen_test_code!(
            OkexWSClient,
            subscribe_orderbook_snapshot,
            &vec!["BTC-USDT-210625".to_string()]
        );
    }

    #[test]
    fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(OkexWSClient, &vec!["BTC-USDT-210625".to_string()], 60);
        gen_test_subscribe_candlestick!(OkexWSClient, &vec!["BTC-USDT-210625".to_string()], 604800);
    }
}

#[cfg(test)]
mod okex_swap {
    use crypto_ws_client::{OkexWSClient, WSClient};
    use std::sync::{Arc, Mutex};

    #[test]
    fn subscribe() {
        gen_test_code!(
            OkexWSClient,
            subscribe,
            &vec!["swap/trade:BTC-USDT-SWAP".to_string()]
        );
    }

    #[test]
    fn subscribe_trade() {
        gen_test_code!(
            OkexWSClient,
            subscribe_trade,
            &vec!["BTC-USDT-SWAP".to_string()]
        );
    }

    #[test]
    fn subscribe_ticker() {
        gen_test_code!(
            OkexWSClient,
            subscribe_ticker,
            &vec!["BTC-USDT-SWAP".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook() {
        gen_test_code!(
            OkexWSClient,
            subscribe_orderbook,
            &vec!["BTC-USDT-SWAP".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook_snapshot() {
        gen_test_code!(
            OkexWSClient,
            subscribe_orderbook_snapshot,
            &vec!["BTC-USDT-SWAP".to_string()]
        );
    }

    #[test]
    fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(OkexWSClient, &vec!["BTC-USDT-SWAP".to_string()], 60);
        gen_test_subscribe_candlestick!(OkexWSClient, &vec!["BTC-USDT-SWAP".to_string()], 604800);
    }

    #[test]
    fn subscribe_funding_rate() {
        gen_test_code!(
            OkexWSClient,
            subscribe,
            &vec!["swap/funding_rate:BTC-USDT-SWAP".to_string()]
        );
    }
}

#[cfg(test)]
mod okex_option {
    use crypto_ws_client::{OkexWSClient, WSClient};
    use std::sync::{Arc, Mutex};

    #[test]
    fn subscribe() {
        gen_test_code!(
            OkexWSClient,
            subscribe,
            &vec![
                "option/delivery:BTC-USD".to_string(),
                "option/instruments:BTC-USD".to_string(),
                "index/ticker:BTC-USD".to_string(),
                "option/trades:BTC-USD".to_string(),
                "option/volume24h:BTC-USD".to_string(),
                "option/fitter:BTC-USD".to_string()
            ]
        );
    }

    #[test]
    #[ignore]
    fn subscribe_trade() {
        gen_test_code!(
            OkexWSClient,
            subscribe_trade,
            &vec!["BTC-USD-210604-30000-P".to_string()]
        );
    }

    #[test]
    fn subscribe_ticker() {
        gen_test_code!(
            OkexWSClient,
            subscribe_ticker,
            &vec!["BTC-USD-210604-30000-P".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook() {
        gen_test_code!(
            OkexWSClient,
            subscribe_orderbook,
            &vec!["BTC-USD-210604-30000-P".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook_snapshot() {
        gen_test_code!(
            OkexWSClient,
            subscribe_orderbook_snapshot,
            &vec!["BTC-USD-210604-30000-P".to_string()]
        );
    }

    #[test]
    fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(
            OkexWSClient,
            &vec!["BTC-USD-210604-30000-P".to_string()],
            60
        );
        gen_test_subscribe_candlestick!(
            OkexWSClient,
            &vec!["BTC-USD-210604-30000-P".to_string()],
            604800
        );
    }
}
