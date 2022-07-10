mod utils;

const EXCHANGE_NAME: &str = "kraken";

mod trade {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_message::TradeSide;
    use crypto_msg_parser::{extract_symbol, extract_timestamp, parse_trade};

    #[test]
    fn spot() {
        let raw_msg = r#"[321,[["57126.70000","0.02063928","1616333924.737428","b","m",""]],"trade","XBT/USD"]"#;
        let trade = &parse_trade(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()[0];

        crate::utils::check_trade_fields(
            EXCHANGE_NAME,
            MarketType::Spot,
            "BTC/USD".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap(),
            trade,
            raw_msg,
        );
        assert_eq!(
            1616333924737,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(trade.quantity_base, 0.02063928);
        assert_eq!(trade.side, TradeSide::Buy);
    }

    #[test]
    fn inverse_swap_snapshot() {
        let raw_msg = r#"{"feed":"trade_snapshot","product_id":"PI_XBTUSD","trades":[{"feed":"trade","product_id":"PI_XBTUSD","uid":"57d30a84-6890-4f5d-9f1b-087d24701ec9","side":"buy","type":"fill","seq":222736,"time":1646472607008,"qty":2519.0,"price":39096.0},{"feed":"trade","product_id":"PI_XBTUSD","uid":"0a265ed2-0c5d-4d81-ba70-4bbbd724783d","side":"sell","type":"fill","seq":222735,"time":1646472569433,"qty":636.0,"price":39077.0}]}"#;
        let trades = &parse_trade(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap();
        assert_eq!(2, trades.len());
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
            1646472607008,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
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
        let trade = &parse_trade(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap()[0];

        crate::utils::check_trade_fields(
            EXCHANGE_NAME,
            MarketType::InverseSwap,
            "BTC/USD".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap(),
            trade,
            raw_msg,
        );
        assert_eq!(
            1646472684700,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
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
        let trades = &parse_trade(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg).unwrap();
        assert_eq!(2, trades.len());
        let trade = &trades[0];

        crate::utils::check_trade_fields(
            EXCHANGE_NAME,
            MarketType::InverseFuture,
            "BTC/USD".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg).unwrap(),
            trade,
            raw_msg,
        );
        assert_eq!(
            1646476382705,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg)
                .unwrap()
                .unwrap()
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
        let trade = &parse_trade(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg).unwrap()[0];

        crate::utils::check_trade_fields(
            EXCHANGE_NAME,
            MarketType::InverseFuture,
            "BTC/USD".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg).unwrap(),
            trade,
            raw_msg,
        );
        assert_eq!(
            1646478498512,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg)
                .unwrap()
                .unwrap()
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
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, extract_timestamp, parse_l2};
    use crypto_msg_type::MessageType;

    #[test]
    fn spot_snapshot() {
        let raw_msg = r#"[6304,{"as":[],"bs":[]},"book-25","PERP/EUR"]"#;
        let result = parse_l2(EXCHANGE_NAME, MarketType::Spot, raw_msg, None);
        assert!(result.unwrap().is_empty());

        let raw_msg = r#"[320,{"as":[["39090.60000","0.00007039","1622714245.847093"],["39094.90000","0.20000000","1622714255.810162"],["39096.20000","0.25584089","1622714249.255261"]],"bs":[["39071.40000","7.93106570","1622714255.963942"],["39071.30000","0.01090000","1622714249.826684"],["39071.20000","0.76000000","1622714253.348549"]]},"book-25","XBT/USD"]"#;
        let orderbook = &parse_l2(EXCHANGE_NAME, MarketType::Spot, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 3);
        assert_eq!(orderbook.bids.len(), 3);
        assert!(orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            EXCHANGE_NAME,
            MarketType::Spot,
            MessageType::L2Event,
            "BTC/USD".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );
        assert_eq!(
            1622714255963,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
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
        let orderbook = &parse_l2(EXCHANGE_NAME, MarketType::Spot, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 0);
        assert_eq!(orderbook.bids.len(), 1);
        assert!(!orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            EXCHANGE_NAME,
            MarketType::Spot,
            MessageType::L2Event,
            "BTC/USD".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );
        assert_eq!(
            1622714256068,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1622714256068);

        assert_eq!(orderbook.bids[0].price, 39071.4);
        assert_eq!(orderbook.bids[0].quantity_base, 7.26106570);
        assert_eq!(orderbook.bids[0].quantity_quote, 39071.4 * 7.26106570);

        let raw_msg = r#"[320,{"a":[["38800.00000","0.02203518","1622766170.577187"]]},{"b":[["38800.00000","0.03017320","1622766170.577304"]],"c":"2479000840"},"book-25","XBT/USD"]"#;
        let orderbook = &parse_l2(EXCHANGE_NAME, MarketType::Spot, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 1);
        assert_eq!(orderbook.bids.len(), 1);
        assert!(!orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            EXCHANGE_NAME,
            MarketType::Spot,
            MessageType::L2Event,
            "BTC/USD".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap(),
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
        let orderbook =
            &parse_l2(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 3);
        assert_eq!(orderbook.bids.len(), 3);
        assert!(orderbook.snapshot);
        assert_eq!(orderbook.timestamp, 1646478671000);
        assert_eq!(orderbook.seq_id, Some(270511410));

        crate::utils::check_orderbook_fields(
            EXCHANGE_NAME,
            MarketType::InverseSwap,
            MessageType::L2Event,
            "BTC/USD".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );
        assert_eq!(
            1646478671000,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
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
        let orderbook =
            &parse_l2(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 0);
        assert_eq!(orderbook.bids.len(), 1);
        assert!(!orderbook.snapshot);
        assert_eq!(orderbook.timestamp, 1646479025941);
        assert_eq!(orderbook.seq_id, Some(270613033));

        crate::utils::check_orderbook_fields(
            EXCHANGE_NAME,
            MarketType::InverseSwap,
            MessageType::L2Event,
            "BTC/USD".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );
        assert_eq!(
            1646479025941,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.bids[0].price, 39080.5);
        assert_eq!(orderbook.bids[0].quantity_contract, Some(0.0));
        assert_eq!(orderbook.bids[0].quantity_quote, 0.0);
        assert_eq!(orderbook.bids[0].quantity_base, 0.0);
    }

    #[test]
    fn inverse_future_snapshot() {
        let raw_msg = r#"{"feed":"book_snapshot","product_id":"FI_XBTUSD_220624","timestamp":1646480395477,"seq":21312965,"tickSize":null,"bids":[{"price":39347.5,"qty":1911.0},{"price":39333.0,"qty":132252.0},{"price":39323.5,"qty":3444.0}],"asks":[{"price":39406.5,"qty":1911.0},{"price":39407.0,"qty":30000.0},{"price":39412.0,"qty":500.0}]}"#;
        let orderbook =
            &parse_l2(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 3);
        assert_eq!(orderbook.bids.len(), 3);
        assert!(orderbook.snapshot);
        assert_eq!(orderbook.timestamp, 1646480395477);
        assert_eq!(orderbook.seq_id, Some(21312965));

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
            1646480395477,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg)
                .unwrap()
                .unwrap()
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
        let orderbook =
            &parse_l2(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 1);
        assert_eq!(orderbook.bids.len(), 0);
        assert!(!orderbook.snapshot);
        assert_eq!(orderbook.timestamp, 1646480579478);
        assert_eq!(orderbook.seq_id, Some(21313956));

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
            1646480579478,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.asks[0].price, 39442.5);
        assert_eq!(orderbook.asks[0].quantity_contract, Some(332.0));
        assert_eq!(orderbook.asks[0].quantity_quote, 332.0);
        assert_eq!(orderbook.asks[0].quantity_base, 332.0 / 39442.5);
    }
}

#[cfg(test)]
mod bbo {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, extract_timestamp, parse_bbo};
    use crypto_msg_type::MessageType;


    #[test]
    fn spot() {
        let raw_msg = r#"[341,["31760.00000","31760.10000","1654031976.197239","0.02167307","6.46761464"],"spread","XBT/USD"]"#;

        assert_eq!(
            1654031976197,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "XBT/USD",
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );
        let timestamp: Option<i64> = Some(1654081540967);
        let bbo_msg =parse_bbo(EXCHANGE_NAME, MarketType::Spot, raw_msg, timestamp).ok().unwrap();

        assert_eq!(MessageType::BBO, bbo_msg.msg_type);
        assert_eq!("XBT/USD", bbo_msg.symbol);
        assert_eq!(1654081540967, bbo_msg.timestamp);
        assert_eq!(Some(341), bbo_msg.id);

        assert_eq!(31760.1, bbo_msg.ask_price);
        assert_eq!(6.46761464, bbo_msg.ask_quantity_base);
        assert_eq!(205412.087727864, bbo_msg.ask_quantity_quote);
        assert_eq!(None, bbo_msg.ask_quantity_contract);

        assert_eq!(31760.0, bbo_msg.bid_price);
        assert_eq!(0.02167307, bbo_msg.bid_quantity_base);
        assert_eq!(688.3367032, bbo_msg.bid_quantity_quote);
        assert_eq!(None, bbo_msg.bid_quantity_contract);
    }
}

#[cfg(test)]
mod candlestick {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, extract_timestamp};

    #[test]
    fn spot() {
        let raw_msg = r#"[343,["1654081540.967902","1654081560.000000","31527.70000","31527.70000","31527.70000","31527.70000","31527.70000","0.00526133",2],"ohlc-1","XBT/USD"]"#;

        assert_eq!(
            1654081540967,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "XBT/USD",
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );
    }
}

#[cfg(test)]
mod ticker {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, extract_timestamp};

    #[test]
    fn spot() {
        let raw_msg = r#"[340,{"a":["29938.60000",12,"12.41074632"],"b":["29938.50000",0,"0.08410000"],"c":["29938.60000","0.10146338"],"v":["735.06931643","5243.60824494"],"p":["29812.62518","30272.79057"],"t":[6120,30832],"l":["29581.10000","29328.60000"],"h":["30085.80000","31869.30000"],"o":["29790.00000","31614.50000"]},"ticker","XBT/USD"]"#;

        assert_eq!(
            None,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );
        assert_eq!(
            "XBT/USD",
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );
    }

    #[test]
    fn inverse_future() {
        let raw_msg = r#"{"time":1654164693039,"product_id":"FI_XBTUSD_220624","feed":"ticker","bid":29892.5,"ask":29916.0,"bid_size":4977.0,"ask_size":1477.0,"volume":5710706.0,"dtm":22,"leverage":"50x","index":29920.98,"premium":-0.1,"last":29876.0,"change":-5.511014121479518,"suspended":false,"tag":"month","pair":"XBT:USD","openInterest":7790153.0,"markPrice":29904.25,"maturityTime":1656082800000,"post_only":false}"#;

        assert_eq!(
            1654164693039,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "FI_XBTUSD_220624",
            extract_symbol(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg).unwrap()
        );
    }

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"time":1654164951042,"product_id":"PI_XBTUSD","funding_rate":3.216424e-12,"funding_rate_prediction":-8.63581351e-10,"relative_funding_rate":9.6321875e-8,"relative_funding_rate_prediction":-0.000025847309375,"next_funding_rate_time":1654171200000,"feed":"ticker","bid":29914.5,"ask":29925.0,"bid_size":8400.0,"ask_size":10000.0,"volume":100353210.0,"dtm":0,"leverage":"50x","index":29927.42,"premium":-0.0,"last":29929.0,"change":-5.280481050716035,"suspended":false,"tag":"perpetual","pair":"XBT:USD","openInterest":43967525.0,"markPrice":29919.75,"maturityTime":0,"post_only":false}"#;

