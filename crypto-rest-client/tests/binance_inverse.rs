#[cfg(test)]
mod inverse_swap {
    use crypto_market_type::MarketType;
    use crypto_rest_client::{fetch_l2_snapshot, fetch_open_interest, BinanceInverseRestClient};

    #[test]
    fn test_agg_trades() {
        let text =
            BinanceInverseRestClient::fetch_agg_trades("BTCUSD_PERP", None, None, None).unwrap();
        assert!(text.starts_with("[{"));
    }

    #[test]
    fn test_l2_snapshot() {
        let text =
            fetch_l2_snapshot("binance", MarketType::InverseSwap, "BTCUSD_PERP", Some(3)).unwrap();
        assert!(text.starts_with("{"));
    }

    #[test]
    fn test_open_interest() {
        let text =
            fetch_open_interest("binance", MarketType::InverseSwap, Some("BTCUSD_PERP")).unwrap();
        assert!(text.starts_with("{"));
    }
}

#[cfg(test)]
mod inverse_future {
    use crypto_market_type::MarketType;
    use crypto_rest_client::{fetch_l2_snapshot, fetch_open_interest, BinanceInverseRestClient};

    #[test]
    fn test_agg_trades() {
        let text =
            BinanceInverseRestClient::fetch_agg_trades("BTCUSD_220930", None, None, None).unwrap();
        assert!(text.starts_with("[{"));
    }

    #[test]
    fn test_l2_snapshot() {
        let text = fetch_l2_snapshot(
            "binance",
            MarketType::InverseFuture,
            "BTCUSD_220930",
            Some(3),
        )
        .unwrap();
        assert!(text.starts_with("{"));
    }

    #[test]
    fn test_open_interest() {
        let text = fetch_open_interest("binance", MarketType::InverseFuture, Some("BTCUSD_220930"))
            .unwrap();
        assert!(text.starts_with("{"));
    }
}
