mod utils;

const EXCHANGE_NAME: &str = "huobi";

#[cfg(test)]
mod trade {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_message::TradeSide;
    use crypto_msg_parser::{extract_symbol, extract_timestamp, parse_trade};

    #[test]
    fn spot() {
        let raw_msg = r#"{"ch":"market.btcusdt.trade.detail","ts":1616243199157,"tick":{"id":123140716701,"ts":1616243199156,"data":[{"id":123140716701236887569077664,"ts":1616243199156,"tradeId":102357140867,"amount":1.98E-4,"price":58911.07,"direction":"sell"}]}}"#;
        let trade = &parse_trade(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()[0];

        crate::utils::check_trade_fields(
            EXCHANGE_NAME,
            MarketType::Spot,
            "BTC/USDT".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap(),
            trade,
            raw_msg,
        );
        assert_eq!(
            1616243199157,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(trade.quantity_base, 1.98E-4);
        assert_eq!(trade.side, TradeSide::Sell);
    }

    #[test]
    fn inverse_future() {
        let raw_msg = r#"{"ch":"market.BTC_CQ.trade.detail","ts":1616231995793,"tick":{"id":128974648797,"ts":1616231995768,"data":[{"amount":2,"quantity":0.0031859832031779545255059460801016711,"ts":1616231995768,"id":1289746487970000,"price":62774.97,"direction":"buy"}]}}"#;
        let trade = &parse_trade(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg).unwrap()[0];
        crate::utils::check_trade_fields(
            EXCHANGE_NAME,
            MarketType::InverseFuture,
            "BTC/USD".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg).unwrap(),
            trade,
            raw_msg,
        );
        assert_eq!(trade.quantity_base, 200.0 / 62774.97);
        assert_eq!(trade.quantity_quote, 200.0);
        assert_eq!(trade.quantity_contract, Some(2.0));
        assert_eq!(trade.side, TradeSide::Buy);

        let raw_msg = r#"{"ch":"market.ETH_CQ.trade.detail","ts":1616269629976,"tick":{"id":128632765054,"ts":1616269629958,"data":[{"amount":2,"quantity":0.0100143605930904917651912843016886215,"ts":1616269629958,"id":1286327650540000,"price":1997.132,"direction":"sell"}]}}"#;
        let trade = &parse_trade(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg).unwrap()[0];
        crate::utils::check_trade_fields(
            EXCHANGE_NAME,
            MarketType::InverseFuture,
            "ETH/USD".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg).unwrap(),
            trade,
            raw_msg,
        );
        assert_eq!(
            1616269629976,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(trade.quantity_base, 20.0 / 1997.132);
        assert_eq!(trade.quantity_quote, 20.0);
        assert_eq!(trade.quantity_contract, Some(2.0));
        assert_eq!(trade.side, TradeSide::Sell);
    }

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"ch":"market.BTC-USD.trade.detail","ts":1616233683377,"tick":{"id":84230699705,"ts":1616233683352,"data":[{"amount":6,"quantity":0.0102273366481267780650901795408948579,"ts":1616233683352,"id":842306997050000,"price":58666.3,"direction":"buy"}]}}"#;
        let trade = &parse_trade(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap()[0];
        crate::utils::check_trade_fields(
            EXCHANGE_NAME,
            MarketType::InverseSwap,
            "BTC/USD".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap(),
            trade,
            raw_msg,
        );
        assert_eq!(trade.quantity_base, 600.0 / 58666.3);
        assert_eq!(trade.quantity_quote, 600.0);
        assert_eq!(trade.quantity_contract, Some(6.0));
        assert_eq!(trade.side, TradeSide::Buy);

        let raw_msg = r#"{"ch":"market.ETH-USD.trade.detail","ts":1616269812566,"tick":{"id":79855942906,"ts":1616269812548,"data":[{"amount":346,"quantity":1.871099622535394066559231659438237489,"ts":1616269812548,"id":798559429060000,"price":1849.18,"direction":"sell"}]}}"#;
        let trade = &parse_trade(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap()[0];
        crate::utils::check_trade_fields(
            EXCHANGE_NAME,
            MarketType::InverseSwap,
            "ETH/USD".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap(),
            trade,
            raw_msg,
        );
        assert_eq!(
            1616269812566,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(trade.quantity_base, 3460.0 / 1849.18);
        assert_eq!(trade.quantity_quote, 3460.0);
        assert_eq!(trade.quantity_contract, Some(346.0));
        assert_eq!(trade.side, TradeSide::Sell);
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"ch":"market.BTC-USDT.trade.detail","ts":1616233478594,"tick":{"id":22419995164,"ts":1616233478583,"data":[{"amount":40,"quantity":0.04,"trade_turnover":2350.796,"ts":1616233478583,"id":224199951640000,"price":58769.9,"direction":"sell"}]}}"#;
        let trade = &parse_trade(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()[0];
        crate::utils::check_trade_fields(
            EXCHANGE_NAME,
            MarketType::LinearSwap,
            "BTC/USDT".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap(),
            trade,
            raw_msg,
        );
        assert_eq!(
            1616233478594,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(trade.quantity_base, 0.04);
        assert_eq!(trade.quantity_quote, 2350.796);
        assert_eq!(trade.quantity_contract, Some(40.0));
        assert_eq!(trade.side, TradeSide::Sell);

        let raw_msg = r#"{"ch":"market.ETH-USDT.trade.detail","ts":1616270565862,"tick":{"id":19056652696,"ts":1616270565838,"data":[{"amount":18,"quantity":0.18,"trade_turnover":332.487,"ts":1616270565838,"id":190566526960000,"price":1847.15,"direction":"sell"}]}}"#;
        let trade = &parse_trade(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()[0];
        crate::utils::check_trade_fields(
            EXCHANGE_NAME,
            MarketType::LinearSwap,
            "ETH/USDT".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap(),
            trade,
            raw_msg,
        );
        assert_eq!(trade.quantity_base, 0.18);
        assert_eq!(trade.quantity_quote, 332.487);
        assert_eq!(trade.quantity_contract, Some(18.0));
        assert_eq!(trade.side, TradeSide::Sell);
    }

    #[test]
    fn linear_option() {
        let raw_msg = r#"{"ch":"market.BTC-USDT-210326-C-32000.trade.detail","ts":1616246303142,"tick":{"id":674495368,"ts":1616246303133,"data":[{"amount":36,"quantity":0.036,"trade_turnover":971.69976,"ts":1616246303133,"id":6744953680000,"price":26991.66,"direction":"buy"},{"amount":42,"quantity":0.042,"trade_turnover":1134,"ts":1616246303133,"id":6744953680001,"price":27000,"direction":"buy"}]}}"#;
        let trades = &parse_trade(EXCHANGE_NAME, MarketType::EuropeanOption, raw_msg).unwrap();
        assert_eq!(trades.len(), 2);

        for trade in trades.iter() {
            crate::utils::check_trade_fields(
                EXCHANGE_NAME,
                MarketType::EuropeanOption,
                "BTC/USDT".to_string(),
                extract_symbol(EXCHANGE_NAME, MarketType::EuropeanOption, raw_msg).unwrap(),
                trade,
                raw_msg,
            );
        }
        assert_eq!(
            1616246303142,
            extract_timestamp(EXCHANGE_NAME, MarketType::EuropeanOption, raw_msg)
                .unwrap()
                .unwrap()
        );

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
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, extract_timestamp, parse_funding_rate};

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"op":"notify","topic":"public.BTC-USD.funding_rate","ts":1617309842839,"data":[{"symbol":"BTC","contract_code":"BTC-USD","fee_asset":"BTC","funding_time":"1617309840000","funding_rate":"0.000624180443735412","estimated_rate":"0.000807076648698898","settlement_time":"1617321600000"}]}"#;
        let funding_rates =
            &parse_funding_rate(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg, None).unwrap();

        assert_eq!(funding_rates.len(), 1);

        for rate in funding_rates.iter() {
            crate::utils::check_funding_rate_fields(
                EXCHANGE_NAME,
                MarketType::InverseSwap,
                rate,
                raw_msg,
            );
        }
        assert_eq!(
            "BTC-USD",
            extract_symbol(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap()
        );
        assert_eq!(
            1617309842839,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(funding_rates[0].pair, "BTC/USD".to_string());
        assert_eq!(funding_rates[0].funding_rate, 0.000624180443735412);
        assert_eq!(funding_rates[0].estimated_rate, Some(0.000807076648698898));
        assert_eq!(funding_rates[0].funding_time, 1617321600000);
        assert_eq!(funding_rates[0].timestamp, 1617309842839);
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"op":"notify","topic":"public.BTC-USDT.funding_rate","ts":1617309787271,"data":[{"symbol":"BTC","contract_code":"BTC-USDT","fee_asset":"USDT","funding_time":"1617309780000","funding_rate":"0.000754108135233895","estimated_rate":"0.000429934303518805","settlement_time":"1617321600000"}]}"#;
        let funding_rates =
            &parse_funding_rate(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg, None).unwrap();

        assert_eq!(funding_rates.len(), 1);

        for rate in funding_rates.iter() {
            crate::utils::check_funding_rate_fields(
                EXCHANGE_NAME,
                MarketType::LinearSwap,
                rate,
                raw_msg,
            );
        }
        assert_eq!(
            "BTC-USDT",
            extract_symbol(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap()
        );
        assert_eq!(
            1617309787271,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(funding_rates[0].pair, "BTC/USDT".to_string());
        assert_eq!(funding_rates[0].funding_rate, 0.000754108135233895);
        assert_eq!(funding_rates[0].estimated_rate, Some(0.000429934303518805));
        assert_eq!(funding_rates[0].funding_time, 1617321600000);
        assert_eq!(funding_rates[0].timestamp, 1617309787271);
    }

    #[test]
    fn all() {
        let raw_msg = r#"{"op":"notify","topic":"public.*.funding_rate","ts":1654174017332,"data":[{"symbol":"BTC","contract_code":"BTC-USD","fee_asset":"BTC","funding_time":"1654173960000","funding_rate":"0.000046774664737679","estimated_rate":"-0.000042194357938054","settlement_time":"1654185600000"},{"symbol":"ETH","contract_code":"ETH-USD","fee_asset":"ETH","funding_time":"1654173960000","funding_rate":"-0.000050627986553411","estimated_rate":"0.000074887269002104","settlement_time":"1654185600000"}]}"#;
        let funding_rates =
            &parse_funding_rate(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg, None).unwrap();

        assert_eq!(funding_rates.len(), 2);

        for rate in funding_rates.iter() {
            crate::utils::check_funding_rate_fields(
                EXCHANGE_NAME,
                MarketType::InverseSwap,
                rate,
                raw_msg,
            );
        }
        assert_eq!(
            "ALL",
            extract_symbol(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap()
        );
        assert_eq!(
            1654174017332,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(funding_rates[0].pair, "BTC/USD".to_string());
        assert_eq!(funding_rates[0].funding_rate, 0.000046774664737679);
        assert_eq!(funding_rates[0].estimated_rate, Some(-0.000042194357938054));
        assert_eq!(funding_rates[0].funding_time, 1654185600000);
        assert_eq!(funding_rates[0].timestamp, 1654174017332);

        assert_eq!(funding_rates[1].pair, "ETH/USD".to_string());
        assert_eq!(funding_rates[1].funding_rate, -0.000050627986553411);
        assert_eq!(funding_rates[1].estimated_rate, Some(0.000074887269002104));
        assert_eq!(funding_rates[1].funding_time, 1654185600000);
        assert_eq!(funding_rates[1].timestamp, 1654174017332);
    }
}

#[cfg(test)]
mod l2_event {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, extract_timestamp, parse_l2, round};
    use crypto_msg_type::MessageType;

    #[test]
    fn spot_update() {
        let raw_msg = r#"{"ch":"market.btcusdt.mbp.20","ts":1622707662703,"tick":{"seqNum":129803485567,"prevSeqNum":129803485424,"bids":[[38765.39,0.0],[38762.87,0.009708]],"asks":[[38762.88,0.102302]]}}"#;
        let orderbook = &parse_l2(EXCHANGE_NAME, MarketType::Spot, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 1);
        assert_eq!(orderbook.bids.len(), 2);
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
            1622707662703,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
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
        let orderbook = &parse_l2(EXCHANGE_NAME, MarketType::Spot, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 1);
        assert_eq!(orderbook.bids.len(), 0);
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
            1634601197516,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1634601197516);

        assert_eq!(orderbook.asks[0].price, 61945.07);
        assert_eq!(orderbook.asks[0].quantity_base, 0.000533);
        assert_eq!(orderbook.asks[0].quantity_quote, 61945.07 * 0.000533);
    }

    #[test]
    fn inverse_future_snapshot() {
        let raw_msg = r#"{"ch":"market.BTC_CQ.depth.size_150.high_freq","tick":{"asks":[[38884.91,652],[38886.32,21],[38887.88,4]],"bids":[[38884.9,6],[38883.86,6],[38880.25,3]],"ch":"market.BTC_CQ.depth.size_150.high_freq","event":"snapshot","id":138216299603,"mrid":138216299603,"ts":1622708089134,"version":1223482159},"ts":1622708089134}"#;
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
            1622708089134,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg)
                .unwrap()
                .unwrap()
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
        let orderbook =
            &parse_l2(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 3);
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
            1622711041458,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg)
                .unwrap()
                .unwrap()
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
    fn inverse_future_snapshot_2() {
        let raw_msg = r#"{"ch":"market.DOT_CW.depth.size_20.high_freq","tick":{"asks":[[9.9252,1569],[9.9301,1964],[10.0029,44]],"bids":null,"ch":"market.DOT_CW.depth.size_20.high_freq","event":"snapshot","id":222100052218419,"mrid":222100052218419,"ts":1653033431770,"version":865562932},"ts":1653033431770}"#;
        let orderbook =
            &parse_l2(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 3);
        assert_eq!(orderbook.bids.len(), 0);
        assert!(orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            EXCHANGE_NAME,
            MarketType::InverseFuture,
            MessageType::L2Event,
            "DOT/USD".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );
        assert_eq!(
            "DOT_CW",
            extract_symbol(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg).unwrap()
        );
        assert_eq!(
            1653033431770,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1653033431770);
        assert_eq!(orderbook.seq_id, Some(222100052218419));

        assert_eq!(orderbook.asks[0].price, 9.9252);
        assert_eq!(orderbook.asks[0].quantity_base, 15690.0 / 9.9252);
        assert_eq!(orderbook.asks[0].quantity_quote, 15690.0);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 1569.0);

        assert_eq!(orderbook.asks[2].price, 10.0029);
        assert_eq!(orderbook.asks[2].quantity_base, 440.0 / 10.0029);
        assert_eq!(orderbook.asks[2].quantity_quote, 440.0);
        assert_eq!(orderbook.asks[2].quantity_contract.unwrap(), 44.0);
    }

    #[test]
    fn inverse_swap_snapshot() {
        let raw_msg = r#"{"ch":"market.BTC-USD.depth.size_150.high_freq","tick":{"asks":[[38888,9949],[38888.1,1],[38888.2,1]],"bids":[[38887.9,3832],[38887.8,4],[38887.7,3]],"ch":"market.BTC-USD.depth.size_150.high_freq","event":"snapshot","id":99893955238,"mrid":99893955238,"ts":1622711365595,"version":1300632701},"ts":1622711365595}"#;
        let orderbook =
            &parse_l2(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 3);
        assert_eq!(orderbook.bids.len(), 3);
        assert!(orderbook.snapshot);

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
            1622711365595,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
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
        let orderbook =
            &parse_l2(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 1);
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
            1622711368355,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
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
            1622711946534,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1622711946534);
        assert_eq!(orderbook.seq_id, Some(39536665398));

        assert_eq!(orderbook.asks[0].price, 39055.0);
        assert_eq!(orderbook.asks[0].quantity_base, 19.345);
        assert_eq!(orderbook.asks[0].quantity_quote, 39055.0 * 19.345);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 19345.0);

        assert_eq!(orderbook.asks[2].price, 39057.5);
        assert_eq!(orderbook.asks[2].quantity_base, 0.085);
        assert_eq!(orderbook.asks[2].quantity_quote, round(39057.5 * 0.085));
        assert_eq!(orderbook.asks[2].quantity_contract.unwrap(), 85.0);

        assert_eq!(orderbook.bids[0].price, 39054.9);
        assert_eq!(orderbook.bids[0].quantity_base, 4.754);
        assert_eq!(orderbook.bids[0].quantity_quote, round(39054.9 * 4.754));
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 4754.0);

        assert_eq!(orderbook.bids[2].price, 39054.7);
        assert_eq!(orderbook.bids[2].quantity_base, 0.001);
        assert_eq!(orderbook.bids[2].quantity_quote, 39054.7 * 0.001);
        assert_eq!(orderbook.bids[2].quantity_contract.unwrap(), 1.0);
    }

