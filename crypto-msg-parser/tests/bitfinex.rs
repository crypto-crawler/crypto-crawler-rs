mod utils;

#[cfg(test)]
mod trade {
    use crypto_msg_parser::{parse_trade, MarketType, TradeSide};

    #[test]
    fn spot_te() {
        let raw_msg = r#"[{"symbol":"tBTCUST","channel":"trades"},"te",[637771130,1615232733897,0.11546588,51350]]"#;
        let trade = &parse_trade("bitfinex", MarketType::Spot, raw_msg).unwrap()[0];

        crate::utils::check_trade_fields(
            "bitfinex",
            MarketType::Spot,
            "BTC/USDT".to_string(),
            trade,
        );

        assert_eq!(trade.side, TradeSide::Buy);
    }

    #[test]
    fn spot_tu() {
        let raw_msg = r#"[{"symbol":"tBTCUST","channel":"trades"},"tu",[637771130,1615232733897,0.11546588,51350]]"#;
        let trade = &parse_trade("bitfinex", MarketType::Spot, raw_msg).unwrap()[0];

        crate::utils::check_trade_fields(
            "bitfinex",
            MarketType::Spot,
            "BTC/USDT".to_string(),
            trade,
        );

        assert_eq!(trade.side, TradeSide::Buy);
    }

    #[test]
    fn spot_snapshot() {
        let raw_msg = r#"[{"channel":"trades","symbol":"tBTCUST"},[[647229117,1616217509543,0.0033,58239],[647229114,1616217326462,0.05605347,58296],[647229113,1616217326462,0.00102018,58296]]]"#;
        let trades = &parse_trade("bitfinex", MarketType::Spot, raw_msg).unwrap();

        assert_eq!(trades.len(), 3);

        for trade in trades.iter() {
            crate::utils::check_trade_fields(
                "bitfinex",
                MarketType::Spot,
                "BTC/USDT".to_string(),
                trade,
            );
        }
    }

    #[test]
    fn swap_te() {
        let raw_msg = r#"[{"channel":"trades","symbol":"tBTCF0:USTF0"},"te",[647256282,1616219711336,0.00020449,58244]]"#;
        let trade = &parse_trade("bitfinex", MarketType::LinearSwap, raw_msg).unwrap()[0];

        crate::utils::check_trade_fields(
            "bitfinex",
            MarketType::LinearSwap,
            "BTC/USDT".to_string(),
            trade,
        );

        assert_eq!(trade.side, TradeSide::Buy);
    }

    #[test]
    fn swap_tu() {
        let raw_msg = r#"[{"channel":"trades","symbol":"tBTCF0:USTF0"},"tu",[647256282,1616219711336,0.00020449,58244]]"#;
        let trade = &parse_trade("bitfinex", MarketType::LinearSwap, raw_msg).unwrap()[0];

        crate::utils::check_trade_fields(
            "bitfinex",
            MarketType::LinearSwap,
            "BTC/USDT".to_string(),
            trade,
        );

        assert_eq!(trade.side, TradeSide::Buy);
    }

    #[test]
    fn swap_snapshot() {
        let raw_msg = r#"[{"channel":"trades","symbol":"tBTCF0:USTF0"},[[647256201,1616219105954,-0.06153795,58119],[647256191,1616219094921,0.0257,58138],[647256188,1616219088734,0.01679516,58138]]]"#;
        let trades = &parse_trade("bitfinex", MarketType::LinearSwap, raw_msg).unwrap();

        assert_eq!(trades.len(), 3);

        for trade in trades.iter() {
            crate::utils::check_trade_fields(
                "bitfinex",
                MarketType::LinearSwap,
                "BTC/USDT".to_string(),
                trade,
            );
        }
    }
}
