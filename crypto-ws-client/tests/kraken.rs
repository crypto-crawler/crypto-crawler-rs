use crypto_ws_client::{KrakenSpotWSClient, WSClient};

#[macro_use]
mod utils;

#[test]
fn kraken_spot() {
    gen_test!(
        KrakenSpotWSClient,
        &vec![
            "trade:XBT/USD".to_string(),
            "ticker:XBT/USD".to_string(),
            "spread:XBT/USD".to_string(),
            "book:XBT/USD".to_string()
        ]
    );
}
