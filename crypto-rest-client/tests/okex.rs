#[cfg(test)]
mod okex_swap {
    use crypto_rest_client::OKExRestClient;

    #[test]
    fn test_trades() {
        let text = OKExRestClient::fetch_trades("BTC-USDT-SWAP").unwrap();

        assert!(text.starts_with("[{"));
    }

    #[test]
    fn test_l2_snapshot() {
        let text = OKExRestClient::fetch_l2_snapshot("BTC-USDT-SWAP").unwrap();
        assert!(text.starts_with("{"));
    }
}
