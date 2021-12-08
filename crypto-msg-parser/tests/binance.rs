mod utils;

#[cfg(test)]
mod trade {
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, parse_trade, TradeSide};

    #[test]
    fn spot() {
        let raw_msg = r#"{"stream":"btcusdt@aggTrade","data":{"e":"aggTrade","E":1616176861895,"s":"BTCUSDT","a":640283266,"p":"58942.01000000","q":"0.00035600","f":716849523,"l":716849523,"T":1616176861893,"m":false,"M":true}}"#;
        let trade = &parse_trade("binance", MarketType::Spot, raw_msg).unwrap()[0];

        crate::utils::check_trade_fields(
            "binance",
            MarketType::Spot,
            "BTC/USDT".to_string(),
            extract_symbol("binance", MarketType::Spot, raw_msg).unwrap(),
            trade,
            raw_msg,
        );

        assert_eq!(trade.quantity_base, 0.00035600);
        assert_eq!(trade.quantity_quote, 0.00035600 * 58942.01);
        assert_eq!(trade.quantity_contract, None);
        assert_eq!(trade.side, TradeSide::Buy);
    }

    #[test]
    fn inverse_future() {
        let raw_msg = r#"{"stream":"btcusd_210625@aggTrade","data":{"e":"aggTrade","E":1616201787561,"a":5091038,"s":"BTCUSD_210625","p":"62838.0","q":"5","f":7621250,"l":7621250,"T":1616201787407,"m":true}}"#;
        let trade = &parse_trade("binance", MarketType::InverseFuture, raw_msg).unwrap()[0];

        crate::utils::check_trade_fields(
            "binance",
            MarketType::InverseFuture,
            "BTC/USD".to_string(),
            extract_symbol("binance", MarketType::InverseFuture, raw_msg).unwrap(),
            trade,
            raw_msg,
        );

        assert_eq!(trade.quantity_base, 500.0 / 62838.0);
        assert_eq!(trade.quantity_quote, 500.0);
        assert_eq!(trade.quantity_contract, Some(5.0));
        assert_eq!(trade.side, TradeSide::Sell);
    }

    #[test]
    fn linear_future() {
        let raw_msg = r#"{"stream":"btcusdt_210625@aggTrade","data":{"e":"aggTrade","E":1616201036113,"a":21021,"s":"BTCUSDT_210625","p":"62595.8","q":"0.094","f":21824,"l":21824,"T":1616201035958,"m":false}}"#;
        let trade = &parse_trade("binance", MarketType::LinearFuture, raw_msg).unwrap()[0];

        crate::utils::check_trade_fields(
            "binance",
            MarketType::LinearFuture,
            "BTC/USDT".to_string(),
            extract_symbol("binance", MarketType::LinearFuture, raw_msg).unwrap(),
            trade,
            raw_msg,
        );

        assert_eq!(trade.quantity_base, 0.094);
        assert_eq!(trade.quantity_quote, 0.094 * 62595.8);
        assert_eq!(trade.quantity_contract, Some(0.094));

        assert_eq!(trade.side, TradeSide::Buy);
    }

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"stream":"btcusd_perp@aggTrade","data":{"e":"aggTrade","E":1616201883458,"a":41045788,"s":"BTCUSD_PERP","p":"58570.1","q":"58","f":91864326,"l":91864327,"T":1616201883304,"m":true}}"#;
        let trade = &parse_trade("binance", MarketType::InverseSwap, raw_msg).unwrap()[0];

        crate::utils::check_trade_fields(
            "binance",
            MarketType::InverseSwap,
            "BTC/USD".to_string(),
            extract_symbol("binance", MarketType::InverseSwap, raw_msg).unwrap(),
            trade,
            raw_msg,
        );

        assert_eq!(trade.price, 58570.1);
        assert_eq!(trade.quantity_base, 5800.0 / 58570.1);
        assert_eq!(trade.quantity_quote, 5800.0);
        assert_eq!(trade.quantity_contract, Some(58.0));

        assert_eq!(trade.side, TradeSide::Sell);
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"stream":"btcusdt@aggTrade","data":{"e":"aggTrade","E":1616202009196,"a":389551486,"s":"BTCUSDT","p":"58665.00","q":"0.043","f":621622993,"l":621622993,"T":1616202009188,"m":false}}"#;
        let trade = &parse_trade("binance", MarketType::LinearSwap, raw_msg).unwrap()[0];

        crate::utils::check_trade_fields(
            "binance",
            MarketType::LinearSwap,
            "BTC/USDT".to_string(),
            extract_symbol("binance", MarketType::LinearSwap, raw_msg).unwrap(),
            trade,
            raw_msg,
        );

        assert_eq!(trade.quantity_base, 0.043);
        assert_eq!(trade.quantity_quote, 0.043 * 58665.00);
        assert_eq!(trade.quantity_contract, Some(0.043));

        assert_eq!(trade.side, TradeSide::Buy);
    }

    #[test]
    #[ignore]
    fn option() {
        let raw_msg = r#"{"stream":"BTCUSDT_C@TRADE_ALL","data":{"e":"trade_all","E":1616205287778,"s":"BTCUSDT_C","t":[{"t":"315","p":"4842.24","q":"0.0001","b":"4612047757752932782","a":"4612057653433061439","T":1616204382000,"s":"1","S":"BTC-210430-68000-C"},{"t":"805","p":"5616.36","q":"0.0001","b":"4612047757752932781","a":"4612057653433055969","T":1616204357000,"s":"1","S":"BTC-210430-64000-C"},{"t":"313","p":"7028.44","q":"0.0001","b":"4612015871915728334","a":"4612057653433051715","T":1616204344000,"s":"1","S":"BTC-210430-60000-C"}]}}"#;
        let trades = &parse_trade("binance", MarketType::EuropeanOption, raw_msg).unwrap();

        assert_eq!(trades.len(), 3);

        for trade in trades.iter() {
            crate::utils::check_trade_fields(
                "binance",
                MarketType::EuropeanOption,
                "BTC/USDT".to_string(),
                extract_symbol("binance", MarketType::EuropeanOption, raw_msg).unwrap(),
                trade,
                raw_msg,
            );
        }

        assert_eq!(trades[0].quantity_base, 0.0001);
        assert_eq!(trades[0].quantity_quote, 0.0001 * 4842.24);
        assert_eq!(trades[0].quantity_contract, Some(0.0001));
    }
}

