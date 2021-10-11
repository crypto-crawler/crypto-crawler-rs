#[macro_use]
mod utils;

#[cfg(test)]
mod dydx_linear_swap {
    use crypto_ws_client::{DydxSwapWSClient, WSClient};
    use std::sync::mpsc::{Receiver, Sender};

    #[test]
    fn subscribe() {
        gen_test_code!(
            DydxSwapWSClient,
            subscribe,
            &vec![
                r#"{"type": "subscribe", "channel": "v3_trades", "id": "BTC-USD"}"#.to_string(),
                r#"{"type": "subscribe", "channel": "v3_trades", "id": "ETH-USD"}"#.to_string()
            ]
        );
    }

    #[test]
    fn subscribe_trade() {
        gen_test_code!(
            DydxSwapWSClient,
            subscribe_trade,
            &vec!["BTC-USD".to_string(), "ETH-USD".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook() {
        gen_test_code!(
            DydxSwapWSClient,
            subscribe_orderbook,
            &vec!["BTC-USD".to_string(), "ETH-USD".to_string()]
        );
    }
}
