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
