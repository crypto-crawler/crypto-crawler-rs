mod utils;

mod trade {
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, parse_trade, TradeSide};

    #[test]
    fn spot() {
        let raw_msg = r#"[321,[["57126.70000","0.02063928","1616333924.737428","b","m",""]],"trade","XBT/USD"]"#;
        let trade = &parse_trade("kraken", MarketType::Spot, raw_msg).unwrap()[0];

        crate::utils::check_trade_fields(
            "kraken",
            MarketType::Spot,
            "BTC/USD".to_string(),
            extract_symbol("kraken", MarketType::Spot, raw_msg).unwrap(),
            trade,
            raw_msg,
        );

        assert_eq!(trade.quantity_base, 0.02063928);
        assert_eq!(trade.side, TradeSide::Buy);
    }

    #[test]
    fn inverse_swap_snapshot() {
        let raw_msg = r#"{"feed":"trade_snapshot","product_id":"PI_XBTUSD","trades":[{"feed":"trade","product_id":"PI_XBTUSD","uid":"57d30a84-6890-4f5d-9f1b-087d24701ec9","side":"buy","type":"fill","seq":222736,"time":1646472607008,"qty":2519.0,"price":39096.0},{"feed":"trade","product_id":"PI_XBTUSD","uid":"0a265ed2-0c5d-4d81-ba70-4bbbd724783d","side":"sell","type":"fill","seq":222735,"time":1646472569433,"qty":636.0,"price":39077.0}]}"#;
        let trades = &parse_trade("kraken", MarketType::InverseSwap, raw_msg).unwrap();
        assert_eq!(2, trades.len());
        let trade = &trades[0];

        crate::utils::check_trade_fields(
            "kraken",
            MarketType::InverseSwap,
            "BTC/USD".to_string(),
            extract_symbol("kraken", MarketType::InverseSwap, raw_msg).unwrap(),
            trade,
            raw_msg,
        );

        assert_eq!(trade.timestamp, 1646472607008);
        assert_eq!(trade.price, 39096.0);
        assert_eq!(trade.quantity_contract, Some(2519.0));
        assert_eq!(trade.quantity_quote, 2519.0);
        assert_eq!(trade.quantity_base, 2519.0 / 39096.0);
        assert_eq!(trade.side, TradeSide::Buy);
    }

    #[test]
    fn inverse_swap_update() {
        let raw_msg = r#"{"feed":"trade","product_id":"PI_XBTUSD","uid":"df029bc0-1e27-4c19-8dd7-d6dc89217508","side":"sell","type":"fill","seq":222737,"time":1646472684700,"qty":386.0,"price":39054.5}"#;
        let trade = &parse_trade("kraken", MarketType::InverseSwap, raw_msg).unwrap()[0];

        crate::utils::check_trade_fields(
            "kraken",
            MarketType::InverseSwap,
            "BTC/USD".to_string(),
            extract_symbol("kraken", MarketType::InverseSwap, raw_msg).unwrap(),
            trade,
            raw_msg,
        );

        assert_eq!(trade.timestamp, 1646472684700);
        assert_eq!(trade.price, 39054.5);
        assert_eq!(trade.quantity_contract, Some(386.0));
        assert_eq!(trade.quantity_quote, 386.0);
        assert_eq!(trade.quantity_base, 386.0 / 39054.5);
        assert_eq!(trade.side, TradeSide::Sell);
    }

    #[test]
    fn inverse_future_snapshot() {
        let raw_msg = r#"{"feed":"trade_snapshot","product_id":"FI_XBTUSD_220624","trades":[{"feed":"trade","product_id":"FI_XBTUSD_220624","uid":"8b8b4be5-c092-415e-bba2-b84e705d007d","side":"sell","type":"fill","seq":14865,"time":1646476382705,"qty":200.0,"price":39244.5},{"feed":"trade","product_id":"FI_XBTUSD_220624","uid":"c389dcdc-431c-43f7-b428-d69f995c5869","side":"sell","type":"fill","seq":14864,"time":1646476120293,"qty":3249.0,"price":39202.0}]}"#;
        let trades = &parse_trade("kraken", MarketType::InverseFuture, raw_msg).unwrap();
        assert_eq!(2, trades.len());
        let trade = &trades[0];

        crate::utils::check_trade_fields(
            "kraken",
            MarketType::InverseFuture,
            "BTC/USD".to_string(),
            extract_symbol("kraken", MarketType::InverseFuture, raw_msg).unwrap(),
            trade,
            raw_msg,
        );

        assert_eq!(trade.timestamp, 1646476382705);
        assert_eq!(trade.price, 39244.5);
        assert_eq!(trade.quantity_contract, Some(200.0));
        assert_eq!(trade.quantity_quote, 200.0);
        assert_eq!(trade.quantity_base, 200.0 / 39244.5);
        assert_eq!(trade.side, TradeSide::Sell);
    }

    #[test]
    fn inverse_future_update() {
        let raw_msg = r#"{"feed":"trade","product_id":"FI_XBTUSD_220624","uid":"64ae86c9-c4da-421d-bed7-b385feb0aecd","side":"buy","type":"fill","seq":14866,"time":1646478498512,"qty":15742.0,"price":39456.5}"#;
        let trade = &parse_trade("kraken", MarketType::InverseFuture, raw_msg).unwrap()[0];

        crate::utils::check_trade_fields(
            "kraken",
            MarketType::InverseFuture,
            "BTC/USD".to_string(),
            extract_symbol("kraken", MarketType::InverseFuture, raw_msg).unwrap(),
            trade,
            raw_msg,
        );

        assert_eq!(trade.timestamp, 1646478498512);
        assert_eq!(trade.price, 39456.5);
        assert_eq!(trade.quantity_contract, Some(15742.0));
        assert_eq!(trade.quantity_quote, 15742.0);
        assert_eq!(trade.quantity_base, 15742.0 / 39456.5);
        assert_eq!(trade.side, TradeSide::Buy);
    }
}

