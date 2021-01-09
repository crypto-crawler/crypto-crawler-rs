#[macro_use]
mod utils;

#[cfg(test)]
mod huobi_spot {
    use crypto_ws_client::{HuobiSpotWSClient, WSClient};
    use std::{cell::RefCell, rc::Rc};

    #[test]
    fn subscribe() {
        gen_test_code!(
            HuobiSpotWSClient,
            subscribe,
            &vec!["market.btcusdt.trade.detail".to_string()]
        );
    }

    #[test]
    fn subscribe_raw_json() {
        gen_test_code!(
            HuobiSpotWSClient,
            subscribe,
            &vec![r#"{"sub":"market.btcusdt.trade.detail","id":"crypto-ws-client"}"#.to_string()]
        );
    }

    #[test]
    fn subscribe_trade() {
        gen_test_code!(
            HuobiSpotWSClient,
            subscribe_trade,
            &vec!["btcusdt".to_string()]
        );
    }

    #[test]
    fn subscribe_ticker() {
        gen_test_code!(
            HuobiSpotWSClient,
            subscribe_ticker,
            &vec!["btcusdt".to_string()]
        );
    }

    #[test]
    fn subscribe_bbo() {
        gen_test_code!(
            HuobiSpotWSClient,
            subscribe_bbo,
            &vec!["btcusdt".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook() {
        let mut messages = Vec::<String>::new();
        {
            let on_msg = |msg: String| messages.push(msg);
            let mut ws_client = HuobiSpotWSClient::new(
                Rc::new(RefCell::new(on_msg)),
                Some("wss://api.huobi.pro/feed"),
            );
            ws_client.subscribe_orderbook(&vec!["btcusdt".to_string()]);
            ws_client.run(Some(0)); // return immediately once after getting a normal message
        }
        assert!(!messages.is_empty());
    }

    #[test]
    fn subscribe_orderbook_snapshot() {
        gen_test_code!(
            HuobiSpotWSClient,
            subscribe_orderbook_snapshot,
            &vec!["btcusdt".to_string()]
        );
    }

    #[test]
    fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(HuobiSpotWSClient, &vec!["btcusdt".to_string()], 60);
        gen_test_subscribe_candlestick!(HuobiSpotWSClient, &vec!["btcusdt".to_string()], 2592000);
    }

    #[test]
    fn huobi_hb10() {
        gen_test_code!(
            HuobiSpotWSClient,
            subscribe,
            &vec![
                "market.hb10usdt.trade.detail".to_string(),
                "market.huobi10.kline.1min".to_string()
            ]
        );
    }
}

#[cfg(test)]
mod huobi_future {
    use crypto_ws_client::{HuobiFutureWSClient, WSClient};
    use std::{cell::RefCell, rc::Rc};

    #[test]
    fn subscribe() {
        gen_test_code!(
            HuobiFutureWSClient,
            subscribe,
            &vec!["market.BTC_CQ.trade.detail".to_string()]
        );
    }

    #[test]
    fn subscribe_trade() {
        gen_test_code!(
            HuobiFutureWSClient,
            subscribe_trade,
            &vec!["BTC_CQ".to_string()]
        );
    }

    #[test]
    fn subscribe_ticker() {
        gen_test_code!(
            HuobiFutureWSClient,
            subscribe_ticker,
            &vec!["BTC_CQ".to_string()]
        );
    }

    #[test]
    fn subscribe_bbo() {
        gen_test_code!(
            HuobiFutureWSClient,
            subscribe_bbo,
            &vec!["BTC_CQ".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook() {
        gen_test_code!(
            HuobiFutureWSClient,
            subscribe_orderbook,
            &vec!["BTC_CQ".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook_snapshot() {
        gen_test_code!(
            HuobiFutureWSClient,
            subscribe_orderbook_snapshot,
            &vec!["BTC_CQ".to_string()]
        );
    }

    #[test]
    fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(HuobiFutureWSClient, &vec!["BTC_CQ".to_string()], 60);
        gen_test_subscribe_candlestick!(HuobiFutureWSClient, &vec!["BTC_CQ".to_string()], 2592000);
    }
}

#[cfg(test)]
mod huobi_linear_swap {
    use crypto_ws_client::{HuobiLinearSwapWSClient, WSClient};
    use std::{cell::RefCell, rc::Rc};

    #[test]
    fn subscribe() {
        gen_test_code!(
            HuobiLinearSwapWSClient,
            subscribe,
            &vec!["market.BTC-USDT.trade.detail".to_string()]
        );
    }

    #[test]
    fn subscribe_trade() {
        gen_test_code!(
            HuobiLinearSwapWSClient,
            subscribe_trade,
            &vec!["BTC-USDT".to_string()]
        );
    }

    #[test]
    fn subscribe_ticker() {
        gen_test_code!(
            HuobiLinearSwapWSClient,
            subscribe_ticker,
            &vec!["BTC-USDT".to_string()]
        );
    }

    #[test]
    fn subscribe_bbo() {
        gen_test_code!(
            HuobiLinearSwapWSClient,
            subscribe_bbo,
            &vec!["BTC-USDT".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook() {
        gen_test_code!(
            HuobiLinearSwapWSClient,
            subscribe_orderbook,
            &vec!["BTC-USDT".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook_snapshot() {
        gen_test_code!(
            HuobiLinearSwapWSClient,
            subscribe_orderbook_snapshot,
            &vec!["BTC-USDT".to_string()]
        );
    }

    #[test]
    fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(HuobiLinearSwapWSClient, &vec!["BTC-USDT".to_string()], 60);
        gen_test_subscribe_candlestick!(
            HuobiLinearSwapWSClient,
            &vec!["BTC-USDT".to_string()],
            2592000
        );
    }
}

#[cfg(test)]
mod huobi_inverse_swap {
    use crypto_ws_client::{HuobiInverseSwapWSClient, WSClient};
    use std::{cell::RefCell, rc::Rc};

    #[test]
    fn subscribe() {
        gen_test_code!(
            HuobiInverseSwapWSClient,
            subscribe,
            &vec!["market.BTC-USD.trade.detail".to_string()]
        );
    }

    #[test]
    fn subscribe_trade() {
        gen_test_code!(
            HuobiInverseSwapWSClient,
            subscribe_trade,
            &vec!["BTC-USD".to_string()]
        );
    }

    #[test]
    fn subscribe_ticker() {
        gen_test_code!(
            HuobiInverseSwapWSClient,
            subscribe_ticker,
            &vec!["BTC-USD".to_string()]
        );
    }

    #[test]
    fn subscribe_bbo() {
        gen_test_code!(
            HuobiInverseSwapWSClient,
            subscribe_bbo,
            &vec!["BTC-USD".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook() {
        gen_test_code!(
            HuobiInverseSwapWSClient,
            subscribe_orderbook,
            &vec!["BTC-USD".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook_snapshot() {
        gen_test_code!(
            HuobiInverseSwapWSClient,
            subscribe_orderbook_snapshot,
            &vec!["BTC-USD".to_string()]
        );
    }

    #[test]
    fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(HuobiInverseSwapWSClient, &vec!["BTC-USD".to_string()], 60);
        gen_test_subscribe_candlestick!(
            HuobiInverseSwapWSClient,
            &vec!["BTC-USD".to_string()],
            2592000
        );
    }
}

#[cfg(test)]
mod huobi_option {
    use crypto_ws_client::{HuobiOptionWSClient, WSClient};
    use std::{cell::RefCell, rc::Rc};

    #[test]
    fn subscribe() {
        gen_test_code!(
            HuobiOptionWSClient,
            subscribe,
            &vec!["market.BTC-USDT-210326-C-32000.trade.detail".to_string()]
        );
    }

    #[test]
    fn subscribe_trade() {
        gen_test_code!(
            HuobiOptionWSClient,
            subscribe_trade,
            &vec!["BTC-USDT-210326-C-32000".to_string()]
        );
    }

    #[test]
    fn subscribe_ticker() {
        gen_test_code!(
            HuobiOptionWSClient,
            subscribe_ticker,
            &vec!["BTC-USDT-210326-C-32000".to_string()]
        );
    }

    #[test]
    fn subscribe_bbo() {
        gen_test_code!(
            HuobiOptionWSClient,
            subscribe_bbo,
            &vec!["BTC-USDT-210326-C-32000".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook() {
        gen_test_code!(
            HuobiOptionWSClient,
            subscribe_orderbook,
            &vec!["BTC-USDT-210326-C-32000".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook_snapshot() {
        gen_test_code!(
            HuobiOptionWSClient,
            subscribe_orderbook_snapshot,
            &vec!["BTC-USDT-210326-C-32000".to_string()]
        );
    }

    #[test]
    fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(
            HuobiOptionWSClient,
            &vec!["BTC-USDT-210326-C-32000".to_string()],
            60
        );
        gen_test_subscribe_candlestick!(
            HuobiOptionWSClient,
            &vec!["BTC-USDT-210326-C-32000".to_string()],
            2592000
        );
    }

    #[test]
    fn subscribe_overview() {
        gen_test_code!(
            HuobiOptionWSClient,
            subscribe,
            &vec!["market.overview".to_string()]
        );
    }
}
