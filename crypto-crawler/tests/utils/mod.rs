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
        $crawl_func($exchange, $market_type, Some(&symbols), on_msg, Some(0));

        MESSAGES.with(|slf| {
            let messages = slf.borrow();

            assert!(!messages.is_empty());
            assert_eq!(messages[0].exchange, $exchange.to_string());
            assert_eq!(messages[0].market_type, $market_type);
            assert_eq!(messages[0].msg_type, $msg_type);
        });
    }};
}

#[allow(unused_macros)]
macro_rules! gen_test_snapshot_without_symbol_code {
    ($crawl_func:ident, $exchange:expr, $market_type:expr, $msg_type:expr) => {{
        thread_local! {
            static MESSAGES: RefCell<Vec<Message>> = RefCell::new(Vec::new());
        }

        let on_msg = Arc::new(Mutex::new(|msg: Message| {
            MESSAGES.with(|messages| messages.borrow_mut().push(msg))
        }));
        let symbols = if $market_type == MarketType::Spot {
            let spot_symbols = fetch_symbols_retry($exchange, $market_type);
            get_hot_spot_symbols($exchange, &spot_symbols)
        } else {
            fetch_symbols_retry($exchange, $market_type)
        };
        $crawl_func($exchange, $market_type, Some(&symbols), on_msg, Some(0));

        MESSAGES.with(|slf| {
            let messages = slf.borrow();

            assert!(!messages.is_empty());
            assert_eq!(messages.len(), symbols.len());

            assert_eq!(messages[0].exchange, $exchange.to_string());
            assert_eq!(messages[0].market_type, $market_type);
            assert_eq!(messages[0].msg_type, $msg_type);
        });
    }};
}
