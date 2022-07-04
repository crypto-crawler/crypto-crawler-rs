#[cfg(test)]
mod linear_swap {
    use crypto_market_type::MarketType;
    use crypto_rest_client::{fetch_l2_snapshot, fetch_open_interest, BinanceLinearRestClient};

    #[test]
    fn test_agg_trades() {
        let text = BinanceLinearRestClient::fetch_agg_trades("BTCUSDT", None, None, None).unwrap();
        assert!(text.starts_with("[{"));
    }

    #[test]
    fn test_l2_snapshot() {
        let text =
            fetch_l2_snapshot("binance", MarketType::LinearSwap, "BTCUSDT", Some(3)).unwrap();
        assert!(text.starts_with("{"));
    }

    #[test]
    fn test_open_interest() {
        let text = fetch_open_interest("binance", MarketType::LinearSwap, Some("BTCUSDT")).unwrap();
        assert!(text.starts_with("{"));
    }
}

#[cfg(test)]
mod linear_future {
    use crypto_market_type::MarketType;
    use crypto_rest_client::{fetch_l2_snapshot, fetch_open_interest, BinanceLinearRestClient};

    #[test]
    fn test_agg_trades() {
        let text =
            BinanceLinearRestClient::fetch_agg_trades("BTCUSDT_220930", None, None, None).unwrap();
        assert!(text.starts_with("[{"));
    }

    #[test]
    fn test_l2_snapshot() {
        let text = fetch_l2_snapshot(
            "binance",
            MarketType::LinearFuture,
            "BTCUSDT_220930",
            Some(3),
        )
        .unwrap();
        assert!(text.starts_with("{"));
    }

    #[test]
    fn test_open_interest() {
        let text = fetch_open_interest("binance", MarketType::LinearFuture, Some("BTCUSDT_220930"))
            .unwrap();
        assert!(text.starts_with("{"));
    }
}
