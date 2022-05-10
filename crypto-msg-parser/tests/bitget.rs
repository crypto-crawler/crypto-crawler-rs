mod utils;

#[cfg(test)]
mod trade {
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, extract_timestamp, parse_trade, TradeSide};
    use float_cmp::approx_eq;

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"data":[{"instrument_id":"btcusd","price":"58722.0","side":"sell","size":"158","timestamp":"1616236107276"},{"instrument_id":"btcusd","price":"58722.0","side":"sell","size":"450","timestamp":"1616236107276"},{"instrument_id":"btcusd","price":"58722.0","side":"sell","size":"762","timestamp":"1616236107276"}],"table":"swap/trade"}"#;
        let trades = &parse_trade("bitget", MarketType::InverseSwap, raw_msg).unwrap();

        assert_eq!(trades.len(), 3);

        for trade in trades.iter() {
            crate::utils::check_trade_fields(
                "bitget",
                MarketType::InverseSwap,
                "BTC/USD".to_string(),
                extract_symbol("bitget", MarketType::InverseSwap, raw_msg).unwrap(),
                trade,
                raw_msg,
            );
            assert_eq!(trade.side, TradeSide::Sell);
        }
        assert_eq!(
            1616236107276,
            extract_timestamp("bitget", MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(trades[0].quantity_base, 158.0 / 58722.0);
        assert_eq!(trades[0].quantity_quote, 158.0);
        assert_eq!(trades[0].quantity_contract, Some(158.0));

        assert_eq!(trades[1].quantity_base, 450.0 / 58722.0);
        assert_eq!(trades[1].quantity_quote, 450.0);
        assert_eq!(trades[1].quantity_contract, Some(450.0));

        assert_eq!(trades[2].quantity_base, 762.0 / 58722.0);
        assert_eq!(trades[2].quantity_quote, 762.0);
        assert_eq!(trades[2].quantity_contract, Some(762.0));
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"data":[{"instrument_id":"cmt_btcusdt","price":"58784.0","side":"sell","size":"1265","timestamp":"1616236212569"},{"instrument_id":"cmt_btcusdt","price":"58784.0","side":"sell","size":"25","timestamp":"1616236212569"},{"instrument_id":"cmt_btcusdt","price":"58784.0","side":"sell","size":"181","timestamp":"1616236212569"}],"table":"swap/trade"}"#;
        let trades = &parse_trade("bitget", MarketType::LinearSwap, raw_msg).unwrap();

        assert_eq!(trades.len(), 3);

        for trade in trades.iter() {
            crate::utils::check_trade_fields(
                "bitget",
                MarketType::LinearSwap,
                "BTC/USDT".to_string(),
                extract_symbol("bitget", MarketType::LinearSwap, raw_msg).unwrap(),
                trade,
                raw_msg,
            );

            assert_eq!(trade.side, TradeSide::Sell);
        }
        assert_eq!(
            1616236212569,
            extract_timestamp("bitget", MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert!(approx_eq!(
            f64,
            trades[0].quantity_base,
            1265.0 * 0.001,
            epsilon = 0.0000001
        ));
        assert!(approx_eq!(
            f64,
            trades[1].quantity_base,
            25.0 * 0.001,
            epsilon = 0.0000001
        ));
        assert!(approx_eq!(
            f64,
            trades[2].quantity_base,
            181.0 * 0.001,
            epsilon = 0.0000001
        ));
    }
}

#[cfg(test)]
mod funding_rate {
    use crypto_market_type::MarketType;
    use crypto_msg_parser::parse_funding_rate;

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"data":[{"funding_rate":"0.000258514264","funding_time":"1617346800000","instrument_id":"btcusd"}],"table":"swap/funding_rate"}"#;
        let funding_rates =
            &parse_funding_rate("bitget", MarketType::InverseSwap, raw_msg).unwrap();

        assert_eq!(funding_rates.len(), 1);

        for rate in funding_rates.iter() {
            crate::utils::check_funding_rate_fields(
                "bitget",
                MarketType::InverseSwap,
                rate,
                raw_msg,
            );
        }

        assert_eq!(funding_rates[0].pair, "BTC/USD".to_string());
        assert_eq!(funding_rates[0].funding_rate, 0.000258514264);
        assert_eq!(funding_rates[0].funding_time, 1617346800000);
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"data":[{"funding_rate":"0.000106539854","funding_time":"1617346800000","instrument_id":"cmt_btcusdt"}],"table":"swap/funding_rate"}"#;
        let funding_rates = &parse_funding_rate("bitget", MarketType::LinearSwap, raw_msg).unwrap();

        assert_eq!(funding_rates.len(), 1);

        for rate in funding_rates.iter() {
            crate::utils::check_funding_rate_fields(
                "bitget",
                MarketType::LinearSwap,
                rate,
                raw_msg,
            );
        }

        assert_eq!(funding_rates[0].pair, "BTC/USDT".to_string());
        assert_eq!(funding_rates[0].funding_rate, 0.000106539854);
        assert_eq!(funding_rates[0].funding_time, 1617346800000);
    }
}

