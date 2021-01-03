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
    fn subscribe_trade() {
        gen_test_subscribe_trade!(
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
}
