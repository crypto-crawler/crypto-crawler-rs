mod utils;

#[cfg(test)]
mod trade {
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, parse_trade, TradeSide};
    use float_cmp::approx_eq;

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"table":"trade","action":"insert","data":[{"timestamp":"2021-03-12T02:00:04.608Z","symbol":"XBTUSD","side":"Sell","size":900,"price":56927,"tickDirection":"MinusTick","trdMatchID":"d1b82d61-d902-349c-936c-2588b8204aff","grossValue":1581300,"homeNotional":0.015813,"foreignNotional":900}]}"#;
        let trade = &parse_trade("bitmex", MarketType::Unknown, raw_msg).unwrap()[0];

        crate::utils::check_trade_fields(
            "bitmex",
            MarketType::InverseSwap,
            "BTC/USD".to_string(),
            extract_symbol("bitmex", MarketType::InverseSwap, raw_msg).unwrap(),
            trade,
            raw_msg,
        );

        assert_eq!(trade.quantity_base, 0.015813);
        assert_eq!(trade.quantity_quote, 900.0);
        assert_eq!(trade.quantity_contract, Some(900.0));
        assert_eq!(trade.side, TradeSide::Sell);
    }

    #[test]
    fn quanto_swap() {
        let raw_msg = r#"{"table":"trade","action":"partial","data":[{"timestamp":"2021-03-21T00:22:09.258Z","symbol":"ETHUSD","side":"Buy","size":1,"price":1811.6,"tickDirection":"ZeroPlusTick","trdMatchID":"46fcd532-c20e-ac2c-eaed-392f2d599487","grossValue":181160,"homeNotional":0.058513750731421885,"foreignNotional":106.00351082504389}]}"#;
        let trade = &parse_trade("bitmex", MarketType::Unknown, raw_msg).unwrap()[0];

        crate::utils::check_trade_fields(
            "bitmex",
            MarketType::QuantoSwap,
            "ETH/USD".to_string(),
            extract_symbol("bitmex", MarketType::QuantoSwap, raw_msg).unwrap(),
            trade,
            raw_msg,
        );

        assert_eq!(trade.quantity_base, 0.058513750731421885);
        assert!(approx_eq!(
            f64,
            trade.quantity_quote,
            106.00351082504389,
            ulps = 12
        ));
        assert_eq!(trade.quantity_contract, Some(1.0));
        assert_eq!(trade.side, TradeSide::Buy);
    }

    #[test]
    fn inverse_future() {
        let raw_msg = r#"{"table":"trade","action":"partial","data":[{"timestamp":"2021-03-21T01:12:42.361Z","symbol":"XBTM21","side":"Sell","size":8000,"price":62695.5,"tickDirection":"ZeroPlusTick","trdMatchID":"68624a99-e949-33cd-d7e9-63307cf15cfc","grossValue":12760000,"homeNotional":0.1276,"foreignNotional":8000}]}"#;
        let trade = &parse_trade("bitmex", MarketType::Unknown, raw_msg).unwrap()[0];

        crate::utils::check_trade_fields(
            "bitmex",
            MarketType::InverseFuture,
            "BTC/USD".to_string(),
            extract_symbol("bitmex", MarketType::InverseFuture, raw_msg).unwrap(),
            trade,
            raw_msg,
        );

        assert_eq!(trade.quantity_base, 0.1276);
        assert_eq!(trade.quantity_quote, 8000.0);
        assert_eq!(trade.quantity_contract, Some(8000.0));
        assert_eq!(trade.side, TradeSide::Sell);
    }

    #[test]
    fn linear_future() {
        let raw_msg = r#"{"table":"trade","action":"insert","data":[{"timestamp":"2021-03-12T01:46:03.886Z","symbol":"ETHH21","side":"Buy","size":1,"price":0.03191,"tickDirection":"PlusTick","trdMatchID":"a9371640-78d6-53d9-c9e4-31f7b7afb06d","grossValue":3191000,"homeNotional":1,"foreignNotional":0.03191}]}"#;
        let trade = &parse_trade("bitmex", MarketType::Unknown, raw_msg).unwrap()[0];

        crate::utils::check_trade_fields(
            "bitmex",
            MarketType::LinearFuture,
            "ETH/BTC".to_string(),
            extract_symbol("bitmex", MarketType::LinearFuture, raw_msg).unwrap(),
            trade,
            raw_msg,
        );

        assert_eq!(trade.quantity_base, 1.0);
        assert_eq!(trade.quantity_quote, 0.03191);
        assert_eq!(trade.quantity_contract, Some(1.0));
        assert_eq!(trade.side, TradeSide::Buy);
    }

    #[test]
    fn quanto_future() {
        let raw_msg = r#"{"table":"trade","action":"insert","data":[{"timestamp":"2021-03-12T02:13:43.222Z","symbol":"ETHUSDH21","side":"Sell","size":12,"price":1892.8,"tickDirection":"PlusTick","trdMatchID":"14c7d828-80c4-2c91-ad9e-1662081aeaec","grossValue":2271360,"homeNotional":0.6814310051107325,"foreignNotional":1289.8126064735945}]}"#;
        let trade = &parse_trade("bitmex", MarketType::Unknown, raw_msg).unwrap()[0];

        crate::utils::check_trade_fields(
            "bitmex",
            MarketType::QuantoFuture,
            "ETH/USD".to_string(),
            extract_symbol("bitmex", MarketType::QuantoFuture, raw_msg).unwrap(),
            trade,
            raw_msg,
        );

        assert_eq!(trade.quantity_base, 0.6814310051107325);
        assert!(approx_eq!(
            f64,
            trade.quantity_quote,
            1289.8126064735945,
            ulps = 12
        ));
        assert_eq!(trade.quantity_contract, Some(12.0));
        assert!(approx_eq!(
            f64,
            trade.quantity_quote,
            trade.price * trade.quantity_base,
            ulps = 12
        ));
        assert_eq!(trade.side, TradeSide::Sell);
    }
}

