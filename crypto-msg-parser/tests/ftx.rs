mod utils;

#[cfg(test)]
mod trade {
    use crypto_msg_parser::{parse_trade, MarketType, TradeSide};

    #[test]
    fn spot() {
        let raw_msg = r#"{"channel": "trades", "market": "BTC/USD", "type": "update", "data": [{"id": 632052557, "price": 56335.0, "size": 0.0444, "side": "buy", "liquidation": false, "time": "2021-03-21T10:24:37.319680+00:00"}]}"#;
        let trades = &parse_trade("ftx", MarketType::Spot, raw_msg).unwrap();

        assert_eq!(trades.len(), 1);

        for trade in trades.iter() {
            crate::utils::check_trade_fields("ftx", MarketType::Spot, "BTC/USD".to_string(), trade);

            assert_eq!(trade.volume, trade.price * trade.quantity);
            assert_eq!(trade.side, TradeSide::Buy);
        }
    }

    #[test]
    fn linear_futre() {
        let raw_msg = r#"{"channel": "trades", "market": "BTC-0326", "type": "update", "data": [{"id": 632137285, "price": 56244.0, "size": 0.0043, "side": "sell", "liquidation": false, "time": "2021-03-21T10:58:26.498464+00:00"}]}"#;
        let trades = &parse_trade("ftx", MarketType::LinearFuture, raw_msg).unwrap();

        assert_eq!(trades.len(), 1);

        for trade in trades.iter() {
            crate::utils::check_trade_fields(
                "ftx",
                MarketType::LinearFuture,
                "BTC/USD".to_string(),
                trade,
            );

            assert_eq!(trade.volume, trade.price * trade.quantity);
            assert_eq!(trade.side, TradeSide::Sell);
            println!("{}", serde_json::to_string_pretty(&trade).unwrap());
        }
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"channel": "trades", "market": "BTC-PERP", "type": "update", "data": [{"id": 632141274, "price": 56115.0, "size": 0.005, "side": "buy", "liquidation": false, "time": "2021-03-21T11:00:38.933676+00:00"}]}"#;
        let trades = &parse_trade("ftx", MarketType::LinearSwap, raw_msg).unwrap();

        assert_eq!(trades.len(), 1);

        for trade in trades.iter() {
            crate::utils::check_trade_fields(
                "ftx",
                MarketType::LinearSwap,
                "BTC/USD".to_string(),
                trade,
            );

            assert_eq!(trade.volume, trade.price * trade.quantity);
            assert_eq!(trade.side, TradeSide::Buy);
        }
    }

    #[test]
    fn volatility_move() {
        let raw_msg = r#"{"channel": "trades", "market": "BTC-MOVE-WK-0402", "type": "update", "data": [{"id": 619750489, "price": 5862.0, "size": 0.1136, "side": "buy", "liquidation": false, "time": "2021-03-18T17:47:50.727425+00:00"}]}"#;
        let trades = &parse_trade("ftx", MarketType::Move, raw_msg).unwrap();

        assert_eq!(trades.len(), 1);

        for trade in trades.iter() {
            crate::utils::check_trade_fields(
                "ftx",
                MarketType::Move,
                "BTC-MOVE/USD".to_string(),
                trade,
            );

            assert_eq!(trade.volume, trade.price * trade.quantity);
            assert_eq!(trade.side, TradeSide::Buy);
        }
    }
}