    #[test]
    fn linear_swap_update() {
        let raw_msg = r#"{"ch":"market.BTC-USDT.depth.size_150.high_freq","tick":{"asks":[[39055,16634],[39060.1,0]],"bids":[[39050.8,40]],"ch":"market.BTC-USDT.depth.size_150.high_freq","event":"update","id":39536668357,"mrid":39536668357,"ts":1622711948514,"version":709648808},"ts":1622711948514}"#;
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
            1622711948514,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1622711948514);
        assert_eq!(orderbook.seq_id, Some(39536668357));

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
        assert_eq!(orderbook.bids[0].quantity_quote, round(39050.8 * 0.04));
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 40.0);
    }
}

#[cfg(test)]
mod l2_topk {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, extract_timestamp, parse_l2_topk, round};
    use crypto_msg_type::MessageType;

    #[test]
    fn spot() {
        let raw_msg = r#"{"ch":"market.btcusdt.depth.step1","ts":1653985338657,"tick":{"bids":[[31638.9,2.436837],[31638.5,0.349474],[31637.9,0.862589]],"asks":[[31639.0,1.062193],[31642.4,0.381939],[31642.7,0.190963]],"version":155386874272,"ts":1653985338000}}"#;
        let orderbook = &parse_l2_topk(EXCHANGE_NAME, MarketType::Spot, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 3);
        assert_eq!(orderbook.bids.len(), 3);
        assert!(orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            EXCHANGE_NAME,
            MarketType::Spot,
            MessageType::L2TopK,
            "BTC/USDT".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );
        assert_eq!(
            1653985338657,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1653985338657);
        assert_eq!(orderbook.seq_id, None);
        assert_eq!(orderbook.prev_seq_id, None);

        assert_eq!(orderbook.asks[0].price, 31639.0);
        assert_eq!(orderbook.asks[0].quantity_base, 1.062193);
        assert_eq!(orderbook.asks[0].quantity_quote, 31639.0 * 1.062193);
        assert_eq!(orderbook.asks[0].quantity_contract, None);

        assert_eq!(orderbook.asks[2].price, 31642.7);
        assert_eq!(orderbook.asks[2].quantity_base, 0.190963);
        assert_eq!(orderbook.asks[2].quantity_quote, 31642.7 * 0.190963);
        assert_eq!(orderbook.asks[2].quantity_contract, None);

        assert_eq!(orderbook.bids[0].price, 31638.9);
        assert_eq!(orderbook.bids[0].quantity_base, 2.436837);
        assert_eq!(orderbook.bids[0].quantity_quote, 31638.9 * 2.436837);
        assert_eq!(orderbook.bids[0].quantity_contract, None);

        assert_eq!(orderbook.bids[2].price, 31637.9);
        assert_eq!(orderbook.bids[2].quantity_base, 0.862589);
        assert_eq!(orderbook.bids[2].quantity_quote, 31637.9 * 0.862589);
        assert_eq!(orderbook.bids[2].quantity_contract, None);
    }

    #[test]
    fn inverse_future() {
        let raw_msg = r#"{"ch":"market.BTC_CQ.depth.step7","ts":1653986872201,"tick":{"mrid":222601050340438,"id":1653986872,"bids":[[31676.53,42],[31676,4],[31675.98,800]],"asks":[[31676.54,1],[31676.95,1],[31676.96,215]],"ts":1653986872197,"version":1653986872,"ch":"market.BTC_CQ.depth.step7"}}"#;
        let orderbook =
            &parse_l2_topk(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 3);
        assert_eq!(orderbook.bids.len(), 3);
        assert!(orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            EXCHANGE_NAME,
            MarketType::InverseFuture,
            MessageType::L2TopK,
            "BTC/USD".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );
        assert_eq!(
            1653986872201,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1653986872201);
        assert_eq!(orderbook.seq_id, Some(222601050340438));
        assert_eq!(orderbook.prev_seq_id, None);

        assert_eq!(orderbook.asks[0].price, 31676.54);
        assert_eq!(orderbook.asks[0].quantity_base, 100.0 / 31676.54);
        assert_eq!(orderbook.asks[0].quantity_quote, 100.0);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 1.0);

        assert_eq!(orderbook.asks[2].price, 31676.96);
        assert_eq!(orderbook.asks[2].quantity_base, 21500.0 / 31676.96);
        assert_eq!(orderbook.asks[2].quantity_quote, 21500.0);
        assert_eq!(orderbook.asks[2].quantity_contract.unwrap(), 215.0);

        assert_eq!(orderbook.bids[0].price, 31676.53);
        assert_eq!(orderbook.bids[0].quantity_base, 4200.0 / 31676.53);
        assert_eq!(orderbook.bids[0].quantity_quote, 4200.0);
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 42.0);

        assert_eq!(orderbook.bids[2].price, 31675.98);
        assert_eq!(orderbook.bids[2].quantity_base, 80000.0 / 31675.98);
        assert_eq!(orderbook.bids[2].quantity_quote, 80000.0);
        assert_eq!(orderbook.bids[2].quantity_contract.unwrap(), 800.0);
    }

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"ch":"market.BTC-USD.depth.step7","ts":1653988195290,"tick":{"mrid":136445301207,"id":1653988195,"bids":[[31565.4,564],[31564.1,7],[31563.4,200]],"asks":[[31565.5,2749],[31566.6,95],[31567,65]],"ts":1653988195288,"version":1653988195,"ch":"market.BTC-USD.depth.step7"}}"#;
        let orderbook =
            &parse_l2_topk(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 3);
        assert_eq!(orderbook.bids.len(), 3);
        assert!(orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            EXCHANGE_NAME,
            MarketType::InverseSwap,
            MessageType::L2TopK,
            "BTC/USD".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );
        assert_eq!(
            1653988195290,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1653988195290);
        assert_eq!(orderbook.seq_id, Some(136445301207));
        assert_eq!(orderbook.prev_seq_id, None);

        assert_eq!(orderbook.asks[0].price, 31565.5);
        assert_eq!(orderbook.asks[0].quantity_base, 274900.0 / 31565.5);
        assert_eq!(orderbook.asks[0].quantity_quote, 274900.0);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 2749.0);

        assert_eq!(orderbook.asks[2].price, 31567.0);
        assert_eq!(orderbook.asks[2].quantity_base, 6500.0 / 31567.0);
        assert_eq!(orderbook.asks[2].quantity_quote, 6500.0);
        assert_eq!(orderbook.asks[2].quantity_contract.unwrap(), 65.0);

        assert_eq!(orderbook.bids[0].price, 31565.4);
        assert_eq!(orderbook.bids[0].quantity_base, 56400.0 / 31565.4);
        assert_eq!(orderbook.bids[0].quantity_quote, 56400.0);
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 564.0);

        assert_eq!(orderbook.bids[2].price, 31563.4);
        assert_eq!(orderbook.bids[2].quantity_base, 20000.0 / 31563.4);
        assert_eq!(orderbook.bids[2].quantity_quote, 20000.0);
        assert_eq!(orderbook.bids[2].quantity_contract.unwrap(), 200.0);
    }

    #[test]
    fn inverse_swap_2() {
        let raw_msg = r#"{"ch":"market.ANT-USD.depth.step7","ts":1653868800233,"tick":{"mrid":68112277468,"id":1653868800,"ts":1653868800233,"version":1653868800,"ch":"market.ANT-USD.depth.step7"}}"#;
        let orderbook =
            &parse_l2_topk(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg, None).unwrap()[0];

        assert!(orderbook.asks.is_empty());
        assert!(orderbook.bids.is_empty());
        assert!(orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            EXCHANGE_NAME,
            MarketType::InverseSwap,
            MessageType::L2TopK,
            "ANT/USD".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );
        assert_eq!(
            1653868800233,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1653868800233);
        assert_eq!(orderbook.seq_id, Some(68112277468));
        assert_eq!(orderbook.prev_seq_id, None);
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"ch":"market.BTC-USDT.depth.step7","ts":1653988444928,"tick":{"mrid":108706801887,"id":1653988444,"bids":[[31589.9,2397],[31589.6,500],[31588.6,1]],"asks":[[31590,3053],[31590.5,6],[31590.6,692]],"ts":1653988444925,"version":1653988444,"ch":"market.BTC-USDT.depth.step7"}}"#;
        let orderbook =
            &parse_l2_topk(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 3);
        assert_eq!(orderbook.bids.len(), 3);
        assert!(orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            EXCHANGE_NAME,
            MarketType::LinearSwap,
            MessageType::L2TopK,
            "BTC/USDT".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );
        assert_eq!(
            1653988444928,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1653988444928);
        assert_eq!(orderbook.seq_id, Some(108706801887));
        assert_eq!(orderbook.prev_seq_id, None);

        assert_eq!(orderbook.asks[0].price, 31590.0);
        assert_eq!(orderbook.asks[0].quantity_base, 3.053);
        assert_eq!(orderbook.asks[0].quantity_quote, 31590.0 * 3.053);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 3053.0);

        assert_eq!(orderbook.asks[2].price, 31590.6);
        assert_eq!(orderbook.asks[2].quantity_base, 0.692);
        assert_eq!(orderbook.asks[2].quantity_quote, round(31590.6 * 0.692));
        assert_eq!(orderbook.asks[2].quantity_contract.unwrap(), 692.0);

        assert_eq!(orderbook.bids[0].price, 31589.9);
        assert_eq!(orderbook.bids[0].quantity_base, 2.397);
        assert_eq!(orderbook.bids[0].quantity_quote, round(31589.9 * 2.397));
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 2397.0);

        assert_eq!(orderbook.bids[2].price, 31588.6);
        assert_eq!(orderbook.bids[2].quantity_base, 0.001);
        assert_eq!(orderbook.bids[2].quantity_quote, 31588.6 * 0.001);
        assert_eq!(orderbook.bids[2].quantity_contract.unwrap(), 1.0);
    }

    #[test]
    fn linear_swap_2() {
        let raw_msg = r#"{"ch":"market.GST-USDT.depth.step7","ts":1651233614936,"tick":{"mrid":34526821266,"id":1651233614,"asks":[[7.5042,4218],[7.7385,194],[7.7451,67],[7.7484,281],[7.7517,439]],"ts":1651233614936,"version":1651233614,"ch":"market.GST-USDT.depth.step7"}}"#;
        let orderbook =
            &parse_l2_topk(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 5);
        assert_eq!(orderbook.bids.len(), 0);
        assert!(orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            EXCHANGE_NAME,
            MarketType::LinearSwap,
            MessageType::L2TopK,
            "GST/USDT".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );
        assert_eq!(
            "GST-USDT",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );
        assert_eq!(
            1651233614936,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1651233614936);
        assert_eq!(orderbook.seq_id, Some(34526821266));
        assert_eq!(orderbook.prev_seq_id, None);

        assert_eq!(orderbook.asks[0].price, 7.5042);
        assert_eq!(orderbook.asks[0].quantity_base, 0.1 * 4218.0);
        assert_eq!(orderbook.asks[0].quantity_quote, 0.1 * 4218.0 * 7.5042);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 4218.0);

        assert_eq!(orderbook.asks[4].price, 7.7517);
        assert_eq!(orderbook.asks[4].quantity_base, round(0.1 * 439.0));
        assert_eq!(
            orderbook.asks[4].quantity_quote,
            round(7.7517 * 0.1 * 439.0)
        );
        assert_eq!(orderbook.asks[4].quantity_contract.unwrap(), 439.0);
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
        let raw_msg = r#"{"ch":"market.btcusdt.bbo","ts":1654031600066,"tick":{"seqId":155441231856,"ask": 29010.91000000,"askSize":3.99953000,"bid":29010.90000000,"bidSize":13.94302000,"quoteTime":1654031600064,"symbol":"btcusdt"}}"#;

        assert_eq!(
            1654031600066,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "btcusdt",
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );

        let received_at = 1651122265862;
        let bbo_msg = parse_bbo(EXCHANGE_NAME, MarketType::Spot, raw_msg, Some(received_at)).unwrap();

        assert_eq!(MessageType::BBO, bbo_msg.msg_type);
        assert_eq!("btcusdt", bbo_msg.symbol);
        assert_eq!(received_at, bbo_msg.timestamp);
        assert_eq!(Some(155441231856), bbo_msg.id);

        assert_eq!(29010.91, bbo_msg.ask_price);
        assert_eq!(3.99953, bbo_msg.ask_quantity_base);
        assert_eq!(29010.91 * 3.99953, bbo_msg.ask_quantity_quote);
        assert_eq!(None, bbo_msg.ask_quantity_contract);

        assert_eq!(29010.9, bbo_msg.bid_price);
        assert_eq!(13.94302, bbo_msg.bid_quantity_base);
        assert_eq!(29010.9 * 13.94302, bbo_msg.bid_quantity_quote);
        assert_eq!(None, bbo_msg.bid_quantity_contract);
    }

    #[test]
    fn inverse_future() {
        let raw_msg = r#"{"ch":"market.BTC_CQ.bbo","ts":1654031781978,"tick":{"mrid":222601060251593,"id":1654031781,"bid":[31781.79,609],"ask":[31781.8,22],"ts":1654031781978,"version":222601060251593,"ch":"market.BTC_CQ.bbo"}}"#;

        assert_eq!(
            1654031781978,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "BTC_CQ",
            extract_symbol(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap()
        );
    }

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"ch":"market.BTC-USD.bbo","ts":1654031818692,"tick":{"mrid":136465693726,"id":1654031818,"bid":[31753.2,2495],"ask":[31753.3,249],"ts":1654031818692,"version":136465693726,"ch":"market.BTC-USD.bbo"}}"#;

        assert_eq!(
            1654031818692,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "BTC-USD",
            extract_symbol(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap()
        );


        let received_at = 1651122265862;
        let bbo_msg = parse_bbo(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg, Some(received_at)).unwrap();

        assert_eq!(MessageType::BBO, bbo_msg.msg_type);
        assert_eq!("BTC-USD", bbo_msg.symbol);
        assert_eq!(1654031818692, bbo_msg.timestamp);
        assert_eq!(Some(1654031818), bbo_msg.id);

        assert_eq!(31753.3, bbo_msg.ask_price);
        assert_eq!(0.7841704641722278, bbo_msg.ask_quantity_base);
        assert_eq!(31753.3 * 0.7841704641722278, bbo_msg.ask_quantity_quote);
        assert_eq!(Some(249.0), bbo_msg.ask_quantity_contract);

        assert_eq!(31753.2, bbo_msg.bid_price);
        assert_eq!(7.857475781968431, bbo_msg.bid_quantity_base);
        assert_eq!(31753.2 * 7.857475781968431, bbo_msg.bid_quantity_quote);
        assert_eq!(Some(2495.0), bbo_msg.bid_quantity_contract);

    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"ch":"market.BTC-USDT.bbo","ts":1654031855127,"tick":{"mrid":108746530167,"id":1654031855,"bid":[31784.1,5911],"ask":[31784.2,4],"ts":1654031855127,"version":108746530167,"ch":"market.BTC-USDT.bbo"}}"#;

        assert_eq!(
            1654031855127,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "BTC-USDT",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );
    }
}