#[cfg(test)]
mod funding_rate {
    use crypto_market_type::MarketType;
    use crypto_msg_parser::parse_funding_rate;

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"table":"funding","action":"partial","data":[{"timestamp":"2021-04-01T20:00:00.000Z","symbol":"XBTUSD","fundingInterval":"2000-01-01T08:00:00.000Z","fundingRate":0.000817,"fundingRateDaily":0.002451}]}"#;
        let funding_rates = &parse_funding_rate("bitmex", MarketType::Unknown, raw_msg).unwrap();

        assert_eq!(funding_rates.len(), 1);

        for rate in funding_rates.iter() {
            crate::utils::check_funding_rate_fields(
                "bitmex",
                MarketType::InverseSwap,
                rate,
                raw_msg,
            );
        }

        assert_eq!(funding_rates[0].pair, "BTC/USD".to_string());
        assert_eq!(funding_rates[0].funding_rate, 0.000817);
        assert_eq!(funding_rates[0].funding_time, 1617307200000);
    }

    #[test]
    fn quanto_swap() {
        let raw_msg = r#"{"table":"funding","action":"partial","data":[{"timestamp":"2021-04-01T20:00:00.000Z","symbol":"ETHUSD","fundingInterval":"2000-01-01T08:00:00.000Z","fundingRate":0.002142,"fundingRateDaily":0.006425999999999999}]}"#;
        let funding_rates = &parse_funding_rate("bitmex", MarketType::Unknown, raw_msg).unwrap();

        assert_eq!(funding_rates.len(), 1);

        for rate in funding_rates.iter() {
            crate::utils::check_funding_rate_fields(
                "bitmex",
                MarketType::QuantoSwap,
                rate,
                raw_msg,
            );
        }

        assert_eq!(funding_rates[0].pair, "ETH/USD".to_string());
        assert_eq!(funding_rates[0].funding_rate, 0.002142);
        assert_eq!(funding_rates[0].funding_time, 1617307200000);
    }
}

#[cfg(test)]
mod l2_orderbook {
    use chrono::prelude::*;
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{exchanges::bitmex::price_to_id, extract_symbol, parse_l2};
    use float_cmp::approx_eq;

