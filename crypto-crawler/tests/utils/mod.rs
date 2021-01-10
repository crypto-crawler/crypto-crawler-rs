macro_rules! gen_test_code {
    ($crawl_func:ident, $exchange:expr, $market_type:expr, $symbol:expr, $msg_type:expr) => {{
        let mut messages = Vec::<Message>::new();

        let on_msg = Arc::new(Mutex::new(|msg: Message| messages.push(msg)));
        $crawl_func(
            $exchange,
            $market_type,
            &vec![$symbol.to_string()],
            on_msg,
            Some(0),
        );

        assert!(!messages.is_empty());
        assert_eq!(messages[0].exchange, $exchange.to_string());
        assert_eq!(messages[0].market_type, $market_type);
        if $exchange == "Binance" {
            // Binance REST APIs use uppercase while websocket uses lowercase
            assert_eq!(messages[0].symbol, $symbol.to_string().to_uppercase());
        } else {
            assert_eq!(messages[0].symbol, $symbol.to_string());
        }
        assert_eq!(messages[0].msg_type, $msg_type);
    }};
}
