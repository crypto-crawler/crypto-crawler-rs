use crypto_ws_client::{KrakenWSClient, WSClient, BBO};

#[macro_use]
mod utils;

#[test]
fn subscribe() {
    gen_test_subscribe!(
        KrakenWSClient,
        &vec![
            "trade:XBT/USD".to_string(),
            "ticker:XBT/USD".to_string(),
            "spread:XBT/USD".to_string(),
            "book:XBT/USD".to_string()
        ]
    );
}

#[test]
fn subscribe_raw_json() {
    gen_test_subscribe!(
        KrakenWSClient,
        &vec![
            r#"{"event":"subscribe","pair":["XBT/USD"],"subscription":{"name":"trade"}}"#
                .to_string()
        ]
    );
}

#[test]
fn subscribe_trade() {
    gen_test_subscribe_trade!(
        KrakenWSClient,
        &vec!["XBT/USD".to_string(), "ETH/USD".to_string()]
    );
}

#[test]
fn subscribe_ticker() {
    gen_test_subscribe_ticker!(
        KrakenWSClient,
        &vec!["XBT/USD".to_string(), "ETH/USD".to_string()]
    );
}

#[test]
fn subscribe_bbo() {
    gen_test_subscribe_bbo!(
        KrakenWSClient,
        &vec!["XBT/USD".to_string(), "ETH/USD".to_string()]
    );
}
