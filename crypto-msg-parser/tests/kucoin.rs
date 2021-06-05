mod utils;

#[cfg(test)]
mod trade {
    use crypto_msg_parser::{parse_trade, MarketType, TradeSide};
    use float_cmp::approx_eq;

    #[test]
    fn spot() {
        let raw_msg = r#"{"data":{"symbol":"BTC-USDT","sequence":"1614503482134","side":"buy","size":"0.00013064","price":"57659.6","takerOrderId":"6057bb821220fc00060f26bf","time":"1616362370760468781","type":"match","makerOrderId":"6057bb81b5ab390006532c9d","tradeId":"6057bb822e113d292396c272"},"subject":"trade.l3match","topic":"/market/match:BTC-USDT","type":"message"}"#;
        let trades = &parse_trade("kucoin", MarketType::Spot, raw_msg).unwrap();

        assert_eq!(trades.len(), 1);
        let trade = &trades[0];

        crate::utils::check_trade_fields("kucoin", MarketType::Spot, "BTC/USDT".to_string(), trade);

        assert_eq!(trade.quantity_base, 0.00013064);
        assert_eq!(trade.quantity_contract, None);
        assert_eq!(trade.side, TradeSide::Buy);
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"data":{"makerUserId":"5e568500eb029b0008715785","symbol":"XBTUSDTM","sequence":8267947,"side":"buy","size":16,"price":57850,"takerOrderId":"6057bc95660a7d0006dc1171","makerOrderId":"6057bc92652ce800067e841a","takerUserId":"601f35b4d42fad0006b2df21","tradeId":"6057bc953c7feb667195bac9","ts":1616362645429686578},"subject":"match","topic":"/contractMarket/execution:XBTUSDTM","type":"message"}"#;
        let trades = &parse_trade("kucoin", MarketType::LinearSwap, raw_msg).unwrap();

        assert_eq!(trades.len(), 1);
        let trade = &trades[0];

        crate::utils::check_trade_fields(
            "kucoin",
            MarketType::LinearSwap,
            "BTC/USDT".to_string(),
            trade,
        );

        assert!(approx_eq!(
            f64,
            trade.quantity_base,
            0.001 * 16.0,
            epsilon = 0.000000001
        ));
        assert!(approx_eq!(
            f64,
            trade.quantity_quote,
            0.016 * 57850.0,
            epsilon = 0.0001
        ));
        assert_eq!(trade.quantity_contract, Some(16.0));
        assert_eq!(trade.side, TradeSide::Buy);
    }

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"data":{"makerUserId":"5d85a240c788c62738732dd9","symbol":"XBTUSDM","sequence":5174061,"side":"buy","size":5000,"price":57798,"takerOrderId":"6057bc692cfab900061f8b11","makerOrderId":"6057bc4df4b11f0006a7743b","takerUserId":"5dba895d134ab72ce156079a","tradeId":"6057bc693c7feb6705f9a248","ts":1616362601277456186},"subject":"match","topic":"/contractMarket/execution:XBTUSDM","type":"message"}"#;
        let trades = &parse_trade("kucoin", MarketType::InverseSwap, raw_msg).unwrap();

        assert_eq!(trades.len(), 1);
        let trade = &trades[0];

        crate::utils::check_trade_fields(
            "kucoin",
            MarketType::InverseSwap,
            "BTC/USD".to_string(),
            trade,
        );

        assert_eq!(trade.quantity_base, 5000.0 / 57798.0);
        assert_eq!(trade.quantity_quote, 5000.0);
        assert_eq!(trade.quantity_contract, Some(5000.0));
        assert_eq!(trade.side, TradeSide::Buy);
    }

    #[test]
    fn inverse_future() {
        let raw_msg = r#"{"data":{"makerUserId":"5f802947889eb80006a1ba0f","symbol":"XBTMH21","sequence":31319,"side":"sell","size":1510,"price":57963.0,"takerOrderId":"6057be2685c6a0000610a89a","makerOrderId":"6057be11652ce800067fafb9","takerUserId":"5f802947889eb80006a1ba0f","tradeId":"6057be2677a0c431d1d1f5b6","ts":1616363046546528915},"subject":"match","topic":"/contractMarket/execution:XBTMH21","type":"message"}"#;
        let trades = &parse_trade("kucoin", MarketType::InverseFuture, raw_msg).unwrap();

        assert_eq!(trades.len(), 1);
        let trade = &trades[0];

        crate::utils::check_trade_fields(
            "kucoin",
            MarketType::InverseFuture,
            "BTC/USD".to_string(),
            trade,
        );

        assert_eq!(trade.quantity_base, 1510.0 / 57963.0);
        assert_eq!(trade.quantity_quote, 1510.0);
        assert_eq!(trade.quantity_contract, Some(1510.0));
        assert_eq!(trade.side, TradeSide::Sell);
    }
}

