mod utils;

#[cfg(test)]
mod trade {
    use crypto_msg_parser::{parse_trade, MarketType, TradeSide};

    #[test]
    fn spot() {
        let raw_msg = r#"[["T","329","1616384937","BTC_USDT","bid","57347.4","0.048800"]]"#;
        let trades = &parse_trade("zbg", MarketType::Spot, raw_msg).unwrap();

        assert_eq!(trades.len(), 1);
        let trade = &trades[0];

        crate::utils::check_trade_fields("zbg", MarketType::Spot, "BTC/USDT".to_string(), trade);

        assert_eq!(trade.volume, trade.price * trade.quantity);
        assert_eq!(trade.side, TradeSide::Buy);
    }

    #[test]
    fn linear_swap() {
        let raw_msg =
            r#"["future_tick",{"contractId":1000000,"trades":[1616385064674265,"57326","31",-1]}]"#;
        let trades = &parse_trade("zbg", MarketType::LinearSwap, raw_msg).unwrap();

        assert_eq!(trades.len(), 1);
        let trade = &trades[0];

        crate::utils::check_trade_fields(
            "zbg",
            MarketType::LinearSwap,
            "BTC/USDT".to_string(),
            trade,
        );

        assert_eq!(trade.volume, trade.price * trade.quantity);
        assert_eq!(trade.quantity, 0.01 * 31.0);
        assert_eq!(trade.side, TradeSide::Sell);
    }

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"["future_tick",{"contractId":1000001,"trades":[1616385036580662,"57370","188",-1]}]"#;
        let trades = &parse_trade("zbg", MarketType::InverseSwap, raw_msg).unwrap();

        assert_eq!(trades.len(), 1);
        let trade = &trades[0];

        crate::utils::check_trade_fields(
            "zbg",
            MarketType::InverseSwap,
            "BTC/USD".to_string(),
            trade,
        );

        assert_eq!(trade.volume, trade.price * trade.quantity);
        assert_eq!(trade.volume, 188.0);
        assert_eq!(trade.side, TradeSide::Sell);
    }
}
