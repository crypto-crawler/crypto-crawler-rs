mod utils;

#[cfg(test)]
mod trade {
    use crypto_msg_parser::{parse_trade, MarketType, TradeSide};

    #[test]
    fn inverse_future() {
        let raw_msg = r#"{"topic":"trade.BTCUSDM21","data":[{"trade_time_ms":1616304614117,"timestamp":"2021-03-21T05:30:14.000Z","symbol":"BTCUSDM21","side":"Buy","size":100,"price":61094.5,"tick_direction":"ZeroPlusTick","trade_id":"e61fb2dc-a658-5a7d-88fb-d166a4bd29b8","cross_seq":233452601},{"trade_time_ms":1616304614117,"timestamp":"2021-03-21T05:30:14.000Z","symbol":"BTCUSDM21","side":"Sell","size":300,"price":61097.5,"tick_direction":"ZeroPlusTick","trade_id":"2cbeff0d-16da-5946-a7b0-0ccfb78d3ab5","cross_seq":233452601}]}"#;
        let trades = &parse_trade("bybit", MarketType::InverseFuture, raw_msg).unwrap();

        assert_eq!(trades.len(), 2);

        for trade in trades.iter() {
            crate::utils::check_trade_fields(
                "bybit",
                MarketType::InverseFuture,
                "BTC/USD".to_string(),
                trade,
            );
        }

        assert_eq!(trades[0].quantity_base, 100.0 / 61094.5);
        assert_eq!(trades[0].quantity_quote, 100.0);
        assert_eq!(trades[0].quantity_contract, Some(100.0));
        assert_eq!(trades[0].side, TradeSide::Buy);

        assert_eq!(trades[1].quantity_base, 300.0 / 61097.5);
        assert_eq!(trades[1].quantity_quote, 300.0);
        assert_eq!(trades[1].quantity_contract, Some(300.0));
        assert_eq!(trades[1].side, TradeSide::Sell);
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

        assert_eq!(trade.quantity_base, 237.0 / 57073.5);
        assert_eq!(trade.quantity_quote, 237.0);
        assert_eq!(trade.quantity_contract, Some(237.0));
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

        assert_eq!(trade.quantity_base, 0.04);
        assert_eq!(trade.quantity_quote, 0.04 * 57170.0);
        assert_eq!(trade.quantity_contract, Some(0.04));
        assert_eq!(trade.side, TradeSide::Buy);
    }
}

#[cfg(test)]
mod l2_orderbook {
    use crypto_msg_parser::{parse_l2, MarketType};

    #[test]
    fn inverse_future_snapshot() {
        let raw_msg = r#"{"topic":"orderBookL2_25.BTCUSDM21","type":"snapshot","data":[{"price":"36338.50","symbol":"BTCUSDM21","id":363385000,"side":"Buy","size":85235},{"price":"36344.50","symbol":"BTCUSDM21","id":363445000,"side":"Buy","size":1947},{"price":"36346.00","symbol":"BTCUSDM21","id":363460000,"side":"Buy","size":234},{"price":"36400.00","symbol":"BTCUSDM21","id":364000000,"side":"Sell","size":12500},{"price":"36407.50","symbol":"BTCUSDM21","id":364075000,"side":"Sell","size":21460},{"price":"36408.00","symbol":"BTCUSDM21","id":364080000,"side":"Sell","size":40076}],"cross_seq":2573025748,"timestamp_e6":1622538339073398}"#;
        let orderbook = &parse_l2("bybit", MarketType::InverseFuture, raw_msg).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 3);
        assert_eq!(orderbook.bids.len(), 3);
        assert!(orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "bybit",
            MarketType::InverseFuture,
            "BTC/USD".to_string(),
            orderbook,
        );

        assert_eq!(orderbook.timestamp, 1622538339073);

