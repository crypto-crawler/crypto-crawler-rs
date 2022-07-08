mod utils;

const EXCHANGE_NAME: &str = "bybit";

#[cfg(test)]
mod trade {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_message::TradeSide;
    use crypto_msg_parser::{extract_symbol, extract_timestamp, parse_trade};

    #[test]
    fn inverse_future() {
        let raw_msg = r#"{"topic":"trade.BTCUSDM21","data":[{"trade_time_ms":1616304614117,"timestamp":"2021-03-21T05:30:14.000Z","symbol":"BTCUSDM21","side":"Buy","size":100,"price":61094.5,"tick_direction":"ZeroPlusTick","trade_id":"e61fb2dc-a658-5a7d-88fb-d166a4bd29b8","cross_seq":233452601},{"trade_time_ms":1616304614117,"timestamp":"2021-03-21T05:30:14.000Z","symbol":"BTCUSDM21","side":"Sell","size":300,"price":61097.5,"tick_direction":"ZeroPlusTick","trade_id":"2cbeff0d-16da-5946-a7b0-0ccfb78d3ab5","cross_seq":233452601}]}"#;
        let trades = &parse_trade(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg).unwrap();

        assert_eq!(trades.len(), 2);

        for trade in trades.iter() {
            crate::utils::check_trade_fields(
                EXCHANGE_NAME,
                MarketType::InverseFuture,
                "BTC/USD".to_string(),
                extract_symbol(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg).unwrap(),
                trade,
                raw_msg,
            );
        }
        assert_eq!(
            1616304614117,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg)
                .unwrap()
                .unwrap()
        );

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
        let trades = &parse_trade(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap();

        assert_eq!(trades.len(), 1);

        let trade = &trades[0];

        crate::utils::check_trade_fields(
            EXCHANGE_NAME,
            MarketType::InverseSwap,
            "BTC/USD".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap(),
            trade,
            raw_msg,
        );
        assert_eq!(
            1616304710061,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(trade.quantity_base, 237.0 / 57073.5);
        assert_eq!(trade.quantity_quote, 237.0);
        assert_eq!(trade.quantity_contract, Some(237.0));
        assert_eq!(trade.side, TradeSide::Buy);
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"topic":"trade.BTCUSDT","data":[{"symbol":"BTCUSDT","tick_direction":"ZeroPlusTick","price":"57170.00","size":0.04,"timestamp":"2021-03-21T05:32:17.000Z","trade_time_ms":"1616304737092","side":"Buy","trade_id":"fe9ef57c-2571-5728-847b-7bc039b6b52d"}]}"#;
        let trades = &parse_trade(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap();

        assert_eq!(trades.len(), 1);

        let trade = &trades[0];

        crate::utils::check_trade_fields(
            EXCHANGE_NAME,
            MarketType::LinearSwap,
            "BTC/USDT".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap(),
            trade,
            raw_msg,
        );
        assert_eq!(
            1616304737092,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(trade.quantity_base, 0.04);
        assert_eq!(trade.quantity_quote, 0.04 * 57170.0);
        assert_eq!(trade.quantity_contract, Some(0.04));
        assert_eq!(trade.side, TradeSide::Buy);
    }
}

#[cfg(test)]
mod l2_orderbook {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, extract_timestamp, parse_l2, round};
    use crypto_msg_type::MessageType;

    #[test]
    fn inverse_future_snapshot() {
        let raw_msg = r#"{"topic":"orderBookL2_25.BTCUSDM21","type":"snapshot","data":[{"price":"36338.50","symbol":"BTCUSDM21","id":363385000,"side":"Buy","size":85235},{"price":"36344.50","symbol":"BTCUSDM21","id":363445000,"side":"Buy","size":1947},{"price":"36346.00","symbol":"BTCUSDM21","id":363460000,"side":"Buy","size":234},{"price":"36400.00","symbol":"BTCUSDM21","id":364000000,"side":"Sell","size":12500},{"price":"36407.50","symbol":"BTCUSDM21","id":364075000,"side":"Sell","size":21460},{"price":"36408.00","symbol":"BTCUSDM21","id":364080000,"side":"Sell","size":40076}],"cross_seq":2573025748,"timestamp_e6":1622538339073398}"#;
        let orderbook =
            &parse_l2(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 3);
        assert_eq!(orderbook.bids.len(), 3);
        assert!(orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            EXCHANGE_NAME,
            MarketType::InverseFuture,
            MessageType::L2Event,
            "BTC/USD".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );
        assert_eq!(
            1622538339073,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1622538339073);

        assert_eq!(orderbook.bids[2].price, 36338.5);
        assert_eq!(orderbook.bids[2].quantity_base, 85235.0 / 36338.5);
        assert_eq!(orderbook.bids[2].quantity_quote, 85235.0);
        assert_eq!(orderbook.bids[2].quantity_contract.unwrap(), 85235.0);

        assert_eq!(orderbook.bids[0].price, 36346.0);
        assert_eq!(orderbook.bids[0].quantity_base, 234.0 / 36346.0);
        assert_eq!(orderbook.bids[0].quantity_quote, 234.0);
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 234.0);

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
        let orderbook =
            &parse_l2(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 0);
        assert_eq!(orderbook.bids.len(), 3);
        assert!(!orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            EXCHANGE_NAME,
            MarketType::InverseFuture,
            MessageType::L2Event,
            "BTC/USD".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );
        assert_eq!(
            1622540847513,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg)
                .unwrap()
                .unwrap()
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
        let orderbook =
            &parse_l2(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 3);
        assert_eq!(orderbook.bids.len(), 3);
        assert!(orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            EXCHANGE_NAME,
            MarketType::InverseFuture,
            MessageType::L2Event,
            "BTC/USD".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );
        assert_eq!(
            1622542809357,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1622542809357);

        assert_eq!(orderbook.bids[2].price, 36409.5);
        assert_eq!(orderbook.bids[2].quantity_base, 68602.0 / 36409.5);
        assert_eq!(orderbook.bids[2].quantity_quote, 68602.0);
        assert_eq!(orderbook.bids[2].quantity_contract.unwrap(), 68602.0);

        assert_eq!(orderbook.bids[0].price, 36410.5);
        assert_eq!(orderbook.bids[0].quantity_base, 73496.0 / 36410.5);
        assert_eq!(orderbook.bids[0].quantity_quote, 73496.0);
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 73496.0);

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
        let orderbook =
            &parse_l2(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 3);
        assert_eq!(orderbook.bids.len(), 0);
        assert!(!orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            EXCHANGE_NAME,
            MarketType::InverseFuture,
            MessageType::L2Event,
            "BTC/USD".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );
        assert_eq!(
            1622542809497,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg)
                .unwrap()
                .unwrap()
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
        let orderbook = &parse_l2(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 3);
        assert_eq!(orderbook.bids.len(), 3);
        assert!(orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            EXCHANGE_NAME,
            MarketType::LinearSwap,
            MessageType::L2Event,
            "BTC/USDT".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );
        assert_eq!(
            1622543529282,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1622543529282);

