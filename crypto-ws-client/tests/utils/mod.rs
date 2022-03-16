macro_rules! gen_test_code {
    ($client:ident, $func_name:ident, $pairs:expr) => {
        let (tx, rx): (Sender<String>, Receiver<String>) = std::sync::mpsc::channel();
        let mut messages = Vec::<String>::new();
        {
            let ws_client = $client::new(tx, None);
            ws_client.$func_name($pairs);
            ws_client.run(Some(0)); // return immediately once after a normal message
            ws_client.close();
        }
        for msg in rx {
            messages.push(msg);
        }
        assert!(!messages.is_empty());
    };
}

#[allow(unused_macros)]
macro_rules! gen_test_subscribe_candlestick {
    ($client:ident, $symbol_interval_list:expr) => {
        let (tx, rx): (Sender<String>, Receiver<String>) = std::sync::mpsc::channel();
        let mut messages = Vec::<String>::new();
        {
            let ws_client = $client::new(tx, None);
            ws_client.subscribe_candlestick($symbol_interval_list);
            ws_client.run(Some(0)); // return immediately once after a normal message
            ws_client.close();
        }
        for msg in rx {
            messages.push(msg);
        }
        assert!(!messages.is_empty());
    };
}
