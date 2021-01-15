#[cfg(test)]
mod mxc_spot {
    use crypto_rest_client::MxcSpotRestClient;

    #[test]
    #[ignore]
    fn test_trades() {
        let client = MxcSpotRestClient::new("your_access_key".to_string(), None);
        let text = client.fetch_trades("BTC_USDT").unwrap();
        assert!(text.starts_with("{"));
    }

    #[test]
    #[ignore]
    fn test_l2_snapshot() {
        let client = MxcSpotRestClient::new("your_access_key".to_string(), None);
        let text = client.fetch_l2_snapshot("BTC_USDT").unwrap();
        assert!(text.starts_with("{"));
    }
}

#[cfg(test)]
mod mxc_swap {
    use crypto_rest_client::MxcSwapRestClient;

    #[test]
    fn test_trades() {
        let text = MxcSwapRestClient::fetch_trades("BTC_USDT").unwrap();
        assert!(text.starts_with("{"));
    }

    #[test]
    fn test_l2_snapshot() {
        let text = MxcSwapRestClient::fetch_l2_snapshot("BTC_USDT").unwrap();
        assert!(text.starts_with("{"));
    }
}
