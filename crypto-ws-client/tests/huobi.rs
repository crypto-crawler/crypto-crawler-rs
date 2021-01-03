#[macro_use]
mod utils;

#[cfg(test)]
mod huobi_spot {
    use crypto_ws_client::{HuobiSpotWSClient, WSClient, BBO};

    #[test]
    fn subscribe() {
        gen_test_subscribe!(
            HuobiSpotWSClient,
            &vec!["market.btcusdt.trade.detail".to_string()]
        );
    }

    #[test]
    fn subscribe_raw_json() {
        gen_test_subscribe!(
            HuobiSpotWSClient,
            &vec![r#"{"sub":"market.btcusdt.trade.detail","id":"crypto-ws-client"}"#.to_string()]
        );
    }

    #[test]
    fn subscribe_trade() {
        gen_test_subscribe_trade!(HuobiSpotWSClient, &vec!["btcusdt".to_string()]);
    }

    #[test]
    fn subscribe_ticker() {
        gen_test_subscribe_ticker!(HuobiSpotWSClient, &vec!["btcusdt".to_string()]);
    }

    #[test]
    fn subscribe_bbo() {
        gen_test_subscribe_bbo!(HuobiSpotWSClient, &vec!["btcusdt".to_string()]);
    }

    #[test]
    fn huobi_hb10() {
        gen_test_subscribe!(
            HuobiSpotWSClient,
            &vec![
                "market.hb10usdt.trade.detail".to_string(),
                "market.huobi10.kline.1min".to_string()
            ]
        );
    }
}

#[cfg(test)]
mod huobi_future {
    use crypto_ws_client::{HuobiFutureWSClient, WSClient, BBO};

    #[test]
    fn subscribe() {
        gen_test_subscribe!(
            HuobiFutureWSClient,
            &vec!["market.BTC_CQ.trade.detail".to_string()]
        );
    }

    #[test]
    fn subscribe_trade() {
        gen_test_subscribe_trade!(HuobiFutureWSClient, &vec!["BTC_CQ".to_string()]);
    }

    #[test]
    fn subscribe_ticker() {
        gen_test_subscribe_ticker!(HuobiFutureWSClient, &vec!["BTC_CQ".to_string()]);
    }

    #[test]
    fn subscribe_bbo() {
        gen_test_subscribe_bbo!(HuobiFutureWSClient, &vec!["BTC_CQ".to_string()]);
    }
}

#[cfg(test)]
mod huobi_linear_swap {
    use crypto_ws_client::{HuobiLinearSwapWSClient, WSClient, BBO};

    #[test]
    fn subscribe() {
        gen_test_subscribe!(
            HuobiLinearSwapWSClient,
            &vec!["market.BTC-USDT.trade.detail".to_string()]
        );
    }

    #[test]
    fn subscribe_trade() {
        gen_test_subscribe_trade!(HuobiLinearSwapWSClient, &vec!["BTC-USDT".to_string()]);
    }

    #[test]
    fn subscribe_ticker() {
        gen_test_subscribe_ticker!(HuobiLinearSwapWSClient, &vec!["BTC-USDT".to_string()]);
    }

    #[test]
    fn subscribe_bbo() {
        gen_test_subscribe_bbo!(HuobiLinearSwapWSClient, &vec!["BTC-USDT".to_string()]);
    }
}

#[cfg(test)]
mod huobi_inverse_swap {
    use crypto_ws_client::{HuobiInverseSwapWSClient, WSClient, BBO};

    #[test]
    fn subscribe() {
        gen_test_subscribe!(
            HuobiInverseSwapWSClient,
            &vec!["market.BTC-USD.trade.detail".to_string()]
        );
    }

    #[test]
    fn subscribe_trade() {
        gen_test_subscribe_trade!(HuobiInverseSwapWSClient, &vec!["BTC-USD".to_string()]);
    }

    #[test]
    fn subscribe_ticker() {
        gen_test_subscribe_ticker!(HuobiInverseSwapWSClient, &vec!["BTC-USD".to_string()]);
    }

    #[test]
    fn subscribe_bbo() {
        gen_test_subscribe_bbo!(HuobiInverseSwapWSClient, &vec!["BTC-USD".to_string()]);
    }
}

#[cfg(test)]
mod huobi_option {
    use crypto_ws_client::{HuobiOptionWSClient, WSClient, BBO};

    #[test]
    fn subscribe() {
        gen_test_subscribe!(
            HuobiOptionWSClient,
            &vec!["market.BTC-USDT-210326-C-32000.trade.detail".to_string()]
        );
    }

    #[test]
    fn subscribe_trade() {
        gen_test_subscribe_trade!(
            HuobiOptionWSClient,
            &vec!["BTC-USDT-210326-C-32000".to_string()]
        );
    }

    #[test]
    fn subscribe_ticker() {
        gen_test_subscribe_ticker!(
            HuobiOptionWSClient,
            &vec!["BTC-USDT-210326-C-32000".to_string()]
        );
    }

    #[test]
    fn subscribe_bbo() {
        gen_test_subscribe_bbo!(
            HuobiOptionWSClient,
            &vec!["BTC-USDT-210326-C-32000".to_string()]
        );
    }

    #[test]
    fn subscribe_overview() {
        gen_test_subscribe!(HuobiOptionWSClient, &vec!["market.overview".to_string()]);
    }
}
