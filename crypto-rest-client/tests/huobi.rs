#[cfg(test)]
mod huobi_spot {
    use crypto_rest_client::HuobiSpotRestClient;

    #[test]
    fn test_trades() {
        let text = HuobiSpotRestClient::fetch_trades("btcusdt").unwrap();
        assert!(text.starts_with("{"));
    }

    #[test]
    fn test_l2_snapshot() {
        let text = HuobiSpotRestClient::fetch_l2_snapshot("btcusdt", 0).unwrap();
        assert!(text.starts_with("{"));
    }
}

#[cfg(test)]
mod huobi_future {
    use crypto_rest_client::HuobiFutureRestClient;

    #[test]
    fn test_trades() {
        let text = HuobiFutureRestClient::fetch_trades("BTC_CQ").unwrap();
        assert!(text.starts_with("{"));
    }

    #[test]
    fn test_l2_snapshot() {
        let text = HuobiFutureRestClient::fetch_l2_snapshot("BTC_CQ", 0).unwrap();
        assert!(text.starts_with("{"));
    }
}

#[cfg(test)]
mod huobi_linear_swap {
    use crypto_rest_client::HuobiLinearSwapRestClient;

    #[test]
    fn test_trades() {
        let text = HuobiLinearSwapRestClient::fetch_trades("BTC-USDT").unwrap();
        assert!(text.starts_with("{"));
    }

    #[test]
    fn test_l2_snapshot() {
        let text = HuobiLinearSwapRestClient::fetch_l2_snapshot("BTC-USDT", 0).unwrap();
        assert!(text.starts_with("{"));
    }
}

#[cfg(test)]
mod huobi_inverse_swap {
    use crypto_rest_client::HuobiInverseSwapRestClient;

    #[test]
    fn test_trades() {
        let text = HuobiInverseSwapRestClient::fetch_trades("BTC-USD").unwrap();
        assert!(text.starts_with("{"));
    }

    #[test]
    fn test_l2_snapshot() {
        let text = HuobiInverseSwapRestClient::fetch_l2_snapshot("BTC-USD", 0).unwrap();
        assert!(text.starts_with("{"));
    }
}

#[cfg(test)]
mod huobi_option {
    use crypto_rest_client::HuobiOptionRestClient;

    #[test]
    fn test_trades() {
        let text = HuobiOptionRestClient::fetch_trades("BTC-USDT-210326-C-32000").unwrap();
        assert!(text.starts_with("{"));
    }

    #[test]
    fn test_l2_snapshot() {
        let text = HuobiOptionRestClient::fetch_l2_snapshot("BTC-USDT-210326-C-32000", 0).unwrap();
        assert!(text.starts_with("{"));
    }
}
