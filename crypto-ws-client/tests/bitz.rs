#[macro_use]
mod utils;

#[cfg(test)]
mod bitz_spot {
    use crypto_ws_client::{BitzSpotWSClient, WSClient};
    use std::time::{SystemTime, UNIX_EPOCH};

    #[tokio::test(flavor = "multi_thread")]
    #[ignore = "bitz.com has shutdown since October 2021"]
    async fn subscribe() {
        gen_test_code!(
            BitzSpotWSClient,
            subscribe,
            &[
                ("market".to_string(), "btc_usdt".to_string()),
                ("depth".to_string(), "btc_usdt".to_string()),
                ("order".to_string(), "btc_usdt".to_string())
            ]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    #[ignore = "bitz.com has shutdown since October 2021"]
    async fn subscribe_raw_json() {
        gen_test_code!(
            BitzSpotWSClient,
            send,
            &[format!(
                r#"{{"action":"Topic.sub", "data":{{"symbol":"btc_usdt", "type":"market,depth,order", "_CDID":"100002", "dataType":"1"}}, "msg_id":{}}}"#,
                SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()
            )]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    #[ignore = "bitz.com has shutdown since October 2021"]
    async fn subscribe_trade() {
        gen_test_code!(BitzSpotWSClient, subscribe_trade, &["btc_usdt".to_string()]);
    }

    #[tokio::test(flavor = "multi_thread")]
    #[ignore = "bitz.com has shutdown since October 2021"]
    async fn subscribe_orderbook() {
        gen_test_code!(BitzSpotWSClient, subscribe_orderbook, &["btc_usdt".to_string()]);
    }

    #[tokio::test(flavor = "multi_thread")]
    #[ignore = "bitz.com has shutdown since October 2021"]
    async fn subscribe_ticker() {
        gen_test_code!(BitzSpotWSClient, subscribe_ticker, &["btc_usdt".to_string()]);
    }

    #[tokio::test(flavor = "multi_thread")]
    #[ignore = "bitz.com has shutdown since October 2021"]
    async fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(BitzSpotWSClient, &[("btc_usdt".to_string(), 60)]);
        gen_test_subscribe_candlestick!(BitzSpotWSClient, &[("btc_usdt".to_string(), 2592000)]);
    }
}
