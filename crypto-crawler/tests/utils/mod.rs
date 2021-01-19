macro_rules! gen_test_code {
    ($crawl_func:ident, $exchange:expr, $market_type:expr, $symbol:expr, $msg_type:expr) => {{
        thread_local! {
            static MESSAGES: RefCell<Vec<Message>> = RefCell::new(Vec::new());
        }

        let on_msg = Arc::new(Mutex::new(|msg: Message| {
            MESSAGES.with(|messages| messages.borrow_mut().push(msg))
        }));
        let symbols = vec![$symbol.to_string()];
        $crawl_func($exchange, $market_type, Some(&symbols), on_msg, Some(0));

        MESSAGES.with(|slf| {
            let messages = slf.borrow();

            assert!(!messages.is_empty());
            assert_eq!(messages[0].exchange, $exchange.to_string());
            assert_eq!(messages[0].market_type, $market_type);
            if $exchange == "binance" {
                // Binance REST APIs use uppercase while websocket uses lowercase
                assert_eq!(messages[0].symbol, $symbol.to_string().to_uppercase());
            } else {
                assert_eq!(messages[0].symbol, $symbol.to_string());
            }
            assert_eq!(messages[0].msg_type, $msg_type);
        });
    }};
}

macro_rules! gen_test_snapshot_code {
    ($crawl_func:ident, $exchange:expr, $market_type:expr, $symbol:expr, $msg_type:expr) => {{
        thread_local! {
            static MESSAGES: RefCell<Vec<Message>> = RefCell::new(Vec::new());
        }

        let on_msg = Arc::new(Mutex::new(|msg: Message| {
            MESSAGES.with(|messages| messages.borrow_mut().push(msg))
        }));
        let symbols = vec![$symbol.to_string()];
        $crawl_func(
            $exchange,
            $market_type,
            Some(&symbols),
            on_msg,
            None,
            Some(0),
        );

        MESSAGES.with(|slf| {
            let messages = slf.borrow();

            assert!(!messages.is_empty());
            assert_eq!(messages[0].exchange, $exchange.to_string());
            assert_eq!(messages[0].market_type, $market_type);
            if $exchange == "binance" {
                // Binance REST APIs use uppercase while websocket uses lowercase
                assert_eq!(messages[0].symbol, $symbol.to_string().to_uppercase());
            } else {
                assert_eq!(messages[0].symbol, $symbol.to_string());
            }
            assert_eq!(messages[0].msg_type, $msg_type);
        });
    }};
}
