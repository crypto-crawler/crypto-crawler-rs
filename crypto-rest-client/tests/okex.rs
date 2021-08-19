use crypto_market_type::MarketType;
use crypto_rest_client::fetch_l2_snapshot;
use test_case::test_case;

#[test_case(MarketType::Spot, "BTC-USDT")]
#[test_case(MarketType::InverseFuture, "BTC-USD-210924")]
#[test_case(MarketType::LinearFuture, "BTC-USDT-210924")]
#[test_case(MarketType::InverseSwap, "BTC-USD-SWAP")]
#[test_case(MarketType::LinearSwap, "BTC-USDT-SWAP")]
#[test_case(MarketType::EuropeanOption, "BTC-USD-210924-18000-P")]
fn test_l2_snapshot(market_type: MarketType, symbol: &str) {
    let text = fetch_l2_snapshot("okex", market_type, symbol).unwrap();
    assert!(text.starts_with("{"));
}

#[cfg(test)]
mod okex_swap {
    use crypto_rest_client::OkexRestClient;

    #[test]
    fn test_trades() {
        let text = OkexRestClient::fetch_trades("BTC-USDT-SWAP").unwrap();

        assert!(text.starts_with("[{"));
    }

    #[test]
    fn test_option_underlying() {
        let arr = OkexRestClient::fetch_option_underlying().unwrap();
        assert!(!arr.is_empty());
    }
}