        assert_eq!(orderbook.bids[2].price, 36385.5);
        assert_eq!(orderbook.bids[2].quantity_base, 6.457);
        assert_eq!(orderbook.bids[2].quantity_quote, 36385.5 * 6.457);
        assert_eq!(orderbook.bids[2].quantity_contract.unwrap(), 6.457);

        assert_eq!(orderbook.bids[0].price, 36386.5);
        assert_eq!(orderbook.bids[0].quantity_base, 5.93);
        assert_eq!(orderbook.bids[0].quantity_quote, round(36386.5 * 5.93));
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 5.93);

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
        let orderbook = &parse_l2(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 2);
        assert_eq!(orderbook.bids.len(), 1);
        assert!(!orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            EXCHANGE_NAME,
            MarketType::LinearSwap,
            MessageType::L2Event,
            "BTC/USDT".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );
        assert_eq!(
            1622544088904,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
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

#[cfg(test)]
mod candlestick {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, extract_timestamp};

    #[test]
    fn inverse_future() {
        let raw_msg = r#"{"topic":"klineV2.1.BTCUSDM22","data":[{"start":1654078440,"end":1654078500,"open":31633,"close":31632,"high":31633,"low":31629,"volume":1250,"turnover":0.0395179,"confirm":false,"cross_seq":8475023823,"timestamp":1654078470426793}],"timestamp_e6":1654078470426793}"#;

        assert_eq!(
            1654078470426,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "BTCUSDM22",
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );
    }

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"topic":"klineV2.1.BTCUSD","data":[{"start":1654078800,"end":1654078860,"open":31570.5,"close":31570.5,"high":31571,"low":31570.5,"volume":10384,"turnover":0.32891023,"confirm":false,"cross_seq":13442847589,"timestamp":1654078824173072}],"timestamp_e6":1654078824173072}"#;

        assert_eq!(
            1654078824173,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "BTCUSD",
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"topic":"candle.1.BTCUSDT","data":[{"start":1654078860,"end":1654078920,"period":"1","open":31604.5,"close":31605,"high":31605,"low":31604.5,"volume":"1.926","turnover":"60871.1915","confirm":false,"cross_seq":12201260080,"timestamp":1654078873190426}],"timestamp_e6":1654078873190426}"#;

        assert_eq!(
            1654078873190,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "BTCUSDT",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );
    }
}

