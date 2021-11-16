mod utils;

#[cfg(test)]
mod trade {
    use crypto_msg_parser::{extract_symbol, parse_trade, MarketType, TradeSide};
    use float_cmp::approx_eq;

    #[test]
    fn inverse_future() {
        let raw_msg = r#"{"jsonrpc":"2.0","method":"subscription","params":{"channel":"trades.BTC-26MAR21.raw","data":[{"trade_seq":5326971,"trade_id":"137486952","timestamp":1616321287195,"tick_direction":0,"price":56273.5,"mark_price":56243.86,"instrument_name":"BTC-26MAR21","index_price":56127.59,"direction":"buy","amount":6000.0}]}}"#;
        let trades = &parse_trade("deribit", MarketType::InverseFuture, raw_msg).unwrap();

        assert_eq!(trades.len(), 1);

        for trade in trades.iter() {
            crate::utils::check_trade_fields(
                "deribit",
                MarketType::InverseFuture,
                "BTC/USD".to_string(),
                extract_symbol("deribit", MarketType::InverseFuture, raw_msg).unwrap(),
                trade,
            );
        }

        let contract_value = crypto_contract_value::get_contract_value(
            "deribit",
            MarketType::InverseFuture,
            "BTC/USD",
        )
        .unwrap();

        assert_eq!(trades[0].quantity_base, contract_value * 6000.0 / 56273.5);
        assert_eq!(trades[0].quantity_quote, contract_value * 6000.0);
        assert_eq!(trades[0].quantity_contract, Some(6000.0));
        assert_eq!(trades[0].side, TradeSide::Buy);
    }

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"jsonrpc":"2.0","method":"subscription","params":{"channel":"trades.BTC-PERPETUAL.raw","data":[{"trade_seq":92836831,"trade_id":"137487241","timestamp":1616321478553,"tick_direction":1,"price":56168.0,"mark_price":56172.08,"instrument_name":"BTC-PERPETUAL","index_price":56173.74,"direction":"buy","amount":5580.0},{"trade_seq":92836832,"trade_id":"137487242","timestamp":1616321478553,"tick_direction":1,"price":56168.0,"mark_price":56172.08,"instrument_name":"BTC-PERPETUAL","index_price":56173.74,"direction":"buy","amount":60.0}]}}"#;
        let trades = &parse_trade("deribit", MarketType::InverseSwap, raw_msg).unwrap();

        assert_eq!(trades.len(), 2);

        for trade in trades.iter() {
            crate::utils::check_trade_fields(
                "deribit",
                MarketType::InverseSwap,
                "BTC/USD".to_string(),
                extract_symbol("deribit", MarketType::InverseSwap, raw_msg).unwrap(),
                trade,
            );

            assert_eq!(trade.side, TradeSide::Buy);
        }

        let contract_value = crypto_contract_value::get_contract_value(
            "deribit",
            MarketType::InverseSwap,
            "BTC/USD",
        )
        .unwrap();

        assert!(approx_eq!(
            f64,
            trades[0].quantity_base,
            contract_value * 5580.0 / 56168.5,
            epsilon = 0.0001
        ));
        assert_eq!(trades[0].quantity_quote, contract_value * 5580.0);
        assert_eq!(trades[0].quantity_contract, Some(5580.0));
        assert_eq!(trades[0].side, TradeSide::Buy);

        // volume == amount
        assert_eq!(trades[0].quantity_quote, contract_value * 5580.0);
        assert_eq!(trades[1].quantity_quote, contract_value * 60.0);
    }

    #[test]
    fn option() {
        let raw_msg = r#"{"jsonrpc":"2.0","method":"subscription","params":{"channel":"trades.option.any.raw","data":[{"trade_seq":1706,"trade_id":"137488100","timestamp":1616321732986,"tick_direction":0,"price":0.007,"mark_price":0.00670817,"iv":78.44,"instrument_name":"BTC-26MAR21-62000-C","index_price":56151.63,"direction":"buy","amount":0.1}]}}"#;
        let trades = &parse_trade("deribit", MarketType::EuropeanOption, raw_msg).unwrap();

        assert_eq!(trades.len(), 1);

        for trade in trades.iter() {
            crate::utils::check_trade_fields(
                "deribit",
                MarketType::EuropeanOption,
                "BTC/BTC".to_string(),
                extract_symbol("deribit", MarketType::EuropeanOption, raw_msg).unwrap(),
                trade,
            );

            assert_eq!(trade.side, TradeSide::Buy);
        }

        assert_eq!(trades[0].quantity_base, 0.1);
        assert_eq!(trades[0].quantity_quote, 0.007 * 0.1);
        assert_eq!(trades[0].quantity_contract, Some(0.1));
    }
}

