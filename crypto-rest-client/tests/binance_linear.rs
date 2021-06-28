#[cfg(test)]
mod linear_swap {
    use crypto_market_type::MarketType;
    use crypto_rest_client::{fetch_l2_snapshot, BinanceLinearRestClient};

    #[test]
    fn test_agg_trades() {
        let text = BinanceLinearRestClient::fetch_agg_trades("BTCUSDT", None, None, None).unwrap();
        assert!(text.starts_with("[{"));
    }

    #[test]
    fn test_l2_snapshot() {
        let text = fetch_l2_snapshot("binance", MarketType::LinearSwap, "BTCUSDT").unwrap();
        assert!(text.starts_with("{"));
    }
}

#[cfg(test)]
mod linear_future {
    use crypto_market_type::MarketType;
    use crypto_rest_client::{fetch_l2_snapshot, BinanceLinearRestClient};

    #[test]
    fn test_agg_trades() {
        let text =
            BinanceLinearRestClient::fetch_agg_trades("BTCUSDT_210924", None, None, None).unwrap();
        assert!(text.starts_with("[{"));
    }

    #[test]
    fn test_l2_snapshot() {
        let text = fetch_l2_snapshot("binance", MarketType::LinearSwap, "BTCUSDT_210924").unwrap();
        assert!(text.starts_with("{"));
    }
}