        assert_eq!(
            1654164951042,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "PI_XBTUSD",
            extract_symbol(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap()
        );
    }
}

#[cfg(test)]
mod l2_snapshot {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, extract_timestamp};

    #[test]
    fn spot() {
        let raw_msg = r#"{"error":[],"result":{"XXBTZUSD":{"asks":[["29727.10000","0.420",1654302625],["29728.30000","5.047",1654302622],["29728.70000","0.474",1654302582]],"bids":[["29727.00000","3.127",1654302625],["29725.10000","0.085",1654302625],["29725.00000","0.586",1654302613]]}}}"#;

        assert_eq!(
            None,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );
        assert_eq!(
            "XXBTZUSD",
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );
    }

    #[test]
    fn inverse_future() {
        let raw_msg = r#"{"result":"success","orderBook":{"bids":[[29623.5, 1480], [29621.5, 3500], [29621, 14795]],"asks":[[29641, 1480], [29646.5, 14802], [29647, 80937]]},"serverTime":"2022-06-04T06:30:03.653Z"}"#;

        assert_eq!(
            1654324203653,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "NONE",
            extract_symbol(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg).unwrap()
        );
    }

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"result":"success","orderBook":{"bids":[[29648, 66], [29647, 28976], [29646.5, 24786]],"asks":[[29656, 20000], [29656.5, 10000], [29657, 8000]]},"serverTime":"2022-06-04T06:30:00.247Z"}"#;

        assert_eq!(
            1654324200247,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "NONE",
            extract_symbol(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap()
        );
    }
}
