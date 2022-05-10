mod utils;

#[cfg(test)]
mod trade {
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, extract_timestamp, parse_trade, TradeSide};

    #[test]
    fn spot_te() {
        let raw_msg = r#"[{"symbol":"tBTCUST","channel":"trades"},"te",[637771130,1615232733897,0.11546588,51350]]"#;
        let trade = &parse_trade("bitfinex", MarketType::Spot, raw_msg).unwrap()[0];

        crate::utils::check_trade_fields(
            "bitfinex",
            MarketType::Spot,
            "BTC/USDT".to_string(),
            extract_symbol("bitfinex", MarketType::Spot, raw_msg).unwrap(),
            trade,
            raw_msg,
        );
        assert_eq!(
            1615232733897,
            extract_timestamp("bitfinex", MarketType::Spot, raw_msg, None).unwrap()
        );

        assert_eq!(trade.quantity_base, 0.11546588);
        assert_eq!(trade.quantity_quote, 0.11546588 * 51350.0);
        assert_eq!(trade.quantity_contract, None);

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
            extract_symbol("bitfinex", MarketType::Spot, raw_msg).unwrap(),
            trade,
            raw_msg,
        );
        assert_eq!(
            1615232733897,
            extract_timestamp("bitfinex", MarketType::Spot, raw_msg, None).unwrap()
        );

        assert_eq!(trade.quantity_base, 0.11546588);
        assert_eq!(trade.quantity_quote, 0.11546588 * 51350.0);
        assert_eq!(trade.quantity_contract, None);

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
                extract_symbol("bitfinex", MarketType::Spot, raw_msg).unwrap(),
                trade,
                raw_msg,
            );
        }
        assert_eq!(
            1616217509543,
            extract_timestamp("bitfinex", MarketType::Spot, raw_msg, None).unwrap()
        );
    }

    #[test]
    fn swap_te() {
        let raw_msg = r#"[{"channel":"trades","symbol":"tBTCF0:USTF0"},"te",[647256282,1616219711336,0.00020449,58244]]"#;
        let trade = &parse_trade("bitfinex", MarketType::LinearSwap, raw_msg).unwrap()[0];

        crate::utils::check_trade_fields(
            "bitfinex",
            MarketType::LinearSwap,
            "BTC/USDT".to_string(),
            extract_symbol("bitfinex", MarketType::LinearSwap, raw_msg).unwrap(),
            trade,
            raw_msg,
        );
        assert_eq!(
            1616219711336,
            extract_timestamp("bitfinex", MarketType::Spot, raw_msg, None).unwrap()
        );

        assert_eq!(trade.quantity_base, 0.00020449);
        assert_eq!(trade.quantity_quote, 0.00020449 * 58244.0);
        assert_eq!(trade.quantity_contract, Some(0.00020449));

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
            extract_symbol("bitfinex", MarketType::LinearSwap, raw_msg).unwrap(),
            trade,
            raw_msg,
        );
        assert_eq!(
            1616219711336,
            extract_timestamp("bitfinex", MarketType::Spot, raw_msg, None).unwrap()
        );

        assert_eq!(trade.quantity_base, 0.00020449);
        assert_eq!(trade.quantity_quote, 0.00020449 * 58244.0);
        assert_eq!(trade.quantity_contract, Some(0.00020449));

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
                extract_symbol("bitfinex", MarketType::LinearSwap, raw_msg).unwrap(),
                trade,
                raw_msg,
            );
        }
        assert_eq!(
            1616219105954,
            extract_timestamp("bitfinex", MarketType::Spot, raw_msg, None).unwrap()
        );
    }
}

