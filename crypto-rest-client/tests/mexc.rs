#[cfg(test)]
mod mexc_spot {
    use crypto_rest_client::MexcSpotRestClient;

    #[test]
    fn test_trades() {
        let text = MexcSpotRestClient::fetch_trades("BTC_USDT").unwrap();
        assert!(text.starts_with('{'));
    }

    #[test]
    fn test_l2_snapshot() {
        let text = MexcSpotRestClient::fetch_l2_snapshot("BTC_USDT").unwrap();
        assert!(text.starts_with('{'));
    }
}

#[cfg(test)]
mod mexc_swap {
    use crypto_market_type::MarketType;
    use crypto_rest_client::{fetch_l2_snapshot, MexcSwapRestClient};

    #[test]
    fn test_trades() {
        let text = MexcSwapRestClient::fetch_trades("BTC_USDT").unwrap();
        assert!(text.starts_with('{'));
    }

    #[test]
    fn test_l2_snapshot() {
        let text = fetch_l2_snapshot("mexc", MarketType::LinearSwap, "BTC_USDT", Some(3)).unwrap();
        assert!(text.starts_with('{'));
    }
}
