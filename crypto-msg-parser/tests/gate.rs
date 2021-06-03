mod utils;

#[cfg(test)]
mod trade {
    use crypto_msg_parser::{parse_trade, MarketType, TradeSide};
    use float_cmp::approx_eq;

    #[test]
    fn spot() {
        let raw_msg = r#"{"method": "trades.update", "params": ["BTC_USDT", [{"id": 643716793, "time": 1616327474.6243241, "price": "56173.28", "amount": "0.0037", "type": "sell"}]], "id": null}"#;
        let trades = &parse_trade("gate", MarketType::Spot, raw_msg).unwrap();

        assert_eq!(trades.len(), 1);
        let trade = &trades[0];

        crate::utils::check_trade_fields("gate", MarketType::Spot, "BTC/USDT".to_string(), trade);

        assert_eq!(trades[0].quantity_base, 0.0037);
        assert_eq!(trades[0].quantity_quote, 0.0037 * 56173.28);
        assert_eq!(trades[0].quantity_contract, None);
        assert_eq!(trades[0].side, TradeSide::Sell);
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

        assert!(approx_eq!(
            f64,
            trade.quantity_base,
            19.0 * 0.0001,
            epsilon = 0.0000000001
        ));
        assert!(approx_eq!(
            f64,
            trade.quantity_quote,
            0.0019 * 53560.5,
            epsilon = 0.0001
        ));
        assert_eq!(trade.quantity_contract, Some(19.0));
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

        assert_eq!(trade.quantity_base, 7.0 / 56155.2);
        assert_eq!(trade.quantity_quote, 7.0);
        assert_eq!(trade.quantity_contract, Some(7.0));
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

        assert!(approx_eq!(
            f64,
            trade.quantity_base,
            0.0001 * 50.0,
            epsilon = 0.00000001
        ));
        assert!(approx_eq!(
            f64,
            trade.quantity_quote,
            0.005 * 56233.3,
            epsilon = 0.00001
        ));
        assert_eq!(trade.quantity_contract, Some(50.0));
        assert_eq!(trade.side, TradeSide::Buy);
    }
}

#[cfg(test)]
mod l2_orderbook {
    use crypto_msg_parser::{parse_l2, MarketType};
    use float_cmp::approx_eq;

    #[test]
    fn spot_snapshot() {
        let raw_msg = r#"{"method": "depth.update", "params": [true, {"asks": [["37483.21", "0.048"], ["37483.89", "0.0739"], ["37486.86", "0.1639"]], "bids": [["37483.19", "0.01"], ["37480.69", "0.0183"], ["37479.16", "0.0292"]], "id": 3166483561}, "BTC_USDT"], "id": null}"#;
        let orderbook = &parse_l2("gate", MarketType::Spot, raw_msg).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 3);
        assert_eq!(orderbook.bids.len(), 3);
        assert!(orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "gate",
            MarketType::Spot,
            "BTC/USDT".to_string(),
            orderbook,
        );

        assert_eq!(orderbook.asks[0][0], 37483.21);
        assert_eq!(orderbook.asks[0][1], 0.048);
        assert_eq!(orderbook.asks[0][2], 37483.21 * 0.048);

        assert_eq!(orderbook.asks[2][0], 37486.86);
        assert_eq!(orderbook.asks[2][1], 0.1639);
        assert_eq!(orderbook.asks[2][2], 37486.86 * 0.1639);

        assert_eq!(orderbook.bids[0][0], 37483.19);
        assert_eq!(orderbook.bids[0][1], 0.01);
        assert_eq!(orderbook.bids[0][2], 37483.19 * 0.01);