#[cfg(test)]
mod l2_orderbook {
    use chrono::prelude::*;
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, extract_timestamp, parse_l2};
    use crypto_msg_type::MessageType;

    #[test]
    fn spot_snapshot() {
        let raw_msg = r#"[{"symbol":"tBTCUST","len":"25","freq":"F0","channel":"book","prec":"P0"},[[36167,1,0.48403686],[36162,2,0.22625024],[36161,1,0.43250047],[36158,1,0.209],[36155,2,0.48229814],[36171,1,-0.000006],[36172,1,-0.0002],[36173,1,-0.0002],[36174,2,-0.0102],[36175,1,-0.0002]]]"#;
        let received_at = Utc::now().timestamp_millis();
        let orderbook =
            &parse_l2("bitfinex", MarketType::Spot, raw_msg, Some(received_at)).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 5);
        assert_eq!(orderbook.bids.len(), 5);
        assert!(orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "bitfinex",
            MarketType::Spot,
            MessageType::L2Event,
            "BTC/USDT".to_string(),
            extract_symbol("bitfinex", MarketType::Spot, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );
        assert_eq!(
            received_at,
            extract_timestamp("bitfinex", MarketType::Spot, raw_msg, Some(received_at)).unwrap()
        );

        assert_eq!(orderbook.bids[0].price, 36167.0);
        assert_eq!(orderbook.bids[0].quantity_base, 0.48403686);
        assert_eq!(orderbook.bids[0].quantity_quote, 36167.0 * 0.48403686);

        assert_eq!(orderbook.bids[4].price, 36155.0);
        assert_eq!(orderbook.bids[4].quantity_base, 0.48229814);
        assert_eq!(orderbook.bids[4].quantity_quote, 36155.0 * 0.48229814);

        assert_eq!(orderbook.asks[0].price, 36171.0);
        assert_eq!(orderbook.asks[0].quantity_base, 0.000006);
        assert_eq!(orderbook.asks[0].quantity_quote, 36171.0 * 0.000006);

        assert_eq!(orderbook.asks[4].price, 36175.0);
        assert_eq!(orderbook.asks[4].quantity_base, 0.0002);
        assert_eq!(orderbook.asks[4].quantity_quote, 36175.0 * 0.0002);
    }

    #[test]
    fn spot_update() {
        let raw_msg = r#"[{"symbol":"tBTCUST","channel":"book","len":"25","freq":"F0","prec":"P0"},[34668,1,-0.00813136]]"#;
        let received_at = Utc::now().timestamp_millis();
        let orderbook =
            &parse_l2("bitfinex", MarketType::Spot, raw_msg, Some(received_at)).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 1);
        assert_eq!(orderbook.bids.len(), 0);
        assert!(!orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "bitfinex",
            MarketType::Spot,
            MessageType::L2Event,
            "BTC/USDT".to_string(),
            extract_symbol("bitfinex", MarketType::Spot, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );
        assert_eq!(
            received_at,
            extract_timestamp("bitfinex", MarketType::Spot, raw_msg, Some(received_at)).unwrap()
        );

        assert_eq!(orderbook.asks[0].price, 34668.0);
        assert_eq!(orderbook.asks[0].quantity_base, 0.00813136);
        assert_eq!(orderbook.asks[0].quantity_quote, 34668.0 * 0.00813136);
    }

    #[test]
    fn linear_swap_snapshot() {
        let raw_msg = r#"[{"freq":"F0","channel":"book","prec":"P0","len":"25","symbol":"tBTCF0:USTF0"},[[34840,2,0.20047952],[34837,1,0.17573],[34829,1,0.0857],[34828,1,0.17155],[34826,2,0.25510833],[34841,1,-0.00034929],[34843,4,-0.70368583],[34844,1,-0.51672161],[34845,2,-0.78960194],[34846,1,-1.0339621]]]"#;
        let received_at = Utc::now().timestamp_millis();
        let orderbook = &parse_l2(
            "bitfinex",
            MarketType::LinearSwap,
            raw_msg,
            Some(received_at),
        )
        .unwrap()[0];

        assert_eq!(orderbook.asks.len(), 5);
        assert_eq!(orderbook.bids.len(), 5);
        assert!(orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "bitfinex",
            MarketType::LinearSwap,
            MessageType::L2Event,
            "BTC/USDT".to_string(),
            extract_symbol("bitfinex", MarketType::LinearSwap, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );
        assert_eq!(
            received_at,
            extract_timestamp("bitfinex", MarketType::Spot, raw_msg, Some(received_at)).unwrap()
        );

        assert_eq!(orderbook.bids[0].price, 34840.0);
        assert_eq!(orderbook.bids[0].quantity_base, 0.20047952);
        assert_eq!(orderbook.bids[0].quantity_quote, 34840.0 * 0.20047952);
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 0.20047952);

        assert_eq!(orderbook.bids[4].price, 34826.0);
        assert_eq!(orderbook.bids[4].quantity_base, 0.25510833);
        assert_eq!(orderbook.bids[4].quantity_quote, 34826.0 * 0.25510833);
        assert_eq!(orderbook.bids[4].quantity_contract.unwrap(), 0.25510833);

        assert_eq!(orderbook.asks[0].price, 34841.0);
        assert_eq!(orderbook.asks[0].quantity_base, 0.00034929);
        assert_eq!(orderbook.asks[0].quantity_quote, 34841.0 * 0.00034929);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 0.00034929);

        assert_eq!(orderbook.asks[4].price, 34846.0);
        assert_eq!(orderbook.asks[4].quantity_base, 1.0339621);
        assert_eq!(orderbook.asks[4].quantity_quote, 34846.0 * 1.0339621);
        assert_eq!(orderbook.asks[4].quantity_contract.unwrap(), 1.0339621);
    }

    #[test]
    fn linear_swap_update() {
        let raw_msg = r#"[{"freq":"F0","symbol":"tBTCF0:USTF0","channel":"book","len":"25","prec":"P0"},[34442,2,2.27726294]]"#;
        let received_at = Utc::now().timestamp_millis();
        let orderbook = &parse_l2(
            "bitfinex",
            MarketType::LinearSwap,
            raw_msg,
            Some(received_at),
        )
        .unwrap()[0];

        assert_eq!(orderbook.asks.len(), 0);
        assert_eq!(orderbook.bids.len(), 1);
        assert!(!orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "bitfinex",
            MarketType::LinearSwap,
            MessageType::L2Event,
            "BTC/USDT".to_string(),
            extract_symbol("bitfinex", MarketType::LinearSwap, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );
        assert_eq!(
            received_at,
            extract_timestamp("bitfinex", MarketType::Spot, raw_msg, Some(received_at)).unwrap()
        );

        assert_eq!(orderbook.bids[0].price, 34442.0);
        assert_eq!(orderbook.bids[0].quantity_base, 2.27726294);
        assert_eq!(orderbook.bids[0].quantity_quote, 34442.0 * 2.27726294);
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 2.27726294);
    }
}