#[cfg(test)]
mod ticker {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, extract_timestamp};

    #[test]
    fn inverse_future_snapshot() {
        let raw_msg = r#"{"topic":"instrument_info.100ms.BTCUSDM22","type":"snapshot","data":{"id":10,"symbol":"BTCUSDM22","symbol_name":"BTCUSD0624","symbol_year":2022,"contract_type":"InverseFutures","coin":"BTC","quote_symbol":"BTCUSD","mode":"BothSide","is_up_borrowable":0,"import_time_e9":0,"start_trading_time_e9":1639699200000000000,"time_to_settle":1896310,"settle_time_e9":1656057600000000000,"settle_fee_rate_e8":50000,"contract_status":"Trading","system_subsidy_e8":0,"last_price_e4":299170000,"last_price":"29917.00","last_tick_direction":"MinusTick","bid1_price_e4":299145000,"bid1_price":"29914.50","ask1_price_e4":299150000,"ask1_price":"29915.00","prev_price_24h_e4":316080000,"prev_price_24h":"31608.00","price_24h_pcnt_e6":-53499,"high_price_24h_e4":318875000,"high_price_24h":"31887.50","low_price_24h_e4":292635000,"low_price_24h":"29263.50","prev_price_1h_e4":299150000,"prev_price_1h":"29915.00","price_1h_pcnt_e6":66,"mark_price_e4":299173200,"mark_price":"29917.32","index_price_e4":299345200,"index_price":"29934.52","open_interest":70172094,"open_value_e8":0,"total_turnover_e8":27128037154787,"turnover_24h_e8":365962616379,"total_volume":10511364035,"volume_24h":110918371,"fair_basis_e8":-1752000000,"fair_basis_rate_e8":-66712,"basis_in_year_e8":-762899,"expect_price_e4":0,"expect_price":"0.00","cross_seq":8485553665,"created_at_e9":0,"updated_at_e9":1654161286545264000},"cross_seq":8485553994,"timestamp_e6":1654161290233830}"#;

        assert_eq!(
            1654161290233,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "BTCUSDM22",
            extract_symbol(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg).unwrap()
        );
    }

    #[test]
    fn inverse_future_update() {
        let raw_msg = r#"{"topic":"instrument_info.100ms.BTCUSDM22","type":"delta","data":{"delete":[],"update":[{"id":10,"symbol":"BTCUSDM22","symbol_name":"BTCUSD0624","symbol_year":2022,"contract_type":"InverseFutures","coin":"BTC","quote_symbol":"BTCUSD","mode":"BothSide","start_trading_time_e9":1639699200000000000,"time_to_settle":1896309,"settle_time_e9":1656057600000000000,"mark_price_e4":299142000,"mark_price":"29914.20","index_price_e4":299313700,"index_price":"29931.37","fair_basis_e8":-1687000000,"fair_basis_rate_e8":-55025,"expect_price":"0.00","cross_seq":8485554044}],"insert":[]},"cross_seq":8485554159,"timestamp_e6":1654161291734100}"#;

        assert_eq!(
            1654161291734,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "BTCUSDM22",
            extract_symbol(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg).unwrap()
        );
    }

    #[test]
    fn inverse_swap_snapshot() {
        let raw_msg = r#"{"topic":"instrument_info.100ms.BTCUSD","type":"snapshot","data":{"id":1,"symbol":"BTCUSD","last_price_e4":299305000,"last_price":"29930.50","bid1_price_e4":299305000,"bid1_price":"29930.50","ask1_price_e4":299310000,"ask1_price":"29931.00","last_tick_direction":"ZeroMinusTick","prev_price_24h_e4":315895000,"prev_price_24h":"31589.50","price_24h_pcnt_e6":-52517,"high_price_24h_e4":318740000,"high_price_24h":"31874.00","low_price_24h_e4":292780000,"low_price_24h":"29278.00","prev_price_1h_e4":299600000,"prev_price_1h":"29960.00","price_1h_pcnt_e6":-984,"mark_price_e4":299463400,"mark_price":"29946.34","index_price_e4":299461300,"index_price":"29946.13","open_interest":654525102,"open_value_e8":1461680310351,"total_turnover_e8":10586114469373775,"turnover_24h_e8":5730317572180,"total_volume":2747007694755,"volume_24h":1745270625,"funding_rate_e6":8,"predicted_funding_rate_e6":-126,"cross_seq":13458633262,"created_at":"2018-11-14T16:33:26Z","updated_at":"2022-06-02T09:18:34Z","next_funding_time":"2022-06-02T16:00:00Z","countdown_hour":7,"funding_rate_interval":8,"settle_time_e9":0,"delisting_status":"0"},"cross_seq":13458633487,"timestamp_e6":1654161517001968}"#;

        assert_eq!(
            1654161517001,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "BTCUSD",
            extract_symbol(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap()
        );
    }

    #[test]
    fn inverse_swap_update() {
        let raw_msg = r#"{"topic":"instrument_info.100ms.BTCUSD","type":"delta","data":{"delete":[],"update":[{"id":1,"symbol":"BTCUSD","price_24h_pcnt_e6":-52517,"price_1h_pcnt_e6":-984,"total_turnover_e8":10586114472674754,"turnover_24h_e8":5730320873159,"total_volume":2747007695743,"volume_24h":1745271613,"cross_seq":13458633508,"created_at":"2018-11-14T16:33:26Z","updated_at":"2022-06-02T09:18:37Z"}],"insert":[]},"cross_seq":13458633510,"timestamp_e6":1654161517201318}"#;

        assert_eq!(
            1654161517201,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "BTCUSD",
            extract_symbol(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap()
        );
    }

    #[test]
    fn linear_swap_snapshot() {
        let raw_msg = r#"{"topic":"instrument_info.100ms.BTCUSDT","type":"snapshot","data":{"id":1,"symbol":"BTCUSDT","last_price_e4":"299590000","last_price":"29959.00","bid1_price_e4":"299585000","bid1_price":"29958.50","ask1_price_e4":"299590000","ask1_price":"29959.00","last_tick_direction":"ZeroPlusTick","prev_price_24h_e4":"315980000","prev_price_24h":"31598.00","price_24h_pcnt_e6":"-51870","high_price_24h_e4":"318945000","high_price_24h":"31894.50","low_price_24h_e4":"292910000","low_price_24h":"29291.00","prev_price_1h_e4":"299735000","prev_price_1h":"29973.50","price_1h_pcnt_e6":"-483","mark_price_e4":"299770300","mark_price":"29977.03","index_price_e4":"299781200","index_price":"29978.12","open_interest_e8":"2818586500000","total_turnover_e8":"1313857544013650000","turnover_24h_e8":"508351634095750100","total_volume_e8":"3500773405099924","volume_24h_e8":"16697641599999","funding_rate_e6":"-43","predicted_funding_rate_e6":"-87","cross_seq":"12230809673","created_at":"1970-01-01T00:00:00.000Z","updated_at":"2022-06-02T09:19:37.000Z","next_funding_time":"2022-06-02T16:00:00Z","count_down_hour":"7","funding_rate_interval":"8","settle_time_e9":"0","delisting_status":"0"},"cross_seq":"12230809709","timestamp_e6":"1654161577978011"}"#;

        assert_eq!(
            1654161577978,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "BTCUSDT",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );
    }

    #[test]
    fn linear_swap_update() {
        let raw_msg = r#"{"topic":"instrument_info.100ms.BTCUSDT","type":"delta","data":{"update":[{"id":1,"symbol":"BTCUSDT","index_price_e4":"299781300","index_price":"29978.13","cross_seq":"12230809673","created_at":"1970-01-01T00:00:00.000Z","updated_at":"2022-06-02T09:19:37.000Z"}]},"cross_seq":"12230809787","timestamp_e6":"1654161578478930"}"#;

        assert_eq!(
            1654161578478,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "BTCUSDT",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );
    }
}

#[cfg(test)]
mod l2_snapshot {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, extract_timestamp};

    #[test]
    fn inverse_future() {
        let raw_msg = r#"{"ret_code":0,"ret_msg":"OK","ext_code":"","ext_info":"","result":[{"symbol":"BTCUSDM22","price":"30489","size":58149,"side":"Buy"},{"symbol":"BTCUSDM22","price":"30488.5","size":32603,"side":"Buy"},{"symbol":"BTCUSDM22","price":"30488","size":21611,"side":"Buy"},{"symbol":"BTCUSDM22","price":"30489.5","size":25574,"side":"Sell"},{"symbol":"BTCUSDM22","price":"30490","size":15317,"side":"Sell"},{"symbol":"BTCUSDM22","price":"30490.5","size":9282,"side":"Sell"}],"time_now":"1654244100.527475"}"#;

        assert_eq!(
            "BTCUSDM22",
            extract_symbol(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg).unwrap()
        );

        assert_eq!(
            1654244100527,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg)
                .unwrap()
                .unwrap()
        );
    }

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"ret_code":0,"ret_msg":"OK","ext_code":"","ext_info":"","result":[{"symbol":"BTCUSD","price":"30440","size":31756,"side":"Buy"},{"symbol":"BTCUSD","price":"30436","size":13371,"side":"Buy"},{"symbol":"BTCUSD","price":"30434","size":4783,"side":"Buy"},{"symbol":"BTCUSD","price":"30440.5","size":666758,"side":"Sell"},{"symbol":"BTCUSD","price":"30441.5","size":35117,"side":"Sell"},{"symbol":"BTCUSD","price":"30442.5","size":20100,"side":"Sell"}],"time_now":"1654245002.615582"}"#;

        assert_eq!(
            "BTCUSD",
            extract_symbol(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap()
        );

        assert_eq!(
            1654245002615,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"ret_code":0,"ret_msg":"OK","ext_code":"","ext_info":"","result":[{"symbol":"BTCUSDT","price":"30453.5","size":16.872,"side":"Buy"},{"symbol":"BTCUSDT","price":"30453","size":0.01,"side":"Buy"},{"symbol":"BTCUSDT","price":"30451.5","size":0.66,"side":"Buy"},{"symbol":"BTCUSDT","price":"30454","size":25.093,"side":"Sell"},{"symbol":"BTCUSDT","price":"30454.5","size":0.309,"side":"Sell"},{"symbol":"BTCUSDT","price":"30455","size":1.12,"side":"Sell"}],"time_now":"1654245012.731544"}"#;

        assert_eq!(
            1654245012731,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "BTCUSDT",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );
    }
}

