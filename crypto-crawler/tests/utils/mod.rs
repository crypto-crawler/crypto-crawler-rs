#[allow(unused_macros)]
macro_rules! test_one_symbol {
    ($crawl_func:ident, $exchange:expr, $market_type:expr, $symbol:expr, $msg_type:expr) => {{
        let (tx, rx) = std::sync::mpsc::channel();
        let mut messages = Vec::new();
        let symbols = vec![$symbol.to_string()];
        $crawl_func($exchange, $market_type, Some(&symbols), tx, Some(0));

        for msg in rx {
            messages.push(msg);
        }

        assert!(!messages.is_empty());
        assert_eq!(messages[0].exchange, $exchange.to_string());
        assert_eq!(messages[0].market_type, $market_type);
        assert_eq!(messages[0].msg_type, $msg_type);
    }};
}

#[allow(unused_macros)]
macro_rules! test_all_symbols {
    ($crawl_func:ident, $exchange:expr, $market_type:expr, $msg_type:expr) => {{
        let (tx, rx) = std::sync::mpsc::channel();
        let mut messages = Vec::new();
        let symbols = if $market_type == MarketType::Spot {
            let spot_symbols = fetch_symbols_retry($exchange, $market_type);
            get_hot_spot_symbols($exchange, &spot_symbols)
        } else {
            fetch_symbols_retry($exchange, $market_type)
        };
        $crawl_func($exchange, $market_type, Some(&symbols), tx, Some(0));

        for msg in rx {
            messages.push(msg);
        }

        assert!(!messages.is_empty());
        assert_eq!(messages[0].exchange, $exchange.to_string());
        assert_eq!(messages[0].market_type, $market_type);
        assert_eq!(messages[0].msg_type, $msg_type);
    }};
}

#[allow(unused_macros)]
macro_rules! gen_test_crawl_candlestick {
    ($exchange:expr, $market_type:expr) => {{
        let (tx, rx) = std::sync::mpsc::channel();
        let mut messages = Vec::new();
        crawl_candlestick($exchange, $market_type, None, tx, Some(0));
        for msg in rx {
            messages.push(msg);
        }
        assert!(!messages.is_empty());
        assert_eq!(messages[0].exchange, EXCHANGE_NAME.to_string());
        assert_eq!(messages[0].market_type, $market_type);
        assert_eq!(messages[0].msg_type, MessageType::Candlestick);
    }};
}
