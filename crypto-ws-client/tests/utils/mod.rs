macro_rules! gen_test_code {
    ($client:ident, $func_name:ident, $pairs:expr) => {
        let mut messages = Vec::<String>::new();
        {
            let on_msg = Arc::new(Mutex::new(|msg: String| messages.push(msg)));
            let ws_client = $client::new(on_msg.clone(), None);
            ws_client.$func_name($pairs);
            ws_client.run(Some(0)); // return immediately once after a normal message
            ws_client.close();
        }
        assert!(!messages.is_empty());
    };
}

// TODO: this macro is actually being used
#[allow(unused_macros)]
macro_rules! gen_test_subscribe_candlestick {
    ($client:ident, $symbol_interval_list:expr) => {
        let mut messages = Vec::<String>::new();
        {
            let on_msg = Arc::new(Mutex::new(|msg: String| messages.push(msg)));
            let ws_client = $client::new(on_msg.clone(), None);
            ws_client.subscribe_candlestick($symbol_interval_list);
            ws_client.run(Some(0)); // return immediately once after a normal message
            ws_client.close();
        }
        assert!(!messages.is_empty());
    };
}
