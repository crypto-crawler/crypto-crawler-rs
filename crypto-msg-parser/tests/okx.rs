mod utils;

#[cfg(test)]
mod trade {
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, parse_trade, TradeSide};

    #[test]
    fn spot() {
        let raw_msg = r#"{"arg":{"channel":"trades","instId":"BTC-USDT"},"data":[{"instId":"BTC-USDT","tradeId":"314161276","px":"43474.1","sz":"0.00373695","side":"buy","ts":"1646311839593"}]}"#;
        let trades = &parse_trade("okx", MarketType::Spot, raw_msg).unwrap();

        assert_eq!(trades.len(), 1);
        let trade = &trades[0];

        crate::utils::check_trade_fields(
            "okx",
            MarketType::Spot,
            "BTC/USDT".to_string(),
            extract_symbol("okx", MarketType::Spot, raw_msg).unwrap(),
            trade,
            raw_msg,
        );

        assert_eq!(trade.timestamp, 1646311839593);
        assert_eq!(trade.quantity_base, 0.00373695);
        assert_eq!(trade.price, 43474.1);
        assert_eq!(trade.quantity_contract, None);
        assert_eq!(trade.side, TradeSide::Buy);
    }

    #[test]
    fn linear_future() {
        let raw_msg = r#"{"arg":{"channel":"trades","instId":"BTC-USDT-220325"},"data":[{"instId":"BTC-USDT-220325","tradeId":"17400303","px":"43535.3","sz":"2","side":"sell","ts":"1646311972504"}]}"#;
        let trades = &parse_trade("okx", MarketType::LinearFuture, raw_msg).unwrap();

        assert_eq!(trades.len(), 1);
        let trade = &trades[0];

        crate::utils::check_trade_fields(
            "okx",
            MarketType::LinearFuture,
            "BTC/USDT".to_string(),
            extract_symbol("okx", MarketType::LinearFuture, raw_msg).unwrap(),
            trade,
            raw_msg,
        );

        assert_eq!(trade.timestamp, 1646311972504);
        assert_eq!(trade.price, 43535.3);
        assert_eq!(trade.quantity_contract, Some(2.0));
        assert_eq!(trade.quantity_base, 2.0 * 0.01);
        assert_eq!(trade.quantity_quote, 2.0 * 0.01 * 43535.3);
        assert_eq!(trade.side, TradeSide::Sell);
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"arg":{"channel":"trades","instId":"BTC-USDT-SWAP"},"data":[{"instId":"BTC-USDT-SWAP","tradeId":"219066264","px":"43568.8","sz":"7","side":"buy","ts":"1646312440645"}]}"#;
        let trades = &parse_trade("okx", MarketType::LinearSwap, raw_msg).unwrap();

        assert_eq!(trades.len(), 1);
        let trade = &trades[0];

        crate::utils::check_trade_fields(
            "okx",
            MarketType::LinearSwap,
            "BTC/USDT".to_string(),
            extract_symbol("okx", MarketType::LinearSwap, raw_msg).unwrap(),
            trade,
            raw_msg,
        );

        assert_eq!(trade.timestamp, 1646312440645);
        assert_eq!(trade.price, 43568.8);
        assert_eq!(trade.quantity_contract, Some(7.0));
        assert_eq!(trade.quantity_base, 7.0 * 0.01);
        assert_eq!(trade.quantity_quote, 7.0 * 0.01 * 43568.8);
        assert_eq!(trade.side, TradeSide::Buy);
    }

    #[test]
    fn inverse_future() {
        let raw_msg = r#"{"arg":{"channel":"trades","instId":"BTC-USD-220325"},"data":[{"instId":"BTC-USD-220325","tradeId":"18928717","px":"43568.7","sz":"1","side":"sell","ts":"1646312543604"}]}"#;
        let trades = &parse_trade("okx", MarketType::InverseFuture, raw_msg).unwrap();

        assert_eq!(trades.len(), 1);
        let trade = &trades[0];

        crate::utils::check_trade_fields(
            "okx",
            MarketType::InverseFuture,
            "BTC/USD".to_string(),
            extract_symbol("okx", MarketType::InverseFuture, raw_msg).unwrap(),
            trade,
            raw_msg,
        );

        assert_eq!(trade.timestamp, 1646312543604);
        assert_eq!(trade.price, 43568.7);
        assert_eq!(trade.quantity_contract, Some(1.0));
        assert_eq!(trade.quantity_quote, 100.0 * 1.0);
        assert_eq!(trade.quantity_base, 100.0 * 1.0 / 43568.7);
        assert_eq!(trade.side, TradeSide::Sell);
    }

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"arg":{"channel":"trades","instId":"BTC-USD-SWAP"},"data":[{"instId":"BTC-USD-SWAP","tradeId":"173543957","px":"43574.9","sz":"1","side":"sell","ts":"1646312664791"}]}"#;
        let trades = &parse_trade("okx", MarketType::InverseSwap, raw_msg).unwrap();

        assert_eq!(trades.len(), 1);
        let trade = &trades[0];

        crate::utils::check_trade_fields(
            "okx",
            MarketType::InverseSwap,
            "BTC/USD".to_string(),
            extract_symbol("okx", MarketType::InverseSwap, raw_msg).unwrap(),
            trade,
            raw_msg,
        );

        assert_eq!(trade.timestamp, 1646312664791);
        assert_eq!(trade.price, 43574.9);
        assert_eq!(trade.quantity_contract, Some(1.0));
        assert_eq!(trade.quantity_quote, 100.0 * 1.0);
        assert_eq!(trade.quantity_base, 100.0 * 1.0 / 43574.9);
        assert_eq!(trade.side, TradeSide::Sell);
    }

    #[test]
    fn option() {
        let raw_msg = r#"{"arg":{"channel":"trades","instId":"BTC-USD-220304-32000-P"},"data":[{"instId":"BTC-USD-220304-32000-P","tradeId":"81","px":"0.001","sz":"85","side":"buy","ts":"1646138219181"}]}"#;
        let trades = &parse_trade("okx", MarketType::EuropeanOption, raw_msg).unwrap();

        assert_eq!(trades.len(), 1);
        let trade = &trades[0];

        crate::utils::check_trade_fields(
            "okx",
            MarketType::EuropeanOption,
            "BTC/USD".to_string(),
            extract_symbol("okx", MarketType::EuropeanOption, raw_msg).unwrap(),
            trade,
            raw_msg,
        );

        assert_eq!(trade.timestamp, 1646138219181);
        assert_eq!(trade.price, 0.001);
        assert_eq!(trade.quantity_contract, Some(85.0));
        assert_eq!(trade.quantity_base, 85.0 * 0.1);
        assert_eq!(trade.quantity_quote, 85.0 * 0.1 * 0.001);
        assert_eq!(trade.side, TradeSide::Buy);
    }
}