        assert_eq!(orderbook.bids[0].price, 36338.5);
        assert_eq!(orderbook.bids[0].quantity_base, 85235.0 / 36338.5);
        assert_eq!(orderbook.bids[0].quantity_quote, 85235.0);
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 85235.0);

        assert_eq!(orderbook.bids[2].price, 36346.0);
        assert_eq!(orderbook.bids[2].quantity_base, 234.0 / 36346.0);
        assert_eq!(orderbook.bids[2].quantity_quote, 234.0);
        assert_eq!(orderbook.bids[2].quantity_contract.unwrap(), 234.0);

        assert_eq!(orderbook.asks[0].price, 36400.0);
        assert_eq!(orderbook.asks[0].quantity_base, 12500.0 / 36400.0);
        assert_eq!(orderbook.asks[0].quantity_quote, 12500.0);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 12500.0);

        assert_eq!(orderbook.asks[2].price, 36408.0);
        assert_eq!(orderbook.asks[2].quantity_base, 40076.0 / 36408.0);
        assert_eq!(orderbook.asks[2].quantity_quote, 40076.0);
        assert_eq!(orderbook.asks[2].quantity_contract.unwrap(), 40076.0);
    }

    #[test]
    fn inverse_future_update() {
        let raw_msg = r#"{"topic":"orderBookL2_25.BTCUSDM21","type":"delta","data":{"delete":[{"price":"36382.50","symbol":"BTCUSDM21","id":363825000,"side":"Buy","size":0}],"update":[{"price":"36401.50","symbol":"BTCUSDM21","id":364015000,"side":"Buy","size":19133}],"insert":[{"price":"36382.00","symbol":"BTCUSDM21","id":363820000,"side":"Buy","size":30067}],"transactTimeE6":0},"cross_seq":2573877429,"timestamp_e6":1622540847513498}"#;
        let orderbook = &parse_l2("bybit", MarketType::InverseFuture, raw_msg).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 0);
        assert_eq!(orderbook.bids.len(), 3);
        assert!(!orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "bybit",
            MarketType::InverseFuture,
            "BTC/USD".to_string(),
            orderbook,
        );

        assert_eq!(orderbook.timestamp, 1622540847513);

        assert_eq!(orderbook.bids[0].price, 36382.5);
        assert_eq!(orderbook.bids[0].quantity_base, 0.0);
        assert_eq!(orderbook.bids[0].quantity_quote, 0.0);
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 0.0);

        assert_eq!(orderbook.bids[1].price, 36401.5);
        assert_eq!(orderbook.bids[1].quantity_base, 19133.0 / 36401.5);
        assert_eq!(orderbook.bids[1].quantity_quote, 19133.0);
        assert_eq!(orderbook.bids[1].quantity_contract.unwrap(), 19133.0);

        assert_eq!(orderbook.bids[2].price, 36382.0);
        assert_eq!(orderbook.bids[2].quantity_base, 30067.0 / 36382.0);
        assert_eq!(orderbook.bids[2].quantity_quote, 30067.0);
        assert_eq!(orderbook.bids[2].quantity_contract.unwrap(), 30067.0);
    }

    #[test]
    fn inverse_swap_snapshot() {
        let raw_msg = r#"{"topic":"orderBookL2_25.BTCUSD","type":"snapshot","data":[{"price":"36409.50","symbol":"BTCUSD","id":364095000,"side":"Buy","size":68602},{"price":"36410.00","symbol":"BTCUSD","id":364100000,"side":"Buy","size":89497},{"price":"36410.50","symbol":"BTCUSD","id":364105000,"side":"Buy","size":73496},{"price":"36424.50","symbol":"BTCUSD","id":364245000,"side":"Sell","size":4271363},{"price":"36425.00","symbol":"BTCUSD","id":364250000,"side":"Sell","size":1},{"price":"36425.50","symbol":"BTCUSD","id":364255000,"side":"Sell","size":604}],"cross_seq":7407067519,"timestamp_e6":1622542809357177}"#;
        let orderbook = &parse_l2("bybit", MarketType::InverseFuture, raw_msg).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 3);
        assert_eq!(orderbook.bids.len(), 3);
        assert!(orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "bybit",
            MarketType::InverseFuture,
            "BTC/USD".to_string(),
            orderbook,
        );

        assert_eq!(orderbook.timestamp, 1622542809357);

        assert_eq!(orderbook.bids[0].price, 36409.5);
        assert_eq!(orderbook.bids[0].quantity_base, 68602.0 / 36409.5);
        assert_eq!(orderbook.bids[0].quantity_quote, 68602.0);
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 68602.0);

        assert_eq!(orderbook.bids[2].price, 36410.5);
        assert_eq!(orderbook.bids[2].quantity_base, 73496.0 / 36410.5);
        assert_eq!(orderbook.bids[2].quantity_quote, 73496.0);
        assert_eq!(orderbook.bids[2].quantity_contract.unwrap(), 73496.0);

        assert_eq!(orderbook.asks[0].price, 36424.5);
        assert_eq!(orderbook.asks[0].quantity_base, 4271363.0 / 36424.5);
        assert_eq!(orderbook.asks[0].quantity_quote, 4271363.0);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 4271363.0);

        assert_eq!(orderbook.asks[2].price, 36425.5);
        assert_eq!(orderbook.asks[2].quantity_base, 604.0 / 36425.5);
        assert_eq!(orderbook.asks[2].quantity_quote, 604.0);
        assert_eq!(orderbook.asks[2].quantity_contract.unwrap(), 604.0);
    }

    #[test]
    fn inverse_swap_update() {
        let raw_msg = r#"{"topic":"orderBookL2_25.BTCUSD","type":"delta","data":{"delete":[{"price":"36427.00","symbol":"BTCUSD","id":364270000,"side":"Sell"}],"update":[{"price":"36424.50","symbol":"BTCUSD","id":364245000,"side":"Sell","size":4271098}],"insert":[{"price":"36438.50","symbol":"BTCUSD","id":364385000,"side":"Sell","size":169932}],"transactTimeE6":0},"cross_seq":7407067525,"timestamp_e6":1622542809497981}"#;
        let orderbook = &parse_l2("bybit", MarketType::InverseFuture, raw_msg).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 3);
        assert_eq!(orderbook.bids.len(), 0);
        assert!(!orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "bybit",
            MarketType::InverseFuture,
            "BTC/USD".to_string(),
            orderbook,
        );

        assert_eq!(orderbook.timestamp, 1622542809497);

        assert_eq!(orderbook.asks[0].price, 36427.0);
        assert_eq!(orderbook.asks[0].quantity_base, 0.0);
        assert_eq!(orderbook.asks[0].quantity_quote, 0.0);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 0.0);

        assert_eq!(orderbook.asks[1].price, 36424.5);
        assert_eq!(orderbook.asks[1].quantity_base, 4271098.0 / 36424.5);
        assert_eq!(orderbook.asks[1].quantity_quote, 4271098.0);
        assert_eq!(orderbook.asks[1].quantity_contract.unwrap(), 4271098.0);

        assert_eq!(orderbook.asks[2].price, 36438.5);
        assert_eq!(orderbook.asks[2].quantity_base, 169932.0 / 36438.5);
        assert_eq!(orderbook.asks[2].quantity_quote, 169932.0);
        assert_eq!(orderbook.asks[2].quantity_contract.unwrap(), 169932.0);
    }

    #[test]
    fn linear_swap_snapshot() {
        let raw_msg = r#"{"topic":"orderBookL2_25.BTCUSDT","type":"snapshot","data":{"order_book":[{"price":"36385.50","symbol":"BTCUSDT","id":"363855000","side":"Buy","size":6.457},{"price":"36386.00","symbol":"BTCUSDT","id":"363860000","side":"Buy","size":8.3550005},{"price":"36386.50","symbol":"BTCUSDT","id":"363865000","side":"Buy","size":5.93},{"price":"36400.00","symbol":"BTCUSDT","id":"364000000","side":"Sell","size":13.931001},{"price":"36400.50","symbol":"BTCUSDT","id":"364005000","side":"Sell","size":9.754},{"price":"36401.00","symbol":"BTCUSDT","id":"364010000","side":"Sell","size":5.426}]},"cross_seq":"5737626212","timestamp_e6":"1622543529282954"}"#;
        let orderbook = &parse_l2("bybit", MarketType::LinearSwap, raw_msg).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 3);
        assert_eq!(orderbook.bids.len(), 3);
        assert!(orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "bybit",
            MarketType::LinearSwap,
            "BTC/USDT".to_string(),
            orderbook,
        );

        assert_eq!(orderbook.timestamp, 1622543529282);

        assert_eq!(orderbook.bids[0].price, 36385.5);
        assert_eq!(orderbook.bids[0].quantity_base, 6.457);
        assert_eq!(orderbook.bids[0].quantity_quote, 36385.5 * 6.457);
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 6.457);

        assert_eq!(orderbook.bids[2].price, 36386.5);
        assert_eq!(orderbook.bids[2].quantity_base, 5.93);
        assert_eq!(orderbook.bids[2].quantity_quote, 36386.5 * 5.93);
        assert_eq!(orderbook.bids[2].quantity_contract.unwrap(), 5.93);

        assert_eq!(orderbook.asks[0].price, 36400.0);
        assert_eq!(orderbook.asks[0].quantity_base, 13.931001);
        assert_eq!(orderbook.asks[0].quantity_quote, 36400.0 * 13.931001);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 13.931001);

        assert_eq!(orderbook.asks[2].price, 36401.0);
        assert_eq!(orderbook.asks[2].quantity_base, 5.426);
        assert_eq!(orderbook.asks[2].quantity_quote, 36401.0 * 5.426);
        assert_eq!(orderbook.asks[2].quantity_contract.unwrap(), 5.426);
    }

    #[test]
    fn linear_swap_update() {
        let raw_msg = r#"{"topic":"orderBookL2_25.BTCUSDT","type":"delta","data":{"delete":[{"price":"36397.50","symbol":"BTCUSDT","id":"363975000","side":"Sell"}],"update":[{"price":"36381.50","symbol":"BTCUSDT","id":"363815000","side":"Buy","size":6.906}],"insert":[{"price":"36407.00","symbol":"BTCUSDT","id":"364070000","side":"Sell","size":4.96}]},"cross_seq":"5737704047","timestamp_e6":"1622544088904367"}"#;
        let orderbook = &parse_l2("bybit", MarketType::LinearSwap, raw_msg).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 2);
        assert_eq!(orderbook.bids.len(), 1);
        assert!(!orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "bybit",
            MarketType::LinearSwap,
            "BTC/USDT".to_string(),
            orderbook,
        );

        assert_eq!(orderbook.timestamp, 1622544088904);

        assert_eq!(orderbook.asks[0].price, 36397.5);
        assert_eq!(orderbook.asks[0].quantity_base, 0.0);
        assert_eq!(orderbook.asks[0].quantity_quote, 0.0);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 0.0);

        assert_eq!(orderbook.asks[1].price, 36407.0);
        assert_eq!(orderbook.asks[1].quantity_base, 4.96);
        assert_eq!(orderbook.asks[1].quantity_quote, 36407.0 * 4.96);
        assert_eq!(orderbook.asks[1].quantity_contract.unwrap(), 4.96);

        assert_eq!(orderbook.bids[0].price, 36381.5);
        assert_eq!(orderbook.bids[0].quantity_base, 6.906);
        assert_eq!(orderbook.bids[0].quantity_quote, 36381.5 * 6.906);
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 6.906);
    }
}
