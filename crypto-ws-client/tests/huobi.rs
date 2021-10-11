#[macro_use]
mod utils;

#[cfg(test)]
mod huobi_spot {
    use crypto_ws_client::{HuobiSpotWSClient, WSClient};
    use std::sync::mpsc::{Receiver, Sender};

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
        let (tx, rx): (Sender<String>, Receiver<String>) = std::sync::mpsc::channel();
        let mut messages = Vec::<String>::new();
        {
            let ws_client = HuobiSpotWSClient::new(tx, Some("wss://api.huobi.pro/feed"));
            ws_client.subscribe_orderbook(&vec!["btcusdt".to_string()]);
            ws_client.run(Some(0)); // return immediately once after getting a normal message
        }
        for msg in rx {
            messages.push(msg);
        }
        assert!(!messages.is_empty());
    }

    #[test]
    fn subscribe_orderbook_topk() {
        gen_test_code!(
            HuobiSpotWSClient,
            subscribe_orderbook_topk,
            &vec!["btcusdt".to_string()]
        );
    }

    #[test]
    fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(HuobiSpotWSClient, &vec![("btcusdt".to_string(), 60)]);
        gen_test_subscribe_candlestick!(HuobiSpotWSClient, &vec![("btcusdt".to_string(), 2592000)]);
    }
}

#[cfg(test)]
mod huobi_inverse_future {
    use crypto_ws_client::{HuobiFutureWSClient, WSClient};
    use std::sync::mpsc::{Receiver, Sender};

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
    fn subscribe_orderbook_topk() {
        gen_test_code!(
            HuobiFutureWSClient,
            subscribe_orderbook_topk,
            &vec!["BTC_CQ".to_string()]
        );
    }

    #[test]
    fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(HuobiFutureWSClient, &vec![("BTC_CQ".to_string(), 60)]);
        gen_test_subscribe_candlestick!(
            HuobiFutureWSClient,
            &vec![("BTC_CQ".to_string(), 2592000)]
        );
    }
}

#[cfg(test)]
mod huobi_linear_swap {
    use crypto_ws_client::{HuobiLinearSwapWSClient, WSClient};
    use std::sync::mpsc::{Receiver, Sender};

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
    fn subscribe_orderbook_topk() {
        gen_test_code!(
            HuobiLinearSwapWSClient,
            subscribe_orderbook_topk,
            &vec!["BTC-USDT".to_string()]
        );
    }

    #[test]
    fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(
            HuobiLinearSwapWSClient,
            &vec![("BTC-USDT".to_string(), 60)]
        );
        gen_test_subscribe_candlestick!(
            HuobiLinearSwapWSClient,
            &vec![("BTC-USDT".to_string(), 2592000)]
        );
    }

    #[test]
    fn subscribe_funding_rate() {
        let (tx, rx): (Sender<String>, Receiver<String>) = std::sync::mpsc::channel();
        let mut messages = Vec::<String>::new();
        {
            let ws_client = HuobiLinearSwapWSClient::new(
                tx,
                Some("wss://api.hbdm.com/linear-swap-notification"),
            );
            ws_client.subscribe(&vec![
                r#"{"topic":"public.BTC-USDT.funding_rate","op":"sub"}"#.to_string(),
            ]);
            ws_client.run(Some(0)); // return immediately once after a normal message
            ws_client.close();
        }
        for msg in rx {
            messages.push(msg);
        }
        assert!(!messages.is_empty());
    }

    #[test]
    fn subscribe_funding_rate_all() {
        let (tx, rx): (Sender<String>, Receiver<String>) = std::sync::mpsc::channel();
        let mut messages = Vec::<String>::new();
        {
            let ws_client = HuobiLinearSwapWSClient::new(
                tx,
                Some("wss://api.hbdm.com/linear-swap-notification"),
            );
            ws_client.subscribe(&vec![
                r#"{"topic":"public.*.funding_rate","op":"sub"}"#.to_string()
            ]);
            ws_client.run(Some(0)); // return immediately once after a normal message
            ws_client.close();
        }
        for msg in rx {
            messages.push(msg);
        }
        assert!(!messages.is_empty());
    }
}

