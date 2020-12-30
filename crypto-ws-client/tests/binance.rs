use crypto_ws_client::{BinanceSpotWSClient, WSClient};

#[test]
fn binance_spot() {
    let mut ws_client = BinanceSpotWSClient::init(|msg| println!("{}", msg), None);
    let channels = vec!["btcusdt@aggTrade".to_string()];
    ws_client.subscribe(&channels);
    ws_client.run(Some(5));
    ws_client.close();
    assert_eq!(5, 5);
}
