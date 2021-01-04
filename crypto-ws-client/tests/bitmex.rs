use crypto_ws_client::{BitMEXWSClient, WSClient};

#[macro_use]
mod utils;

#[test]
fn bitmex_instrument() {
    gen_test_subscribe!(BitMEXWSClient, &vec!["instrument".to_string()]);
}

#[cfg(test)]
mod bitmex_swap {
    use crypto_ws_client::{BitMEXWSClient, WSClient};

    #[test]
    fn subscribe() {
        gen_test_subscribe!(
            BitMEXWSClient,
            &vec!["trade:XBTUSD".to_string(), "quote:XBTUSD".to_string()]
        );
    }

    #[test]
    fn subscribe_raw_json() {
        gen_test_subscribe!(
            BitMEXWSClient,
            &vec![r#"{"op":"subscribe","args":["trade:XBTUSD","quote:XBTUSD"]}"#.to_string()]
        );
    }

    #[test]
    fn subscribe_trade() {
        gen_test_subscribe_trade!(
            BitMEXWSClient,
            &vec!["XBTUSD".to_string(), "ETHUSD".to_string()]
        );
    }

    #[test]
    fn subscribe_bbo() {
        gen_test_subscribe_bbo!(
            BitMEXWSClient,
            &vec!["XBTUSD".to_string(), "ETHUSD".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook() {
        gen_test_subscribe_orderbook!(
            BitMEXWSClient,
            &vec!["XBTUSD".to_string(), "ETHUSD".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook_snapshot() {
        gen_test_subscribe_orderbook_snapshot!(
            BitMEXWSClient,
            &vec!["XBTUSD".to_string(), "ETHUSD".to_string()]
        );
    }
}

#[cfg(test)]
mod bitmex_future {
    use crypto_ws_client::{BitMEXWSClient, WSClient};

    #[test]
    fn subscribe() {
        gen_test_subscribe!(
            BitMEXWSClient,
            &vec!["trade:XBTM21".to_string(), "quote:XBTM21".to_string()]
        );
    }

    #[test]
    fn subscribe_trade() {
        gen_test_subscribe_trade!(
            BitMEXWSClient,
            &vec!["XBTM21".to_string(), "ETHH21".to_string()]
        );
    }

    #[test]
    fn subscribe_bbo() {
        gen_test_subscribe_bbo!(
            BitMEXWSClient,
            &vec!["XBTM21".to_string(), "ETHH21".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook() {
        gen_test_subscribe_orderbook!(
            BitMEXWSClient,
            &vec!["XBTM21".to_string(), "ETHH21".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook_snapshot() {
        gen_test_subscribe_orderbook_snapshot!(
            BitMEXWSClient,
            &vec!["XBTM21".to_string(), "ETHH21".to_string()]
        );
    }
}
