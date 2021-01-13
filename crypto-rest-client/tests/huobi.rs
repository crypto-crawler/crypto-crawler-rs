#[cfg(test)]
mod huobi_spot {
    use crypto_rest_client::HuobiSpotRestClient;
    use serde_json::Value;
    use std::collections::HashMap;

    #[test]
    fn test_fetch_symbols() {
        let symbols = HuobiSpotRestClient::fetch_symbols().unwrap();
        assert!(!symbols.is_empty());
    }

    #[test]
    fn test_trades() {
        let text = HuobiSpotRestClient::fetch_trades("btcusdt").unwrap();
        assert!(text.starts_with("{"));
    }

    #[test]
    fn test_l2_snapshot() {
        let text = HuobiSpotRestClient::fetch_l2_snapshot("btcusdt").unwrap();
        assert!(text.starts_with("{"));

        let obj = serde_json::from_str::<HashMap<String, Value>>(&text).unwrap();
        assert_eq!("ok", obj.get("status").unwrap().as_str().unwrap());

        let bids = obj
            .get("tick")
            .unwrap()
            .as_object()
            .unwrap()
            .get("bids")
            .unwrap()
            .as_array()
            .unwrap();
        assert_eq!(150, bids.len());
        let asks = obj
            .get("tick")
            .unwrap()
            .as_object()
            .unwrap()
            .get("asks")
            .unwrap()
            .as_array()
            .unwrap();
        assert_eq!(150, asks.len());
    }
}

#[cfg(test)]
mod huobi_future {
    use crypto_rest_client::HuobiFutureRestClient;
    use serde_json::Value;
    use std::collections::HashMap;

    #[test]
    fn test_fetch_symbols() {
        let symbols = HuobiFutureRestClient::fetch_symbols().unwrap();
        assert!(!symbols.is_empty());
        for symbol in symbols.iter() {
            assert!(
                symbol.ends_with("_CW")
                    || symbol.ends_with("_NW")
                    || symbol.ends_with("_CQ")
                    || symbol.ends_with("_NQ")
            );
        }
    }

    #[test]
    fn test_trades() {
        let text = HuobiFutureRestClient::fetch_trades("BTC_CQ").unwrap();
        assert!(text.starts_with("{"));
    }

    #[test]
    fn test_l2_snapshot() {
        let text = HuobiFutureRestClient::fetch_l2_snapshot("BTC_CQ").unwrap();
        assert!(text.starts_with("{"));

        let obj = serde_json::from_str::<HashMap<String, Value>>(&text).unwrap();
        assert_eq!("ok", obj.get("status").unwrap().as_str().unwrap());

        let bids = obj
            .get("tick")
            .unwrap()
            .as_object()
            .unwrap()
            .get("bids")
            .unwrap()
            .as_array()
            .unwrap();
        assert_eq!(150, bids.len());
        let asks = obj
            .get("tick")
            .unwrap()
            .as_object()
            .unwrap()
            .get("asks")
            .unwrap()
            .as_array()
            .unwrap();
        assert_eq!(150, asks.len());
    }
}

#[cfg(test)]
mod huobi_linear_swap {
    use crypto_rest_client::HuobiLinearSwapRestClient;
    use serde_json::Value;
    use std::collections::HashMap;

    #[test]
    fn test_fetch_symbols() {
        let symbols = HuobiLinearSwapRestClient::fetch_symbols().unwrap();
        assert!(!symbols.is_empty());
        for symbol in symbols.iter() {
            assert!(symbol.ends_with("-USDT"));
        }
    }

    #[test]
    fn test_trades() {
        let text = HuobiLinearSwapRestClient::fetch_trades("BTC-USDT").unwrap();
        assert!(text.starts_with("{"));
    }

    #[test]
    fn test_l2_snapshot() {
        let text = HuobiLinearSwapRestClient::fetch_l2_snapshot("BTC-USDT").unwrap();
        assert!(text.starts_with("{"));

        let obj = serde_json::from_str::<HashMap<String, Value>>(&text).unwrap();
        assert_eq!("ok", obj.get("status").unwrap().as_str().unwrap());

        let bids = obj
            .get("tick")
            .unwrap()
            .as_object()
            .unwrap()
            .get("bids")
            .unwrap()
            .as_array()
            .unwrap();
        assert_eq!(150, bids.len());
        let asks = obj
            .get("tick")
            .unwrap()
            .as_object()
            .unwrap()
            .get("asks")
            .unwrap()
            .as_array()
            .unwrap();
        assert_eq!(150, asks.len());
    }
}

#[cfg(test)]
mod huobi_inverse_swap {
    use crypto_rest_client::HuobiInverseSwapRestClient;
    use serde_json::Value;
    use std::collections::HashMap;

    #[test]
    fn test_fetch_symbols() {
        let symbols = HuobiInverseSwapRestClient::fetch_symbols().unwrap();
        assert!(!symbols.is_empty());
        for symbol in symbols.iter() {
            assert!(symbol.ends_with("-USD"));
        }
    }

    #[test]
    fn test_trades() {
        let text = HuobiInverseSwapRestClient::fetch_trades("BTC-USD").unwrap();
        assert!(text.starts_with("{"));
    }

    #[test]
    fn test_l2_snapshot() {
        let text = HuobiInverseSwapRestClient::fetch_l2_snapshot("BTC-USD").unwrap();
        assert!(text.starts_with("{"));

        let obj = serde_json::from_str::<HashMap<String, Value>>(&text).unwrap();
        assert_eq!("ok", obj.get("status").unwrap().as_str().unwrap());

        let bids = obj
            .get("tick")
            .unwrap()
            .as_object()
            .unwrap()
            .get("bids")
            .unwrap()
            .as_array()
            .unwrap();
        assert_eq!(150, bids.len());
        let asks = obj
            .get("tick")
            .unwrap()
            .as_object()
            .unwrap()
            .get("asks")
            .unwrap()
            .as_array()
            .unwrap();
        assert_eq!(150, asks.len());
    }
}

#[cfg(test)]
mod huobi_option {
    use crypto_rest_client::HuobiOptionRestClient;
    use serde_json::Value;
    use std::collections::HashMap;

    #[test]
    fn test_fetch_symbols() {
        let symbols = HuobiOptionRestClient::fetch_symbols().unwrap();
        assert!(!symbols.is_empty());
        for symbol in symbols.iter() {
            assert!(symbol.contains("-C-") || symbol.contains("-P-"));
        }
    }

    #[test]
    fn test_trades() {
        let text = HuobiOptionRestClient::fetch_trades("BTC-USDT-210326-C-32000").unwrap();
        assert!(text.starts_with("{"));
    }

    #[test]
    fn test_l2_snapshot() {
        let text = HuobiOptionRestClient::fetch_l2_snapshot("BTC-USDT-210326-C-32000").unwrap();
        assert!(text.starts_with("{"));

        let obj = serde_json::from_str::<HashMap<String, Value>>(&text).unwrap();
        assert_eq!("ok", obj.get("status").unwrap().as_str().unwrap());

        let bids = obj
            .get("tick")
            .unwrap()
            .as_object()
            .unwrap()
            .get("bids")
            .unwrap()
            .as_array()
            .unwrap();
        assert!(!bids.is_empty());
        let asks = obj
            .get("tick")
            .unwrap()
            .as_object()
            .unwrap()
            .get("asks")
            .unwrap()
            .as_array()
            .unwrap();
        assert!(!asks.is_empty());
    }
}