#[cfg(test)]
mod open_interest {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, extract_timestamp};

    #[test]
    fn inverse_future() {
        let raw_msg = r#"{"ret_code":0,"ret_msg":"OK","ext_code":"","ext_info":"","result":[{"open_interest":73900363,"timestamp":1654338300,"symbol":"BTCUSDM22"},{"open_interest":73897361,"timestamp":1654338000,"symbol":"BTCUSDM22"},{"open_interest":73850664,"timestamp":1654337700,"symbol":"BTCUSDM22"}],"time_now":"1654338598.173452"}"#;

        assert_eq!(
            "BTCUSDM22",
            extract_symbol(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg).unwrap()
        );

        assert_eq!(
            1654338598173,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg)
                .unwrap()
                .unwrap()
        );
    }

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"ret_code":0,"ret_msg":"OK","ext_code":"","ext_info":"","result":[{"open_interest":645245219,"timestamp":1654338300,"symbol":"BTCUSD"},{"open_interest":645240649,"timestamp":1654338000,"symbol":"BTCUSD"},{"open_interest":643893467,"timestamp":1654337700,"symbol":"BTCUSD"}],"time_now":"1654338600.495296"}"#;

        assert_eq!(
            "BTCUSD",
            extract_symbol(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap()
        );

        assert_eq!(
            1654338600495,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"ret_code":0,"ret_msg":"OK","ext_code":"","ext_info":"","result":[{"symbol":"BTCUSDT","price":"30453.5","size":16.872,"side":"Buy"},{"symbol":"BTCUSDT","price":"30453","size":0.01,"side":"Buy"},{"symbol":"BTCUSDT","price":"30451.5","size":0.66,"side":"Buy"},{"symbol":"BTCUSDT","price":"30454","size":25.093,"side":"Sell"},{"symbol":"BTCUSDT","price":"30454.5","size":0.309,"side":"Sell"},{"symbol":"BTCUSDT","price":"30455","size":1.12,"side":"Sell"}],"time_now":"1654245012.731544"}"#;

        assert_eq!(
            1654245012731,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "BTCUSDT",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );
    }
}
