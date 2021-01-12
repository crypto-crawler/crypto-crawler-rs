#[cfg(test)]
mod mxc_spot {
    use crypto_rest_client::MXCSpotRestClient;

    #[test]
    #[ignore]
    fn test_fetch_symbols() {
        let client = MXCSpotRestClient::new("your_access_key".to_string(), None);
        let symbols = client.fetch_symbols().unwrap();
        assert!(!symbols.is_empty());
    }

    #[test]
    #[ignore]
    fn test_trades() {
        let client = MXCSpotRestClient::new("your_access_key".to_string(), None);
        let text = client.fetch_trades("BTC_USDT").unwrap();
        assert!(text.starts_with("{"));
    }

    #[test]
    #[ignore]
    fn test_l2_snapshot() {
        let client = MXCSpotRestClient::new("your_access_key".to_string(), None);
        let text = client.fetch_l2_snapshot("BTC_USDT").unwrap();
        assert!(text.starts_with("{"));
    }
}

#[cfg(test)]
mod mxc_swap {
    use crypto_rest_client::MXCSwapRestClient;

    #[test]
    fn test_fetch_symbols() {
        let symbols = MXCSwapRestClient::fetch_symbols().unwrap();
        assert!(!symbols.is_empty());
        for symbol in symbols.iter() {
            assert!(symbol.ends_with("_USDT") || symbol.ends_with("_USD"));
        }
    }

    #[test]
    fn test_trades() {
        let text = MXCSwapRestClient::fetch_trades("BTC_USDT").unwrap();
        assert!(text.starts_with("{"));
    }

    #[test]
    fn test_l2_snapshot() {
        let text = MXCSwapRestClient::fetch_l2_snapshot("BTC_USDT").unwrap();
        assert!(text.starts_with("{"));
    }
}
