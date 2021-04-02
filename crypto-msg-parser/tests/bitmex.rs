mod utils;

#[cfg(test)]
mod trade {
    use crypto_msg_parser::{parse_trade, MarketType, TradeSide};
    use float_cmp::approx_eq;

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"table":"trade","action":"insert","data":[{"timestamp":"2021-03-12T02:00:04.608Z","symbol":"XBTUSD","side":"Sell","size":900,"price":56927,"tickDirection":"MinusTick","trdMatchID":"d1b82d61-d902-349c-936c-2588b8204aff","grossValue":1581300,"homeNotional":0.015813,"foreignNotional":900}]}"#;
        let trade = &parse_trade("bitmex", MarketType::InverseSwap, raw_msg).unwrap()[0];

        crate::utils::check_trade_fields(
            "bitmex",
            MarketType::InverseSwap,
            "BTC/USD".to_string(),
            trade,
        );

        // assert_eq!(trade.volume, trade.price * trade.quantity);
        assert_eq!(trade.side, TradeSide::Sell);
    }

    #[test]
    fn quanto_swap() {
        let raw_msg = r#"{"table":"trade","action":"partial","data":[{"timestamp":"2021-03-21T00:22:09.258Z","symbol":"ETHUSD","side":"Buy","size":1,"price":1811.6,"tickDirection":"ZeroPlusTick","trdMatchID":"46fcd532-c20e-ac2c-eaed-392f2d599487","grossValue":181160,"homeNotional":0.058513750731421885,"foreignNotional":106.00351082504389}]}"#;
        let trade = &parse_trade("bitmex", MarketType::QuantoSwap, raw_msg).unwrap()[0];

        crate::utils::check_trade_fields(
            "bitmex",
            MarketType::QuantoSwap,
            "ETH/USD".to_string(),
            trade,
        );

        assert_eq!(trade.volume, trade.price * trade.quantity);
        assert_eq!(trade.side, TradeSide::Buy);
    }

    #[test]
    fn inverse_future() {
        let raw_msg = r#"{"table":"trade","action":"partial","data":[{"timestamp":"2021-03-21T01:12:42.361Z","symbol":"XBTM21","side":"Sell","size":8000,"price":62695.5,"tickDirection":"ZeroPlusTick","trdMatchID":"68624a99-e949-33cd-d7e9-63307cf15cfc","grossValue":12760000,"homeNotional":0.1276,"foreignNotional":8000}]}"#;
        let trade = &parse_trade("bitmex", MarketType::InverseFuture, raw_msg).unwrap()[0];

        crate::utils::check_trade_fields(
            "bitmex",
            MarketType::InverseFuture,
            "BTC/USD".to_string(),
            trade,
        );

        // assert_eq!(trade.volume, trade.price * trade.quantity);
        assert_eq!(trade.side, TradeSide::Sell);
    }

    #[test]
    fn linear_future() {
        let raw_msg = r#"{"table":"trade","action":"insert","data":[{"timestamp":"2021-03-12T01:46:03.886Z","symbol":"ETHH21","side":"Buy","size":1,"price":0.03191,"tickDirection":"PlusTick","trdMatchID":"a9371640-78d6-53d9-c9e4-31f7b7afb06d","grossValue":3191000,"homeNotional":1,"foreignNotional":0.03191}]}"#;
        let trade = &parse_trade("bitmex", MarketType::LinearFuture, raw_msg).unwrap()[0];

        crate::utils::check_trade_fields(
            "bitmex",
            MarketType::LinearFuture,
            "ETH/BTC".to_string(),
            trade,
        );

        assert_eq!(trade.volume, trade.price * trade.quantity);
        assert_eq!(trade.side, TradeSide::Buy);
    }

    #[test]
    fn quanto_future() {
        let raw_msg = r#"{"table":"trade","action":"insert","data":[{"timestamp":"2021-03-12T02:13:43.222Z","symbol":"ETHUSDH21","side":"Sell","size":12,"price":1892.8,"tickDirection":"PlusTick","trdMatchID":"14c7d828-80c4-2c91-ad9e-1662081aeaec","grossValue":2271360,"homeNotional":0.6814310051107325,"foreignNotional":1289.8126064735945}]}"#;
        let trade = &parse_trade("bitmex", MarketType::QuantoFuture, raw_msg).unwrap()[0];

        crate::utils::check_trade_fields(
            "bitmex",
            MarketType::QuantoFuture,
            "ETH/USD".to_string(),
            trade,
        );

        assert!(approx_eq!(
            f64,
            trade.volume,
            trade.price * trade.quantity,
            ulps = 9
        ));
        assert_eq!(trade.side, TradeSide::Sell);
    }
}

#[cfg(test)]
mod funding_rate {
    use crypto_msg_parser::{parse_funding_rate, MarketType};

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"table":"funding","action":"partial","data":[{"timestamp":"2021-04-01T20:00:00.000Z","symbol":"XBTUSD","fundingInterval":"2000-01-01T08:00:00.000Z","fundingRate":0.000817,"fundingRateDaily":0.002451}]}"#;
        let funding_rates =
            &parse_funding_rate("bitmex", MarketType::InverseSwap, raw_msg).unwrap();

        assert_eq!(funding_rates.len(), 1);

        for rate in funding_rates.iter() {
            crate::utils::check_funding_rate_fields("bitmex", MarketType::InverseSwap, rate);
        }

        assert_eq!(funding_rates[0].pair, "BTC/USD".to_string());
        assert_eq!(funding_rates[0].funding_rate, 0.000817);
        assert_eq!(funding_rates[0].funding_time, 1617307200000);
    }

    #[test]
    fn quanto_swap() {
        let raw_msg = r#"{"table":"funding","action":"partial","data":[{"timestamp":"2021-04-01T20:00:00.000Z","symbol":"ETHUSD","fundingInterval":"2000-01-01T08:00:00.000Z","fundingRate":0.002142,"fundingRateDaily":0.006425999999999999}]}"#;
        let funding_rates = &parse_funding_rate("bitmex", MarketType::QuantoSwap, raw_msg).unwrap();

        assert_eq!(funding_rates.len(), 1);

        for rate in funding_rates.iter() {
            crate::utils::check_funding_rate_fields("bitmex", MarketType::QuantoSwap, rate);
        }

        assert_eq!(funding_rates[0].pair, "ETH/USD".to_string());
        assert_eq!(funding_rates[0].funding_rate, 0.002142);
        assert_eq!(funding_rates[0].funding_time, 1617307200000);
    }
}