#[cfg(test)]
mod funding_rate {
    use crypto_market_type::MarketType;
    use crypto_msg_parser::parse_funding_rate;

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"arg":{"channel":"funding-rate","instId":"BTC-USD-SWAP"},"data":[{"fundingRate":"0.0000734174532791","fundingTime":"1646323200000","instId":"BTC-USD-SWAP","instType":"SWAP","nextFundingRate":"0.0001163723201487"}]}"#;
        let funding_rates = &parse_funding_rate("okx", MarketType::InverseSwap, raw_msg).unwrap();

        assert_eq!(funding_rates.len(), 1);

        for rate in funding_rates.iter() {
            crate::utils::check_funding_rate_fields("okx", MarketType::InverseSwap, rate, raw_msg);
        }

        assert_eq!(funding_rates[0].pair, "BTC/USD".to_string());
        assert_eq!(funding_rates[0].funding_rate, 0.0000734174532791);
        assert_eq!(funding_rates[0].estimated_rate, Some(0.0001163723201487));
        assert_eq!(funding_rates[0].funding_time, 1646323200000);
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"arg":{"channel":"funding-rate","instId":"BTC-USDT-SWAP"},"data":[{"fundingRate":"0.0001534702159002","fundingTime":"1646323200000","instId":"BTC-USDT-SWAP","instType":"SWAP","nextFundingRate":"0.0001542145319804"}]}"#;
        let funding_rates = &parse_funding_rate("okx", MarketType::InverseSwap, raw_msg).unwrap();

        assert_eq!(funding_rates.len(), 1);

        for rate in funding_rates.iter() {
            crate::utils::check_funding_rate_fields("okx", MarketType::InverseSwap, rate, raw_msg);
        }

        assert_eq!(funding_rates[0].pair, "BTC/USDT".to_string());
        assert_eq!(funding_rates[0].funding_rate, 0.0001534702159002);
        assert_eq!(funding_rates[0].estimated_rate, Some(0.0001542145319804));
        assert_eq!(funding_rates[0].funding_time, 1646323200000);
    }
}

