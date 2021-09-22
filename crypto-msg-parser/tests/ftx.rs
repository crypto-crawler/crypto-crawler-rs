mod utils;

#[cfg(test)]
mod trade {
    use crypto_msg_parser::{extract_symbol, parse_trade, MarketType, TradeSide};

    #[test]
    fn spot() {
        let raw_msg = r#"{"channel": "trades", "market": "BTC/USD", "type": "update", "data": [{"id": 632052557, "price": 56335.0, "size": 0.0444, "side": "buy", "liquidation": false, "time": "2021-03-21T10:24:37.319680+00:00"}]}"#;
        let trades = &parse_trade("ftx", MarketType::Spot, raw_msg).unwrap();

        assert_eq!(trades.len(), 1);

        for trade in trades.iter() {
            crate::utils::check_trade_fields(
                "ftx",
                MarketType::Spot,
                "BTC/USD".to_string(),
                extract_symbol("ftx", MarketType::Spot, raw_msg).unwrap(),
                trade,
            );

            assert_eq!(trade.side, TradeSide::Buy);
        }
        assert_eq!(trades[0].quantity_base, 0.0444);
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
                extract_symbol("ftx", MarketType::LinearFuture, raw_msg).unwrap(),
                trade,
            );
        }

        assert_eq!(trades[0].quantity_base, 0.0043);
        assert_eq!(trades[0].quantity_quote, 0.0043 * 56244.0);
        assert_eq!(trades[0].quantity_contract, Some(0.0043));
        assert_eq!(trades[0].side, TradeSide::Sell);
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
                extract_symbol("ftx", MarketType::LinearSwap, raw_msg).unwrap(),
                trade,
            );
        }

        assert_eq!(trades[0].quantity_base, 0.005);
        assert_eq!(trades[0].quantity_quote, 0.005 * 56115.0);
        assert_eq!(trades[0].quantity_contract, Some(0.005));
        assert_eq!(trades[0].side, TradeSide::Buy);
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
                extract_symbol("ftx", MarketType::Move, raw_msg).unwrap(),
                trade,
            );
        }

        assert_eq!(trades[0].quantity_base, 0.1136);
        assert_eq!(trades[0].quantity_quote, 0.1136 * 5862.0);
        assert_eq!(trades[0].quantity_contract, Some(0.1136));
        assert_eq!(trades[0].side, TradeSide::Buy);
    }
}

#[cfg(test)]
mod l2_orderbook {
    use crypto_msg_parser::{extract_symbol, parse_l2, MarketType};

    #[test]
    fn spot_snapshot() {
        let raw_msg = r#"{"channel": "orderbook", "market": "BTC/USD", "type": "partial", "data": {"time": 1622668801.966823, "checksum": 4093133381, "bids": [[37875.0, 0.4537], [37874.0, 0.5673], [37872.0, 0.328]], "asks": [[37876.0, 0.1749], [37877.0, 0.0001], [37878.0, 0.5]], "action": "partial"}}"#;
        let orderbook = &parse_l2("ftx", MarketType::Spot, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 3);
        assert_eq!(orderbook.bids.len(), 3);
        assert!(orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "ftx",
            MarketType::Spot,
            "BTC/USD".to_string(),
            extract_symbol("ftx", MarketType::Spot, raw_msg).unwrap(),
            orderbook,
        );

        assert_eq!(orderbook.timestamp, 1622668801966);

        assert_eq!(orderbook.bids[0].price, 37875.0);
        assert_eq!(orderbook.bids[0].quantity_base, 0.4537);
        assert_eq!(orderbook.bids[0].quantity_quote, 37875.0 * 0.4537);

        assert_eq!(orderbook.bids[2].price, 37872.0);
        assert_eq!(orderbook.bids[2].quantity_base, 0.328);
        assert_eq!(orderbook.bids[2].quantity_quote, 37872.0 * 0.328);

        assert_eq!(orderbook.asks[0].price, 37876.0);
        assert_eq!(orderbook.asks[0].quantity_base, 0.1749);
        assert_eq!(orderbook.asks[0].quantity_quote, 37876.0 * 0.1749);

        assert_eq!(orderbook.asks[2].price, 37878.0);
        assert_eq!(orderbook.asks[2].quantity_base, 0.5);
        assert_eq!(orderbook.asks[2].quantity_quote, 37878.0 * 0.5);
    }

