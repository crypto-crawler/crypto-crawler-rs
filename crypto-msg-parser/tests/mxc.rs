mod utils;

#[cfg(test)]
mod trade {
    use crypto_msg_parser::{parse_trade, MarketType, TradeSide};
    use float_cmp::approx_eq;

    #[test]
    fn spot() {
        let raw_msg = r#"["push.symbol",{"symbol":"BTC_USDT","data":{"deals":[{"t":1616373554541,"p":"57005.89","q":"0.007811","T":1}]}}]"#;
        let trades = &parse_trade("mxc", MarketType::Spot, raw_msg).unwrap();

        assert_eq!(trades.len(), 1);
        let trade = &trades[0];

        crate::utils::check_trade_fields("mxc", MarketType::Spot, "BTC/USDT".to_string(), trade);

        assert_eq!(trade.quantity_base, 0.007811);
        assert_eq!(trade.side, TradeSide::Buy);
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"channel":"push.deal","data":{"M":1,"O":3,"T":2,"p":57602,"t":1616370338806,"v":14},"symbol":"BTC_USDT","ts":1616370338806}"#;
        let trades = &parse_trade("mxc", MarketType::LinearSwap, raw_msg).unwrap();

        assert_eq!(trades.len(), 1);
        let trade = &trades[0];

        crate::utils::check_trade_fields(
            "mxc",
            MarketType::LinearSwap,
            "BTC/USDT".to_string(),
            trade,
        );

        assert!(approx_eq!(
            f64,
            trade.quantity_base,
            0.0001 * 14.0,
            epsilon = 0.0000000001
        ));
        assert!(approx_eq!(
            f64,
            trade.quantity_quote,
            0.0001 * 14.0 * 57602.0,
            epsilon = 0.001
        ));
        assert_eq!(trade.quantity_contract, Some(14.0));
        assert_eq!(trade.side, TradeSide::Sell);
    }

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"channel":"push.deal","data":{"M":1,"O":3,"T":1,"p":57476.5,"t":1616370470356,"v":79},"symbol":"BTC_USD","ts":1616370470356}"#;
        let trades = &parse_trade("mxc", MarketType::InverseSwap, raw_msg).unwrap();

        assert_eq!(trades.len(), 1);
        let trade = &trades[0];

        crate::utils::check_trade_fields(
            "mxc",
            MarketType::InverseSwap,
            "BTC/USD".to_string(),
            trade,
        );

        assert!(approx_eq!(
            f64,
            trade.quantity_base,
            79.0 * 100.0 / 57476.5,
            epsilon = 0.0000000001
        ));
        assert_eq!(trade.quantity_quote, 79.0 * 100.0);
        assert_eq!(trade.quantity_contract, Some(79.0));
        assert_eq!(trade.side, TradeSide::Buy);
    }
}

#[cfg(test)]
mod l2_orderbook {
    use crypto_msg_parser::{parse_l2, MarketType};

    #[test]
    fn spot_update() {
        let raw_msg = r#"["push.symbol",{"symbol":"BTC_USDT","data":{"bids":[{"p":"38932.19","q":"0.049010","a":"1908.06663"},{"p":"38931.18","q":"0.038220","a":"1487.94969"}],"asks":[{"p":"38941.81","q":"0.000000","a":"0.00000000"},{"p":"38940.71","q":"0.000000","a":"0.00000000"}]}}]"#;
        let orderbook = &parse_l2("mxc", MarketType::Spot, raw_msg).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 2);
        assert_eq!(orderbook.bids.len(), 2);
        assert!(!orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "mxc",
            MarketType::Spot,
            "BTC/USDT".to_string(),
            orderbook,
        );

        assert_eq!(orderbook.bids[0].price, 38932.19);
        assert_eq!(orderbook.bids[0].quantity_base, 0.04901);
        assert_eq!(orderbook.bids[0].quantity_quote, 1908.06663);
    }

    #[test]
    fn linear_swap_update() {
        let raw_msg = r#"{"channel":"push.depth","data":{"asks":[[38704.5,138686,1]],"bids":[],"version":2427341830},"symbol":"BTC_USDT","ts":1622722473816}"#;
        let orderbook = &parse_l2("mxc", MarketType::LinearSwap, raw_msg).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 1);
        assert_eq!(orderbook.bids.len(), 0);
        assert!(!orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "mxc",
            MarketType::LinearSwap,
            "BTC/USDT".to_string(),
            orderbook,
        );

        assert_eq!(orderbook.timestamp, 1622722473816);

        assert_eq!(orderbook.asks[0].price, 38704.5);
        assert_eq!(orderbook.asks[0].quantity_base, 13.8686);
        assert_eq!(orderbook.asks[0].quantity_quote, 38704.5 * 13.8686);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 138686.0);
    }

    #[test]
    fn inverse_swap_update() {
        let raw_msg = r#"{"channel":"push.depth","data":{"asks":[[38758.5,4172,2]],"bids":[],"version":1151578213},"symbol":"BTC_USD","ts":1622723010000}"#;
        let orderbook = &parse_l2("mxc", MarketType::InverseSwap, raw_msg).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 1);
        assert_eq!(orderbook.bids.len(), 0);
        assert!(!orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "mxc",
            MarketType::InverseSwap,
            "BTC/USD".to_string(),
            orderbook,
        );

        assert_eq!(orderbook.timestamp, 1622723010000);

        assert_eq!(orderbook.asks[0].price, 38758.5);
        assert_eq!(orderbook.asks[0].quantity_base, 417200.0 / 38758.5);
        assert_eq!(orderbook.asks[0].quantity_quote, 417200.0);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 4172.0);
    }
}
