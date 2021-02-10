#[cfg(test)]
mod okex_swap {
    use crypto_rest_client::OkexRestClient;

    #[test]
    fn test_trades() {
        let text = OkexRestClient::fetch_trades("BTC-USDT-SWAP").unwrap();

        assert!(text.starts_with("[{"));
    }

    #[test]
    fn test_l2_snapshot() {
        let text = OkexRestClient::fetch_l2_snapshot("BTC-USDT-SWAP").unwrap();
        assert!(text.starts_with("{"));
    }

    #[test]
    fn test_option_underlying() {
        let arr = OkexRestClient::fetch_option_underlying().unwrap();
        assert!(!arr.is_empty());
    }
}