    #[test]
    fn spot_update() {
        let raw_msg = r#"{"channel": "orderbook", "market": "BTC/USD", "type": "update", "data": {"time": 1622668802.0262146, "checksum": 2044263315, "bids": [[37875.0, 0.446]], "asks": [[37886.0, 5.2109], [37889.0, 0.8493]], "action": "update"}}"#;
        let orderbook = &parse_l2("ftx", MarketType::Spot, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 2);
        assert_eq!(orderbook.bids.len(), 1);
        assert!(!orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "ftx",
            MarketType::Spot,
            "BTC/USD".to_string(),
            extract_symbol("ftx", MarketType::Spot, raw_msg).unwrap(),
            orderbook,
        );

        assert_eq!(orderbook.timestamp, 1622668802026);

        assert_eq!(orderbook.bids[0].price, 37875.0);
        assert_eq!(orderbook.bids[0].quantity_base, 0.446);
        assert_eq!(orderbook.bids[0].quantity_quote, 37875.0 * 0.446);

        assert_eq!(orderbook.asks[0].price, 37886.0);
        assert_eq!(orderbook.asks[0].quantity_base, 5.2109);
        assert_eq!(orderbook.asks[0].quantity_quote, 37886.0 * 5.2109);

        assert_eq!(orderbook.asks[1].price, 37889.0);
        assert_eq!(orderbook.asks[1].quantity_base, 0.8493);
        assert_eq!(orderbook.asks[1].quantity_quote, 37889.0 * 0.8493);
    }

    #[test]
    fn linear_future_snapshot() {
        let raw_msg = r#"{"channel": "orderbook", "market": "BTC-0625", "type": "partial", "data": {"time": 1622669504.8200636, "checksum": 1739399809, "bids": [[37965.0, 2.7939], [37961.0, 0.005], [37960.0, 11.4351]], "asks": [[37980.0, 0.2474], [37987.0, 0.0957], [37991.0, 0.0005]], "action": "partial"}}"#;
        let orderbook = &parse_l2("ftx", MarketType::LinearFuture, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 3);
        assert_eq!(orderbook.bids.len(), 3);
        assert!(orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "ftx",
            MarketType::LinearFuture,
            "BTC/USD".to_string(),
            extract_symbol("ftx", MarketType::LinearFuture, raw_msg).unwrap(),
            orderbook,
        );

        assert_eq!(orderbook.timestamp, 1622669504820);

        assert_eq!(orderbook.bids[0].price, 37965.0);
        assert_eq!(orderbook.bids[0].quantity_base, 2.7939);
        assert_eq!(orderbook.bids[0].quantity_quote, 37965.0 * 2.7939);
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 2.7939);

        assert_eq!(orderbook.bids[2].price, 37960.0);
        assert_eq!(orderbook.bids[2].quantity_base, 11.4351);
        assert_eq!(orderbook.bids[2].quantity_quote, 37960.0 * 11.4351);
        assert_eq!(orderbook.bids[2].quantity_contract.unwrap(), 11.4351);

