#[cfg(test)]
mod inverse_swap {
    use crypto_market_type::MarketType;
    use crypto_rest_client::{fetch_l2_snapshot, BinanceInverseRestClient};

    #[test]
    fn test_agg_trades() {
        let text =
            BinanceInverseRestClient::fetch_agg_trades("BTCUSD_PERP", None, None, None).unwrap();
        assert!(text.starts_with("[{"));
    }

    #[test]
    fn test_l2_snapshot() {
        let text = fetch_l2_snapshot("binance", MarketType::InverseSwap, "BTCUSD_PERP").unwrap();
        assert!(text.starts_with("{"));
    }
}

#[cfg(test)]
mod inverse_future {
    use crypto_market_type::MarketType;
    use crypto_rest_client::{fetch_l2_snapshot, BinanceInverseRestClient};

    #[test]
    fn test_agg_trades() {
        let text =
            BinanceInverseRestClient::fetch_agg_trades("BTCUSD_210924", None, None, None).unwrap();
        assert!(text.starts_with("[{"));
    }

    #[test]
    fn test_l2_snapshot() {
        let text =
            fetch_l2_snapshot("binance", MarketType::InverseFuture, "BTCUSD_210924").unwrap();
        assert!(text.starts_with("{"));
    }
}
