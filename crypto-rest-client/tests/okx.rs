use crypto_market_type::MarketType;
use crypto_rest_client::{fetch_l2_snapshot, fetch_open_interest};
use serde_json::Value;
use std::collections::HashMap;
use test_case::test_case;

#[test_case(MarketType::Spot, "BTC-USDT")]
#[test_case(MarketType::InverseFuture, "BTC-USD-220930")]
#[test_case(MarketType::LinearFuture, "BTC-USDT-220930")]
#[test_case(MarketType::InverseSwap, "BTC-USD-SWAP")]
#[test_case(MarketType::LinearSwap, "BTC-USDT-SWAP")]
#[test_case(MarketType::EuropeanOption, "BTC-USD-220930-10000-P")]
fn test_l2_snapshot(market_type: MarketType, symbol: &str) {
    let text = fetch_l2_snapshot("okx", market_type, symbol, Some(3)).unwrap();
    assert!(text.starts_with("{"));
}

#[test_case(MarketType::InverseFuture, "BTC-USD-220930")]
#[test_case(MarketType::LinearFuture, "BTC-USDT-220930")]
#[test_case(MarketType::InverseSwap, "BTC-USD-SWAP")]
#[test_case(MarketType::LinearSwap, "BTC-USDT-SWAP")]
#[test_case(MarketType::EuropeanOption, "BTC-USD-220930-10000-P")]
fn test_open_interest(market_type: MarketType, symbol: &str) {
    let text = fetch_open_interest("okx", market_type, Some(symbol)).unwrap();
    let json_obj = serde_json::from_str::<HashMap<String, Value>>(&text).unwrap();
    let arr = json_obj.get("data").unwrap().as_array().unwrap();
    assert!(arr.len() > 0);
}

#[cfg(test)]
mod okex_swap {
    use std::collections::HashMap;

    use crypto_rest_client::OkxRestClient;
    use serde_json::Value;

    #[test]
    fn test_trades() {
        let text = OkxRestClient::fetch_trades("BTC-USDT-SWAP").unwrap();
        let json_obj = serde_json::from_str::<HashMap<String, Value>>(&text).unwrap();
        let code = json_obj.get("code").unwrap().as_str().unwrap();
        assert_eq!("0", code);
    }

    #[test]
    fn test_option_underlying() {
        let arr = OkxRestClient::fetch_option_underlying().unwrap();
        assert!(!arr.is_empty());
    }
}