        assert_eq!(orderbook.asks[0].price, 37980.0);
        assert_eq!(orderbook.asks[0].quantity_base, 0.2474);
        assert_eq!(orderbook.asks[0].quantity_quote, 37980.0 * 0.2474);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 0.2474);

        assert_eq!(orderbook.asks[2].price, 37991.0);
        assert_eq!(orderbook.asks[2].quantity_base, 0.0005);
        assert_eq!(orderbook.asks[2].quantity_quote, 37991.0 * 0.0005);
        assert_eq!(orderbook.asks[2].quantity_contract.unwrap(), 0.0005);
    }

    #[test]
    fn linear_future_update() {
        let raw_msg = r#"{"channel": "orderbook", "market": "BTC-0625", "type": "update", "data": {"time": 1622669504.8437843, "checksum": 1584262478, "bids": [], "asks": [[37999.0, 0.0], [38440.0, 0.0026]], "action": "update"}}"#;
        let orderbook = &parse_l2("ftx", MarketType::LinearFuture, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 2);
        assert_eq!(orderbook.bids.len(), 0);
        assert!(!orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "ftx",
            MarketType::LinearFuture,
            "BTC/USD".to_string(),
            extract_symbol("ftx", MarketType::LinearFuture, raw_msg).unwrap(),
            orderbook,
        );

        assert_eq!(orderbook.timestamp, 1622669504843);

        assert_eq!(orderbook.asks[0].price, 37999.0);
        assert_eq!(orderbook.asks[0].quantity_base, 0.0);
        assert_eq!(orderbook.asks[0].quantity_quote, 0.0);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 0.0);

        assert_eq!(orderbook.asks[1].price, 38440.0);
        assert_eq!(orderbook.asks[1].quantity_base, 0.0026);
        assert_eq!(orderbook.asks[1].quantity_quote, 38440.0 * 0.0026);
        assert_eq!(orderbook.asks[1].quantity_contract.unwrap(), 0.0026);
    }

    #[test]
    fn linear_swap_snapshot() {
        let raw_msg = r#"{"channel": "orderbook", "market": "BTC-PERP", "type": "partial", "data": {"time": 1622660997.436228, "checksum": 1855139817, "bids": [[37955.0, 0.2212], [37954.0, 0.0025], [37953.0, 0.0025]], "asks": [[37956.0, 4.8852], [37957.0, 0.022], [37958.0, 0.4818]], "action": "partial"}}"#;
        let orderbook = &parse_l2("ftx", MarketType::LinearSwap, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 3);
        assert_eq!(orderbook.bids.len(), 3);
        assert!(orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "ftx",
            MarketType::LinearSwap,
            "BTC/USD".to_string(),
            extract_symbol("ftx", MarketType::LinearSwap, raw_msg).unwrap(),
            orderbook,
        );

        assert_eq!(orderbook.timestamp, 1622660997436);

        assert_eq!(orderbook.bids[0].price, 37955.0);
        assert_eq!(orderbook.bids[0].quantity_base, 0.2212);
        assert_eq!(orderbook.bids[0].quantity_quote, 37955.0 * 0.2212);
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 0.2212);

        assert_eq!(orderbook.bids[2].price, 37953.0);
        assert_eq!(orderbook.bids[2].quantity_base, 0.0025);
        assert_eq!(orderbook.bids[2].quantity_quote, 37953.0 * 0.0025);
        assert_eq!(orderbook.bids[2].quantity_contract.unwrap(), 0.0025);

        assert_eq!(orderbook.asks[0].price, 37956.0);
        assert_eq!(orderbook.asks[0].quantity_base, 4.8852);
        assert_eq!(orderbook.asks[0].quantity_quote, 37956.0 * 4.8852);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 4.8852);

        assert_eq!(orderbook.asks[2].price, 37958.0);
        assert_eq!(orderbook.asks[2].quantity_base, 0.4818);
        assert_eq!(orderbook.asks[2].quantity_quote, 37958.0 * 0.4818);
        assert_eq!(orderbook.asks[2].quantity_contract.unwrap(), 0.4818);
    }

    #[test]
    fn linear_swap_update() {
        let raw_msg = r#"{"channel": "orderbook", "market": "BTC-PERP", "type": "update", "data": {"time": 1622660997.4591022, "checksum": 276300987, "bids": [], "asks": [[37965.0, 19.6097]], "action": "update"}}"#;
        let orderbook = &parse_l2("ftx", MarketType::LinearSwap, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 1);
        assert_eq!(orderbook.bids.len(), 0);
        assert!(!orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "ftx",
            MarketType::LinearSwap,
            "BTC/USD".to_string(),
            extract_symbol("ftx", MarketType::LinearSwap, raw_msg).unwrap(),
            orderbook,
        );

        assert_eq!(orderbook.timestamp, 1622660997459);

        assert_eq!(orderbook.asks[0].price, 37965.0);
        assert_eq!(orderbook.asks[0].quantity_base, 19.6097);
        assert_eq!(orderbook.asks[0].quantity_quote, 37965.0 * 19.6097);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 19.6097);
    }
}