#[cfg(test)]
mod l2_orderbook {
    use crypto_msg_parser::{extract_symbol, parse_l2, MarketType};

    #[test]
    fn inverse_future_snapshot() {
        let raw_msg = r#"{"jsonrpc":"2.0","method":"subscription","params":{"channel":"book.BTC-25JUN21.100ms","data":{"type":"snapshot","timestamp":1622626472678,"instrument_name":"BTC-25JUN21","change_id":31479219781,"bids":[["new",37317.0,2960.0],["new",37311.5,530.0],["new",37311.0,45170.0]],"asks":[["new",37327.0,10.0],["new",37327.5,20000.0],["new",37328.0,3000.0]]}}}"#;
        let orderbook = &parse_l2("deribit", MarketType::InverseFuture, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 3);
        assert_eq!(orderbook.bids.len(), 3);
        assert!(orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "deribit",
            MarketType::InverseFuture,
            "BTC/USD".to_string(),
            extract_symbol("deribit", MarketType::InverseFuture, raw_msg).unwrap(),
            orderbook,
        );

        let contract_value = crypto_contract_value::get_contract_value(
            "deribit",
            MarketType::InverseFuture,
            "BTC/USD",
        )
        .unwrap();

        assert_eq!(orderbook.bids[0].price, 37317.0);
        assert_eq!(
            orderbook.bids[0].quantity_base,
            contract_value * 2960.0 / 37317.0
        );
        assert_eq!(orderbook.bids[0].quantity_quote, contract_value * 2960.0);
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 2960.0);

        assert_eq!(orderbook.bids[2].price, 37311.0);
        assert_eq!(
            orderbook.bids[2].quantity_base,
            contract_value * 45170.0 / 37311.0
        );
        assert_eq!(orderbook.bids[2].quantity_quote, contract_value * 45170.0);
        assert_eq!(orderbook.bids[2].quantity_contract.unwrap(), 45170.0);

