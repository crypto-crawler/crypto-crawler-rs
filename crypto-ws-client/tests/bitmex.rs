use crypto_ws_client::{BitmexWSClient, WSClient};
use std::sync::{Arc, Mutex};

#[macro_use]
mod utils;

#[test]
fn bitmex_instrument() {
    gen_test_code!(BitmexWSClient, subscribe, &vec!["instrument".to_string()]);
}

#[cfg(test)]
mod bitmex_swap {
    use crypto_ws_client::{BitmexWSClient, WSClient};
    use std::sync::{Arc, Mutex};

    #[test]
    fn subscribe() {
        gen_test_code!(
            BitmexWSClient,
            subscribe,
            &vec!["trade:XBTUSD".to_string(), "quote:XBTUSD".to_string()]
        );
    }

    #[test]
    fn subscribe_raw_json() {
        gen_test_code!(
            BitmexWSClient,
            subscribe,
            &vec![r#"{"op":"subscribe","args":["trade:XBTUSD","quote:XBTUSD"]}"#.to_string()]
        );
    }

    #[test]
    fn subscribe_trade() {
        gen_test_code!(
            BitmexWSClient,
            subscribe_trade,
            &vec!["XBTUSD".to_string(), "ETHUSD".to_string()]
        );
    }

    #[test]
    fn subscribe_bbo() {
        gen_test_code!(
            BitmexWSClient,
            subscribe_bbo,
            &vec!["XBTUSD".to_string(), "ETHUSD".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook() {
        gen_test_code!(
            BitmexWSClient,
            subscribe_orderbook,
            &vec!["XBTUSD".to_string(), "ETHUSD".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook_snapshot() {
        gen_test_code!(
            BitmexWSClient,
            subscribe_orderbook_snapshot,
            &vec!["XBTUSD".to_string(), "ETHUSD".to_string()]
        );
    }

    #[test]
    fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(
            BitmexWSClient,
            &vec!["XBTUSD".to_string(), "ETHUSD".to_string()],
            60
        );
        gen_test_subscribe_candlestick!(
            BitmexWSClient,
            &vec!["XBTUSD".to_string(), "ETHUSD".to_string()],
            86400
        );
    }
}

#[cfg(test)]
mod bitmex_future {
    use crypto_ws_client::{BitmexWSClient, WSClient};
    use std::sync::{Arc, Mutex};

    #[test]
    fn subscribe() {
        gen_test_code!(
            BitmexWSClient,
            subscribe,
            &vec!["trade:XBTM21".to_string(), "quote:XBTM21".to_string()]
        );
    }

    #[test]
    fn subscribe_trade() {
        gen_test_code!(
            BitmexWSClient,
            subscribe_trade,
            &vec!["XBTM21".to_string(), "ETHH21".to_string()]
        );
    }

    #[test]
    fn subscribe_bbo() {
        gen_test_code!(
            BitmexWSClient,
            subscribe_bbo,
            &vec!["XBTM21".to_string(), "ETHH21".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook() {
        gen_test_code!(
            BitmexWSClient,
            subscribe_orderbook,
            &vec!["XBTM21".to_string(), "ETHH21".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook_snapshot() {
        gen_test_code!(
            BitmexWSClient,
            subscribe_orderbook_snapshot,
            &vec!["XBTM21".to_string(), "ETHH21".to_string()]
        );
    }

    #[test]
    fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(
            BitmexWSClient,
            &vec!["XBTM21".to_string(), "ETHH21".to_string()],
            60
        );
        gen_test_subscribe_candlestick!(
            BitmexWSClient,
            &vec!["XBTM21".to_string(), "ETHH21".to_string()],
            86400
        );
    }
}
