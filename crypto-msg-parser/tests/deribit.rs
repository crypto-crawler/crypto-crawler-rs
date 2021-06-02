mod utils;

#[cfg(test)]
mod trade {
    use crypto_msg_parser::{parse_trade, MarketType, TradeSide};

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
                trade,
            );
        }

        assert_eq!(trades[0].quantity_base, 6000.0 / 56273.5);
        assert_eq!(trades[0].quantity_quote, 6000.0);
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
                trade,
            );

            assert_eq!(trade.side, TradeSide::Buy);
        }
        // volume == amount
        assert_eq!(trades[0].quantity_quote, 5580.0);
        assert_eq!(trades[1].quantity_quote, 60.0);
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
                "BTC/USD".to_string(),
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
    use crypto_msg_parser::{parse_l2, MarketType};

    #[test]
    fn inverse_future_snapshot() {
        let raw_msg = r#"{"jsonrpc":"2.0","method":"subscription","params":{"channel":"book.BTC-25JUN21.100ms","data":{"type":"snapshot","timestamp":1622626472678,"instrument_name":"BTC-25JUN21","change_id":31479219781,"bids":[["new",37317.0,2960.0],["new",37311.5,530.0],["new",37311.0,45170.0]],"asks":[["new",37327.0,10.0],["new",37327.5,20000.0],["new",37328.0,3000.0]]}}}"#;
        let orderbook = &parse_l2("deribit", MarketType::InverseFuture, raw_msg).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 3);
        assert_eq!(orderbook.bids.len(), 3);
        assert!(orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "deribit",
            MarketType::InverseFuture,
            "BTC/USD".to_string(),
            orderbook,
        );

        assert_eq!(orderbook.bids[0][0], 37317.0);
        assert_eq!(orderbook.bids[0][1], 2960.0 / 37317.0);
        assert_eq!(orderbook.bids[0][2], 2960.0);
        assert_eq!(orderbook.bids[0][3], 2960.0);

        assert_eq!(orderbook.bids[2][0], 37311.0);
        assert_eq!(orderbook.bids[2][1], 45170.0 / 37311.0);
        assert_eq!(orderbook.bids[2][2], 45170.0);
        assert_eq!(orderbook.bids[2][3], 45170.0);

        assert_eq!(orderbook.asks[0][0], 37327.0);
        assert_eq!(orderbook.asks[0][1], 10.0 / 37327.0);
        assert_eq!(orderbook.asks[0][2], 10.0);
        assert_eq!(orderbook.asks[0][3], 10.0);

        assert_eq!(orderbook.asks[2][0], 37328.0);
        assert_eq!(orderbook.asks[2][1], 3000.0 / 37328.0);
        assert_eq!(orderbook.asks[2][2], 3000.0);
        assert_eq!(orderbook.asks[2][3], 3000.0);
    }

    #[test]
    fn inverse_future_update() {
        let raw_msg = r#"{"jsonrpc":"2.0","method":"subscription","params":{"channel":"book.BTC-25JUN21.100ms","data":{"type":"change","timestamp":1622626784890,"prev_change_id":31479339296,"instrument_name":"BTC-25JUN21","change_id":31479339507,"bids":[["new",37392.5,3000.0],["change",37399.0,6530.0]],"asks":[["new",37850.0,8850.0],["delete",37848.5,0.0]]}}}"#;
        let orderbook = &parse_l2("deribit", MarketType::InverseSwap, raw_msg).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 2);
        assert_eq!(orderbook.bids.len(), 2);
        assert!(!orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "deribit",
            MarketType::InverseSwap,
            "BTC/USD".to_string(),
            orderbook,
        );

        assert_eq!(orderbook.bids[0][0], 37392.5);
        assert_eq!(orderbook.bids[0][1], 3000.0 / 37392.5);
        assert_eq!(orderbook.bids[0][2], 3000.0);
        assert_eq!(orderbook.bids[0][3], 3000.0);

        assert_eq!(orderbook.bids[1][0], 37399.0);
        assert_eq!(orderbook.bids[1][1], 6530.0 / 37399.0);
        assert_eq!(orderbook.bids[1][2], 6530.0);
        assert_eq!(orderbook.bids[1][3], 6530.0);

        assert_eq!(orderbook.asks[0][0], 37850.0);
        assert_eq!(orderbook.asks[0][1], 8850.0 / 37850.0);
        assert_eq!(orderbook.asks[0][2], 8850.0);
        assert_eq!(orderbook.asks[0][3], 8850.0);

        assert_eq!(orderbook.asks[1][0], 37848.5);
        assert_eq!(orderbook.asks[1][1], 0.0 / 37848.5);
        assert_eq!(orderbook.asks[1][2], 0.0);
        assert_eq!(orderbook.asks[1][3], 0.0);
    }

    #[test]
    fn inverse_swap_snapshot() {
        let raw_msg = r#"{"jsonrpc":"2.0","method":"subscription","params":{"channel":"book.BTC-PERPETUAL.100ms","data":{"type":"snapshot","timestamp":1622627433440,"instrument_name":"BTC-PERPETUAL","change_id":31479596557,"bids":[["new",37240.0,20.0],["new",37237.0,14270.0],["new",37233.0,50.0]],"asks":[["new",37240.5,14240.0],["new",37248.5,15690.0],["new",37251.0,650.0]]}}}"#;
        let orderbook = &parse_l2("deribit", MarketType::InverseSwap, raw_msg).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 3);
        assert_eq!(orderbook.bids.len(), 3);
        assert!(orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "deribit",
            MarketType::InverseSwap,
            "BTC/USD".to_string(),
            orderbook,
        );

        assert_eq!(orderbook.bids[0][0], 37240.0);
        assert_eq!(orderbook.bids[0][1], 20.0 / 37240.0);
        assert_eq!(orderbook.bids[0][2], 20.0);
        assert_eq!(orderbook.bids[0][3], 20.0);

        assert_eq!(orderbook.bids[2][0], 37233.0);
        assert_eq!(orderbook.bids[2][1], 50.0 / 37233.0);
        assert_eq!(orderbook.bids[2][2], 50.0);
        assert_eq!(orderbook.bids[2][3], 50.0);

        assert_eq!(orderbook.asks[0][0], 37240.5);
        assert_eq!(orderbook.asks[0][1], 14240.0 / 37240.5);
        assert_eq!(orderbook.asks[0][2], 14240.0);
        assert_eq!(orderbook.asks[0][3], 14240.0);

        assert_eq!(orderbook.asks[2][0], 37251.0);
        assert_eq!(orderbook.asks[2][1], 650.0 / 37251.0);
        assert_eq!(orderbook.asks[2][2], 650.0);
        assert_eq!(orderbook.asks[2][3], 650.0);
    }

    #[test]
    fn inverse_swap_update() {
        let raw_msg = r#"{"jsonrpc":"2.0","method":"subscription","params":{"channel":"book.BTC-PERPETUAL.100ms","data":{"type":"change","timestamp":1622627435737,"prev_change_id":31479598064,"instrument_name":"BTC-PERPETUAL","change_id":31479598217,"bids":[["delete",36779.0,0.0],["new",36809.5,254870.0]],"asks":[["delete",37462.5,0.0],["change",37394.0,42670.0]]}}}"#;
        let orderbook = &parse_l2("deribit", MarketType::InverseSwap, raw_msg).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 2);
        assert_eq!(orderbook.bids.len(), 2);
        assert!(!orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "deribit",
            MarketType::InverseSwap,
            "BTC/USD".to_string(),
            orderbook,
        );

        assert_eq!(orderbook.bids[0][0], 36779.0);
        assert_eq!(orderbook.bids[0][1], 0.0 / 36779.0);
        assert_eq!(orderbook.bids[0][2], 0.0);
        assert_eq!(orderbook.bids[0][3], 0.0);

        assert_eq!(orderbook.bids[1][0], 36809.5);
        assert_eq!(orderbook.bids[1][1], 254870.0 / 36809.5);
        assert_eq!(orderbook.bids[1][2], 254870.0);
        assert_eq!(orderbook.bids[1][3], 254870.0);

        assert_eq!(orderbook.asks[0][0], 37462.5);
        assert_eq!(orderbook.asks[0][1], 0.0 / 37462.5);
        assert_eq!(orderbook.asks[0][2], 0.0);
        assert_eq!(orderbook.asks[0][3], 0.0);

        assert_eq!(orderbook.asks[1][0], 37394.0);
        assert_eq!(orderbook.asks[1][1], 42670.0 / 37394.0);
        assert_eq!(orderbook.asks[1][2], 42670.0);
        assert_eq!(orderbook.asks[1][3], 42670.0);
    }

    #[test]
    fn option_snapshot() {
        let raw_msg = r#"{"jsonrpc":"2.0","method":"subscription","params":{"channel":"book.BTC-11JUN21-25000-P.100ms","data":{"type":"snapshot","timestamp":1622627851747,"instrument_name":"BTC-11JUN21-25000-P","change_id":31479771122,"bids":[["new",0.005,13.7],["new",0.0045,5.7],["new",0.004,61.6]],"asks":[["new",0.006,64.5],["new",0.0065,48.0],["new",0.0085,0.5]]}}}"#;
        let orderbook = &parse_l2("deribit", MarketType::EuropeanOption, raw_msg).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 3);
        assert_eq!(orderbook.bids.len(), 3);
        assert!(orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            "deribit",
            MarketType::EuropeanOption,
            "BTC/USD".to_string(),
            orderbook,
        );

        assert_eq!(orderbook.bids[0][0], 0.005);
        assert_eq!(orderbook.bids[0][1], 13.7);
        assert_eq!(orderbook.bids[0][2], 0.005 * 13.7);
        assert_eq!(orderbook.bids[0][3], 13.7);

        assert_eq!(orderbook.bids[2][0], 0.004);
        assert_eq!(orderbook.bids[2][1], 61.6);
        assert_eq!(orderbook.bids[2][2], 0.004 * 61.6);
        assert_eq!(orderbook.bids[2][3], 61.6);

        assert_eq!(orderbook.asks[0][0], 0.006);
        assert_eq!(orderbook.asks[0][1], 64.5);
        assert_eq!(orderbook.asks[0][2], 0.006 * 64.5);
        assert_eq!(orderbook.asks[0][3], 64.5);

        assert_eq!(orderbook.asks[2][0], 0.0085);
        assert_eq!(orderbook.asks[2][1], 0.5);
        assert_eq!(orderbook.asks[2][2], 0.0085 * 0.5);
        assert_eq!(orderbook.asks[2][3], 0.5);
    }
}