#[cfg(test)]
mod l2_orderbook {
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, extract_timestamp, parse_l2};
    use crypto_msg_type::MessageType;
    use float_cmp::approx_eq;

    #[test]
    fn linear_swap_snapshot() {
        let raw_msg = r#"{"action":"partial","data":[{"asks":[["34589.0","507"],["34589.5","958"],["34590.0","6751"],["34590.5","898"],["34591.0","1987"]],"bids":[["34588.0","1199"],["34587.0","1339"],["34586.5","506"],["34586.0","4018"],["34585.0","1259"]],"instrument_id":"cmt_btcusdt","timestamp":"1622432420458"}],"table":"swap/depth"}"#;
        let orderbook = &parse_l2("bitget", MarketType::LinearSwap, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 5);
        assert_eq!(orderbook.bids.len(), 5);
        assert!(orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "bitget",
            MarketType::LinearSwap,
            MessageType::L2Event,
            "BTC/USDT".to_string(),
            extract_symbol("bitget", MarketType::LinearSwap, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );
        assert_eq!(
            1622432420458,
            extract_timestamp("bitget", MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1622432420458);

        assert_eq!(orderbook.bids[0].price, 34588.0);
        assert!(approx_eq!(
            f64,
            orderbook.bids[0].quantity_base,
            1.199,
            epsilon = 0.0000001
        ));
        assert!(approx_eq!(
            f64,
            orderbook.bids[0].quantity_quote,
            1.199 * 34588.0,
            epsilon = 0.01
        ));
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 1199.0);

        assert_eq!(orderbook.bids[4].price, 34585.0);
        assert!(approx_eq!(
            f64,
            orderbook.bids[4].quantity_base,
            1.259,
            epsilon = 0.0000001
        ));
        assert!(approx_eq!(
            f64,
            orderbook.bids[4].quantity_quote,
            1.259 * 34585.0,
            epsilon = 0.01
        ));
        assert_eq!(orderbook.bids[4].quantity_contract.unwrap(), 1259.0);

        assert_eq!(orderbook.asks[0].price, 34589.0);
        assert!(approx_eq!(
            f64,
            orderbook.asks[0].quantity_base,
            0.507,
            epsilon = 0.0000001
        ));
        assert!(approx_eq!(
            f64,
            orderbook.asks[0].quantity_quote,
            0.507 * 34589.0,
            epsilon = 0.001
        ));
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 507.0);

        assert_eq!(orderbook.asks[4].price, 34591.0);
        assert!(approx_eq!(
            f64,
            orderbook.asks[4].quantity_base,
            1.987,
            epsilon = 0.0000001
        ));
        assert!(approx_eq!(
            f64,
            orderbook.asks[4].quantity_quote,
            1.987 * 34591.0,
            epsilon = 0.01
        ));
        assert_eq!(orderbook.asks[4].quantity_contract.unwrap(), 1987.0);
    }

    #[test]
    fn linear_swap_update() {
        let raw_msg = r#"{"action":"update","data":[{"asks":[["34523","510"]],"bids":[["34522","9079"],["34521.5","31174"]],"instrument_id":"cmt_btcusdt","timestamp":"1622434075797"}],"table":"swap/depth"}"#;
        let orderbook = &parse_l2("bitget", MarketType::LinearSwap, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 1);
        assert_eq!(orderbook.bids.len(), 2);
        assert!(!orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "bitget",
            MarketType::LinearSwap,
            MessageType::L2Event,
            "BTC/USDT".to_string(),
            extract_symbol("bitget", MarketType::LinearSwap, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );
        assert_eq!(
            1622434075797,
            extract_timestamp("bitget", MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1622434075797);

        assert_eq!(orderbook.bids[0].price, 34522.0);
        assert!(approx_eq!(
            f64,
            orderbook.bids[0].quantity_base,
            9.079,
            epsilon = 0.000001
        ));
        assert!(approx_eq!(
            f64,
            orderbook.bids[0].quantity_quote,
            9.079 * 34522.0,
            epsilon = 0.1
        ));
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 9079.0);

        assert_eq!(orderbook.bids[1].price, 34521.5);
        assert!(approx_eq!(
            f64,
            orderbook.bids[1].quantity_base,
            31.174,
            epsilon = 0.00001
        ));
        assert!(approx_eq!(
            f64,
            orderbook.bids[1].quantity_quote,
            31.174 * 34521.5,
            epsilon = 0.1
        ));
        assert_eq!(orderbook.bids[1].quantity_contract.unwrap(), 31174.0);

        assert_eq!(orderbook.asks[0].price, 34523.0);
        assert!(approx_eq!(
            f64,
            orderbook.asks[0].quantity_base,
            0.51,
            epsilon = 0.0000001
        ));
        assert!(approx_eq!(
            f64,
            orderbook.asks[0].quantity_quote,
            0.51 * 34523.0,
            epsilon = 0.001
        ));
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 510.0);
    }

    #[test]
    fn inverse_swap_snapshot() {
        let raw_msg = r#"{"action":"partial","data":[{"asks":[["34880.5","506"],["34881.0","4496"],["34881.5","73280"],["34882.0","84782"],["34882.5","135651"]],"bids":[["34879.0","14946"],["34878.5","24386"],["34878.0","10048"],["34877.5","161361"],["34877.0","61292"]],"instrument_id":"btcusd","timestamp":"1622426574770"}],"table":"swap/depth"}"#;
        let orderbook = &parse_l2("bitget", MarketType::InverseSwap, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 5);
        assert_eq!(orderbook.bids.len(), 5);
        assert!(orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "bitget",
            MarketType::InverseSwap,
            MessageType::L2Event,
            "BTC/USD".to_string(),
            extract_symbol("bitget", MarketType::InverseSwap, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );
        assert_eq!(
            1622426574770,
            extract_timestamp("bitget", MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1622426574770);

        assert_eq!(orderbook.bids[0].price, 34879.0);
        assert_eq!(orderbook.bids[0].quantity_base, 14946.0 / 34879.0);
        assert_eq!(orderbook.bids[0].quantity_quote, 14946.0);
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 14946.0);

        assert_eq!(orderbook.bids[4].price, 34877.0);
        assert_eq!(orderbook.bids[4].quantity_base, 61292.0 / 34877.0);
        assert_eq!(orderbook.bids[4].quantity_quote, 61292.0);
        assert_eq!(orderbook.bids[4].quantity_contract.unwrap(), 61292.0);

        assert_eq!(orderbook.asks[0].price, 34880.5);
        assert_eq!(orderbook.asks[0].quantity_base, 506.0 / 34880.5);
        assert_eq!(orderbook.asks[0].quantity_quote, 506.0);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 506.0);

        assert_eq!(orderbook.asks[4].price, 34882.5);
        assert_eq!(orderbook.asks[4].quantity_base, 135651.0 / 34882.5);
        assert_eq!(orderbook.asks[4].quantity_quote, 135651.0);
        assert_eq!(orderbook.asks[4].quantity_contract.unwrap(), 135651.0);
    }

    #[test]
    fn inverse_swap_update() {
        let raw_msg = r#"{"action":"update","data":[{"asks":[["34641.5","101367"],["34642","25822"]],"bids":[["34637","510"]],"instrument_id":"btcusd","timestamp":"1622431636806"}],"table":"swap/depth"}"#;
        let orderbook = &parse_l2("bitget", MarketType::InverseSwap, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 2);
        assert_eq!(orderbook.bids.len(), 1);
        assert!(!orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "bitget",
            MarketType::InverseSwap,
            MessageType::L2Event,
            "BTC/USD".to_string(),
            extract_symbol("bitget", MarketType::InverseSwap, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );
        assert_eq!(
            1622431636806,
            extract_timestamp("bitget", MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1622431636806);

        assert_eq!(orderbook.bids[0].price, 34637.0);
        assert_eq!(orderbook.bids[0].quantity_base, 510.0 / 34637.0);
        assert_eq!(orderbook.bids[0].quantity_quote, 510.0);
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 510.0);

        assert_eq!(orderbook.asks[0].price, 34641.5);
        assert_eq!(orderbook.asks[0].quantity_base, 101367.0 / 34641.5);
        assert_eq!(orderbook.asks[0].quantity_quote, 101367.0);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 101367.0);

        assert_eq!(orderbook.asks[1].price, 34642.0);
        assert_eq!(orderbook.asks[1].quantity_base, 25822.0 / 34642.0);
        assert_eq!(orderbook.asks[1].quantity_quote, 25822.0);
        assert_eq!(orderbook.asks[1].quantity_contract.unwrap(), 25822.0);
    }
}
