mod utils;

#[cfg(test)]
mod trade {
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, parse_trade, TradeSide};

    #[test]
    fn spot() {
        let raw_msg = r#"[["T","329","1616384937","BTC_USDT","bid","57347.4","0.048800"]]"#;
        let trades = &parse_trade("zbg", MarketType::Spot, raw_msg).unwrap();

        assert_eq!(trades.len(), 1);
        let trade = &trades[0];

        crate::utils::check_trade_fields(
            "zbg",
            MarketType::Spot,
            "BTC/USDT".to_string(),
            extract_symbol("zbg", MarketType::Spot, raw_msg).unwrap(),
            trade,
            raw_msg,
        );

        assert_eq!(trade.quantity_base, 0.048800);
        assert_eq!(trade.side, TradeSide::Buy);

        let raw_msg = r#"["T","329","1616486457","BTC_USDT","ask","54139.4","0.654172"]"#;
        let trades = &parse_trade("zbg", MarketType::Spot, raw_msg).unwrap();

        assert_eq!(trades.len(), 1);
        let trade = &trades[0];

        crate::utils::check_trade_fields(
            "zbg",
            MarketType::Spot,
            "BTC/USDT".to_string(),
            extract_symbol("zbg", MarketType::Spot, raw_msg).unwrap(),
            trade,
            raw_msg,
        );

        assert_eq!(trade.quantity_base, 0.654172);
        assert_eq!(trade.side, TradeSide::Sell);
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
            extract_symbol("zbg", MarketType::LinearSwap, raw_msg).unwrap(),
            trade,
            raw_msg,
        );

        assert_eq!(trade.quantity_base, 0.01 * 31.0);
        assert_eq!(trade.quantity_quote, 0.01 * 31.0 * 57326.0);
        assert_eq!(trade.quantity_contract, Some(31.0));
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
            extract_symbol("zbg", MarketType::InverseSwap, raw_msg).unwrap(),
            trade,
            raw_msg,
        );

        assert_eq!(trade.quantity_base, 188.0 / 57370.0);
        assert_eq!(trade.quantity_quote, 188.0);
        assert_eq!(trade.quantity_contract, Some(188.0));
        assert_eq!(trade.side, TradeSide::Sell);
    }
}

#[cfg(test)]
mod l2_orderbook {
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, parse_l2};
    use crypto_msg_type::MessageType;

    #[test]
    fn spot_snapshot() {
        let raw_msg = r#"[["AE","329","BTC_USDT","1622729950",{"asks":[["38394.8","0.01917"],["38394.2","0.195885"]]},{"bids":[["38388.7","0.146025"],["38388.1","0.155175"]]}]]"#;
        let orderbook = &parse_l2("zbg", MarketType::Spot, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 2);
        assert_eq!(orderbook.bids.len(), 2);
        assert!(orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "zbg",
            MarketType::Spot,
            MessageType::L2Event,
            "BTC/USDT".to_string(),
            extract_symbol("zbg", MarketType::Spot, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );

        assert_eq!(orderbook.timestamp, 1622729950000);

        assert_eq!(orderbook.bids[0].price, 38388.7);
        assert_eq!(orderbook.bids[0].quantity_base, 0.146025);
        assert_eq!(orderbook.bids[0].quantity_quote, 38388.7 * 0.146025);

        assert_eq!(orderbook.asks[0].price, 38394.2);
        assert_eq!(orderbook.asks[0].quantity_base, 0.195885);
        assert_eq!(orderbook.asks[0].quantity_quote, 38394.2 * 0.195885);
    }

    #[test]
    fn spot_update() {
        let raw_msg = r#"["E","329","1622729958","BTC_USDT","BID","38382.3","0.1842"]"#;
        let orderbook = &parse_l2("zbg", MarketType::Spot, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 0);
        assert_eq!(orderbook.bids.len(), 1);
        assert!(!orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "zbg",
            MarketType::Spot,
            MessageType::L2Event,
            "BTC/USDT".to_string(),
            extract_symbol("zbg", MarketType::Spot, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );

        assert_eq!(orderbook.timestamp, 1622729958000);

        assert_eq!(orderbook.bids[0].price, 38382.3);
        assert_eq!(orderbook.bids[0].quantity_base, 0.1842);
        assert_eq!(orderbook.bids[0].quantity_quote, 38382.3 * 0.1842);
    }

    #[test]
    fn linear_swap_update() {
        let raw_msg = r#"["future_snapshot_depth",{"asks":[["38704","2684"]],"contractId":1000000,"bids":[["38703","1606"],["38702.5","616"]],"tradeDate":20210603,"time":1622733219128160}]"#;
        let orderbook = &parse_l2("zbg", MarketType::LinearSwap, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 1);
        assert_eq!(orderbook.bids.len(), 2);
        assert!(!orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "zbg",
            MarketType::LinearSwap,
            MessageType::L2Event,
            "BTC/USDT".to_string(),
            extract_symbol("zbg", MarketType::LinearSwap, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );

        assert_eq!(orderbook.timestamp, 1622733219128);

        assert_eq!(orderbook.asks[0].price, 38704.0);
        assert_eq!(orderbook.asks[0].quantity_base, 26.84);
        assert_eq!(orderbook.asks[0].quantity_quote, 38704.0 * 26.84);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 2684.0);

        assert_eq!(orderbook.bids[0].price, 38703.0);
        assert_eq!(orderbook.bids[0].quantity_base, 16.06);
        assert_eq!(orderbook.bids[0].quantity_quote, 38703.0 * 16.06);
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 1606.0);
    }

    #[test]
    fn inverse_swap_update() {
        let raw_msg = r#"["future_snapshot_depth",{"asks":[["38547.5","4406"],["38548","11545"]],"contractId":1000001,"bids":[["38547","24345"],["38546.5","63623"]],"tradeDate":20210603,"time":1622734001831219}]"#;
        let orderbook = &parse_l2("zbg", MarketType::InverseSwap, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 2);
        assert_eq!(orderbook.bids.len(), 2);
        assert!(!orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "zbg",
            MarketType::InverseSwap,
            MessageType::L2Event,
            "BTC/USD".to_string(),
            extract_symbol("zbg", MarketType::InverseSwap, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );

        assert_eq!(orderbook.timestamp, 1622734001831);

        assert_eq!(orderbook.asks[0].price, 38547.5);
        assert_eq!(orderbook.asks[0].quantity_base, 4406.0 / 38547.5);
        assert_eq!(orderbook.asks[0].quantity_quote, 4406.0);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 4406.0);

        assert_eq!(orderbook.bids[0].price, 38547.0);
        assert_eq!(orderbook.bids[0].quantity_base, 24345.0 / 38547.0);
        assert_eq!(orderbook.bids[0].quantity_quote, 24345.0);
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 24345.0);
    }
}