#[cfg(test)]
mod huobi_inverse_swap {
    use crypto_ws_client::{HuobiInverseSwapWSClient, WSClient};
    use std::sync::mpsc::{Receiver, Sender};

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
    fn subscribe_orderbook_topk() {
        gen_test_code!(
            HuobiInverseSwapWSClient,
            subscribe_orderbook_topk,
            &vec!["BTC-USD".to_string()]
        );
    }

    #[test]
    fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(
            HuobiInverseSwapWSClient,
            &vec![("BTC-USD".to_string(), 60)]
        );
        gen_test_subscribe_candlestick!(
            HuobiInverseSwapWSClient,
            &vec![("BTC-USD".to_string(), 2592000)]
        );
    }

    #[test]
    fn subscribe_funding_rate() {
        let (tx, rx): (Sender<String>, Receiver<String>) = std::sync::mpsc::channel();
        let mut messages = Vec::<String>::new();
        {
            let ws_client =
                HuobiInverseSwapWSClient::new(tx, Some("wss://api.hbdm.com/swap-notification"));
            ws_client.subscribe(&vec![
                r#"{"topic":"public.BTC-USD.funding_rate","op":"sub"}"#.to_string(),
            ]);
            ws_client.run(Some(0)); // return immediately once after a normal message
            ws_client.close();
        }
        for msg in rx {
            messages.push(msg);
        }
        assert!(!messages.is_empty());
    }

    #[test]
    fn subscribe_funding_rate_all() {
        let (tx, rx): (Sender<String>, Receiver<String>) = std::sync::mpsc::channel();
        let mut messages = Vec::<String>::new();
        {
            let ws_client =
                HuobiInverseSwapWSClient::new(tx, Some("wss://api.hbdm.com/swap-notification"));
            ws_client.subscribe(&vec![
                r#"{"topic":"public.*.funding_rate","op":"sub"}"#.to_string()
            ]);
            ws_client.run(Some(0)); // return immediately once after a normal message
            ws_client.close();
        }
        for msg in rx {
            messages.push(msg);
        }
        assert!(!messages.is_empty());
    }

    #[test]
    fn subscribe_overview() {
        gen_test_code!(
            HuobiInverseSwapWSClient,
            subscribe,
            &vec!["market.overview".to_string()]
        );
    }
}

#[cfg(test)]
mod huobi_option {
    use crypto_ws_client::{HuobiOptionWSClient, WSClient};
    use std::sync::mpsc::{Receiver, Sender};

    #[test]
    #[ignore]
    fn subscribe() {
        gen_test_code!(
            HuobiOptionWSClient,
            subscribe,
            &vec!["market.BTC-USDT-210625-P-27000.trade.detail".to_string()]
        );
    }

    #[test]
    #[ignore]
    fn subscribe_trade() {
        gen_test_code!(
            HuobiOptionWSClient,
            subscribe_trade,
            &vec!["BTC-USDT-210625-P-27000".to_string()]
        );
    }

    #[test]
    #[ignore]
    fn subscribe_ticker() {
        gen_test_code!(
            HuobiOptionWSClient,
            subscribe_ticker,
            &vec!["BTC-USDT-210625-P-27000".to_string()]
        );
    }

    #[test]
    #[ignore]
    fn subscribe_bbo() {
        gen_test_code!(
            HuobiOptionWSClient,
            subscribe_bbo,
            &vec!["BTC-USDT-210625-P-27000".to_string()]
        );
    }

    #[test]
    #[ignore]
    fn subscribe_orderbook() {
        gen_test_code!(
            HuobiOptionWSClient,
            subscribe_orderbook,
            &vec!["BTC-USDT-210625-P-27000".to_string()]
        );
    }

    #[test]
    #[ignore]
    fn subscribe_orderbook_topk() {
        gen_test_code!(
            HuobiOptionWSClient,
            subscribe_orderbook_topk,
            &vec!["BTC-USDT-210625-P-27000".to_string()]
        );
    }

    #[test]
    #[ignore]
    fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(
            HuobiOptionWSClient,
            &vec![("BTC-USDT-210625-P-27000".to_string(), 60)]
        );
        gen_test_subscribe_candlestick!(
            HuobiOptionWSClient,
            &vec![("BTC-USDT-210625-P-27000".to_string(), 2592000)]
        );
    }

    #[test]
    #[ignore]
    fn subscribe_overview() {
        gen_test_code!(
            HuobiOptionWSClient,
            subscribe,
            &vec!["market.overview".to_string()]
        );
    }
}
