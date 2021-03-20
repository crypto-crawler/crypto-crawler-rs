mod utils;

#[cfg(test)]
mod trade {
    use crypto_msg_parser::{parse_trade, MarketType, TradeSide};
    use float_cmp::approx_eq;

    #[test]
    fn spot() {
        let raw_msg = r#"{"ch":"market.btcusdt.trade.detail","ts":1616243199157,"tick":{"id":123140716701,"ts":1616243199156,"data":[{"id":123140716701236887569077664,"ts":1616243199156,"tradeId":102357140867,"amount":1.98E-4,"price":58911.07,"direction":"sell"}]}}"#;
        let trade = &parse_trade("huobi", MarketType::Spot, raw_msg).unwrap()[0];

        crate::utils::check_trade_fields("huobi", MarketType::Spot, "BTC/USDT".to_string(), trade);

        assert_eq!(trade.volume, trade.price * trade.quantity);
        assert_eq!(trade.side, TradeSide::Sell);
    }

    #[test]
    fn inverse_future() {
        let raw_msg = r#"{"ch":"market.BTC_CQ.trade.detail","ts":1616231995793,"tick":{"id":128974648797,"ts":1616231995768,"data":[{"amount":2,"quantity":0.0031859832031779545255059460801016711,"ts":1616231995768,"id":1289746487970000,"price":62774.97,"direction":"buy"}]}}"#;
        let trade = &parse_trade("huobi", MarketType::InverseFuture, raw_msg).unwrap()[0];
        crate::utils::check_trade_fields(
            "huobi",
            MarketType::InverseFuture,
            "BTC/USD".to_string(),
            trade,
        );
        assert_eq!(trade.volume, 200.0);
        assert_eq!(trade.volume, trade.price * trade.quantity);
        assert_eq!(trade.side, TradeSide::Buy);

        let raw_msg = r#"{"ch":"market.ETH_CQ.trade.detail","ts":1616269629976,"tick":{"id":128632765054,"ts":1616269629958,"data":[{"amount":2,"quantity":0.0100143605930904917651912843016886215,"ts":1616269629958,"id":1286327650540000,"price":1997.132,"direction":"sell"}]}}"#;
        let trade = &parse_trade("huobi", MarketType::InverseFuture, raw_msg).unwrap()[0];
        crate::utils::check_trade_fields(
            "huobi",
            MarketType::InverseFuture,
            "ETH/USD".to_string(),
            trade,
        );
        assert_eq!(trade.volume, 20.0);
        assert_eq!(trade.volume, trade.price * trade.quantity);
        assert_eq!(trade.side, TradeSide::Sell);
    }

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"ch":"market.BTC-USD.trade.detail","ts":1616233683377,"tick":{"id":84230699705,"ts":1616233683352,"data":[{"amount":6,"quantity":0.0102273366481267780650901795408948579,"ts":1616233683352,"id":842306997050000,"price":58666.3,"direction":"buy"}]}}"#;
        let trade = &parse_trade("huobi", MarketType::InverseSwap, raw_msg).unwrap()[0];
        crate::utils::check_trade_fields(
            "huobi",
            MarketType::InverseSwap,
            "BTC/USD".to_string(),
            trade,
        );
        assert_eq!(trade.volume, 600.0);
        assert_eq!(trade.volume, trade.price * trade.quantity);
        assert_eq!(trade.side, TradeSide::Buy);

        let raw_msg = r#"{"ch":"market.ETH-USD.trade.detail","ts":1616269812566,"tick":{"id":79855942906,"ts":1616269812548,"data":[{"amount":346,"quantity":1.871099622535394066559231659438237489,"ts":1616269812548,"id":798559429060000,"price":1849.18,"direction":"sell"}]}}"#;
        let trade = &parse_trade("huobi", MarketType::InverseSwap, raw_msg).unwrap()[0];
        crate::utils::check_trade_fields(
            "huobi",
            MarketType::InverseSwap,
            "ETH/USD".to_string(),
            trade,
        );
        assert_eq!(trade.volume, 3460.0);
        assert_eq!(trade.volume, trade.price * trade.quantity);
        assert_eq!(trade.side, TradeSide::Sell);
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"ch":"market.BTC-USDT.trade.detail","ts":1616233478594,"tick":{"id":22419995164,"ts":1616233478583,"data":[{"amount":40,"quantity":0.04,"trade_turnover":2350.796,"ts":1616233478583,"id":224199951640000,"price":58769.9,"direction":"sell"}]}}"#;
        let trade = &parse_trade("huobi", MarketType::LinearSwap, raw_msg).unwrap()[0];
        crate::utils::check_trade_fields(
            "huobi",
            MarketType::LinearSwap,
            "BTC/USDT".to_string(),
            trade,
        );
        assert_eq!(trade.volume, 2350.796);
        assert!(approx_eq!(
            f64,
            trade.volume,
            trade.price * trade.quantity,
            ulps = 9
        ));
        assert_eq!(trade.side, TradeSide::Sell);

        let raw_msg = r#"{"ch":"market.ETH-USDT.trade.detail","ts":1616270565862,"tick":{"id":19056652696,"ts":1616270565838,"data":[{"amount":18,"quantity":0.18,"trade_turnover":332.487,"ts":1616270565838,"id":190566526960000,"price":1847.15,"direction":"sell"}]}}"#;
        let trade = &parse_trade("huobi", MarketType::LinearSwap, raw_msg).unwrap()[0];
        crate::utils::check_trade_fields(
            "huobi",
            MarketType::LinearSwap,
            "ETH/USDT".to_string(),
            trade,
        );
        assert_eq!(trade.volume, 332.487);
        assert!(approx_eq!(
            f64,
            trade.volume,
            trade.price * trade.quantity,
            ulps = 9
        ));
        assert_eq!(trade.side, TradeSide::Sell);
    }
}
