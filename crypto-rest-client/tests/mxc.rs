#[cfg(test)]
mod mxc_spot {
    use crypto_rest_client::MxcSpotRestClient;

    #[test]
    fn test_trades() {
        let text = MxcSpotRestClient::fetch_trades("BTC_USDT").unwrap();
        assert!(text.starts_with("{"));
    }

    #[test]
    fn test_l2_snapshot() {
        let text = MxcSpotRestClient::fetch_l2_snapshot("BTC_USDT").unwrap();
        assert!(text.starts_with("{"));
    }
}

#[cfg(test)]
mod mxc_swap {
    use crypto_market_type::MarketType;
    use crypto_rest_client::{fetch_l2_snapshot, MxcSwapRestClient};

    #[test]
    fn test_trades() {
        let text = MxcSwapRestClient::fetch_trades("BTC_USDT").unwrap();
        assert!(text.starts_with("{"));
    }

    #[test]
    fn test_l2_snapshot() {
        let text = fetch_l2_snapshot("mxc", MarketType::LinearSwap, "BTC_USDT", Some(3)).unwrap();
        assert!(text.starts_with("{"));
    }
}