#[cfg(test)]
mod candlestick {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, extract_timestamp, parse_candlestick};
    use crypto_msg_type::MessageType;

    #[test]
    fn spot() {
        let raw_msg = r#"{"ch":"market.btcusdt.kline.15mon","ts":1654081322624,"tick":{"id":1654081320,"open":31545.71,"close":31545.72,"low":31545.71,"high":31545.72,"amount":0.015443758717188892,"vol":487.1844552,"count":4}}"#;

        assert_eq!(
            1654081322624,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "btcusdt",
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );

        let data = parse_candlestick(EXCHANGE_NAME, MarketType::Spot, raw_msg, MessageType::L2TopK).unwrap();

        assert_eq!(1654081322624, data.timestamp);
        assert_eq!("15M", data.period);
    }

    #[test]
    fn inverse_future() {
        let raw_msg = r#"{"ch":"market.BTC_CQ.kline.1min","ts":1654081396435,"tick":{"id":1654081380,"mrid":222601067490403,"open":31565.04,"close":31565.04,"high":31565.04,"low":31565.04,"amount":0.0063361237622382230467631278148229814,"vol":2,"count":1}}"#;

        assert_eq!(
            1654081396435,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "BTC_CQ",
            extract_symbol(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap()
        );
    }

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"ch":"market.BTC-USD.kline.1min","ts":1654081441264,"tick":{"id":1654081440,"mrid":136483147489,"open":31514.4,"close":31514.4,"high":31514.4,"low":31514.4,"amount":0,"vol":0,"count":0}}"#;

        assert_eq!(
            1654081441264,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "BTC-USD",
            extract_symbol(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap()
        );
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"ch":"market.BTC-USDT.kline.1min","ts":1654081448870,"tick":{"id":1654081440,"mrid":108782988900,"open":31531.9,"close":31531.9,"high":31532,"low":31531.9,"amount":0.532,"vol":532,"trade_turnover":16774.9728,"count":5}}"#;

        assert_eq!(
            1654081448870,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "BTC-USDT",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );

        let data = parse_candlestick(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg, MessageType::L2TopK).unwrap();

        assert_eq!(1654081448870, data.timestamp);
        assert_eq!("1m", data.period);
    }
}