mod l2_event {
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, parse_l2};
    use crypto_msg_type::MessageType;

    #[test]
    fn spot_snapshot() {
        let raw_msg = r#"[6304,{"as":[],"bs":[]},"book-25","PERP/EUR"]"#;
        let result = parse_l2("kraken", MarketType::Spot, raw_msg, None);
        assert!(result.unwrap().is_empty());

        let raw_msg = r#"[320,{"as":[["39090.60000","0.00007039","1622714245.847093"],["39094.90000","0.20000000","1622714255.810162"],["39096.20000","0.25584089","1622714249.255261"]],"bs":[["39071.40000","7.93106570","1622714255.963942"],["39071.30000","0.01090000","1622714249.826684"],["39071.20000","0.76000000","1622714253.348549"]]},"book-25","XBT/USD"]"#;
        let orderbook = &parse_l2("kraken", MarketType::Spot, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 3);
        assert_eq!(orderbook.bids.len(), 3);
        assert!(orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "kraken",
            MarketType::Spot,
            MessageType::L2Event,
            "BTC/USD".to_string(),
            extract_symbol("kraken", MarketType::Spot, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );

        assert_eq!(orderbook.timestamp, 1622714255963);

        assert_eq!(orderbook.bids[0].price, 39071.4);
        assert_eq!(orderbook.bids[0].quantity_base, 7.93106570);
        assert_eq!(orderbook.bids[0].quantity_quote, 39071.4 * 7.93106570);

        assert_eq!(orderbook.bids[2].price, 39071.2);
        assert_eq!(orderbook.bids[2].quantity_base, 0.76);
        assert_eq!(orderbook.bids[2].quantity_quote, 39071.2 * 0.76);

        assert_eq!(orderbook.asks[0].price, 39090.6);
        assert_eq!(orderbook.asks[0].quantity_base, 0.00007039);
        assert_eq!(orderbook.asks[0].quantity_quote, 39090.6 * 0.00007039);

        assert_eq!(orderbook.asks[2].price, 39096.2);
        assert_eq!(orderbook.asks[2].quantity_base, 0.25584089);
        assert_eq!(orderbook.asks[2].quantity_quote, 39096.2 * 0.25584089);
    }

    #[test]
    fn spot_update() {
        let raw_msg = r#"[320,{"b":[["39071.40000","7.26106570","1622714256.068601"]],"c":"2040672112"},"book-25","XBT/USD"]"#;
        let orderbook = &parse_l2("kraken", MarketType::Spot, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 0);
        assert_eq!(orderbook.bids.len(), 1);
        assert!(!orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "kraken",
            MarketType::Spot,
            MessageType::L2Event,
            "BTC/USD".to_string(),
            extract_symbol("kraken", MarketType::Spot, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );

        assert_eq!(orderbook.timestamp, 1622714256068);

        assert_eq!(orderbook.bids[0].price, 39071.4);
        assert_eq!(orderbook.bids[0].quantity_base, 7.26106570);
        assert_eq!(orderbook.bids[0].quantity_quote, 39071.4 * 7.26106570);

        let raw_msg = r#"[320,{"a":[["38800.00000","0.02203518","1622766170.577187"]]},{"b":[["38800.00000","0.03017320","1622766170.577304"]],"c":"2479000840"},"book-25","XBT/USD"]"#;
        let orderbook = &parse_l2("kraken", MarketType::Spot, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 1);
        assert_eq!(orderbook.bids.len(), 1);
        assert!(!orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "kraken",
            MarketType::Spot,
            MessageType::L2Event,
            "BTC/USD".to_string(),
            extract_symbol("kraken", MarketType::Spot, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );

        assert_eq!(orderbook.timestamp, 1622766170577);

        assert_eq!(orderbook.asks[0].price, 38800.0);
        assert_eq!(orderbook.asks[0].quantity_base, 0.02203518);
        assert_eq!(orderbook.asks[0].quantity_quote, 38800.0 * 0.02203518);

        assert_eq!(orderbook.bids[0].price, 38800.0);
        assert_eq!(orderbook.bids[0].quantity_base, 0.03017320);
        assert_eq!(orderbook.bids[0].quantity_quote, 38800.0 * 0.03017320);
    }

    #[test]
    fn inverse_swap_snapshot() {
        let raw_msg = r#"{"feed":"book_snapshot","product_id":"PI_XBTUSD","timestamp":1646478671000,"seq":270511410,"tickSize":null,"bids":[{"price":39253.0,"qty":34400.0},{"price":39252.5,"qty":28812.0},{"price":39251.5,"qty":4452.0}],"asks":[{"price":39279.5,"qty":24550.0},{"price":39282.5,"qty":4331.0},{"price":39288.5,"qty":4603.0}]}"#;
        let orderbook = &parse_l2("kraken", MarketType::InverseSwap, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 3);
        assert_eq!(orderbook.bids.len(), 3);
        assert!(orderbook.snapshot);
        assert_eq!(orderbook.timestamp, 1646478671000);
        assert_eq!(orderbook.seq_id, Some(270511410));

        crate::utils::check_orderbook_fields(
            "kraken",
            MarketType::InverseSwap,
            MessageType::L2Event,
            "BTC/USD".to_string(),
            extract_symbol("kraken", MarketType::InverseSwap, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );

        assert_eq!(orderbook.bids[0].price, 39253.0);
        assert_eq!(orderbook.bids[0].quantity_contract, Some(34400.0));
        assert_eq!(orderbook.bids[0].quantity_quote, 34400.0);
        assert_eq!(orderbook.bids[0].quantity_base, 34400.0 / 39253.0);

        assert_eq!(orderbook.bids[2].price, 39251.5);
        assert_eq!(orderbook.bids[2].quantity_contract, Some(4452.0));
        assert_eq!(orderbook.bids[2].quantity_quote, 4452.0);
        assert_eq!(orderbook.bids[2].quantity_base, 4452.0 / 39251.5);

        assert_eq!(orderbook.asks[0].price, 39279.5);
        assert_eq!(orderbook.asks[0].quantity_contract, Some(24550.0));
        assert_eq!(orderbook.asks[0].quantity_quote, 24550.0);
        assert_eq!(orderbook.asks[0].quantity_base, 24550.0 / 39279.5);

        assert_eq!(orderbook.asks[2].price, 39288.5);
        assert_eq!(orderbook.asks[2].quantity_contract, Some(4603.0));
        assert_eq!(orderbook.asks[2].quantity_quote, 4603.0);
        assert_eq!(orderbook.asks[2].quantity_base, 4603.0 / 39288.5);
    }

    #[test]
    fn inverse_swap_update() {
        let raw_msg = r#"{"feed":"book","product_id":"PI_XBTUSD","side":"buy","seq":270613033,"price":39080.5,"qty":0.0,"timestamp":1646479025941}"#;
        let orderbook = &parse_l2("kraken", MarketType::InverseSwap, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 0);
        assert_eq!(orderbook.bids.len(), 1);
        assert!(!orderbook.snapshot);
        assert_eq!(orderbook.timestamp, 1646479025941);
        assert_eq!(orderbook.seq_id, Some(270613033));

        crate::utils::check_orderbook_fields(
            "kraken",
            MarketType::InverseSwap,
            MessageType::L2Event,
            "BTC/USD".to_string(),
            extract_symbol("kraken", MarketType::InverseSwap, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );

        assert_eq!(orderbook.bids[0].price, 39080.5);
        assert_eq!(orderbook.bids[0].quantity_contract, Some(0.0));
        assert_eq!(orderbook.bids[0].quantity_quote, 0.0);
        assert_eq!(orderbook.bids[0].quantity_base, 0.0);
    }

    #[test]
    fn inverse_future_snapshot() {
        let raw_msg = r#"{"feed":"book_snapshot","product_id":"FI_XBTUSD_220624","timestamp":1646480395477,"seq":21312965,"tickSize":null,"bids":[{"price":39347.5,"qty":1911.0},{"price":39333.0,"qty":132252.0},{"price":39323.5,"qty":3444.0}],"asks":[{"price":39406.5,"qty":1911.0},{"price":39407.0,"qty":30000.0},{"price":39412.0,"qty":500.0}]}"#;
        let orderbook = &parse_l2("kraken", MarketType::InverseFuture, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 3);
        assert_eq!(orderbook.bids.len(), 3);
        assert!(orderbook.snapshot);
        assert_eq!(orderbook.timestamp, 1646480395477);
        assert_eq!(orderbook.seq_id, Some(21312965));

        crate::utils::check_orderbook_fields(
            "kraken",
            MarketType::InverseFuture,
            MessageType::L2Event,
            "BTC/USD".to_string(),
            extract_symbol("kraken", MarketType::InverseFuture, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );

        assert_eq!(orderbook.bids[0].price, 39347.5);
        assert_eq!(orderbook.bids[0].quantity_contract, Some(1911.0));
        assert_eq!(orderbook.bids[0].quantity_quote, 1911.0);
        assert_eq!(orderbook.bids[0].quantity_base, 1911.0 / 39347.5);

        assert_eq!(orderbook.bids[2].price, 39323.5);
        assert_eq!(orderbook.bids[2].quantity_contract, Some(3444.0));
        assert_eq!(orderbook.bids[2].quantity_quote, 3444.0);
        assert_eq!(orderbook.bids[2].quantity_base, 3444.0 / 39323.5);

        assert_eq!(orderbook.asks[0].price, 39406.5);
        assert_eq!(orderbook.asks[0].quantity_contract, Some(1911.0));
        assert_eq!(orderbook.asks[0].quantity_quote, 1911.0);
        assert_eq!(orderbook.asks[0].quantity_base, 1911.0 / 39406.5);

        assert_eq!(orderbook.asks[2].price, 39412.0);
        assert_eq!(orderbook.asks[2].quantity_contract, Some(500.0));
        assert_eq!(orderbook.asks[2].quantity_quote, 500.0);
        assert_eq!(orderbook.asks[2].quantity_base, 500.0 / 39412.0);
    }

    #[test]
    fn inverse_future_update() {
        let raw_msg = r#"{"feed":"book","product_id":"FI_XBTUSD_220624","side":"sell","seq":21313956,"price":39442.5,"qty":332.0,"timestamp":1646480579478}"#;
        let orderbook = &parse_l2("kraken", MarketType::InverseFuture, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 1);
        assert_eq!(orderbook.bids.len(), 0);
        assert!(!orderbook.snapshot);
        assert_eq!(orderbook.timestamp, 1646480579478);
        assert_eq!(orderbook.seq_id, Some(21313956));

        crate::utils::check_orderbook_fields(
            "kraken",
            MarketType::InverseFuture,
            MessageType::L2Event,
            "BTC/USD".to_string(),
            extract_symbol("kraken", MarketType::InverseFuture, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );

        assert_eq!(orderbook.asks[0].price, 39442.5);
        assert_eq!(orderbook.asks[0].quantity_contract, Some(332.0));
        assert_eq!(orderbook.asks[0].quantity_quote, 332.0);
        assert_eq!(orderbook.asks[0].quantity_base, 332.0 / 39442.5);
    }
}