        assert_eq!(orderbook.bids[2][0], 37479.16);
        assert_eq!(orderbook.bids[2][1], 0.0292);
        assert_eq!(orderbook.bids[2][2], 37479.16 * 0.0292);
    }

    #[test]
    fn spot_update() {
        let raw_msg = r#"{"method": "depth.update", "params": [false, {"asks": [["37483.89", "0"]], "bids": [["37479.16", "0"], ["37478.79", "0.0554"]]}, "BTC_USDT"], "id": null}"#;
        let orderbook = &parse_l2("gate", MarketType::Spot, raw_msg).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 1);
        assert_eq!(orderbook.bids.len(), 2);
        assert!(!orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "gate",
            MarketType::Spot,
            "BTC/USDT".to_string(),
            orderbook,
        );

        assert_eq!(orderbook.asks[0][0], 37483.89);
        assert_eq!(orderbook.asks[0][1], 0.0);
        assert_eq!(orderbook.asks[0][2], 0.0);

        assert_eq!(orderbook.bids[0][0], 37479.16);
        assert_eq!(orderbook.bids[0][1], 0.0);
        assert_eq!(orderbook.bids[0][2], 0.0);

        assert_eq!(orderbook.bids[1][0], 37478.79);
        assert_eq!(orderbook.bids[1][1], 0.0554);
        assert_eq!(orderbook.bids[1][2], 37478.79 * 0.0554);
    }

    #[test]
    fn inverse_swap_snapshot() {
        let raw_msg = r#"{"id":null,"time":1622682306,"channel":"futures.order_book","event":"all","error":null,"result":{"t":1622682306315,"id":2861474582,"contract":"BTC_USD","asks":[{"p":"37481.3","s":7766},{"p":"37484.7","s":1775},{"p":"37485.1","s":2004}],"bids":[{"p":"37481.2","s":51735},{"p":"37480.2","s":9111},{"p":"37479.1","s":2004}]}}"#;
        let orderbook = &parse_l2("gate", MarketType::InverseSwap, raw_msg).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 3);
        assert_eq!(orderbook.bids.len(), 3);
        assert!(orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "gate",
            MarketType::InverseSwap,
            "BTC/USD".to_string(),
            orderbook,
        );

        assert_eq!(orderbook.timestamp, 1622682306315);

        assert_eq!(orderbook.asks[0][0], 37481.3);
        assert_eq!(orderbook.asks[0][1], 7766.0 / 37481.3);
        assert_eq!(orderbook.asks[0][2], 7766.0);
        assert_eq!(orderbook.asks[0][3], 7766.0);

        assert_eq!(orderbook.asks[2][0], 37485.1);
        assert_eq!(orderbook.asks[2][1], 2004.0 / 37485.1);
        assert_eq!(orderbook.asks[2][2], 2004.0);
        assert_eq!(orderbook.asks[2][3], 2004.0);

        assert_eq!(orderbook.bids[0][0], 37481.2);
        assert_eq!(orderbook.bids[0][1], 51735.0 / 37481.2);
        assert_eq!(orderbook.bids[0][2], 51735.0);
        assert_eq!(orderbook.bids[0][3], 51735.0);

        assert_eq!(orderbook.bids[2][0], 37479.1);
        assert_eq!(orderbook.bids[2][1], 2004.0 / 37479.1);
        assert_eq!(orderbook.bids[2][2], 2004.0);
        assert_eq!(orderbook.bids[2][3], 2004.0);
    }

    #[test]
    fn linear_swap_snapshot() {
        let raw_msg = r#"{"id":null,"time":1622689062,"channel":"futures.order_book","event":"all","error":null,"result":{"t":1622689062072,"id":4906611559,"contract":"BTC_USDT","asks":[{"p":"37396.5","s":22137},{"p":"37397.3","s":500},{"p":"37401.2","s":790}],"bids":[{"p":"37396.4","s":8553},{"p":"37393.9","s":525},{"p":"37393.6","s":500}]}}"#;
        let orderbook = &parse_l2("gate", MarketType::LinearSwap, raw_msg).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 3);
        assert_eq!(orderbook.bids.len(), 3);
        assert!(orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "gate",
            MarketType::LinearSwap,
            "BTC/USDT".to_string(),
            orderbook,
        );

        assert_eq!(orderbook.timestamp, 1622689062072);

        assert_eq!(orderbook.asks[0][0], 37396.5);
        assert!(approx_eq!(
            f64,
            orderbook.asks[0][1],
            2.2137,
            epsilon = 0.000000000000001
        ));
        assert!(approx_eq!(
            f64,
            orderbook.asks[0][2],
            37396.5 * 2.2137,
            epsilon = 0.0000000001
        ));
        assert_eq!(orderbook.asks[0][3], 22137.0);

        assert_eq!(orderbook.asks[2][0], 37401.2);
        assert_eq!(orderbook.asks[2][1], 0.079);
        assert_eq!(orderbook.asks[2][2], 37401.2 * 0.079);
        assert_eq!(orderbook.asks[2][3], 790.0);

        assert_eq!(orderbook.bids[0][0], 37396.4);
        assert!(approx_eq!(
            f64,
            orderbook.bids[0][1],
            0.8553,
            epsilon = 0.000000000000001
        ));
        assert!(approx_eq!(
            f64,
            orderbook.bids[0][2],
            37396.4 * 0.8553,
            epsilon = 0.001
        ));
        assert_eq!(orderbook.bids[0][3], 8553.0);

        assert_eq!(orderbook.bids[2][0], 37393.6);
        assert!(approx_eq!(
            f64,
            orderbook.bids[2][1],
            0.05,
            epsilon = 0.00000001
        ));
        assert!(approx_eq!(
            f64,
            orderbook.bids[2][2],
            37393.6 * 0.05,
            epsilon = 0.01
        ));
        assert_eq!(orderbook.bids[2][3], 500.0);
    }

    #[test]
    fn linear_future_snapshot() {
        let raw_msg = r#"{"time":1622697760,"channel":"futures.order_book","event":"all","error":null,"result":{"contract":"BTC_USDT_20210625","asks":[{"p":"38624.6","s":500},{"p":"38708.3","s":500},{"p":"38821","s":2000}],"bids":[{"p":"38538","s":500},{"p":"38460","s":500},{"p":"38373","s":2000}]}}"#;
        let orderbook = &parse_l2("gate", MarketType::LinearFuture, raw_msg).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 3);
        assert_eq!(orderbook.bids.len(), 3);
        assert!(orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "gate",
            MarketType::LinearFuture,
            "BTC/USDT".to_string(),
            orderbook,
        );

        assert_eq!(orderbook.timestamp, 1622697760000);

        assert_eq!(orderbook.asks[0][0], 38624.6);
        assert_eq!(orderbook.asks[0][1], 0.05);
        assert_eq!(orderbook.asks[0][2], 38624.6 * 0.05);
        assert_eq!(orderbook.asks[0][3], 500.0);

        assert_eq!(orderbook.asks[2][0], 38821.0);
        assert_eq!(orderbook.asks[2][1], 0.2);
        assert_eq!(orderbook.asks[2][2], 38821.0 * 0.2);
        assert_eq!(orderbook.asks[2][3], 2000.0);

        assert_eq!(orderbook.bids[0][0], 38538.0);
        assert_eq!(orderbook.bids[0][1], 0.05);
        assert_eq!(orderbook.bids[0][2], 38538.0 * 0.05);
        assert_eq!(orderbook.bids[0][3], 500.0);

        assert_eq!(orderbook.bids[2][0], 38373.0);
        assert_eq!(orderbook.bids[2][1], 0.2);
        assert_eq!(orderbook.bids[2][2], 38373.0 * 0.2);
        assert_eq!(orderbook.bids[2][3], 2000.0);
    }
}