#[cfg(test)]
mod funding_rate {
    use crypto_market_type::MarketType;
    use crypto_msg_parser::parse_funding_rate;

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"stream":"btcusd_perp@markPrice","data":{"e":"markPriceUpdate","E":1617309477000,"s":"BTCUSD_PERP","p":"59012.56007222","P":"58896.00503145","r":"0.00073689","T":1617321600000}}"#;
        let funding_rates =
            &parse_funding_rate("binance", MarketType::InverseSwap, raw_msg).unwrap();

        assert_eq!(funding_rates.len(), 1);

        for rate in funding_rates.iter() {
            crate::utils::check_funding_rate_fields(
                "binance",
                MarketType::InverseSwap,
                rate,
                raw_msg,
            );
        }

        assert_eq!(funding_rates[0].pair, "BTC/USD".to_string());
        assert_eq!(funding_rates[0].funding_rate, 0.00073689);
        assert_eq!(funding_rates[0].funding_time, 1617321600000);

        let raw_msg = r#"{"stream":"!markPrice@arr","data":[{"e":"markPriceUpdate","E":1617309501002,"s":"BTCUSD_PERP","p":"59003.37984561","P":"58896.41602208","r":"0.00073684","T":1617321600000},{"e":"markPriceUpdate","E":1617309501002,"s":"ETHUSD_PERP","p":"1981.89000000","P":"1975.18948029","r":"0.00100944","T":1617321600000}]}"#;
        let funding_rates =
            &parse_funding_rate("binance", MarketType::InverseSwap, raw_msg).unwrap();

        assert_eq!(funding_rates.len(), 2);

        for rate in funding_rates.iter() {
            crate::utils::check_funding_rate_fields(
                "binance",
                MarketType::InverseSwap,
                rate,
                raw_msg,
            );
        }

        assert_eq!(funding_rates[0].pair, "BTC/USD".to_string());
        assert_eq!(funding_rates[0].funding_rate, 0.00073684);
        assert_eq!(funding_rates[0].funding_time, 1617321600000);

        assert_eq!(funding_rates[1].pair, "ETH/USD".to_string());
        assert_eq!(funding_rates[1].funding_rate, 0.00100944);
        assert_eq!(funding_rates[1].funding_time, 1617321600000);
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"stream":"btcusdt@markPrice","data":{"e":"markPriceUpdate","E":1617308820003,"s":"BTCUSDT","p":"58940.14924532","P":"58905.14663658","i":"58857.26693664","r":"0.00058455","T":1617321600000}}"#;
        let funding_rates =
            &parse_funding_rate("binance", MarketType::LinearSwap, raw_msg).unwrap();

        assert_eq!(funding_rates.len(), 1);

        for rate in funding_rates.iter() {
            crate::utils::check_funding_rate_fields(
                "binance",
                MarketType::LinearSwap,
                rate,
                raw_msg,
            );
        }

        assert_eq!(funding_rates[0].pair, "BTC/USDT".to_string());
        assert_eq!(funding_rates[0].funding_rate, 0.00058455);
        assert_eq!(funding_rates[0].funding_time, 1617321600000);

        let raw_msg = r#"{"stream":"!markPrice@arr","data":[{"e":"markPriceUpdate","E":1617309024002,"s":"BTCUSDT","p":"59022.53514719","P":"58902.34482833","i":"58936.68384000","r":"0.00058959","T":1617321600000},{"e":"markPriceUpdate","E":1617309024002,"s":"ETHUSDT","p":"1981.15704420","P":"1974.79557094","i":"1978.08197502","r":"0.00059142","T":1617321600000}]}"#;
        let funding_rates =
            &parse_funding_rate("binance", MarketType::LinearSwap, raw_msg).unwrap();

        assert_eq!(funding_rates.len(), 2);

        for rate in funding_rates.iter() {
            crate::utils::check_funding_rate_fields(
                "binance",
                MarketType::LinearSwap,
                rate,
                raw_msg,
            );
        }

        assert_eq!(funding_rates[0].pair, "BTC/USDT".to_string());
        assert_eq!(funding_rates[0].funding_rate, 0.00058959);
        assert_eq!(funding_rates[0].funding_time, 1617321600000);

        assert_eq!(funding_rates[1].pair, "ETH/USDT".to_string());
        assert_eq!(funding_rates[1].funding_rate, 0.00059142);
        assert_eq!(funding_rates[1].funding_time, 1617321600000);
    }
}