    #[test]
    fn inverse_swap_snapshot() {
        let raw_msg = r#"{"table":"orderBookL2_25","action":"partial","data":[{"symbol":"XBTUSD","id":8796381000,"side":"Sell","size":49900,"price":36190},{"symbol":"XBTUSD","id":8796381050,"side":"Sell","size":125714,"price":36189.5},{"symbol":"XBTUSD","id":8796381100,"side":"Sell","size":34600,"price":36189},{"symbol":"XBTUSD","id":8796385500,"side":"Buy","size":136,"price":36145},{"symbol":"XBTUSD","id":8796385600,"side":"Buy","size":26,"price":36144},{"symbol":"XBTUSD","id":8796385800,"side":"Buy","size":18067,"price":36142}]}"#;
        let orderbook = &parse_l2(
            "bitmex",
            MarketType::Unknown,
            raw_msg,
            Some(Utc::now().timestamp_millis()),
        )
        .unwrap()[0];

        assert_eq!(orderbook.asks.len(), 3);
        assert_eq!(orderbook.bids.len(), 3);
        assert!(orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "bitmex",
            MarketType::InverseSwap,
            "BTC/USD".to_string(),
            extract_symbol("bitmex", MarketType::InverseSwap, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );

        assert_eq!(orderbook.bids[0].price, 36145.0);
        assert_eq!(8796385500, price_to_id("XBTUSD", 36145.0));
        assert_eq!(orderbook.bids[0].quantity_base, 136.0 / 36145.0);
        assert_eq!(orderbook.bids[0].quantity_quote, 136.0);
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 136.0);

        assert_eq!(orderbook.bids[2].price, 36142.0);
        assert_eq!(8796385800, price_to_id("XBTUSD", 36142.0));
        assert_eq!(orderbook.bids[2].quantity_base, 18067.0 / 36142.0);
        assert_eq!(orderbook.bids[2].quantity_quote, 18067.0);
        assert_eq!(orderbook.bids[2].quantity_contract.unwrap(), 18067.0);

        assert_eq!(orderbook.asks[2].price, 36190.0);
        assert_eq!(8796381000, price_to_id("XBTUSD", 36190.0));
        assert_eq!(orderbook.asks[2].quantity_base, 49900.0 / 36190.0);
        assert_eq!(orderbook.asks[2].quantity_quote, 49900.0);
        assert_eq!(orderbook.asks[2].quantity_contract.unwrap(), 49900.0);

        assert_eq!(orderbook.asks[0].price, 36189.0);
        assert_eq!(8796381100, price_to_id("XBTUSD", 36189.0));
        assert_eq!(orderbook.asks[0].quantity_base, 34600.0 / 36189.0);
        assert_eq!(orderbook.asks[0].quantity_quote, 34600.0);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 34600.0);
    }

    #[test]
    fn inverse_swap_update() {
        let insert_msg = r#"{"table":"orderBookL2_25","action":"insert","data":[{"symbol":"XBTUSD","id":8796323950,"side":"Sell","size":38760,"price":36760.5}]}"#;
        let _ = parse_l2(
            "bitmex",
            MarketType::Unknown,
            insert_msg,
            Some(Utc::now().timestamp_millis()),
        );
        let update_msg = r#"{"table":"orderBookL2_25","action":"update","data":[{"symbol":"XBTUSD","id":8796323950,"side":"Sell","size":36760}]}"#;
        let orderbook = &parse_l2(
            "bitmex",
            MarketType::InverseSwap,
            update_msg,
            Some(Utc::now().timestamp_millis()),
        )
        .unwrap()[0];

        assert_eq!(orderbook.asks.len(), 1);
        assert_eq!(orderbook.bids.len(), 0);
        assert!(!orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "bitmex",
            MarketType::InverseSwap,
            "BTC/USD".to_string(),
            extract_symbol("bitmex", MarketType::InverseSwap, update_msg).unwrap(),
            orderbook,
            update_msg,
        );

        assert_eq!(orderbook.asks[0].price, 36760.5);
        assert_eq!(orderbook.asks[0].quantity_base, 36760.0 / 36760.5);
        assert_eq!(orderbook.asks[0].quantity_quote, 36760.0);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 36760.0);

        let delete_msg = r#"{"table":"orderBookL2_25","action":"delete","data":[{"symbol":"XBTUSD","id":8796323950,"side":"Sell"}]}"#;
        let orderbook = &parse_l2(
            "bitmex",
            MarketType::InverseSwap,
            delete_msg,
            Some(Utc::now().timestamp_millis()),
        )
        .unwrap()[0];

        assert_eq!(orderbook.asks.len(), 1);
        assert_eq!(orderbook.bids.len(), 0);
        assert!(!orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "bitmex",
            MarketType::InverseSwap,
            "BTC/USD".to_string(),
            extract_symbol("bitmex", MarketType::InverseSwap, delete_msg).unwrap(),
            orderbook,
            delete_msg,
        );

        assert_eq!(orderbook.asks[0].price, 36760.5);
        assert_eq!(orderbook.asks[0].quantity_base, 0.0);
        assert_eq!(orderbook.asks[0].quantity_quote, 0.0);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 0.0);
    }

    #[test]
    fn linear_future_snapshot() {
        let raw_msg = r#"{"table":"orderBookL2_25","action":"partial","data":[{"symbol":"ETHH22","id":75899993108,"side":"Sell","size":50000,"price":0.06892,"timestamp":"2022-03-01T01:55:45.088Z"},{"symbol":"ETHH22","id":75899993113,"side":"Sell","size":125000,"price":0.06887,"timestamp":"2022-03-01T01:55:45.088Z"},{"symbol":"ETHH22","id":75899993250,"side":"Buy","size":3000,"price":0.0675,"timestamp":"2022-03-01T01:55:45.088Z"},{"symbol":"ETHH22","id":75899993260,"side":"Buy","size":117000,"price":0.0674,"timestamp":"2022-03-01T01:55:45.088Z"}]}"#;
        let orderbook = &parse_l2(
            "bitmex",
            MarketType::Unknown,
            raw_msg,
            Some(Utc::now().timestamp_millis()),
        )
        .unwrap()[0];

        assert_eq!(orderbook.asks.len(), 2);
        assert_eq!(orderbook.bids.len(), 2);
        assert!(orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "bitmex",
            MarketType::LinearFuture,
            "ETH/BTC".to_string(),
            extract_symbol("bitmex", MarketType::LinearFuture, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );

        assert_eq!(orderbook.bids[0].price, 0.0675);
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 3000.0);
        assert!(approx_eq!(
            f64,
            orderbook.bids[0].quantity_base,
            0.03,
            ulps = 17
        ));
        assert!(approx_eq!(
            f64,
            orderbook.bids[0].quantity_quote,
            0.03 * 0.0675,
            ulps = 18
        ));

        assert_eq!(orderbook.bids[1].price, 0.0674);
        assert_eq!(orderbook.bids[1].quantity_contract.unwrap(), 117000.0);
        assert!(approx_eq!(
            f64,
            orderbook.bids[1].quantity_base,
            1.170,
            ulps = 15
        ));
        assert!(approx_eq!(
            f64,
            orderbook.bids[1].quantity_quote,
            1.17 * 0.0674,
            ulps = 16
        ));

        assert_eq!(orderbook.asks[0].price, 0.06887);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 125000.0);
        assert_eq!(orderbook.asks[0].quantity_base, 1.25);
        assert_eq!(orderbook.asks[0].quantity_quote, 0.06887 * 1.25);

        assert_eq!(orderbook.asks[1].price, 0.06892);
        assert_eq!(orderbook.asks[1].quantity_contract.unwrap(), 50000.0);
        assert_eq!(orderbook.asks[1].quantity_base, 0.5);
        assert_eq!(orderbook.asks[1].quantity_quote, 0.06892 * 0.5);
    }

    #[test]
    fn linear_future_delete() {
        let raw_msg = r#"{"table":"orderBookL2_25","action":"delete","data":[{"symbol":"ETHZ21","id":63399993018,"side":"Buy"}]}"#;
        let orderbook =
            &parse_l2("bitmex", MarketType::Unknown, raw_msg, Some(1635724802280)).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 0);
        assert_eq!(orderbook.bids.len(), 1);
        assert!(!orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "bitmex",
            MarketType::LinearFuture,
            "ETH/BTC".to_string(),
            extract_symbol("bitmex", MarketType::LinearFuture, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );

        assert!(approx_eq!(f64, orderbook.bids[0].price, 0.06982, ulps = 12));
        assert_eq!(orderbook.bids[0].quantity_base, 0.0);
        assert_eq!(orderbook.bids[0].quantity_quote, 0.0);
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 0.0);
    }
}
