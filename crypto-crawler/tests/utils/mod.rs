macro_rules! gen_test_subscribe {
    ($client:ident, $channels:expr) => {
        let mut messages = Vec::<String>::new();
        {
            let on_msg = |msg: String| messages.push(msg);
            let mut ws_client = $client::new(Box::new(on_msg), None);
            ws_client.subscribe($channels);
            ws_client.run(Some(0)); // return immediately once after a normal message
        }
        assert!(!messages.is_empty());
    };
}

macro_rules! gen_test_code {
    ($crawl_func:ident, $exchange:expr, $market_type:expr, $symbol:expr, $msg_type:expr) => {{
        let mut messages = Vec::<Message>::new();

        let on_msg = |msg: Message| messages.push(msg);
        $crawl_func(
            $exchange,
            $market_type,
            &vec![$symbol.to_string()],
            Box::new(on_msg),
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