#[cfg(test)]
mod l2_orderbook {
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, parse_l2};

    #[test]
    fn spot() {
        let raw_msg = r#"{"stream":"btcusdt@depth@100ms","data":{"e":"depthUpdate","E":1622363903670,"s":"BTCUSDT","U":11294093710,"u":11294093726,"b":[["35743.98000000","0.00000000"],["35743.87000000","0.00001500"]],"a":[["35743.88000000","0.24000000"],["35743.97000000","0.00000000"]]}}"#;
        let orderbook = &parse_l2("binance", MarketType::Spot, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 2);
        assert_eq!(orderbook.bids.len(), 2);
        assert!(!orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "binance",
            MarketType::Spot,
            "BTC/USDT".to_string(),
            extract_symbol("binance", MarketType::Spot, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );

        assert_eq!(orderbook.timestamp, 1622363903670);
        assert_eq!(orderbook.seq_id, Some(11294093726));

        assert_eq!(orderbook.bids[0].price, 35743.98);
        assert_eq!(orderbook.bids[0].quantity_base, 0.0);
        assert_eq!(orderbook.bids[0].quantity_quote, 0.0);

        assert_eq!(orderbook.bids[1].price, 35743.87);
        assert_eq!(orderbook.bids[1].quantity_base, 0.000015);
        assert_eq!(orderbook.bids[1].quantity_quote, 35743.87 * 0.000015);

        assert_eq!(orderbook.asks[0].price, 35743.88);
        assert_eq!(orderbook.asks[0].quantity_base, 0.24);
        assert_eq!(orderbook.asks[0].quantity_quote, 35743.88 * 0.24);

        assert_eq!(orderbook.asks[1].price, 35743.97);
        assert_eq!(orderbook.asks[1].quantity_base, 0.0);
        assert_eq!(orderbook.asks[1].quantity_quote, 0.0);
    }

    #[test]
    fn inverse_future() {
        let raw_msg = r#"{"stream":"btcusd_210625@depth@100ms","data":{"e":"depthUpdate","E":1622368000245,"T":1622368000234,"s":"BTCUSD_210625","ps":"BTCUSD","U":127531213607,"u":127531214406,"pu":127531213513,"b":[["35943.8","60"],["35965.2","896"]],"a":[["36038.3","9"],["36038.4","21"]]}}"#;
        let orderbook = &parse_l2("binance", MarketType::InverseFuture, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 2);
        assert_eq!(orderbook.bids.len(), 2);
        assert!(!orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "binance",
            MarketType::InverseFuture,
            "BTC/USD".to_string(),
            extract_symbol("binance", MarketType::InverseFuture, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );

        assert_eq!(orderbook.timestamp, 1622368000234);
        assert_eq!(orderbook.seq_id, Some(127531214406));
        assert_eq!(orderbook.prev_seq_id, Some(127531213513));

        assert_eq!(orderbook.bids[0].price, 35943.8);
        assert_eq!(orderbook.bids[0].quantity_base, 6000.0 / 35943.8);
        assert_eq!(orderbook.bids[0].quantity_quote, 6000.0);
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 60.0);

        assert_eq!(orderbook.bids[1].price, 35965.2);
        assert_eq!(orderbook.bids[1].quantity_base, 89600.0 / 35965.2);
        assert_eq!(orderbook.bids[1].quantity_quote, 89600.0);
        assert_eq!(orderbook.bids[1].quantity_contract.unwrap(), 896.0);

        assert_eq!(orderbook.asks[0].price, 36038.3);
        assert_eq!(orderbook.asks[0].quantity_base, 900.0 / 36038.3);
        assert_eq!(orderbook.asks[0].quantity_quote, 900.0);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 9.0);

        assert_eq!(orderbook.asks[1].price, 36038.4);
        assert_eq!(orderbook.asks[1].quantity_base, 2100.0 / 36038.4);
        assert_eq!(orderbook.asks[1].quantity_quote, 2100.0);
        assert_eq!(orderbook.asks[1].quantity_contract.unwrap(), 21.0);
    }

    #[test]
    fn linear_future() {
        let raw_msg = r#"{"stream":"ethusdt_210625@depth@100ms","data":{"e":"depthUpdate","E":1622368962075,"T":1622368962065,"s":"ETHUSDT_210625","U":475700780918,"u":475700783070,"pu":475700774972,"b":[["2437.04","82.320"],["2437.07","0.000"]],"a":[["2441.23","1.500"],["2441.24","0.220"]]}}"#;
        let orderbook = &parse_l2("binance", MarketType::LinearFuture, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 2);
        assert_eq!(orderbook.bids.len(), 2);
        assert!(!orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "binance",
            MarketType::LinearFuture,
            "ETH/USDT".to_string(),
            extract_symbol("binance", MarketType::LinearFuture, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );

        assert_eq!(orderbook.timestamp, 1622368962065);
        assert_eq!(orderbook.seq_id, Some(475700783070));
        assert_eq!(orderbook.prev_seq_id, Some(475700774972));

        assert_eq!(orderbook.bids[0].price, 2437.04);
        assert_eq!(orderbook.bids[0].quantity_base, 82.32);
        assert_eq!(orderbook.bids[0].quantity_quote, 2437.04 * 82.32);
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 82.32);

        assert_eq!(orderbook.bids[1].price, 2437.07);
        assert_eq!(orderbook.bids[1].quantity_base, 0.0);
        assert_eq!(orderbook.bids[1].quantity_quote, 0.0);
        assert_eq!(orderbook.bids[1].quantity_contract.unwrap(), 0.0);

        assert_eq!(orderbook.asks[0].price, 2441.23);
        assert_eq!(orderbook.asks[0].quantity_base, 1.5);
        assert_eq!(orderbook.asks[0].quantity_quote, 2441.23 * 1.5);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 1.5);

        assert_eq!(orderbook.asks[1].price, 2441.24);
        assert_eq!(orderbook.asks[1].quantity_base, 0.220);
        assert_eq!(orderbook.asks[1].quantity_quote, 2441.24 * 0.220);
        assert_eq!(orderbook.asks[1].quantity_contract.unwrap(), 0.220);
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"stream":"btcusdt@depth@100ms","data":{"e":"depthUpdate","E":1622371244693,"T":1622371244687,"s":"BTCUSDT","U":475776377463,"u":475776380184,"pu":475776377452,"b":[["35729.77","1.600"],["35750.00","5.106"]],"a":[["35819.20","0.211"],["35820.31","0.001"]]}}"#;
        let orderbook = &parse_l2("binance", MarketType::LinearSwap, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 2);
        assert_eq!(orderbook.bids.len(), 2);
        assert!(!orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "binance",
            MarketType::LinearSwap,
            "BTC/USDT".to_string(),
            extract_symbol("binance", MarketType::LinearSwap, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );

        assert_eq!(orderbook.timestamp, 1622371244687);
        assert_eq!(orderbook.seq_id, Some(475776380184));
        assert_eq!(orderbook.prev_seq_id, Some(475776377452));

        assert_eq!(orderbook.bids[0].price, 35729.77);
        assert_eq!(orderbook.bids[0].quantity_base, 1.6);
        assert_eq!(orderbook.bids[0].quantity_quote, 35729.77 * 1.6);
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 1.6);

        assert_eq!(orderbook.bids[1].price, 35750.0);
        assert_eq!(orderbook.bids[1].quantity_base, 5.106);
        assert_eq!(orderbook.bids[1].quantity_quote, 35750.0 * 5.106);
        assert_eq!(orderbook.bids[1].quantity_contract.unwrap(), 5.106);

        assert_eq!(orderbook.asks[0].price, 35819.2);
        assert_eq!(orderbook.asks[0].quantity_base, 0.211);
        assert_eq!(orderbook.asks[0].quantity_quote, 35819.2 * 0.211);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 0.211);

        assert_eq!(orderbook.asks[1].price, 35820.31);
        assert_eq!(orderbook.asks[1].quantity_base, 0.001);
        assert_eq!(orderbook.asks[1].quantity_quote, 35820.31 * 0.001);
        assert_eq!(orderbook.asks[1].quantity_contract.unwrap(), 0.001);
    }

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"stream":"btcusd_perp@depth@100ms","data":{"e":"depthUpdate","E":1622370862564,"T":1622370862553,"s":"BTCUSD_PERP","ps":"BTCUSD","U":127559587191,"u":127559588177,"pu":127559587113,"b":[["35365.9","1400"],["35425.8","561"]],"a":[["35817.8","7885"],["35818.7","307"]]}}"#;
        let orderbook = &parse_l2("binance", MarketType::InverseSwap, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 2);
        assert_eq!(orderbook.bids.len(), 2);
        assert!(!orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "binance",
            MarketType::InverseSwap,
            "BTC/USD".to_string(),
            extract_symbol("binance", MarketType::InverseSwap, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );

        assert_eq!(orderbook.timestamp, 1622370862553);
        assert_eq!(orderbook.seq_id, Some(127559588177));
        assert_eq!(orderbook.prev_seq_id, Some(127559587113));

        assert_eq!(orderbook.bids[0].price, 35365.9);
        assert_eq!(orderbook.bids[0].quantity_base, 140000.0 / 35365.9);
        assert_eq!(orderbook.bids[0].quantity_quote, 140000.0);
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 1400.0);

        assert_eq!(orderbook.bids[1].price, 35425.8);
        assert_eq!(orderbook.bids[1].quantity_base, 56100.0 / 35425.8);
        assert_eq!(orderbook.bids[1].quantity_quote, 56100.0);
        assert_eq!(orderbook.bids[1].quantity_contract.unwrap(), 561.0);

        assert_eq!(orderbook.asks[0].price, 35817.8);
        assert_eq!(orderbook.asks[0].quantity_base, 788500.0 / 35817.8);
        assert_eq!(orderbook.asks[0].quantity_quote, 788500.0);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 7885.0);

        assert_eq!(orderbook.asks[1].price, 35818.7);
        assert_eq!(orderbook.asks[1].quantity_base, 30700.0 / 35818.7);
        assert_eq!(orderbook.asks[1].quantity_quote, 30700.0);
        assert_eq!(orderbook.asks[1].quantity_contract.unwrap(), 307.0);
    }

    #[test]
    fn option() {}
}