#[cfg(test)]
mod l2_orderbook {
    use crypto_msg_parser::{parse_l2, MarketType};

    #[test]
    fn spot_update() {
        let raw_msg = r#"{"data":{"sequenceStart":1617071937790,"symbol":"BTC-USDT","changes":{"asks":[],"bids":[["39272","0.0530867","1617071937790"]]},"sequenceEnd":1617071937790},"subject":"trade.l2update","topic":"/market/level2:BTC-USDT","type":"message"}"#;
        let orderbook = &parse_l2("kucoin", MarketType::Spot, raw_msg).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 0);
        assert_eq!(orderbook.bids.len(), 1);
        assert!(!orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "kucoin",
            MarketType::Spot,
            "BTC/USDT".to_string(),
            orderbook,
        );

        assert_eq!(orderbook.bids[0].price, 39272.0);
        assert_eq!(orderbook.bids[0].quantity_base, 0.0530867);
        assert_eq!(orderbook.bids[0].quantity_quote, 39272.0 * 0.0530867);
    }

    #[test]
    fn inverse_swap_update() {
        let raw_msg = r#"{"data":{"sequence":1617852459594,"change":"39069.0,buy,23960","timestamp":1622718985044},"subject":"level2","topic":"/contractMarket/level2:XBTUSDM","type":"message"}"#;
        let orderbook = &parse_l2("kucoin", MarketType::InverseSwap, raw_msg).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 0);
        assert_eq!(orderbook.bids.len(), 1);
        assert!(!orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "kucoin",
            MarketType::InverseSwap,
            "BTC/USD".to_string(),
            orderbook,
        );

        assert_eq!(orderbook.timestamp, 1622718985044);

        assert_eq!(orderbook.bids[0].price, 39069.0);
        assert_eq!(orderbook.bids[0].quantity_base, 23960.0 / 39069.0);
        assert_eq!(orderbook.bids[0].quantity_quote, 23960.0);
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 23960.0);
    }

    #[test]
    fn linear_swap_update() {
        let raw_msg = r#"{"data":{"sequence":1618232029293,"change":"38962.0,buy,4374","timestamp":1622719195286},"subject":"level2","topic":"/contractMarket/level2:XBTUSDTM","type":"message"}"#;
        let orderbook = &parse_l2("kucoin", MarketType::LinearSwap, raw_msg).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 0);
        assert_eq!(orderbook.bids.len(), 1);
        assert!(!orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "kucoin",
            MarketType::LinearSwap,
            "BTC/USDT".to_string(),
            orderbook,
        );

        assert_eq!(orderbook.timestamp, 1622719195286);

        assert_eq!(orderbook.bids[0].price, 38962.0);
        assert_eq!(orderbook.bids[0].quantity_base, 4.374);
        assert_eq!(orderbook.bids[0].quantity_quote, 38962.0 * 4.374);
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 4374.0);
    }

    #[test]
    fn inverse_future_update() {
        let raw_msg = r#"{"data":{"sequence":1616827077941,"change":"39006.0,sell,11450","timestamp":1622719594867},"subject":"level2","topic":"/contractMarket/level2:XBTMM21","type":"message"}"#;
        let orderbook = &parse_l2("kucoin", MarketType::InverseFuture, raw_msg).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 1);
        assert_eq!(orderbook.bids.len(), 0);
        assert!(!orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "kucoin",
            MarketType::InverseFuture,
            "BTC/USD".to_string(),
            orderbook,
        );

        assert_eq!(orderbook.timestamp, 1622719594867);

        assert_eq!(orderbook.asks[0].price, 39006.0);
        assert_eq!(orderbook.asks[0].quantity_base, 11450.0 / 39006.0);
        assert_eq!(orderbook.asks[0].quantity_quote, 11450.0);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 11450.0);
    }
}
