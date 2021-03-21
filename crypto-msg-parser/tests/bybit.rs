mod utils;

#[cfg(test)]
mod trade {
    use crypto_msg_parser::{parse_trade, MarketType, TradeSide};

    #[test]
    fn inverse_future() {
        let raw_msg = r#"{"topic":"trade.BTCUSDM21","data":[{"trade_time_ms":1616304614117,"timestamp":"2021-03-21T05:30:14.000Z","symbol":"BTCUSDM21","side":"Buy","size":100,"price":61094.5,"tick_direction":"ZeroPlusTick","trade_id":"e61fb2dc-a658-5a7d-88fb-d166a4bd29b8","cross_seq":233452601},{"trade_time_ms":1616304614117,"timestamp":"2021-03-21T05:30:14.000Z","symbol":"BTCUSDM21","side":"Buy","size":100,"price":61094.5,"tick_direction":"ZeroPlusTick","trade_id":"2cbeff0d-16da-5946-a7b0-0ccfb78d3ab5","cross_seq":233452601}]}"#;
        let trades = &parse_trade("bybit", MarketType::InverseFuture, raw_msg).unwrap();

        assert_eq!(trades.len(), 2);

        for trade in trades.iter() {
            crate::utils::check_trade_fields(
                "bybit",
                MarketType::InverseFuture,
                "BTC/USD".to_string(),
                trade,
            );

            assert_eq!(trade.volume, 100.0); // volume == size
            assert_eq!(trade.volume, trade.price * trade.quantity);
            assert_eq!(trade.side, TradeSide::Buy);
        }
    }

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"topic":"trade.BTCUSD","data":[{"trade_time_ms":1616304710061,"timestamp":"2021-03-21T05:31:50.000Z","symbol":"BTCUSD","side":"Buy","size":237,"price":57073.5,"tick_direction":"ZeroPlusTick","trade_id":"f6198d62-4d4d-5908-9902-32c3aa5d9cfd","cross_seq":5404769827}]}"#;
        let trades = &parse_trade("bybit", MarketType::InverseSwap, raw_msg).unwrap();

        assert_eq!(trades.len(), 1);

        let trade = &trades[0];

        crate::utils::check_trade_fields(
            "bybit",
            MarketType::InverseSwap,
            "BTC/USD".to_string(),
            trade,
        );

        assert_eq!(trade.volume, 237.0); // volume == size
        assert_eq!(trade.volume, trade.price * trade.quantity);
        assert_eq!(trade.side, TradeSide::Buy);
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"topic":"trade.BTCUSDT","data":[{"symbol":"BTCUSDT","tick_direction":"ZeroPlusTick","price":"57170.00","size":0.04,"timestamp":"2021-03-21T05:32:17.000Z","trade_time_ms":"1616304737092","side":"Buy","trade_id":"fe9ef57c-2571-5728-847b-7bc039b6b52d"}]}"#;
        let trades = &parse_trade("bybit", MarketType::LinearSwap, raw_msg).unwrap();

        assert_eq!(trades.len(), 1);

        let trade = &trades[0];

        crate::utils::check_trade_fields(
            "bybit",
            MarketType::LinearSwap,
            "BTC/USDT".to_string(),
            trade,
        );

        assert_eq!(trade.volume, trade.price * trade.quantity);
        assert_eq!(trade.side, TradeSide::Buy);
    }
}
