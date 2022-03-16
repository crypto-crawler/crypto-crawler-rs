#[macro_use]
mod utils;

#[cfg(test)]
mod binance_spot {
    use crypto_ws_client::{BinanceSpotWSClient, WSClient};
    use std::sync::mpsc::{Receiver, Sender};

    #[test]
    fn subscribe() {
        gen_test_code!(
            BinanceSpotWSClient,
            subscribe,
            &vec!["btcusdt@aggTrade".to_string(), "btcusdt@ticker".to_string()]
        );
    }

    #[test]
    fn subscribe_all_bbo() {
        gen_test_code!(
            BinanceSpotWSClient,
            subscribe,
            &vec!["!bookTicker".to_string()]
        );
    }

    #[test]
    fn subscribe_raw_json() {
        gen_test_code!(
            BinanceSpotWSClient,
            subscribe,
            &vec![r#"{"id":9527,"method":"SUBSCRIBE","params":["btcusdt@aggTrade","btcusdt@ticker"]}"#.to_string()]
        );
    }

    #[test]
    fn subscribe_trade() {
        gen_test_code!(
            BinanceSpotWSClient,
            subscribe_trade,
            &vec!["BTCUSDT".to_string(), "ETHUSDT".to_string()]
        );
    }

    #[test]
    fn subscribe_ticker() {
        gen_test_code!(
            BinanceSpotWSClient,
            subscribe_ticker,
            &vec!["BTCUSDT".to_string(), "ETHUSDT".to_string()]
        );
    }

    #[test]
    fn subscribe_tickers_all() {
        gen_test_code!(
            BinanceSpotWSClient,
            subscribe,
            &vec!["!ticker@arr".to_string()]
        );
    }

    #[test]
    fn subscribe_bbo() {
        gen_test_code!(
            BinanceSpotWSClient,
            subscribe_bbo,
            &vec!["BTCUSDT".to_string(), "ETHUSDT".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook() {
        gen_test_code!(
            BinanceSpotWSClient,
            subscribe_orderbook,
            &vec!["BTCUSDT".to_string(), "ETHUSDT".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook_topk() {
        gen_test_code!(
            BinanceSpotWSClient,
            subscribe_orderbook_topk,
            &vec!["BTCUSDT".to_string(), "ETHUSDT".to_string()]
        );
    }

    #[test]
    fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(
            BinanceSpotWSClient,
            &vec![("BTCUSDT".to_string(), 60), ("ETHUSDT".to_string(), 60)]
        );
        gen_test_subscribe_candlestick!(
            BinanceSpotWSClient,
            &vec![
                ("BTCUSDT".to_string(), 2592000),
                ("ETHUSDT".to_string(), 2592000)
            ]
        );
    }
}

#[cfg(test)]
mod binance_inverse_future {
    use crypto_ws_client::{BinanceInverseWSClient, WSClient};
    use std::sync::mpsc::{Receiver, Sender};

    #[test]
    fn subscribe() {
        gen_test_code!(
            BinanceInverseWSClient,
            subscribe,
            &vec!["btcusd_220325@aggTrade".to_string()]
        );
    }

    #[test]
    fn subscribe_all_bbo() {
        gen_test_code!(
            BinanceInverseWSClient,
            subscribe,
            &vec!["!bookTicker".to_string()]
        );
    }

    #[test]
    fn subscribe_trade() {
        gen_test_code!(
            BinanceInverseWSClient,
            subscribe_trade,
            &vec!["BTCUSD_220325".to_string(), "ETHUSD_220325".to_string()]
        );
    }

    #[test]
    fn subscribe_ticker() {
        gen_test_code!(
            BinanceInverseWSClient,
            subscribe_ticker,
            &vec!["BTCUSD_220325".to_string(), "ETHUSD_220325".to_string()]
        );
    }

    #[test]
    fn subscribe_tickers_all() {
        gen_test_code!(
            BinanceInverseWSClient,
            subscribe,
            &vec!["!ticker@arr".to_string()]
        );
    }

    #[test]
    fn subscribe_bbo() {
        gen_test_code!(
            BinanceInverseWSClient,
            subscribe_bbo,
            &vec!["BTCUSD_220325".to_string(), "ETHUSD_220325".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook() {
        gen_test_code!(
            BinanceInverseWSClient,
            subscribe_orderbook,
            &vec!["BTCUSD_220325".to_string(), "ETHUSD_220325".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook_topk() {
        gen_test_code!(
            BinanceInverseWSClient,
            subscribe_orderbook_topk,
            &vec!["BTCUSD_220325".to_string(), "ETHUSD_220325".to_string()]
        );
    }

    #[test]
    fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(
            BinanceInverseWSClient,
            &vec![
                ("BTCUSD_220325".to_string(), 60),
                ("ETHUSD_220325".to_string(), 60)
            ]
        );
        gen_test_subscribe_candlestick!(
            BinanceInverseWSClient,
            &vec![
                ("BTCUSD_220325".to_string(), 2592000),
                ("ETHUSD_220325".to_string(), 2592000)
            ]
        );
    }
}

#[cfg(test)]
mod binance_linear_future {
    use crypto_ws_client::{BinanceLinearWSClient, WSClient};
    use std::sync::mpsc::{Receiver, Sender};

    #[test]
    fn subscribe() {
        gen_test_code!(
            BinanceLinearWSClient,
            subscribe,
            &vec!["btcusdt_220325@aggTrade".to_string()]
        );
    }

    #[test]
    fn subscribe_all_bbo() {
        gen_test_code!(
            BinanceLinearWSClient,
            subscribe,
            &vec!["!bookTicker".to_string()]
        );
    }

    #[test]
    fn subscribe_trade() {
        gen_test_code!(
            BinanceLinearWSClient,
            subscribe_trade,
            &vec!["BTCUSDT_220325".to_string(), "ETHUSDT_220325".to_string()]
        );
    }

    #[test]
    fn subscribe_ticker() {
        gen_test_code!(
            BinanceLinearWSClient,
            subscribe_ticker,
            &vec!["BTCUSDT_220325".to_string(), "ETHUSDT_220325".to_string()]
        );
    }

    #[test]
    fn subscribe_tickers_all() {
        gen_test_code!(
            BinanceLinearWSClient,
            subscribe,
            &vec!["!ticker@arr".to_string()]
        );
    }

    #[test]
    fn subscribe_bbo() {
        gen_test_code!(
            BinanceLinearWSClient,
            subscribe_bbo,
            &vec!["BTCUSDT_220325".to_string(), "ETHUSDT_220325".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook() {
        gen_test_code!(
            BinanceLinearWSClient,
            subscribe_orderbook,
            &vec!["BTCUSDT_220325".to_string(), "ETHUSDT_220325".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook_topk() {
        gen_test_code!(
            BinanceLinearWSClient,
            subscribe_orderbook_topk,
            &vec!["BTCUSDT_220325".to_string(), "ETHUSDT_220325".to_string()]
        );
    }

    #[test]
    fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(
            BinanceLinearWSClient,
            &vec![
                ("BTCUSDT_220325".to_string(), 60),
                ("ETHUSDT_220325".to_string(), 60)
            ]
        );
        gen_test_subscribe_candlestick!(
            BinanceLinearWSClient,
            &vec![
                ("BTCUSDT_220325".to_string(), 2592000),
                ("ETHUSDT_220325".to_string(), 2592000)
            ]
        );
    }
}

#[cfg(test)]
mod binance_inverse_swap {
    use crypto_ws_client::{BinanceInverseWSClient, WSClient};
    use std::sync::mpsc::{Receiver, Sender};

    #[test]
    fn subscribe() {
        gen_test_code!(
            BinanceInverseWSClient,
            subscribe,
            &vec!["btcusd_perp@aggTrade".to_string()]
        );
    }

    #[test]
    fn subscribe_all_bbo() {
        gen_test_code!(
            BinanceInverseWSClient,
            subscribe,
            &vec!["!bookTicker".to_string()]
        );
    }

    #[test]
    fn subscribe_trade() {
        gen_test_code!(
            BinanceInverseWSClient,
            subscribe_trade,
            &vec!["BTCUSD_PERP".to_string(), "ETHUSD_PERP".to_string()]
        );
    }

    #[test]
    fn subscribe_ticker() {
        gen_test_code!(
            BinanceInverseWSClient,
            subscribe_ticker,
            &vec!["BTCUSD_PERP".to_string(), "ETHUSD_PERP".to_string()]
        );
    }

    #[test]
    fn subscribe_tickers_all() {
        gen_test_code!(
            BinanceInverseWSClient,
            subscribe,
            &vec!["!ticker@arr".to_string()]
        );
    }

    #[test]
    fn subscribe_bbo() {
        gen_test_code!(
            BinanceInverseWSClient,
            subscribe_bbo,
            &vec!["BTCUSD_PERP".to_string(), "ETHUSD_PERP".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook() {
        gen_test_code!(
            BinanceInverseWSClient,
            subscribe_orderbook,
            &vec!["BTCUSD_PERP".to_string(), "ETHUSD_PERP".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook_topk() {
        gen_test_code!(
            BinanceInverseWSClient,
            subscribe_orderbook_topk,
            &vec!["BTCUSD_PERP".to_string(), "ETHUSD_PERP".to_string()]
        );
    }

    #[test]
    fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(
            BinanceInverseWSClient,
            &vec![
                ("BTCUSD_PERP".to_string(), 60),
                ("ETHUSD_PERP".to_string(), 60)
            ]
        );
        gen_test_subscribe_candlestick!(
            BinanceInverseWSClient,
            &vec![
                ("BTCUSD_PERP".to_string(), 2592000),
                ("ETHUSD_PERP".to_string(), 2592000)
            ]
        );
    }

    #[test]
    fn subscribe_funding_rate() {
        gen_test_code!(
            BinanceInverseWSClient,
            subscribe,
            &vec!["btcusd_perp@markPrice".to_string()]
        );
    }

    #[test]
    fn subscribe_funding_rate_all() {
        gen_test_code!(
            BinanceInverseWSClient,
            subscribe,
            &vec!["!markPrice@arr".to_string()]
        );
    }
}

#[cfg(test)]
mod binance_linear_swap {
    use crypto_ws_client::{BinanceLinearWSClient, WSClient};
    use std::sync::mpsc::{Receiver, Sender};

    #[test]
    fn subscribe() {
        gen_test_code!(
            BinanceLinearWSClient,
            subscribe,
            &vec!["btcusdt@aggTrade".to_string()]
        );
    }

    #[test]
    fn subscribe_all_bbo() {
        gen_test_code!(
            BinanceLinearWSClient,
            subscribe,
            &vec!["!bookTicker".to_string()]
        );
    }

    #[test]
    fn subscribe_trade() {
        gen_test_code!(
            BinanceLinearWSClient,
            subscribe_trade,
            &vec!["BTCUSDT".to_string(), "ETHUSDT".to_string()]
        );
    }

    #[test]
    fn subscribe_ticker() {
        gen_test_code!(
            BinanceLinearWSClient,
            subscribe_ticker,
            &vec!["BTCUSDT".to_string(), "ETHUSDT".to_string()]
        );
    }

    #[test]
    fn subscribe_tickers_all() {
        gen_test_code!(
            BinanceLinearWSClient,
            subscribe,
            &vec!["!ticker@arr".to_string()]
        );
    }

    #[test]
    fn subscribe_bbo() {
        gen_test_code!(
            BinanceLinearWSClient,
            subscribe_bbo,
            &vec!["BTCUSDT".to_string(), "ETHUSDT".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook() {
        gen_test_code!(
            BinanceLinearWSClient,
            subscribe_orderbook,
            &vec!["BTCUSDT".to_string(), "ETHUSDT".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook_topk() {
        gen_test_code!(
            BinanceLinearWSClient,
            subscribe_orderbook_topk,
            &vec!["BTCUSDT".to_string(), "ETHUSDT".to_string()]
        );
    }

    #[test]
    fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(
            BinanceLinearWSClient,
            &vec![("BTCUSDT".to_string(), 60), ("ETHUSDT".to_string(), 60)]
        );
        gen_test_subscribe_candlestick!(
            BinanceLinearWSClient,
            &vec![
                ("BTCUSDT".to_string(), 2592000),
                ("ETHUSDT".to_string(), 2592000)
            ]
        );
    }

    #[test]
    fn subscribe_funding_rate() {
        gen_test_code!(
            BinanceLinearWSClient,
            subscribe,
            &vec!["btcusdt@markPrice".to_string()]
        );
    }

    #[test]
    fn subscribe_funding_rate_all() {
        gen_test_code!(
            BinanceLinearWSClient,
            subscribe,
            &vec!["!markPrice@arr".to_string()]
        );
    }
}
