use crypto_market_type::MarketType;
use crypto_rest_client::{fetch_l2_snapshot, fetch_open_interest};
use serde_json::Value;
use std::collections::HashMap;
use test_case::test_case;

#[test_case(MarketType::Spot, "btcusdt")]
#[test_case(MarketType::InverseFuture, "BTC_CQ")]
#[test_case(MarketType::InverseSwap, "BTC-USD")]
#[test_case(MarketType::LinearSwap, "BTC-USDT")]
#[test_case(MarketType::EuropeanOption, "BTC-USDT-210625-P-27000"; "inconclusive")]
fn test_l2_snapshot(market_type: MarketType, symbol: &str) {
    let text = fetch_l2_snapshot("huobi", market_type, symbol, Some(3)).unwrap();
    assert!(text.starts_with('{'));

    let obj = serde_json::from_str::<HashMap<String, Value>>(&text).unwrap();
    assert_eq!("ok", obj.get("status").unwrap().as_str().unwrap());

    let bids = obj
        .get("tick")
        .unwrap()
        .as_object()
        .unwrap()
        .get("bids")
        .unwrap()
        .as_array()
        .unwrap();
    assert!(!bids.is_empty());

    let asks = obj
        .get("tick")
        .unwrap()
        .as_object()
        .unwrap()
        .get("asks")
        .unwrap()
        .as_array()
        .unwrap();
    assert!(!asks.is_empty());
}

#[test_case(MarketType::InverseFuture)]
#[test_case(MarketType::InverseSwap)]
#[test_case(MarketType::LinearSwap)]
fn test_open_interest(market_type: MarketType) {
    let text = fetch_open_interest("huobi", market_type, None).unwrap();
    let obj = serde_json::from_str::<HashMap<String, Value>>(&text).unwrap();
    let arr = obj.get("data").unwrap().as_array().unwrap();
    assert!(!arr.is_empty());
}

#[cfg(test)]
mod huobi_spot {
    use crypto_rest_client::HuobiSpotRestClient;

    #[test]
    fn test_trades() {
        let text = HuobiSpotRestClient::fetch_trades("btcusdt").unwrap();
        assert!(text.starts_with('{'));
    }
}

#[cfg(test)]
mod huobi_future {
    use crypto_rest_client::HuobiFutureRestClient;

    #[test]
    fn test_trades() {
        let text = HuobiFutureRestClient::fetch_trades("BTC_CQ").unwrap();
        assert!(text.starts_with('{'));
    }
}

#[cfg(test)]
mod huobi_linear_swap {
    use crypto_rest_client::HuobiLinearSwapRestClient;

    #[test]
    fn test_trades() {
        let text = HuobiLinearSwapRestClient::fetch_trades("BTC-USDT").unwrap();
        assert!(text.starts_with('{'));
    }
}

#[cfg(test)]
mod huobi_inverse_swap {
    use crypto_rest_client::HuobiInverseSwapRestClient;

    #[test]
    fn test_trades() {
        let text = HuobiInverseSwapRestClient::fetch_trades("BTC-USD").unwrap();
        assert!(text.starts_with('{'));
    }
}

#[cfg(test)]
mod huobi_option {
    use crypto_rest_client::HuobiOptionRestClient;

    #[test]
    #[ignore]
    fn test_trades() {
        let text = HuobiOptionRestClient::fetch_trades("BTC-USDT-210625-P-27000").unwrap();
        assert!(text.starts_with('{'));
    }
}
