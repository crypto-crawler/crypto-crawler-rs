#[macro_use]
mod utils;

#[cfg(test)]
mod bitz_spot {
    use crypto_ws_client::{BitzSpotWSClient, WSClient};
    use std::sync::{Arc, Mutex};
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn subscribe() {
        gen_test_code!(
            BitzSpotWSClient,
            subscribe,
            &vec![
                "market:btc_usdt".to_string(),
                "depth:btc_usdt".to_string(),
                "order:btc_usdt".to_string()
            ]
        );
    }

    #[test]
    fn subscribe_raw_json() {
        gen_test_code!(
            BitzSpotWSClient,
            subscribe,
            &vec![format!(
                r#"{{"action":"Topic.sub", "data":{{"symbol":"btc_usdt", "type":"market,depth,order", "_CDID":"100002", "dataType":"1"}}, "msg_id":{}}}"#,
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis()
            )]
        );
    }

    #[test]
    fn subscribe_trade() {
        gen_test_code!(
            BitzSpotWSClient,
            subscribe_trade,
            &vec!["btc_usdt".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook() {
        gen_test_code!(
            BitzSpotWSClient,
            subscribe_orderbook,
            &vec!["btc_usdt".to_string()]
        );
    }

    #[test]
    fn subscribe_ticker() {
        gen_test_code!(
            BitzSpotWSClient,
            subscribe_ticker,
            &vec!["btc_usdt".to_string()]
        );
    }

    #[test]
    fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(BitzSpotWSClient, &vec![("btc_usdt".to_string(), 60)]);
        gen_test_subscribe_candlestick!(BitzSpotWSClient, &vec![("btc_usdt".to_string(), 2592000)]);
    }
}
