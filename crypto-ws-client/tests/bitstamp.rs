use crypto_ws_client::{BitstampSpotWSClient, WSClient};

#[macro_use]
mod utils;

#[test]
fn bitstamp_spot() {
    gen_test!(
        BitstampSpotWSClient,
        &vec![
            "live_trades_btcusd".to_string(),
            "diff_order_book_btcusd".to_string()
        ]
    );
}
