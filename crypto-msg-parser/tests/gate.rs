mod utils;

#[cfg(test)]
mod trade {
    use crypto_msg_parser::{parse_trade, MarketType, TradeSide};

    #[test]
    fn spot() {
        let raw_msg = r#"{"method": "trades.update", "params": ["BTC_USDT", [{"id": 643716793, "time": 1616327474.6243241, "price": "56173.28", "amount": "0.0037", "type": "sell"}]], "id": null}"#;
        let trades = &parse_trade("gate", MarketType::Spot, raw_msg).unwrap();

        assert_eq!(trades.len(), 1);
        let trade = &trades[0];

        crate::utils::check_trade_fields("gate", MarketType::Spot, "BTC/USDT".to_string(), trade);

        assert_eq!(trade.volume, trade.price * trade.quantity);
        assert_eq!(trade.side, TradeSide::Sell);
    }

    #[test]
    fn linear_future() {
        let raw_msg = r#"{"time":1615253386,"channel":"futures.trades","event":"update","error":null,"result":[{"size":-19,"id":48081,"create_time":1615253386,"price":"53560.5","contract":"BTC_USDT_20210326"}]}"#;
        let trades = &parse_trade("gate", MarketType::LinearFuture, raw_msg).unwrap();

        assert_eq!(trades.len(), 1);
        let trade = &trades[0];

        crate::utils::check_trade_fields(
            "gate",
            MarketType::LinearFuture,
            "BTC/USDT".to_string(),
            trade,
        );

        assert_eq!(trade.volume, trade.price * trade.quantity);
        assert_eq!(trade.quantity, 0.0019);
        assert_eq!(trade.side, TradeSide::Sell);
    }

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"time":1616327545,"channel":"futures.trades","event":"update","error":null,"result":[{"size":7,"id":19910126,"create_time":1616327545,"create_time_ms":1616327545436,"price":"56155.2","contract":"BTC_USD"}]}"#;
        let trades = &parse_trade("gate", MarketType::InverseSwap, raw_msg).unwrap();

        assert_eq!(trades.len(), 1);
        let trade = &trades[0];

        crate::utils::check_trade_fields(
            "gate",
            MarketType::InverseSwap,
            "BTC/USD".to_string(),
            trade,
        );

        assert_eq!(trade.volume, trade.price * trade.quantity);
        assert_eq!(trade.volume, 7.0);
        assert_eq!(trade.side, TradeSide::Buy);
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"time":1616327563,"channel":"futures.trades","event":"update","error":null,"result":[{"size":50,"id":15366793,"create_time":1616327563,"create_time_ms":1616327563918,"price":"56233.3","contract":"BTC_USDT"}]}"#;
        let trades = &parse_trade("gate", MarketType::LinearSwap, raw_msg).unwrap();

        assert_eq!(trades.len(), 1);
        let trade = &trades[0];

        crate::utils::check_trade_fields(
            "gate",
            MarketType::LinearSwap,
            "BTC/USDT".to_string(),
            trade,
        );

        assert_eq!(trade.volume, trade.price * trade.quantity);
        assert_eq!(trade.quantity, 0.005);
        assert_eq!(trade.side, TradeSide::Buy);
    }
}
