macro_rules! gen_test_code {
    ($client:ident, $func_name:ident, $pairs:expr) => {
        let messages = Rc::new(RefCell::new(Vec::<String>::new()));
        let messages2 = messages.clone();
        let on_msg = Rc::new(RefCell::new(|msg: String| messages2.borrow_mut().push(msg)));
        {
            let ws_client = $client::new(on_msg.clone(), None);
            ws_client.$func_name($pairs);
            ws_client.run(Some(0)); // return immediately once after a normal message
            ws_client.close();
        }
        assert!(!messages.borrow().is_empty());
    };
}

// TODO: this macro is actually being used
#[allow(unused_macros)]
macro_rules! gen_test_subscribe_candlestick {
    ($client:ident, $pairs:expr, $interval:expr) => {
        let messages = Rc::new(RefCell::new(Vec::<String>::new()));
        let messages2 = messages.clone();
        let on_msg = Rc::new(RefCell::new(|msg: String| messages2.borrow_mut().push(msg)));
        {
            let ws_client = $client::new(on_msg.clone(), None);
            ws_client.subscribe_candlestick($pairs, $interval);
            ws_client.run(Some(0)); // return immediately once after a normal message
            ws_client.close();
        }
        assert!(!messages.borrow().is_empty());
    };
}
