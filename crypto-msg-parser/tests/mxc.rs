mod utils;

#[cfg(test)]
mod trade {
    use crypto_msg_parser::{parse_trade, MarketType, TradeSide};
    use float_cmp::approx_eq;

    #[test]
    fn spot() {
        let raw_msg = r#"["push.symbol",{"symbol":"BTC_USDT","data":{"deals":[{"t":1616373554541,"p":"57005.89","q":"0.007811","T":1}]}}]"#;
        let trades = &parse_trade("mxc", MarketType::Spot, raw_msg).unwrap();

        assert_eq!(trades.len(), 1);
        let trade = &trades[0];

        crate::utils::check_trade_fields("mxc", MarketType::Spot, "BTC/USDT".to_string(), trade);

        assert_eq!(trade.quantity_base, 0.007811);
        assert_eq!(trade.side, TradeSide::Buy);
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"channel":"push.deal","data":{"M":1,"O":3,"T":2,"p":57602,"t":1616370338806,"v":14},"symbol":"BTC_USDT","ts":1616370338806}"#;
        let trades = &parse_trade("mxc", MarketType::LinearSwap, raw_msg).unwrap();

        assert_eq!(trades.len(), 1);
        let trade = &trades[0];

        crate::utils::check_trade_fields(
            "mxc",
            MarketType::LinearSwap,
            "BTC/USDT".to_string(),
            trade,
        );

        assert!(approx_eq!(
            f64,
            trade.quantity_base,
            0.0001 * 14.0,
            epsilon = 0.0000000001
        ));
        assert!(approx_eq!(
            f64,
            trade.quantity_quote,
            0.0001 * 14.0 * 57602.0,
            epsilon = 0.001
        ));
        assert_eq!(trade.quantity_contract, Some(14.0));
        assert_eq!(trade.side, TradeSide::Sell);
    }

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"channel":"push.deal","data":{"M":1,"O":3,"T":1,"p":57476.5,"t":1616370470356,"v":79},"symbol":"BTC_USD","ts":1616370470356}"#;
        let trades = &parse_trade("mxc", MarketType::InverseSwap, raw_msg).unwrap();

        assert_eq!(trades.len(), 1);
        let trade = &trades[0];

        crate::utils::check_trade_fields(
            "mxc",
            MarketType::InverseSwap,
            "BTC/USD".to_string(),
            trade,
        );

        assert!(approx_eq!(
            f64,
            trade.quantity_base,
            79.0 * 100.0 / 57476.5,
            epsilon = 0.0000000001
        ));
        assert_eq!(trade.quantity_quote, 79.0 * 100.0);
        assert_eq!(trade.quantity_contract, Some(79.0));
        assert_eq!(trade.side, TradeSide::Buy);
    }
}
