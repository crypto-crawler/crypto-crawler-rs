mod utils;

#[cfg(test)]
mod trade {
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, parse_trade, TradeSide};

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"type":"channel_data","connection_id":"c685b690-168e-421d-bfd4-60aae426686d","message_id":2,"id":"BTC-USD","channel":"v3_trades","contents":{"trades":[{"size":"0.124","side":"BUY","price":"56503","createdAt":"2021-10-11T10:36:41.464Z"}]}}"#;
        let trade = &parse_trade("dydx", MarketType::LinearSwap, raw_msg).unwrap()[0];

        crate::utils::check_trade_fields(
            "dydx",
            MarketType::LinearSwap,
            "BTC/USD".to_string(),
            extract_symbol("dydx", MarketType::LinearSwap, raw_msg).unwrap(),
            trade,
        );

        assert_eq!(trade.quantity_base, 0.124);
        assert_eq!(trade.quantity_quote, 0.124 * 56503.0);
        assert_eq!(trade.quantity_contract, Some(0.124));

        assert_eq!(trade.side, TradeSide::Buy);
    }
}

#[cfg(test)]
mod l2_orderbook {
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, parse_l2};

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"type":"subscribed","connection_id":"f1e5eecb-7929-4033-8f47-47a2eb71af96","message_id":1,"channel":"v3_orderbook","id":"BTC-USD","contents":{"asks":[{"size":"1.7415","price":"56490"},{"size":"1.7718","price":"56493"}],"bids":[{"size":"1.7088","price":"56489"},{"size":"2.1594","price":"56488"}]}}"#;
        let orderbook =
            &parse_l2("dydx", MarketType::LinearSwap, raw_msg, Some(1633951152106)).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 2);
        assert_eq!(orderbook.bids.len(), 2);
        assert!(orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "dydx",
            MarketType::LinearSwap,
            "BTC/USD".to_string(),
            extract_symbol("dydx", MarketType::LinearSwap, raw_msg).unwrap(),
            orderbook,
        );

        assert_eq!(orderbook.timestamp, 1633951152106);

        assert_eq!(orderbook.bids[0].price, 56489.0);
        assert_eq!(orderbook.bids[0].quantity_base, 1.7088);
        assert_eq!(orderbook.bids[0].quantity_quote, 56489.0 * 1.7088);
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 1.7088);

        assert_eq!(orderbook.bids[1].price, 56488.0);
        assert_eq!(orderbook.bids[1].quantity_base, 2.1594);
        assert_eq!(orderbook.bids[1].quantity_quote, 56488.0 * 2.1594);
        assert_eq!(orderbook.bids[1].quantity_contract.unwrap(), 2.1594);

        assert_eq!(orderbook.asks[0].price, 56490.0);
        assert_eq!(orderbook.asks[0].quantity_base, 1.7415);
        assert_eq!(orderbook.asks[0].quantity_quote, 56490.0 * 1.7415);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 1.7415);

        assert_eq!(orderbook.asks[1].price, 56493.0);
        assert_eq!(orderbook.asks[1].quantity_base, 1.7718);
        assert_eq!(orderbook.asks[1].quantity_quote, 56493.0 * 1.7718);
        assert_eq!(orderbook.asks[1].quantity_contract.unwrap(), 1.7718);

        let raw_msg = r#"{"type":"channel_data","connection_id":"f1e5eecb-7929-4033-8f47-47a2eb71af96","message_id":2,"id":"BTC-USD","channel":"v3_orderbook","contents":{"offset":"2060907065","bids":[],"asks":[["56525","0.4782"],["56527","0.0186"]]}}"#;
        let orderbook =
            &parse_l2("dydx", MarketType::LinearSwap, raw_msg, Some(1633951152106)).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 2);
        assert_eq!(orderbook.bids.len(), 0);
        assert!(!orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "dydx",
            MarketType::LinearSwap,
            "BTC/USD".to_string(),
            extract_symbol("dydx", MarketType::LinearSwap, raw_msg).unwrap(),
            orderbook,
        );

        assert_eq!(orderbook.timestamp, 1633951152106);

        assert_eq!(orderbook.asks[0].price, 56525.0);
        assert_eq!(orderbook.asks[0].quantity_base, 0.4782);
        assert_eq!(orderbook.asks[0].quantity_quote, 56525.0 * 0.4782);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 0.4782);
    }
}