        assert_eq!(orderbook.asks[0].price, 37327.0);
        assert_eq!(
            orderbook.asks[0].quantity_base,
            contract_value * 10.0 / 37327.0
        );
        assert_eq!(orderbook.asks[0].quantity_quote, contract_value * 10.0);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 10.0);

        assert_eq!(orderbook.asks[2].price, 37328.0);
        assert_eq!(
            orderbook.asks[2].quantity_base,
            contract_value * 3000.0 / 37328.0
        );
        assert_eq!(orderbook.asks[2].quantity_quote, contract_value * 3000.0);
        assert_eq!(orderbook.asks[2].quantity_contract.unwrap(), 3000.0);
    }

    #[test]
    fn inverse_future_update() {
        let raw_msg = r#"{"jsonrpc":"2.0","method":"subscription","params":{"channel":"book.BTC-25JUN21.100ms","data":{"type":"change","timestamp":1622626784890,"prev_change_id":31479339296,"instrument_name":"BTC-25JUN21","change_id":31479339507,"bids":[["new",37392.5,3000.0],["change",37399.0,6530.0]],"asks":[["new",37850.0,8850.0],["delete",37848.5,0.0]]}}}"#;
        let orderbook = &parse_l2("deribit", MarketType::InverseSwap, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 2);
        assert_eq!(orderbook.bids.len(), 2);
        assert!(!orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "deribit",
            MarketType::InverseSwap,
            "BTC/USD".to_string(),
            extract_symbol("deribit", MarketType::InverseSwap, raw_msg).unwrap(),
            orderbook,
        );

        let contract_value = crypto_contract_value::get_contract_value(
            "deribit",
            MarketType::InverseSwap,
            "BTC/USD",
        )
        .unwrap();

        assert_eq!(orderbook.seq_id, Some(31479339507));
        assert_eq!(orderbook.prev_seq_id, Some(31479339296));
        assert_eq!(orderbook.bids[0].price, 37392.5);
        assert_eq!(
            orderbook.bids[0].quantity_base,
            contract_value * 3000.0 / 37392.5
        );
        assert_eq!(orderbook.bids[0].quantity_quote, contract_value * 3000.0);
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 3000.0);
        assert_eq!(orderbook.bids[1].price, 37399.0);
        assert_eq!(
            orderbook.bids[1].quantity_base,
            contract_value * 6530.0 / 37399.0
        );
        assert_eq!(orderbook.bids[1].quantity_quote, contract_value * 6530.0);
        assert_eq!(orderbook.bids[1].quantity_contract.unwrap(), 6530.0);

        assert_eq!(orderbook.asks[0].price, 37850.0);
        assert_eq!(
            orderbook.asks[0].quantity_base,
            contract_value * 8850.0 / 37850.0
        );
        assert_eq!(orderbook.asks[0].quantity_quote, contract_value * 8850.0);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 8850.0);

        assert_eq!(orderbook.asks[1].price, 37848.5);
        assert_eq!(orderbook.asks[1].quantity_base, 0.0 / 37848.5);
        assert_eq!(orderbook.asks[1].quantity_quote, 0.0);
        assert_eq!(orderbook.asks[1].quantity_contract.unwrap(), 0.0);
    }

    #[test]
    fn inverse_swap_snapshot() {
        let raw_msg = r#"{"jsonrpc":"2.0","method":"subscription","params":{"channel":"book.BTC-PERPETUAL.100ms","data":{"type":"snapshot","timestamp":1622627433440,"instrument_name":"BTC-PERPETUAL","change_id":31479596557,"bids":[["new",37240.0,20.0],["new",37237.0,14270.0],["new",37233.0,50.0]],"asks":[["new",37240.5,14240.0],["new",37248.5,15690.0],["new",37251.0,650.0]]}}}"#;
        let orderbook = &parse_l2("deribit", MarketType::InverseSwap, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 3);
        assert_eq!(orderbook.bids.len(), 3);
        assert!(orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "deribit",
            MarketType::InverseSwap,
            "BTC/USD".to_string(),
            extract_symbol("deribit", MarketType::InverseSwap, raw_msg).unwrap(),
            orderbook,
        );

        let contract_value = crypto_contract_value::get_contract_value(
            "deribit",
            MarketType::InverseSwap,
            "BTC/USD",
        )
        .unwrap();

        assert_eq!(orderbook.bids[0].price, 37240.0);
        assert_eq!(
            orderbook.bids[0].quantity_base,
            contract_value * 20.0 / 37240.0
        );
        assert_eq!(orderbook.bids[0].quantity_quote, contract_value * 20.0);
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 20.0);

        assert_eq!(orderbook.bids[2].price, 37233.0);
        assert_eq!(
            orderbook.bids[2].quantity_base,
            contract_value * 50.0 / 37233.0
        );
        assert_eq!(orderbook.bids[2].quantity_quote, contract_value * 50.0);
        assert_eq!(orderbook.bids[2].quantity_contract.unwrap(), 50.0);

        assert_eq!(orderbook.asks[0].price, 37240.5);
        assert_eq!(
            orderbook.asks[0].quantity_base,
            contract_value * 14240.0 / 37240.5
        );
        assert_eq!(orderbook.asks[0].quantity_quote, contract_value * 14240.0);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 14240.0);

        assert_eq!(orderbook.asks[2].price, 37251.0);
        assert_eq!(
            orderbook.asks[2].quantity_base,
            contract_value * 650.0 / 37251.0
        );
        assert_eq!(orderbook.asks[2].quantity_quote, contract_value * 650.0);
        assert_eq!(orderbook.asks[2].quantity_contract.unwrap(), 650.0);
    }

    #[test]
    fn inverse_swap_update() {
        let raw_msg = r#"{"jsonrpc":"2.0","method":"subscription","params":{"channel":"book.BTC-PERPETUAL.100ms","data":{"type":"change","timestamp":1622627435737,"prev_change_id":31479598064,"instrument_name":"BTC-PERPETUAL","change_id":31479598217,"bids":[["delete",36779.0,0.0],["new",36809.5,254870.0]],"asks":[["delete",37462.5,0.0],["change",37394.0,42670.0]]}}}"#;
        let orderbook = &parse_l2("deribit", MarketType::InverseSwap, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 2);
        assert_eq!(orderbook.bids.len(), 2);
        assert!(!orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "deribit",
            MarketType::InverseSwap,
            "BTC/USD".to_string(),
            extract_symbol("deribit", MarketType::InverseSwap, raw_msg).unwrap(),
            orderbook,
        );

        let contract_value = crypto_contract_value::get_contract_value(
            "deribit",
            MarketType::InverseSwap,
            "BTC/USD",
        )
        .unwrap();

        assert_eq!(orderbook.bids[0].price, 36779.0);
        assert_eq!(orderbook.bids[0].quantity_base, 0.0 / 36779.0);
        assert_eq!(orderbook.bids[0].quantity_quote, 0.0);
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 0.0);

        assert_eq!(orderbook.bids[1].price, 36809.5);
        assert_eq!(
            orderbook.bids[1].quantity_base,
            contract_value * 254870.0 / 36809.5
        );
        assert_eq!(orderbook.bids[1].quantity_quote, contract_value * 254870.0);
        assert_eq!(orderbook.bids[1].quantity_contract.unwrap(), 254870.0);

        assert_eq!(orderbook.asks[0].price, 37462.5);
        assert_eq!(orderbook.asks[0].quantity_base, 0.0 / 37462.5);
        assert_eq!(orderbook.asks[0].quantity_quote, 0.0);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 0.0);

        assert_eq!(orderbook.asks[1].price, 37394.0);
        assert_eq!(
            orderbook.asks[1].quantity_base,
            contract_value * 42670.0 / 37394.0
        );
        assert_eq!(orderbook.asks[1].quantity_quote, contract_value * 42670.0);
        assert_eq!(orderbook.asks[1].quantity_contract.unwrap(), 42670.0);
    }

    #[test]
    fn option_snapshot() {
        let raw_msg = r#"{"jsonrpc":"2.0","method":"subscription","params":{"channel":"book.BTC-11JUN21-25000-P.100ms","data":{"type":"snapshot","timestamp":1622627851747,"instrument_name":"BTC-11JUN21-25000-P","change_id":31479771122,"bids":[["new",0.005,13.7],["new",0.0045,5.7],["new",0.004,61.6]],"asks":[["new",0.006,64.5],["new",0.0065,48.0],["new",0.0085,0.5]]}}}"#;
        let orderbook = &parse_l2("deribit", MarketType::EuropeanOption, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 3);
        assert_eq!(orderbook.bids.len(), 3);
        assert!(orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "deribit",
            MarketType::EuropeanOption,
            "BTC/BTC".to_string(),
            extract_symbol("deribit", MarketType::EuropeanOption, raw_msg).unwrap(),
            orderbook,
        );

        assert_eq!(orderbook.bids[0].price, 0.005);
        assert_eq!(orderbook.bids[0].quantity_base, 13.7);
        assert_eq!(orderbook.bids[0].quantity_quote, 0.005 * 13.7);
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 13.7);

        assert_eq!(orderbook.bids[2].price, 0.004);
        assert_eq!(orderbook.bids[2].quantity_base, 61.6);
        assert_eq!(orderbook.bids[2].quantity_quote, 0.004 * 61.6);
        assert_eq!(orderbook.bids[2].quantity_contract.unwrap(), 61.6);

        assert_eq!(orderbook.asks[0].price, 0.006);
        assert_eq!(orderbook.asks[0].quantity_base, 64.5);
        assert_eq!(orderbook.asks[0].quantity_quote, 0.006 * 64.5);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 64.5);

        assert_eq!(orderbook.asks[2].price, 0.0085);
        assert_eq!(orderbook.asks[2].quantity_base, 0.5);
        assert_eq!(orderbook.asks[2].quantity_quote, 0.0085 * 0.5);
        assert_eq!(orderbook.asks[2].quantity_contract.unwrap(), 0.5);
    }
}
