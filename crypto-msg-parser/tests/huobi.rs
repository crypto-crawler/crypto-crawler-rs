mod utils;

#[cfg(test)]
mod trade {
    use crypto_msg_parser::{parse_trade, MarketType, TradeSide};

    #[test]
    fn spot() {
        let raw_msg = r#"{"ch":"market.btcusdt.trade.detail","ts":1616243199157,"tick":{"id":123140716701,"ts":1616243199156,"data":[{"id":123140716701236887569077664,"ts":1616243199156,"tradeId":102357140867,"amount":1.98E-4,"price":58911.07,"direction":"sell"}]}}"#;
        let trade = &parse_trade("huobi", MarketType::Spot, raw_msg).unwrap()[0];

        crate::utils::check_trade_fields("huobi", MarketType::Spot, "BTC/USDT".to_string(), trade);

        assert_eq!(trade.quantity_base, 1.98E-4);
        assert_eq!(trade.side, TradeSide::Sell);
    }

    #[test]
    fn inverse_future() {
        let raw_msg = r#"{"ch":"market.BTC_CQ.trade.detail","ts":1616231995793,"tick":{"id":128974648797,"ts":1616231995768,"data":[{"amount":2,"quantity":0.0031859832031779545255059460801016711,"ts":1616231995768,"id":1289746487970000,"price":62774.97,"direction":"buy"}]}}"#;
        let trade = &parse_trade("huobi", MarketType::InverseFuture, raw_msg).unwrap()[0];
        crate::utils::check_trade_fields(
            "huobi",
            MarketType::InverseFuture,
            "BTC/USD".to_string(),
            trade,
        );
        assert_eq!(trade.quantity_base, 200.0 / 62774.97);
        assert_eq!(trade.quantity_quote, 200.0);
        assert_eq!(trade.quantity_contract, Some(2.0));
        assert_eq!(trade.side, TradeSide::Buy);

        let raw_msg = r#"{"ch":"market.ETH_CQ.trade.detail","ts":1616269629976,"tick":{"id":128632765054,"ts":1616269629958,"data":[{"amount":2,"quantity":0.0100143605930904917651912843016886215,"ts":1616269629958,"id":1286327650540000,"price":1997.132,"direction":"sell"}]}}"#;
        let trade = &parse_trade("huobi", MarketType::InverseFuture, raw_msg).unwrap()[0];
        crate::utils::check_trade_fields(
            "huobi",
            MarketType::InverseFuture,
            "ETH/USD".to_string(),
            trade,
        );
        assert_eq!(trade.quantity_base, 20.0 / 1997.132);
        assert_eq!(trade.quantity_quote, 20.0);
        assert_eq!(trade.quantity_contract, Some(2.0));
        assert_eq!(trade.side, TradeSide::Sell);
    }

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"ch":"market.BTC-USD.trade.detail","ts":1616233683377,"tick":{"id":84230699705,"ts":1616233683352,"data":[{"amount":6,"quantity":0.0102273366481267780650901795408948579,"ts":1616233683352,"id":842306997050000,"price":58666.3,"direction":"buy"}]}}"#;
        let trade = &parse_trade("huobi", MarketType::InverseSwap, raw_msg).unwrap()[0];
        crate::utils::check_trade_fields(
            "huobi",
            MarketType::InverseSwap,
            "BTC/USD".to_string(),
            trade,
        );
        assert_eq!(trade.quantity_base, 600.0 / 58666.3);
        assert_eq!(trade.quantity_quote, 600.0);
        assert_eq!(trade.quantity_contract, Some(6.0));
        assert_eq!(trade.side, TradeSide::Buy);

        let raw_msg = r#"{"ch":"market.ETH-USD.trade.detail","ts":1616269812566,"tick":{"id":79855942906,"ts":1616269812548,"data":[{"amount":346,"quantity":1.871099622535394066559231659438237489,"ts":1616269812548,"id":798559429060000,"price":1849.18,"direction":"sell"}]}}"#;
        let trade = &parse_trade("huobi", MarketType::InverseSwap, raw_msg).unwrap()[0];
        crate::utils::check_trade_fields(
            "huobi",
            MarketType::InverseSwap,
            "ETH/USD".to_string(),
            trade,
        );
        assert_eq!(trade.quantity_base, 3460.0 / 1849.18);
        assert_eq!(trade.quantity_quote, 3460.0);
        assert_eq!(trade.quantity_contract, Some(346.0));
        assert_eq!(trade.side, TradeSide::Sell);
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"ch":"market.BTC-USDT.trade.detail","ts":1616233478594,"tick":{"id":22419995164,"ts":1616233478583,"data":[{"amount":40,"quantity":0.04,"trade_turnover":2350.796,"ts":1616233478583,"id":224199951640000,"price":58769.9,"direction":"sell"}]}}"#;
        let trade = &parse_trade("huobi", MarketType::LinearSwap, raw_msg).unwrap()[0];
        crate::utils::check_trade_fields(
            "huobi",
            MarketType::LinearSwap,
            "BTC/USDT".to_string(),
            trade,
        );
        assert_eq!(trade.quantity_base, 0.04);
        assert_eq!(trade.quantity_quote, 2350.796);
        assert_eq!(trade.quantity_contract, Some(40.0));
        assert_eq!(trade.side, TradeSide::Sell);

        let raw_msg = r#"{"ch":"market.ETH-USDT.trade.detail","ts":1616270565862,"tick":{"id":19056652696,"ts":1616270565838,"data":[{"amount":18,"quantity":0.18,"trade_turnover":332.487,"ts":1616270565838,"id":190566526960000,"price":1847.15,"direction":"sell"}]}}"#;
        let trade = &parse_trade("huobi", MarketType::LinearSwap, raw_msg).unwrap()[0];
        crate::utils::check_trade_fields(
            "huobi",
            MarketType::LinearSwap,
            "ETH/USDT".to_string(),
            trade,
        );
        assert_eq!(trade.quantity_base, 0.18);
        assert_eq!(trade.quantity_quote, 332.487);
        assert_eq!(trade.quantity_contract, Some(18.0));
        assert_eq!(trade.side, TradeSide::Sell);
    }

    #[test]
    fn linear_option() {
        let raw_msg = r#"{"ch":"market.BTC-USDT-210326-C-32000.trade.detail","ts":1616246303142,"tick":{"id":674495368,"ts":1616246303133,"data":[{"amount":36,"quantity":0.036,"trade_turnover":971.69976,"ts":1616246303133,"id":6744953680000,"price":26991.66,"direction":"buy"},{"amount":42,"quantity":0.042,"trade_turnover":1134,"ts":1616246303133,"id":6744953680001,"price":27000,"direction":"buy"}]}}"#;
        let trades = &parse_trade("huobi", MarketType::EuropeanOption, raw_msg).unwrap();
        assert_eq!(trades.len(), 2);

        for trade in trades.iter() {
            crate::utils::check_trade_fields(
                "huobi",
                MarketType::EuropeanOption,
                "BTC/USDT".to_string(),
                trade,
            );
        }

        assert_eq!(trades[0].quantity_base, 0.036);
        assert_eq!(trades[0].quantity_quote, 971.69976);
        assert_eq!(trades[0].quantity_contract, Some(36.0));
        assert_eq!(trades[0].side, TradeSide::Buy);

        assert_eq!(trades[1].quantity_base, 0.042);
        assert_eq!(trades[1].quantity_quote, 1134.0);
        assert_eq!(trades[1].quantity_contract, Some(42.0));
        assert_eq!(trades[1].side, TradeSide::Buy);
    }
}

