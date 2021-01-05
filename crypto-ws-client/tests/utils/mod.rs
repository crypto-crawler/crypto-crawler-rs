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

// TODO: this macro is actually being used
#[allow(unused_macros)]
macro_rules! gen_test_subscribe_trade {
    ($client:ident, $pairs:expr) => {
        let mut messages = Vec::<String>::new();
        {
            let on_msg = |msg: String| messages.push(msg);
            let mut ws_client = $client::new(Box::new(on_msg), None);
            ws_client.subscribe_trade($pairs);
            ws_client.run(Some(0)); // return immediately once after a normal message
        }
        assert!(!messages.is_empty());
    };
}

// TODO: this macro is actually being used
#[allow(unused_macros)]
macro_rules! gen_test_subscribe_ticker {
    ($client:ident, $pairs:expr) => {
        let mut messages = Vec::<String>::new();
        {
            let on_msg = |msg: String| messages.push(msg);
            let mut ws_client = $client::new(Box::new(on_msg), None);
            ws_client.subscribe_ticker($pairs);
            ws_client.run(Some(0)); // return immediately once after a normal message
        }
        assert!(!messages.is_empty());
    };
}

// TODO: this macro is actually being used
#[allow(unused_macros)]
macro_rules! gen_test_subscribe_bbo {
    ($client:ident, $pairs:expr) => {
        let mut messages = Vec::<String>::new();
        {
            let on_msg = |msg: String| messages.push(msg);
            let mut ws_client = $client::new(Box::new(on_msg), None);
            ws_client.subscribe_bbo($pairs);
            ws_client.run(Some(0)); // return immediately once after a normal message
        }
        assert!(!messages.is_empty());
    };
}

// TODO: this macro is actually being used
#[allow(unused_macros)]
macro_rules! gen_test_subscribe_orderbook {
    ($client:ident, $pairs:expr) => {
        let mut messages = Vec::<String>::new();
        {
            let on_msg = |msg: String| messages.push(msg);
            let mut ws_client = $client::new(Box::new(on_msg), None);
            ws_client.subscribe_orderbook($pairs);
            ws_client.run(Some(0)); // return immediately once after a normal message
        }
        assert!(!messages.is_empty());
    };
}

// TODO: this macro is actually being used
#[allow(unused_macros)]
macro_rules! gen_test_subscribe_orderbook_snapshot {
    ($client:ident, $pairs:expr) => {
        let mut messages = Vec::<String>::new();
        {
            let on_msg = |msg: String| messages.push(msg);
            let mut ws_client = $client::new(Box::new(on_msg), None);
            ws_client.subscribe_orderbook_snapshot($pairs);
            ws_client.run(Some(0)); // return immediately once after a normal message
        }
        assert!(!messages.is_empty());
    };
}

// TODO: this macro is actually being used
#[allow(unused_macros)]
macro_rules! gen_test_subscribe_candlestick {
    ($client:ident, $pairs:expr, $interval:expr) => {
        let mut messages = Vec::<String>::new();
        {
            let on_msg = |msg: String| messages.push(msg);
            let mut ws_client = $client::new(Box::new(on_msg), None);
            ws_client.subscribe_candlestick($pairs, $interval);
            ws_client.run(Some(0)); // return immediately once after a normal message
        }
        assert!(!messages.is_empty());
    };
}
