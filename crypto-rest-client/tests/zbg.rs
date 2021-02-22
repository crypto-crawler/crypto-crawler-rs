#[cfg(test)]
mod zbg_spot {
    use std::collections::HashMap;

    use crypto_rest_client::ZbgSpotRestClient;
    use serde_json::Value;

    #[test]
    fn test_l2_snapshot() {
        let text = ZbgSpotRestClient::fetch_l2_snapshot("btc_usdt").unwrap();
        let obj = serde_json::from_str::<HashMap<String, Value>>(&text).unwrap();

        assert_eq!(
            obj.get("resMsg")
                .unwrap()
                .as_object()
                .unwrap()
                .get("code")
                .unwrap()
                .as_str()
                .unwrap(),
            "1"
        );

        let data = obj.get("datas").unwrap().as_object().unwrap();
        assert!(data.get("asks").unwrap().as_array().unwrap().len() > 0);
        assert!(data.get("bids").unwrap().as_array().unwrap().len() > 0);
    }
}

#[cfg(test)]
mod zbg_swap {
    use std::collections::HashMap;

    use crypto_rest_client::ZbgSwapRestClient;
    use serde_json::Value;

    use test_case::test_case;

    #[test_case("BTC_USD-R")]
    #[test_case("BTC_USDT")]
    fn test_l2_snapshot(symbol: &str) {
        let text = ZbgSwapRestClient::fetch_l2_snapshot(symbol).unwrap();
        let obj = serde_json::from_str::<HashMap<String, Value>>(&text).unwrap();

        assert_eq!(
            obj.get("resMsg")
                .unwrap()
                .as_object()
                .unwrap()
                .get("code")
                .unwrap()
                .as_str()
                .unwrap(),
            "1"
        );

        let data = obj.get("datas").unwrap().as_object().unwrap();
        assert!(data.get("asks").unwrap().as_array().unwrap().len() > 0);
        assert!(data.get("bids").unwrap().as_array().unwrap().len() > 0);
    }
}