#[cfg(test)]
mod funding_rate {
    use crypto_msg_parser::{parse_funding_rate, MarketType};

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"op":"notify","topic":"public.BTC-USD.funding_rate","ts":1617309842839,"data":[{"symbol":"BTC","contract_code":"BTC-USD","fee_asset":"BTC","funding_time":"1617309840000","funding_rate":"0.000624180443735412","estimated_rate":"0.000807076648698898","settlement_time":"1617321600000"}]}"#;
        let funding_rates = &parse_funding_rate("huobi", MarketType::InverseSwap, raw_msg).unwrap();

        assert_eq!(funding_rates.len(), 1);

        for rate in funding_rates.iter() {
            crate::utils::check_funding_rate_fields("huobi", MarketType::InverseSwap, rate);
        }

        assert_eq!(funding_rates[0].pair, "BTC/USD".to_string());
        assert_eq!(funding_rates[0].funding_rate, 0.000624180443735412);
        assert_eq!(funding_rates[0].estimated_rate, Some(0.000807076648698898));
        assert_eq!(funding_rates[0].funding_time, 1617321600000);
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"op":"notify","topic":"public.BTC-USDT.funding_rate","ts":1617309787271,"data":[{"symbol":"BTC","contract_code":"BTC-USDT","fee_asset":"USDT","funding_time":"1617309780000","funding_rate":"0.000754108135233895","estimated_rate":"0.000429934303518805","settlement_time":"1617321600000"}]}"#;
        let funding_rates = &parse_funding_rate("huobi", MarketType::LinearSwap, raw_msg).unwrap();

        assert_eq!(funding_rates.len(), 1);

        for rate in funding_rates.iter() {
            crate::utils::check_funding_rate_fields("huobi", MarketType::LinearSwap, rate);
        }

        assert_eq!(funding_rates[0].pair, "BTC/USDT".to_string());
        assert_eq!(funding_rates[0].funding_rate, 0.000754108135233895);
        assert_eq!(funding_rates[0].estimated_rate, Some(0.000429934303518805));
        assert_eq!(funding_rates[0].funding_time, 1617321600000);
    }
}
