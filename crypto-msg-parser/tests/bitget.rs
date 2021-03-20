mod utils;

#[cfg(test)]
mod trade {
    use crypto_msg_parser::{parse_trade, MarketType, TradeSide};
    use float_cmp::approx_eq;

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"data":[{"instrument_id":"btcusd","price":"58722.0","side":"sell","size":"158","timestamp":"1616236107276"},{"instrument_id":"btcusd","price":"58722.0","side":"sell","size":"450","timestamp":"1616236107276"},{"instrument_id":"btcusd","price":"58722.0","side":"sell","size":"762","timestamp":"1616236107276"}],"table":"swap/trade"}"#;
        let trades = &parse_trade("bitget", MarketType::InverseSwap, raw_msg).unwrap();

        assert_eq!(trades.len(), 3);

        for trade in trades.iter() {
            crate::utils::check_trade_fields(
                "bitget",
                MarketType::InverseSwap,
                "BTC/USD".to_string(),
                trade,
            );
            assert_eq!(trade.volume, trade.price * trade.quantity);
            assert_eq!(trade.side, TradeSide::Sell);
        }
        assert_eq!(trades[0].volume, 158.0);
        assert_eq!(trades[1].volume, 450.0);
        assert_eq!(trades[2].volume, 762.0);
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"data":[{"instrument_id":"cmt_btcusdt","price":"58784.0","side":"sell","size":"1265","timestamp":"1616236212569"},{"instrument_id":"cmt_btcusdt","price":"58784.0","side":"sell","size":"25","timestamp":"1616236212569"},{"instrument_id":"cmt_btcusdt","price":"58784.0","side":"sell","size":"181","timestamp":"1616236212569"}],"table":"swap/trade"}"#;
        let trades = &parse_trade("bitget", MarketType::LinearSwap, raw_msg).unwrap();

        assert_eq!(trades.len(), 3);

        for trade in trades.iter() {
            crate::utils::check_trade_fields(
                "bitget",
                MarketType::LinearSwap,
                "BTC/USDT".to_string(),
                trade,
            );
            assert_eq!(trade.volume, trade.price * trade.quantity);

            assert_eq!(trade.side, TradeSide::Sell);
        }
        assert!(approx_eq!(f64, trades[0].quantity, 1.265, ulps = 9));
        assert!(approx_eq!(f64, trades[1].quantity, 0.025, ulps = 9));
        assert!(approx_eq!(f64, trades[2].quantity, 0.181, ulps = 9));
    }
}
