use crypto_ws_client::{
    BinanceDeliveryWSClient, BinanceFuturesWSClient, BinanceSpotWSClient, WSClient,
};

#[test]
fn binance_spot() {
    let mut messages = Vec::<String>::new();
    {
        let on_msg = |msg: String| messages.push(msg);
        let mut ws_client = BinanceSpotWSClient::new(Box::new(on_msg), None);
        let channels = vec!["btcusdt@aggTrade".to_string()];
        ws_client.subscribe(&channels);
        ws_client.run(Some(1)); // run for 1 second
        ws_client.close();
    }
    assert!(!messages.is_empty());
}

#[test]
fn binance_futures() {
    let mut messages = Vec::<String>::new();
    {
        let on_msg = |msg: String| messages.push(msg);
        let mut ws_client = BinanceFuturesWSClient::new(Box::new(on_msg), None);
        let channels = vec!["btcusdt@aggTrade".to_string()];
        ws_client.subscribe(&channels);
        ws_client.run(Some(1)); // run for 1 second
        ws_client.close();
    }
    assert!(!messages.is_empty());
}

#[test]
fn binance_delivery() {
    let mut messages = Vec::<String>::new();
    {
        let on_msg = |msg: String| messages.push(msg);
        let mut ws_client = BinanceDeliveryWSClient::new(Box::new(on_msg), None);
        let channels = vec!["btcusd_perp@aggTrade".to_string()];
        ws_client.subscribe(&channels);
        ws_client.run(Some(1)); // run for 1 second
        ws_client.close();
    }
    assert!(!messages.is_empty());
}