#[cfg(test)]
mod ticker {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, extract_timestamp};

    #[test]
    fn spot() {
        let raw_msg = r#"{"ch":"market.btcusdt.detail","ts":1654164208520,"tick":{"id":311070770838,"low":29326.15,"high":31899.51,"open":31666.3,"close":29952.31,"vol":8.181588598785326E8,"amount":26897.159881877742,"version":311070770838,"count":699452}}"#;

        assert_eq!(
            1654164208520,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "btcusdt",
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );
    }

    #[test]
    fn inverse_future() {
        let raw_msg = r#"{"ch":"market.BTC_CQ.detail","ts":1654164240700,"tick":{"id":1654164240,"mrid":222601084481815,"open":30723.59,"close":29936.45,"high":30723.59,"low":29301.65,"amount":12397.7225650589211059171392145260319091988,"vol":3753910,"count":46935,"ask":[29940.08,415],"bid":[29940.07,916]}}"#;

        assert_eq!(
            1654164240700,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "BTC_CQ",
            extract_symbol(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap()
        );
    }

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"ch":"market.BTC-USD.detail","ts":1654164360125,"tick":{"id":1654164360,"mrid":136520778365,"open":30694.1,"close":29925,"high":30694.9,"low":29258.1,"amount":13439.9875087391570765718450164053828468124,"vol":4074828,"count":39735,"ask":[29925.1,555],"bid":[29925,959]}}"#;

        assert_eq!(
            1654164360125,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "BTC-USD",
            extract_symbol(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap()
        );
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"ch":"market.BTC-USDT.detail","ts":1654164279373,"tick":{"id":1654164240,"mrid":108854811173,"open":30728.4,"close":29944.4,"high":30729.5,"low":29306.6,"amount":34473.406,"vol":34473406,"trade_turnover":1047364677.232,"count":203526,"ask":[29940.5,2138],"bid":[29940.4,3743]}}"#;

        assert_eq!(
            1654164279373,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "BTC-USDT",
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
    fn spot() {
        let raw_msg = r#"{"ch":"market.btcusdt.depth.step0","status":"ok","ts":1654253117316,"tick":{"bids":[[29805.62,0.309487],[29804.73,2.0E-4],[29804.43,0.030335],[29804.42,0.325215],[29802.06,0.03]],"asks":[[29805.63,1.650216],[29805.66,0.02902],[29807.73,2.0E-4],[29808.1,0.145506],[29808.74,0.420813]],"version":155671698426,"ts":1654253116701}}"#;

        assert_eq!(
            1654253117316,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "btcusdt",
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );
    }

    #[test]
    fn inverse_future() {
        let raw_msg = r#"{"ch":"market.BTC_CQ.depth.step0","status":"ok","tick":{"asks":[[29799.58,2],[29802.55,216],[29802.79,9],[29803.64,2],[29804.19,20]],"bids":[[29795.25,24],[29795.24,1],[29793.3,2],[29792.15,19],[29792.14,400]],"ch":"market.BTC_CQ.depth.step0","id":1654253101,"mrid":222601100912603,"ts":1654253101500,"version":1654253101},"ts":1654253101541}"#;

        assert_eq!(
            1654253101541,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "BTC_CQ",
            extract_symbol(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap()
        );
    }

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"ch":"market.BTC-USD.depth.step0","status":"ok","tick":{"asks":[[29768.2,915],[29769.1,400],[29769.5,50],[29770.6,400],[29771,1036]],"bids":[[29768.1,516],[29768,1],[29767.4,150],[29767.1,25],[29765.5,218]],"ch":"market.BTC-USD.depth.step0","id":1654253101,"mrid":136558127685,"ts":1654253101089,"version":1654253101},"ts":1654253101098}"#;

        assert_eq!(
            1654253101098,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "BTC-USD",
            extract_symbol(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap()
        );
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"ch":"market.BTC-USDT.depth.step0","status":"ok","tick":{"asks":[[29796.8,4025],[29797.1,49],[29797.2,16],[29797.3,700],[29797.5,4]],"bids":[[29796.7,305],[29795.9,265],[29795.6,1],[29794.4,165],[29793.7,165]],"ch":"market.BTC-USDT.depth.step0","id":1654253113,"mrid":108917760661,"ts":1654253113226,"version":1654253113},"ts":1654253113251}"#;

        assert_eq!(
            1654253113251,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "BTC-USDT",
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
        let raw_msg = r#"{"status":"ok","data":[{"volume":9724.000000000000000000,"amount":10561.414560502221111967,"symbol":"DOT","contract_type":"next_week","contract_code":"DOT220617","trade_amount":202723.214486601744902941274231535293939693,"trade_volume":187732,"trade_turnover":1877320.000000000000000000},{"volume":209593.000000000000000000,"amount":707.442741972231269946,"symbol":"BTC","contract_type":"quarter","contract_code":"BTC220624","trade_amount":6024.809855090449174997774141172323490409,"trade_volume":1784034,"trade_turnover":178403400.000000000000000000}],"ts":1654344900121}"#;

        assert_eq!(
            1654344900121,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "ALL",
            extract_symbol(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap()
        );
    }

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"status":"ok","data":[{"volume":119125.000000000000000000,"amount":945962.042404510442309219,"symbol":"EOS","contract_code":"EOS-USD","trade_amount":10322249.2071798754485976941552166206559822668,"trade_volume":1295420,"trade_turnover":12954200.000000000000000000},{"volume":6629.000000000000000000,"amount":68665.837994613631655272,"symbol":"MANA","contract_code":"MANA-USD","trade_amount":2774709.7417757547949533097365134609798406114,"trade_volume":270184,"trade_turnover":2701840.000000000000000000}],"ts":1654346524275}"#;

        assert_eq!(
            1654346524275,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "ALL",
            extract_symbol(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap()
        );
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"status":"ok","data":[{"volume":288491.000000000000000000,"amount":2884910.000000000000000000,"symbol":"MANA","value":2788265.515000000000000000,"contract_code":"MANA-USDT","trade_amount":8073320,"trade_volume":807332,"trade_turnover":7868174.158,"business_type":"swap","pair":"MANA-USDT","contract_type":"swap","trade_partition":"USDT"},{"volume":270380.000000000000000000,"amount":2703800.000000000000000000,"symbol":"NKN","value":243747.570000000000000000,"contract_code":"NKN-USDT","trade_amount":18409180,"trade_volume":1840918,"trade_turnover":1678370.2842,"business_type":"swap","pair":"NKN-USDT","contract_type":"swap","trade_partition":"USDT"}],"ts":1654346577824}"#;

        assert_eq!(
            1654346577824,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "ALL",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );
    }
}
