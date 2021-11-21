mod utils;

#[cfg(test)]
mod trade {
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, parse_trade, TradeSide};

    #[test]
    fn spot() {
        let raw_msg = r#"{"ch":"market.btcusdt.trade.detail","ts":1616243199157,"tick":{"id":123140716701,"ts":1616243199156,"data":[{"id":123140716701236887569077664,"ts":1616243199156,"tradeId":102357140867,"amount":1.98E-4,"price":58911.07,"direction":"sell"}]}}"#;
        let trade = &parse_trade("huobi", MarketType::Spot, raw_msg).unwrap()[0];

        crate::utils::check_trade_fields(
            "huobi",
            MarketType::Spot,
            "BTC/USDT".to_string(),
            extract_symbol("huobi", MarketType::Spot, raw_msg).unwrap(),
            trade,
        );

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
            extract_symbol("huobi", MarketType::InverseFuture, raw_msg).unwrap(),
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
            extract_symbol("huobi", MarketType::InverseFuture, raw_msg).unwrap(),
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
            extract_symbol("huobi", MarketType::InverseSwap, raw_msg).unwrap(),
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
            extract_symbol("huobi", MarketType::InverseSwap, raw_msg).unwrap(),
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
            extract_symbol("huobi", MarketType::LinearSwap, raw_msg).unwrap(),
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
            extract_symbol("huobi", MarketType::LinearSwap, raw_msg).unwrap(),
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
                extract_symbol("huobi", MarketType::EuropeanOption, raw_msg).unwrap(),
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
    use crypto_market_type::MarketType;
    use crypto_msg_parser::parse_funding_rate;

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

#[cfg(test)]
mod l2_orderbook {
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, parse_l2};
    use float_cmp::approx_eq;

    #[test]
    fn spot_update() {
        let raw_msg = r#"{"ch":"market.btcusdt.mbp.20","ts":1622707662703,"tick":{"seqNum":129803485567,"prevSeqNum":129803485424,"bids":[[38765.39,0.0],[38762.87,0.009708]],"asks":[[38762.88,0.102302]]}}"#;
        let orderbook = &parse_l2("huobi", MarketType::Spot, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 1);
        assert_eq!(orderbook.bids.len(), 2);
        assert!(!orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "huobi",
            MarketType::Spot,
            "BTC/USDT".to_string(),
            extract_symbol("huobi", MarketType::Spot, raw_msg).unwrap(),
            orderbook,
        );

        assert_eq!(orderbook.timestamp, 1622707662703);
        assert_eq!(orderbook.seq_id, Some(129803485567));
        assert_eq!(orderbook.prev_seq_id, Some(129803485424));

        assert_eq!(orderbook.asks[0].price, 38762.88);
        assert_eq!(orderbook.asks[0].quantity_base, 0.102302);
        assert_eq!(orderbook.asks[0].quantity_quote, 38762.88 * 0.102302);

        assert_eq!(orderbook.bids[0].price, 38765.39);
        assert_eq!(orderbook.bids[0].quantity_base, 0.0);
        assert_eq!(orderbook.bids[0].quantity_quote, 0.0);

        assert_eq!(orderbook.bids[1].price, 38762.87);
        assert_eq!(orderbook.bids[1].quantity_base, 0.009708);
        assert_eq!(orderbook.bids[1].quantity_quote, 38762.87 * 0.009708);

        let raw_msg = r#"{"ch":"market.btcusdt.mbp.20","ts":1634601197516,"tick":{"seqNum":140059393690,"prevSeqNum":140059393689,"asks":[[61945.07,5.33E-4]]}}"#;
        let orderbook = &parse_l2("huobi", MarketType::Spot, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 1);
        assert_eq!(orderbook.bids.len(), 0);
        assert!(!orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "huobi",
            MarketType::Spot,
            "BTC/USDT".to_string(),
            extract_symbol("huobi", MarketType::Spot, raw_msg).unwrap(),
            orderbook,
        );

        assert_eq!(orderbook.timestamp, 1634601197516);

        assert_eq!(orderbook.asks[0].price, 61945.07);
        assert_eq!(orderbook.asks[0].quantity_base, 0.000533);
        assert_eq!(orderbook.asks[0].quantity_quote, 61945.07 * 0.000533);
    }

    #[test]
    fn inverse_future_snapshot() {
        let raw_msg = r#"{"ch":"market.BTC_CQ.depth.size_150.high_freq","tick":{"asks":[[38884.91,652],[38886.32,21],[38887.88,4]],"bids":[[38884.9,6],[38883.86,6],[38880.25,3]],"ch":"market.BTC_CQ.depth.size_150.high_freq","event":"snapshot","id":138216299603,"mrid":138216299603,"ts":1622708089134,"version":1223482159},"ts":1622708089134}"#;
        let orderbook = &parse_l2("huobi", MarketType::InverseFuture, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 3);
        assert_eq!(orderbook.bids.len(), 3);
        assert!(orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "huobi",
            MarketType::InverseFuture,
            "BTC/USD".to_string(),
            extract_symbol("huobi", MarketType::InverseFuture, raw_msg).unwrap(),
            orderbook,
        );

        assert_eq!(orderbook.timestamp, 1622708089134);
        assert_eq!(orderbook.seq_id, Some(138216299603));

        assert_eq!(orderbook.asks[0].price, 38884.91);
        assert_eq!(orderbook.asks[0].quantity_base, 65200.0 / 38884.91);
        assert_eq!(orderbook.asks[0].quantity_quote, 65200.0);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 652.0);

        assert_eq!(orderbook.asks[2].price, 38887.88);
        assert_eq!(orderbook.asks[2].quantity_base, 400.0 / 38887.88);
        assert_eq!(orderbook.asks[2].quantity_quote, 400.0);
        assert_eq!(orderbook.asks[2].quantity_contract.unwrap(), 4.0);

        assert_eq!(orderbook.bids[0].price, 38884.9);
        assert_eq!(orderbook.bids[0].quantity_base, 600.0 / 38884.9);
        assert_eq!(orderbook.bids[0].quantity_quote, 600.0);
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 6.0);

        assert_eq!(orderbook.bids[2].price, 38880.25);
        assert_eq!(orderbook.bids[2].quantity_base, 300.0 / 38880.25);
        assert_eq!(orderbook.bids[2].quantity_quote, 300.0);
        assert_eq!(orderbook.bids[2].quantity_contract.unwrap(), 3.0);
    }

    #[test]
    fn inverse_future_update() {
        let raw_msg = r#"{"ch":"market.BTC_CQ.depth.size_150.high_freq","tick":{"asks":[[38939.82,10],[38958.06,100],[38973.97,0]],"bids":[[38932.53,200],[38926.08,0],[38912.29,0]],"ch":"market.BTC_CQ.depth.size_150.high_freq","event":"update","id":138219575176,"mrid":138219575176,"ts":1622711041458,"version":1223606224},"ts":1622711041458}"#;
        let orderbook = &parse_l2("huobi", MarketType::InverseFuture, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 3);
        assert_eq!(orderbook.bids.len(), 3);
        assert!(!orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "huobi",
            MarketType::InverseFuture,
            "BTC/USD".to_string(),
            extract_symbol("huobi", MarketType::InverseFuture, raw_msg).unwrap(),
            orderbook,
        );

        assert_eq!(orderbook.timestamp, 1622711041458);
        assert_eq!(orderbook.seq_id, Some(138219575176));

        assert_eq!(orderbook.asks[0].price, 38939.82);
        assert_eq!(orderbook.asks[0].quantity_base, 1000.0 / 38939.82);
        assert_eq!(orderbook.asks[0].quantity_quote, 1000.0);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 10.0);

        assert_eq!(orderbook.asks[2].price, 38973.97);
        assert_eq!(orderbook.asks[2].quantity_base, 0.0);
        assert_eq!(orderbook.asks[2].quantity_quote, 0.0);
        assert_eq!(orderbook.asks[2].quantity_contract.unwrap(), 0.0);

        assert_eq!(orderbook.bids[0].price, 38932.53);
        assert_eq!(orderbook.bids[0].quantity_base, 20000.0 / 38932.53);
        assert_eq!(orderbook.bids[0].quantity_quote, 20000.0);
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 200.0);

        assert_eq!(orderbook.bids[2].price, 38912.29);
        assert_eq!(orderbook.bids[2].quantity_base, 0.0);
        assert_eq!(orderbook.bids[2].quantity_quote, 0.0);
        assert_eq!(orderbook.bids[2].quantity_contract.unwrap(), 0.0);
    }

    #[test]
    fn inverse_swap_snapshot() {
        let raw_msg = r#"{"ch":"market.BTC-USD.depth.size_150.high_freq","tick":{"asks":[[38888,9949],[38888.1,1],[38888.2,1]],"bids":[[38887.9,3832],[38887.8,4],[38887.7,3]],"ch":"market.BTC-USD.depth.size_150.high_freq","event":"snapshot","id":99893955238,"mrid":99893955238,"ts":1622711365595,"version":1300632701},"ts":1622711365595}"#;
        let orderbook = &parse_l2("huobi", MarketType::InverseSwap, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 3);
        assert_eq!(orderbook.bids.len(), 3);
        assert!(orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "huobi",
            MarketType::InverseSwap,
            "BTC/USD".to_string(),
            extract_symbol("huobi", MarketType::InverseSwap, raw_msg).unwrap(),
            orderbook,
        );

        assert_eq!(orderbook.timestamp, 1622711365595);
        assert_eq!(orderbook.seq_id, Some(99893955238));

        assert_eq!(orderbook.asks[0].price, 38888.0);
        assert_eq!(orderbook.asks[0].quantity_base, 994900.0 / 38888.0);
        assert_eq!(orderbook.asks[0].quantity_quote, 994900.0);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 9949.0);

        assert_eq!(orderbook.asks[2].price, 38888.2);
        assert_eq!(orderbook.asks[2].quantity_base, 100.0 / 38888.2);
        assert_eq!(orderbook.asks[2].quantity_quote, 100.0);
        assert_eq!(orderbook.asks[2].quantity_contract.unwrap(), 1.0);

        assert_eq!(orderbook.bids[0].price, 38887.9);
        assert_eq!(orderbook.bids[0].quantity_base, 383200.0 / 38887.9);
        assert_eq!(orderbook.bids[0].quantity_quote, 383200.0);
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 3832.0);

        assert_eq!(orderbook.bids[2].price, 38887.7);
        assert_eq!(orderbook.bids[2].quantity_base, 300.0 / 38887.7);
        assert_eq!(orderbook.bids[2].quantity_quote, 300.0);
        assert_eq!(orderbook.bids[2].quantity_contract.unwrap(), 3.0);
    }

    #[test]
    fn inverse_swap_update() {
        let raw_msg = r#"{"ch":"market.BTC-USD.depth.size_150.high_freq","tick":{"asks":[[38895.7,1635]],"bids":[[38880.6,0],[38868.2,50]],"ch":"market.BTC-USD.depth.size_150.high_freq","event":"update","id":99893958868,"mrid":99893958868,"ts":1622711368355,"version":1300632845},"ts":1622711368355}"#;
        let orderbook = &parse_l2("huobi", MarketType::InverseSwap, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 1);
        assert_eq!(orderbook.bids.len(), 2);
        assert!(!orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "huobi",
            MarketType::InverseSwap,
            "BTC/USD".to_string(),
            extract_symbol("huobi", MarketType::InverseSwap, raw_msg).unwrap(),
            orderbook,
        );

        assert_eq!(orderbook.timestamp, 1622711368355);
        assert_eq!(orderbook.seq_id, Some(99893958868));

        assert_eq!(orderbook.asks[0].price, 38895.7);
        assert_eq!(orderbook.asks[0].quantity_base, 163500.0 / 38895.7);
        assert_eq!(orderbook.asks[0].quantity_quote, 163500.0);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 1635.0);

        assert_eq!(orderbook.bids[0].price, 38880.6);
        assert_eq!(orderbook.bids[0].quantity_base, 0.0);
        assert_eq!(orderbook.bids[0].quantity_quote, 0.0);
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 0.0);

        assert_eq!(orderbook.bids[1].price, 38868.2);
        assert_eq!(orderbook.bids[1].quantity_base, 5000.0 / 38868.2);
        assert_eq!(orderbook.bids[1].quantity_quote, 5000.0);
        assert_eq!(orderbook.bids[1].quantity_contract.unwrap(), 50.0);
    }

    #[test]
    fn linear_swap_snapshot() {
        let raw_msg = r#"{"ch":"market.BTC-USDT.depth.size_150.high_freq","tick":{"asks":[[39055,19345],[39056.8,1200],[39057.5,85]],"bids":[[39054.9,4754],[39054.8,1],[39054.7,1]],"ch":"market.BTC-USDT.depth.size_150.high_freq","event":"snapshot","id":39536665398,"mrid":39536665398,"ts":1622711946534,"version":709648689},"ts":1622711946534}"#;
        let orderbook = &parse_l2("huobi", MarketType::LinearSwap, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 3);
        assert_eq!(orderbook.bids.len(), 3);
        assert!(orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "huobi",
            MarketType::LinearSwap,
            "BTC/USDT".to_string(),
            extract_symbol("huobi", MarketType::LinearSwap, raw_msg).unwrap(),
            orderbook,
        );

        assert_eq!(orderbook.timestamp, 1622711946534);

        assert_eq!(orderbook.asks[0].price, 39055.0);
        assert_eq!(orderbook.asks[0].quantity_base, 19.345);
        assert_eq!(orderbook.asks[0].quantity_quote, 39055.0 * 19.345);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 19345.0);

        assert_eq!(orderbook.asks[2].price, 39057.5);
        assert_eq!(orderbook.asks[2].quantity_base, 0.085);
        assert_eq!(orderbook.asks[2].quantity_quote, 39057.5 * 0.085);
        assert_eq!(orderbook.asks[2].quantity_contract.unwrap(), 85.0);

        assert_eq!(orderbook.bids[0].price, 39054.9);
        assert!(approx_eq!(
            f64,
            orderbook.bids[0].quantity_base,
            4.754,
            epsilon = 0.000000000000001
        ));
        assert!(approx_eq!(
            f64,
            orderbook.bids[0].quantity_quote,
            39054.9 * 4.754,
            epsilon = 0.0000000001
        ));
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 4754.0);

        assert_eq!(orderbook.bids[2].price, 39054.7);
        assert_eq!(orderbook.bids[2].quantity_base, 0.001);
        assert_eq!(orderbook.bids[2].quantity_quote, 39054.7 * 0.001);
        assert_eq!(orderbook.bids[2].quantity_contract.unwrap(), 1.0);
    }

    #[test]
    fn linear_swap_update() {
        let raw_msg = r#"{"ch":"market.BTC-USDT.depth.size_150.high_freq","tick":{"asks":[[39055,16634],[39060.1,0]],"bids":[[39050.8,40]],"ch":"market.BTC-USDT.depth.size_150.high_freq","event":"update","id":39536668357,"mrid":39536668357,"ts":1622711948514,"version":709648808},"ts":1622711948514}"#;
        let orderbook = &parse_l2("huobi", MarketType::LinearSwap, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 2);
        assert_eq!(orderbook.bids.len(), 1);
        assert!(!orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "huobi",
            MarketType::LinearSwap,
            "BTC/USDT".to_string(),
            extract_symbol("huobi", MarketType::LinearSwap, raw_msg).unwrap(),
            orderbook,
        );

        assert_eq!(orderbook.timestamp, 1622711948514);

        assert_eq!(orderbook.asks[0].price, 39055.0);
        assert_eq!(orderbook.asks[0].quantity_base, 16.634);
        assert_eq!(orderbook.asks[0].quantity_quote, 39055.0 * 16.634);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 16634.0);

        assert_eq!(orderbook.asks[1].price, 39060.1);
        assert_eq!(orderbook.asks[1].quantity_base, 0.0);
        assert_eq!(orderbook.asks[1].quantity_quote, 0.0);
        assert_eq!(orderbook.asks[1].quantity_contract.unwrap(), 0.0);

        assert_eq!(orderbook.bids[0].price, 39050.8);
        assert_eq!(orderbook.bids[0].quantity_base, 0.04);
        assert_eq!(orderbook.bids[0].quantity_quote, 39050.8 * 0.04);
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 40.0);
    }
}
