use crypto_ws_client::{BithumbWSClient, WSClient};

#[macro_use]
mod utils;

#[tokio::test(flavor = "multi_thread")]
async fn subscribe() {
    gen_test_code!(
        BithumbWSClient,
        subscribe,
        &vec![
            ("TRADE".to_string(), "BTC-USDT".to_string()),
            ("TRADE".to_string(), "ETH-USDT".to_string()),
        ]
    );
}

#[test]
#[should_panic]
fn subscribe_illegal_symbol() {
    gen_test_code!(
        BithumbWSClient,
        subscribe,
        &vec![("TRADE".to_string(), "XXX-YYY".to_string())]
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn subscribe_raw_json() {
    gen_test_code!(
        BithumbWSClient,
        send,
        &vec![r#"{"cmd":"subscribe","args":["TRADE:BTC-USDT","TRADE:ETH-USDT"]}"#.to_string()]
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn subscribe_trade() {
    gen_test_code!(
        BithumbWSClient,
        subscribe_trade,
        &vec!["BTC-USDT".to_string(), "ETH-USDT".to_string()]
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn subscribe_orderbook() {
    gen_test_code!(
        BithumbWSClient,
        subscribe_orderbook,
        &vec!["BTC-USDT".to_string(), "ETH-USDT".to_string()]
    );
}

#[test]
#[ignore = "too slow"]
fn subscribe_ticker() {
    gen_test_code!(
        BithumbWSClient,
        subscribe_ticker,
        &vec!["BTC-USDT".to_string(), "ETH-USDT".to_string()]
    );
}