#[cfg(test)]
mod l2_orderbook {
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, parse_l2};
    use crypto_msg_type::MessageType;

    #[test]
    fn spot_snapshot() {
        let raw_msg = r#"{"arg":{"channel":"books-l2-tbt","instId":"BTC-USDT"},"action":"snapshot","data":[{"asks":[["43666.1","1.09431286","0","15"],["43666.3","0.01","0","1"],["43668.1","0.00102036","0","1"]],"bids":[["43666","0.00278174","0","5"],["43664","0.00245053","0","2"],["43662","0.00245065","0","2"]],"ts":"1646313944551","checksum":144433427}]}"#;
        let orderbook = &parse_l2("okx", MarketType::Spot, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 3);
        assert_eq!(orderbook.bids.len(), 3);
        assert!(orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "okx",
            MarketType::Spot,
            MessageType::L2Event,
            "BTC/USDT".to_string(),
            extract_symbol("okx", MarketType::Spot, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );

        assert_eq!(orderbook.timestamp, 1646313944551);

        assert_eq!(orderbook.bids[0].price, 43666.0);
        assert_eq!(orderbook.bids[0].quantity_base, 0.00278174);
        assert_eq!(orderbook.bids[0].quantity_quote, 43666.0 * 0.00278174);

        assert_eq!(orderbook.asks[0].price, 43666.1);
        assert_eq!(orderbook.asks[0].quantity_base, 1.09431286);
        assert_eq!(orderbook.asks[0].quantity_quote, 1.09431286 * 43666.1);
    }

    #[test]
    fn spot_update() {
        let raw_msg = r#"{"arg":{"channel":"books-l2-tbt","instId":"BTC-USDT"},"action":"update","data":[{"asks":[["43736.2","0.1358","0","2"]],"bids":[["43675.6","0.05","0","1"]],"ts":"1646314295200","checksum":796530682}]}"#;
        let orderbook = &parse_l2("okx", MarketType::Spot, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 1);
        assert_eq!(orderbook.bids.len(), 1);
        assert!(!orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "okx",
            MarketType::Spot,
            MessageType::L2Event,
            "BTC/USDT".to_string(),
            extract_symbol("okx", MarketType::Spot, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );

        assert_eq!(orderbook.timestamp, 1646314295200);

        assert_eq!(orderbook.bids[0].price, 43675.6);
        assert_eq!(orderbook.bids[0].quantity_base, 0.05);
        assert_eq!(orderbook.bids[0].quantity_quote, 43675.6 * 0.05);

        assert_eq!(orderbook.asks[0].price, 43736.2);
        assert_eq!(orderbook.asks[0].quantity_base, 0.1358);
        assert_eq!(orderbook.asks[0].quantity_quote, 43736.2 * 0.1358);
    }

    #[test]
    fn linear_future_snapshot() {
        let raw_msg = r#"{"arg":{"channel":"books-l2-tbt","instId":"BTC-USDT-220325"},"action":"snapshot","data":[{"asks":[["43741.9","4","0","1"],["43743.4","1","0","1"],["43743.5","4","0","1"]],"bids":[["43741.8","2","0","1"],["43739.3","4","0","1"],["43738","34","0","1"]],"ts":"1646314548269","checksum":2127111983}]}"#;
        let orderbook = &parse_l2("okx", MarketType::LinearFuture, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 3);
        assert_eq!(orderbook.bids.len(), 3);
        assert!(orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "okx",
            MarketType::LinearFuture,
            MessageType::L2Event,
            "BTC/USDT".to_string(),
            extract_symbol("okx", MarketType::LinearFuture, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );

        assert_eq!(orderbook.timestamp, 1646314548269);

        assert_eq!(orderbook.asks[0].price, 43741.9);
        assert_eq!(orderbook.asks[0].quantity_contract, Some(4.0));
        assert_eq!(orderbook.asks[0].quantity_base, 4.0 * 0.01);
        assert_eq!(orderbook.asks[0].quantity_quote, 4.0 * 0.01 * 43741.9);

        assert_eq!(orderbook.bids[0].price, 43741.8);
        assert_eq!(orderbook.bids[0].quantity_contract, Some(2.0));
        assert_eq!(orderbook.bids[0].quantity_base, 2.0 * 0.01);
        assert_eq!(orderbook.bids[0].quantity_quote, 2.0 * 0.01 * 43741.8);
    }

    #[test]
    fn inverse_swap_snapshot() {
        let raw_msg = r#"{"arg":{"channel":"books-l2-tbt","instId":"BTC-USD-SWAP"},"action":"snapshot","data":[{"asks":[["43726.4","145","0","5"],["43730.5","10","0","1"],["43733.1","45","0","1"]],"bids":[["43726.3","131","0","1"],["43726","10","0","1"],["43722","16","0","1"]],"ts":"1646314888087","checksum":-1817371130}]}"#;
        let orderbook = &parse_l2("okx", MarketType::InverseSwap, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 3);
        assert_eq!(orderbook.bids.len(), 3);
        assert!(orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "okx",
            MarketType::InverseSwap,
            MessageType::L2Event,
            "BTC/USD".to_string(),
            extract_symbol("okx", MarketType::InverseSwap, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );

        assert_eq!(orderbook.timestamp, 1646314888087);

        assert_eq!(orderbook.asks[0].price, 43726.4);
        assert_eq!(orderbook.asks[0].quantity_contract, Some(145.0));
        assert_eq!(orderbook.asks[0].quantity_quote, 100.0 * 145.0);
        assert_eq!(orderbook.asks[0].quantity_base, 100.0 * 145.0 / 43726.4);

        assert_eq!(orderbook.bids[0].price, 43726.3);
        assert_eq!(orderbook.bids[0].quantity_contract, Some(131.0));
        assert_eq!(orderbook.bids[0].quantity_quote, 100.0 * 131.0);
        assert_eq!(orderbook.bids[0].quantity_base, 100.0 * 131.0 / 43726.3);
    }

    #[test]
    fn option_snapshot() {
        let raw_msg = r#"{"arg":{"channel":"books-l2-tbt","instId":"BTC-USD-220304-32000-P"},"action":"snapshot","data":[{"asks":[["0.0005","305","0","1"],["0.001","550","0","2"]],"bids":[],"ts":"1646315100798","checksum":971343753}]}"#;
        let orderbook = &parse_l2("okx", MarketType::EuropeanOption, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 2);
        assert_eq!(orderbook.bids.len(), 0);
        assert!(orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "okx",
            MarketType::EuropeanOption,
            MessageType::L2Event,
            "BTC/USD".to_string(),
            extract_symbol("okx", MarketType::EuropeanOption, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );

        assert_eq!(orderbook.timestamp, 1646315100798);

        assert_eq!(orderbook.asks[0].price, 0.0005);
        assert_eq!(orderbook.asks[0].quantity_contract, Some(305.0));
        assert_eq!(orderbook.asks[0].quantity_base, 305.0 * 0.1);
        assert_eq!(orderbook.asks[0].quantity_quote, 305.0 * 0.1 * 0.0005);
    }
}
