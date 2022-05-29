mod utils;

const EXCHANGE_NAME: &str = "zb";

#[cfg(test)]
mod trade {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, extract_timestamp};

    #[test]
    fn spot() {
        let raw_msg = r#"{"data":[{"date":1653774784,"amount":"0.0380","price":"29029.5","trade_type":"ask","type":"sell","tid":2796890056},{"date":1653774785,"amount":"0.0001","price":"29041.49","trade_type":"bid","type":"buy","tid":2796890057}],"dataType":"trades","channel":"btcusdt_trades"}"#;

        assert_eq!(
            "btcusdt",
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );

        assert_eq!(
            1653774785000,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"channel":"BTC_USDT.Trade","data":[[29011.85,0.441,1,1653774742]]}"#;

        assert_eq!(
            "BTC_USDT",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );

        assert_eq!(
            1653774742000,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
    }
}

#[cfg(test)]
mod l2_event {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, extract_timestamp};

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"channel":"BTC_USDT.Depth","data":{"bids":[[4.98,0.007],[6.83,24.404]],"asks":[[29008.68,0.06],[29008.69,0.08]],"time":"1653782509566"}}"#;

        assert_eq!(
            "BTC_USDT",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );

        assert_eq!(
            1653782509566,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
    }
}

#[cfg(test)]
mod l2_topk {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, extract_timestamp};

    #[test]
    fn spot() {
        let raw_msg = r#"{"asks":[[29400.0,0.0001]],"dataType":"depth","bids":[[29070.0,0.0001]],"channel":"btcusdt_depth","timestamp":1653780136}"#;

        assert_eq!(
            "btcusdt",
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );

        assert_eq!(
            1653780136000,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"channel":"BTC_USDT.DepthWhole","data":{"asks":[[29001.28,0.106],[29001.3,0.05]],"bids":[[28998.66,0.02],[28993.83,0.02]],"time":"1653782876953"}}"#;

        assert_eq!(
            "BTC_USDT",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );

        assert_eq!(
            1653782876953,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
    }
}

#[cfg(test)]
mod ticker {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, extract_timestamp};

    #[test]
    fn spot() {
        let raw_msg = r#"{"date":"1653781987181","ticker":{"high":"29249.63","vol":"4499.6492","last":"29046.17","low":"28527.54","buy":"29039.24","sell":"29056.69","turnover":"129792765.9200","open":"28598.8","riseRate":"1.57"},"dataType":"ticker","channel":"btcusdt_ticker"}"#;

        assert_eq!(
            "btcusdt",
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );

        assert_eq!(
            1653781987181,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"channel":"BTC_USDT.Ticker","data":[28669.4,29244.73,27980,29012.96,24264.005,1.2,1653783012,257344.9552]}"#;

        assert_eq!(
            "BTC_USDT",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );

        assert_eq!(
            1653783012000,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
    }

    #[test]
    fn linear_swap_all_ticker() {
        let raw_msg = r#"{"channel": "All.Ticker","data": {"ETH_USDT": [1739.34, 1807.79, 1721.41, 1790.14, 238051.871, 2.92, 1653783366, 15871.560254],"BTC_USDT": [28735.84, 29244.73, 27980, 28988.05, 24123.201, 0.88, 1653783365, 257010.950105]}}"#;

        assert_eq!(
            "All",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );

        assert_eq!(
            1653783366000,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
    }
}

#[cfg(test)]
mod candlestick {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, extract_timestamp};

    #[test]
    fn spot() {
        let raw_msg = r#"{"datas":{"data":[[1653782100000,29055.22,29055.22,29030.81,29032.9,19.3130],[1653782160000,29036.33,29036.33,29036.33,29036.33,0.0001]]},"channel":"btcusdt_kline_1min","isSuc":true}"#;

        assert_eq!(
            "btcusdt",
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );

        assert_eq!(
            1653782160000,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"channel":"BTC_USDT.KLine_1M","type":"Whole","data":[[28993.54,28996.39,28992.58,28994.78,0.921,1653783840]]}"#;

        assert_eq!(
            "BTC_USDT",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );

        assert_eq!(
            1653783840000,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
    }
}
