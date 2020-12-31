macro_rules! gen_test {
    ($client:ident, $channels:expr) => {
        let mut messages = Vec::<String>::new();
        {
            let on_msg = |msg: String| messages.push(msg);
            let mut ws_client = $client::new(Box::new(on_msg), None);
            ws_client.subscribe($channels);
            ws_client.run(Some(1)); // run for 1 second
            ws_client.close();
        }
        assert!(!messages.is_empty());
    };
}
