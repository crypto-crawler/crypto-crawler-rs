mod utils;

const EXCHANGE_NAME: &str = "zbg";

#[cfg(test)]
mod trade {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, extract_timestamp, parse_trade, TradeSide};

    #[test]
    fn spot() {
        let raw_msg = r#"[["T","329","1616384937","BTC_USDT","bid","57347.4","0.048800"]]"#;
        let trades = &parse_trade(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap();

        assert_eq!(trades.len(), 1);
        let trade = &trades[0];

        crate::utils::check_trade_fields(
            EXCHANGE_NAME,
            MarketType::Spot,
            "BTC/USDT".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap(),
            trade,
            raw_msg,
        );
        assert_eq!(
            1616384937000,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(trade.quantity_base, 0.048800);
        assert_eq!(trade.side, TradeSide::Buy);

        let raw_msg = r#"["T","329","1616486457","BTC_USDT","ask","54139.4","0.654172"]"#;
        let trades = &parse_trade(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap();

        assert_eq!(trades.len(), 1);
        let trade = &trades[0];

        crate::utils::check_trade_fields(
            EXCHANGE_NAME,
            MarketType::Spot,
            "BTC/USDT".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap(),
            trade,
            raw_msg,
        );
        assert_eq!(
            1616486457000,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(trade.quantity_base, 0.654172);
        assert_eq!(trade.side, TradeSide::Sell);
    }

    #[test]
    fn linear_swap() {
        let raw_msg =
            r#"["future_tick",{"contractId":1000000,"trades":[1616385064674265,"57326","31",-1]}]"#;
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
            1616385064674,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(trade.quantity_base, 0.01 * 31.0);
        assert_eq!(trade.quantity_quote, 0.01 * 31.0 * 57326.0);
        assert_eq!(trade.quantity_contract, Some(31.0));
        assert_eq!(trade.side, TradeSide::Sell);
    }

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"["future_tick",{"contractId":1000001,"trades":[1616385036580662,"57370","188",-1]}]"#;
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
            1616385036580,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(trade.quantity_base, 188.0 / 57370.0);
        assert_eq!(trade.quantity_quote, 188.0);
        assert_eq!(trade.quantity_contract, Some(188.0));
        assert_eq!(trade.side, TradeSide::Sell);
    }
}

#[cfg(test)]
mod l2_event {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, extract_timestamp, parse_l2};
    use crypto_msg_type::MessageType;

    #[test]
    fn spot_snapshot_1() {
        let raw_msg = r#"[["AE","329","BTC_USDT","1622729950",{"asks":[["38394.8","0.01917"],["38394.2","0.195885"]]},{"bids":[["38388.7","0.146025"],["38388.1","0.155175"]]}]]"#;
        let orderbook = &parse_l2(EXCHANGE_NAME, MarketType::Spot, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 2);
        assert_eq!(orderbook.bids.len(), 2);
        assert!(orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            EXCHANGE_NAME,
            MarketType::Spot,
            MessageType::L2Event,
            "BTC/USDT".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );
        assert_eq!(
            1622729950000,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
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
    fn spot_snapshot_2() {
        let raw_msg = r#"[["AE","5374","SOS_USDT","1648785278",{"asks":[[0.00000471,2033667.52],[0.000004664,10167976.22]]},{"bids":[[0.000001726,41991455.48],["6E-7",300000000.00]]}]]"#;
        let orderbook = &parse_l2(EXCHANGE_NAME, MarketType::Spot, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 2);
        assert_eq!(orderbook.bids.len(), 2);
        assert!(orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            EXCHANGE_NAME,
            MarketType::Spot,
            MessageType::L2Event,
            "SOS/USDT".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );
        assert_eq!(
            1648785278000,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1648785278000);

        assert_eq!(orderbook.bids[0].price, 0.000001726);
        assert_eq!(orderbook.bids[0].quantity_base, 41991455.48);
        assert_eq!(orderbook.bids[0].quantity_quote, 0.000001726 * 41991455.48);

        assert_eq!(orderbook.bids[1].price, 0.0000006);
        assert_eq!(orderbook.bids[1].quantity_base, 300000000.0);
        assert_eq!(orderbook.bids[1].quantity_quote, 0.0000006 * 300000000.0);

        assert_eq!(orderbook.asks[0].price, 0.000004664);
        assert_eq!(orderbook.asks[0].quantity_base, 10167976.22);
        assert_eq!(orderbook.asks[0].quantity_quote, 0.000004664 * 10167976.22);
    }

    #[test]
    fn spot_snapshot_null() {
        let raw_msg = r#"[["AE","5319","YFI_USDT",null,{"asks":null},{"bids":null}]]"#;
        let orderbooks = &parse_l2(EXCHANGE_NAME, MarketType::Spot, raw_msg, None).unwrap();
        assert!(orderbooks.is_empty());
        assert_eq!(
            "yfi_usdt",
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );
        assert_eq!(
            None,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );
    }

    #[test]
    fn spot_update() {
        let raw_msg = r#"["E","329","1622729958","BTC_USDT","BID","38382.3","0.1842"]"#;
        let orderbook = &parse_l2(EXCHANGE_NAME, MarketType::Spot, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 0);
        assert_eq!(orderbook.bids.len(), 1);
        assert!(!orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            EXCHANGE_NAME,
            MarketType::Spot,
            MessageType::L2Event,
            "BTC/USDT".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );
        assert_eq!(
            1622729958000,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1622729958000);

        assert_eq!(orderbook.bids[0].price, 38382.3);
        assert_eq!(orderbook.bids[0].quantity_base, 0.1842);
        assert_eq!(orderbook.bids[0].quantity_quote, 38382.3 * 0.1842);
    }

    #[test]
    fn linear_swap_update() {
        let raw_msg = r#"["future_snapshot_depth",{"asks":[["38704","2684"]],"contractId":1000000,"bids":[["38703","1606"],["38702.5","616"]],"tradeDate":20210603,"time":1622733219128160}]"#;
        let orderbook = &parse_l2(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 1);
        assert_eq!(orderbook.bids.len(), 2);
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
            1622733219128,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
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
        let orderbook =
            &parse_l2(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 2);
        assert_eq!(orderbook.bids.len(), 2);
        assert!(!orderbook.snapshot);

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
            1622734001831,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
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

#[cfg(test)]
mod candlestick {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, extract_timestamp};

    #[test]
    fn spot_snapshot() {
        let raw_msg = r#"[["K","329","btc_usdt","1654155660","30013.78","30017.31","30003.01","30014.64","0.0227","-0.2957","0","1M","false","0"],["K","329","btc_usdt","1654155600","30016.95","30019.49","29997.36","29997.36","0.3865","-0.2957","0","1M","false","0"]]"#;

        assert_eq!(
            "btc_usdt",
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );

        assert_eq!(
            1654155660000,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );
    }

    #[test]
    fn spot_update() {
        let raw_msg = r#"["K","329","btc_usdt","1654125240","29947.03","29976.14","29937.94","29939.95","0.6417","-0.2957","0","1M","false","0"]"#;

        assert_eq!(
            "btc_usdt",
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );

        assert_eq!(
            1654125240000,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );
    }

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"["future_kline",{"contractId":1000001,"range":"60000","lines":[[1652804280000,"30008.5","30015.5","29994.5","30005","16754"],[1652804340000,"30005","30005.5","29975.5","29976","6186"]]}]"#;

        assert_eq!(
            "BTC_USD-R",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );

        assert_eq!(
            1652804340000,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"["future_kline",{"contractId":1000000,"range":"180000","lines":[[1648876500000,"46535","46550.5","46505.5","46550","848"],[1648876680000,"46550","46615","46542","46613.5","1640"]]}]"#;

        assert_eq!(
            "BTC_USDT",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );

        assert_eq!(
            1648876680000,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
    }
}
