mod utils;

const EXCHANGE_NAME: &str = "deribit";

#[cfg(test)]
mod trade {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, extract_timestamp, parse_trade, TradeSide};

    #[test]
    fn inverse_future() {
        let raw_msg = r#"{"jsonrpc":"2.0","method":"subscription","params":{"channel":"trades.BTC-26MAR21.raw","data":[{"trade_seq":5326971,"trade_id":"137486952","timestamp":1616321287195,"tick_direction":0,"price":56273.5,"mark_price":56243.86,"instrument_name":"BTC-26MAR21","index_price":56127.59,"direction":"buy","amount":6000.0}]}}"#;
        let trades = &parse_trade(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg).unwrap();

        assert_eq!(trades.len(), 1);

        for trade in trades.iter() {
            crate::utils::check_trade_fields(
                EXCHANGE_NAME,
                MarketType::InverseFuture,
                "BTC/USD".to_string(),
                extract_symbol(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg).unwrap(),
                trade,
                raw_msg,
            );
        }
        assert_eq!(
            1616321287195,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(trades[0].quantity_base, 10.0 * 6000.0 / 56273.5);
        assert_eq!(trades[0].quantity_quote, 10.0 * 6000.0);
        assert_eq!(trades[0].quantity_contract, Some(6000.0));
        assert_eq!(trades[0].side, TradeSide::Buy);
    }

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"jsonrpc":"2.0","method":"subscription","params":{"channel":"trades.BTC-PERPETUAL.raw","data":[{"trade_seq":92836831,"trade_id":"137487241","timestamp":1616321478553,"tick_direction":1,"price":56168.0,"mark_price":56172.08,"instrument_name":"BTC-PERPETUAL","index_price":56173.74,"direction":"buy","amount":5580.0},{"trade_seq":92836832,"trade_id":"137487242","timestamp":1616321478553,"tick_direction":1,"price":56168.0,"mark_price":56172.08,"instrument_name":"BTC-PERPETUAL","index_price":56173.74,"direction":"buy","amount":60.0}]}}"#;
        let trades = &parse_trade(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap();

        assert_eq!(trades.len(), 2);

        for trade in trades.iter() {
            crate::utils::check_trade_fields(
                EXCHANGE_NAME,
                MarketType::InverseSwap,
                "BTC/USD".to_string(),
                extract_symbol(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap(),
                trade,
                raw_msg,
            );

            assert_eq!(trade.side, TradeSide::Buy);
        }
        assert_eq!(
            1616321478553,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(trades[0].price, 56168.0);
        assert_eq!(trades[0].quantity_base, 10.0 * 5580.0 / 56168.0);
        assert_eq!(trades[0].quantity_quote, 10.0 * 5580.0);
        assert_eq!(trades[0].quantity_contract, Some(5580.0));
        assert_eq!(trades[0].side, TradeSide::Buy);

        // volume == amount
        assert_eq!(trades[0].quantity_quote, 10.0 * 5580.0);
        assert_eq!(trades[1].quantity_quote, 10.0 * 60.0);
    }

    #[test]
    fn option() {
        let raw_msg = r#"{"jsonrpc":"2.0","method":"subscription","params":{"channel":"trades.option.any.raw","data":[{"trade_seq":1706,"trade_id":"137488100","timestamp":1616321732986,"tick_direction":0,"price":0.007,"mark_price":0.00670817,"iv":78.44,"instrument_name":"BTC-26MAR21-62000-C","index_price":56151.63,"direction":"buy","amount":0.1}]}}"#;
        let trades = &parse_trade(EXCHANGE_NAME, MarketType::EuropeanOption, raw_msg).unwrap();

        assert_eq!(trades.len(), 1);

        for trade in trades.iter() {
            crate::utils::check_trade_fields(
                EXCHANGE_NAME,
                MarketType::EuropeanOption,
                "BTC/BTC".to_string(),
                extract_symbol(EXCHANGE_NAME, MarketType::EuropeanOption, raw_msg).unwrap(),
                trade,
                raw_msg,
            );

            assert_eq!(trade.side, TradeSide::Buy);
        }
        assert_eq!(
            1616321732986,
            extract_timestamp(EXCHANGE_NAME, MarketType::EuropeanOption, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(trades[0].quantity_base, 0.1);
        assert_eq!(trades[0].quantity_quote, 0.007 * 0.1);
        assert_eq!(trades[0].quantity_contract, Some(0.1));
    }
}

#[cfg(test)]
mod l2_event {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, extract_timestamp, parse_l2};
    use crypto_msg_type::MessageType;

    #[test]
    fn inverse_future_snapshot() {
        let raw_msg = r#"{"jsonrpc":"2.0","method":"subscription","params":{"channel":"book.BTC-25JUN21.100ms","data":{"type":"snapshot","timestamp":1622626472678,"instrument_name":"BTC-25JUN21","change_id":31479219781,"bids":[["new",37317.0,2960.0],["new",37311.5,530.0],["new",37311.0,45170.0]],"asks":[["new",37327.0,10.0],["new",37327.5,20000.0],["new",37328.0,3000.0]]}}}"#;
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
            1622626472678,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1622626472678);
        assert_eq!(orderbook.seq_id, Some(31479219781));
        assert_eq!(orderbook.prev_seq_id, None);

        assert_eq!(orderbook.bids[0].price, 37317.0);
        assert_eq!(orderbook.bids[0].quantity_base, 10.0 * 2960.0 / 37317.0);
        assert_eq!(orderbook.bids[0].quantity_quote, 10.0 * 2960.0);
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 2960.0);

        assert_eq!(orderbook.bids[2].price, 37311.0);
        assert_eq!(orderbook.bids[2].quantity_base, 10.0 * 45170.0 / 37311.0);
        assert_eq!(orderbook.bids[2].quantity_quote, 10.0 * 45170.0);
        assert_eq!(orderbook.bids[2].quantity_contract.unwrap(), 45170.0);

        assert_eq!(orderbook.asks[0].price, 37327.0);
        assert_eq!(orderbook.asks[0].quantity_base, 10.0 * 10.0 / 37327.0);
        assert_eq!(orderbook.asks[0].quantity_quote, 10.0 * 10.0);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 10.0);

        assert_eq!(orderbook.asks[2].price, 37328.0);
        assert_eq!(orderbook.asks[2].quantity_base, 10.0 * 3000.0 / 37328.0);
        assert_eq!(orderbook.asks[2].quantity_quote, 10.0 * 3000.0);
        assert_eq!(orderbook.asks[2].quantity_contract.unwrap(), 3000.0);
    }

    #[test]
    fn inverse_future_update() {
        let raw_msg = r#"{"jsonrpc":"2.0","method":"subscription","params":{"channel":"book.BTC-25JUN21.100ms","data":{"type":"change","timestamp":1622626784890,"prev_change_id":31479339296,"instrument_name":"BTC-25JUN21","change_id":31479339507,"bids":[["new",37392.5,3000.0],["change",37399.0,6530.0]],"asks":[["new",37850.0,8850.0],["delete",37848.5,0.0]]}}}"#;
        let orderbook =
            &parse_l2(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 2);
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
            1622626784890,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1622626784890);
        assert_eq!(orderbook.seq_id, Some(31479339507));
        assert_eq!(orderbook.prev_seq_id, Some(31479339296));
        assert_eq!(orderbook.bids[0].price, 37392.5);
        assert_eq!(orderbook.bids[0].quantity_base, 10.0 * 3000.0 / 37392.5);
        assert_eq!(orderbook.bids[0].quantity_quote, 10.0 * 3000.0);
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 3000.0);
        assert_eq!(orderbook.bids[1].price, 37399.0);
        assert_eq!(orderbook.bids[1].quantity_base, 10.0 * 6530.0 / 37399.0);
        assert_eq!(orderbook.bids[1].quantity_quote, 10.0 * 6530.0);
        assert_eq!(orderbook.bids[1].quantity_contract.unwrap(), 6530.0);

        assert_eq!(orderbook.asks[0].price, 37850.0);
        assert_eq!(orderbook.asks[0].quantity_base, 10.0 * 8850.0 / 37850.0);
        assert_eq!(orderbook.asks[0].quantity_quote, 10.0 * 8850.0);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 8850.0);

        assert_eq!(orderbook.asks[1].price, 37848.5);
        assert_eq!(orderbook.asks[1].quantity_base, 0.0 / 37848.5);
        assert_eq!(orderbook.asks[1].quantity_quote, 0.0);
        assert_eq!(orderbook.asks[1].quantity_contract.unwrap(), 0.0);
    }

    #[test]
    fn inverse_swap_snapshot() {
        let raw_msg = r#"{"jsonrpc":"2.0","method":"subscription","params":{"channel":"book.BTC-PERPETUAL.100ms","data":{"type":"snapshot","timestamp":1622627433440,"instrument_name":"BTC-PERPETUAL","change_id":31479596557,"bids":[["new",37240.0,20.0],["new",37237.0,14270.0],["new",37233.0,50.0]],"asks":[["new",37240.5,14240.0],["new",37248.5,15690.0],["new",37251.0,650.0]]}}}"#;
        let orderbook =
            &parse_l2(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 3);
        assert_eq!(orderbook.bids.len(), 3);
        assert!(orderbook.snapshot);

        assert_eq!(orderbook.timestamp, 1622627433440);
        assert_eq!(orderbook.seq_id, Some(31479596557));
        assert_eq!(orderbook.prev_seq_id, None);

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
            1622627433440,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.bids[0].price, 37240.0);
        assert_eq!(orderbook.bids[0].quantity_base, 10.0 * 20.0 / 37240.0);
        assert_eq!(orderbook.bids[0].quantity_quote, 10.0 * 20.0);
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 20.0);

        assert_eq!(orderbook.bids[2].price, 37233.0);
        assert_eq!(orderbook.bids[2].quantity_base, 10.0 * 50.0 / 37233.0);
        assert_eq!(orderbook.bids[2].quantity_quote, 10.0 * 50.0);
        assert_eq!(orderbook.bids[2].quantity_contract.unwrap(), 50.0);

        assert_eq!(orderbook.asks[0].price, 37240.5);
        assert_eq!(orderbook.asks[0].quantity_base, 10.0 * 14240.0 / 37240.5);
        assert_eq!(orderbook.asks[0].quantity_quote, 10.0 * 14240.0);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 14240.0);

        assert_eq!(orderbook.asks[2].price, 37251.0);
        assert_eq!(orderbook.asks[2].quantity_base, 10.0 * 650.0 / 37251.0);
        assert_eq!(orderbook.asks[2].quantity_quote, 10.0 * 650.0);
        assert_eq!(orderbook.asks[2].quantity_contract.unwrap(), 650.0);
    }

    #[test]
    fn inverse_swap_update() {
        let raw_msg = r#"{"jsonrpc":"2.0","method":"subscription","params":{"channel":"book.BTC-PERPETUAL.100ms","data":{"type":"change","timestamp":1622627435737,"prev_change_id":31479598064,"instrument_name":"BTC-PERPETUAL","change_id":31479598217,"bids":[["delete",36779.0,0.0],["new",36809.5,254870.0]],"asks":[["delete",37462.5,0.0],["change",37394.0,42670.0]]}}}"#;
        let orderbook =
            &parse_l2(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 2);
        assert_eq!(orderbook.bids.len(), 2);
        assert!(!orderbook.snapshot);

        assert_eq!(orderbook.timestamp, 1622627435737);
        assert_eq!(orderbook.seq_id, Some(31479598217));
        assert_eq!(orderbook.prev_seq_id, Some(31479598064));

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
            1622627435737,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.bids[0].price, 36779.0);
        assert_eq!(orderbook.bids[0].quantity_base, 0.0 / 36779.0);
        assert_eq!(orderbook.bids[0].quantity_quote, 0.0);
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 0.0);

        assert_eq!(orderbook.bids[1].price, 36809.5);
        assert_eq!(orderbook.bids[1].quantity_base, 10.0 * 254870.0 / 36809.5);
        assert_eq!(orderbook.bids[1].quantity_quote, 10.0 * 254870.0);
        assert_eq!(orderbook.bids[1].quantity_contract.unwrap(), 254870.0);

        assert_eq!(orderbook.asks[0].price, 37462.5);
        assert_eq!(orderbook.asks[0].quantity_base, 0.0 / 37462.5);
        assert_eq!(orderbook.asks[0].quantity_quote, 0.0);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 0.0);

        assert_eq!(orderbook.asks[1].price, 37394.0);
        assert_eq!(orderbook.asks[1].quantity_base, 10.0 * 42670.0 / 37394.0);
        assert_eq!(orderbook.asks[1].quantity_quote, 10.0 * 42670.0);
        assert_eq!(orderbook.asks[1].quantity_contract.unwrap(), 42670.0);
    }

    #[test]
    fn option_snapshot() {
        let raw_msg = r#"{"jsonrpc":"2.0","method":"subscription","params":{"channel":"book.BTC-11JUN21-25000-P.100ms","data":{"type":"snapshot","timestamp":1622627851747,"instrument_name":"BTC-11JUN21-25000-P","change_id":31479771122,"bids":[["new",0.005,13.7],["new",0.0045,5.7],["new",0.004,61.6]],"asks":[["new",0.006,64.5],["new",0.0065,48.0],["new",0.0085,0.5]]}}}"#;
        let orderbook =
            &parse_l2(EXCHANGE_NAME, MarketType::EuropeanOption, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 3);
        assert_eq!(orderbook.bids.len(), 3);
        assert!(orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            EXCHANGE_NAME,
            MarketType::EuropeanOption,
            MessageType::L2Event,
            "BTC/BTC".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::EuropeanOption, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );
        assert_eq!(
            1622627851747,
            extract_timestamp(EXCHANGE_NAME, MarketType::EuropeanOption, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1622627851747);
        assert_eq!(orderbook.seq_id, Some(31479771122));
        assert_eq!(orderbook.prev_seq_id, None);

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

#[cfg(test)]
mod l2_topk {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, extract_timestamp, parse_l2_topk};
    use crypto_msg_type::MessageType;

    #[test]
    fn inverse_future() {
        let raw_msg = r#"{"jsonrpc":"2.0","method":"subscription","params":{"channel":"book.BTC-30SEP22.none.20.100ms","data":{"timestamp":1653982973195,"instrument_name":"BTC-30SEP22","change_id":45176371821,"bids":[[31975.0,1370.0],[31966.0,2200.0],[31961.0,300.0]],"asks":[[31976.5,2500.0],[31979.0,60.0],[31982.0,2200.0]]}}}"#;
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
            1653982973195,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1653982973195);
        assert_eq!(orderbook.seq_id, Some(45176371821));
        assert_eq!(orderbook.prev_seq_id, None);

        assert_eq!(orderbook.bids[0].price, 31975.0);
        assert_eq!(orderbook.bids[0].quantity_base, 10.0 * 1370.0 / 31975.0);
        assert_eq!(orderbook.bids[0].quantity_quote, 10.0 * 1370.0);
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 1370.0);

        assert_eq!(orderbook.bids[2].price, 31961.0);
        assert_eq!(orderbook.bids[2].quantity_base, 10.0 * 300.0 / 31961.0);
        assert_eq!(orderbook.bids[2].quantity_quote, 10.0 * 300.0);
        assert_eq!(orderbook.bids[2].quantity_contract.unwrap(), 300.0);

        assert_eq!(orderbook.asks[0].price, 31976.5);
        assert_eq!(orderbook.asks[0].quantity_base, 10.0 * 2500.0 / 31976.5);
        assert_eq!(orderbook.asks[0].quantity_quote, 10.0 * 2500.0);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 2500.0);

        assert_eq!(orderbook.asks[2].price, 31982.0);
        assert_eq!(orderbook.asks[2].quantity_base, 10.0 * 2200.0 / 31982.0);
        assert_eq!(orderbook.asks[2].quantity_quote, 10.0 * 2200.0);
        assert_eq!(orderbook.asks[2].quantity_contract.unwrap(), 2200.0);
    }

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"jsonrpc":"2.0","method":"subscription","params":{"channel":"book.BTC-PERPETUAL.none.20.100ms","data":{"timestamp":1653983481909,"instrument_name":"BTC-PERPETUAL","change_id":45176552517,"bids":[[31523.5,128780.0],[31523.0,190.0],[31521.5,14500.0]],"asks":[[31524.0,30.0],[31525.0,30.0],[31525.5,6010.0]]}}}"#;
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
            1653983481909,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1653983481909);
        assert_eq!(orderbook.seq_id, Some(45176552517));
        assert_eq!(orderbook.prev_seq_id, None);

        assert_eq!(orderbook.bids[0].price, 31523.5);
        assert_eq!(orderbook.bids[0].quantity_base, 10.0 * 128780.0 / 31523.5);
        assert_eq!(orderbook.bids[0].quantity_quote, 10.0 * 128780.0);
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 128780.0);

        assert_eq!(orderbook.bids[2].price, 31521.5);
        assert_eq!(orderbook.bids[2].quantity_base, 10.0 * 14500.0 / 31521.5);
        assert_eq!(orderbook.bids[2].quantity_quote, 10.0 * 14500.0);
        assert_eq!(orderbook.bids[2].quantity_contract.unwrap(), 14500.0);

        assert_eq!(orderbook.asks[0].price, 31524.0);
        assert_eq!(orderbook.asks[0].quantity_base, 10.0 * 30.0 / 31524.0);
        assert_eq!(orderbook.asks[0].quantity_quote, 10.0 * 30.0);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 30.0);

        assert_eq!(orderbook.asks[2].price, 31525.5);
        assert_eq!(orderbook.asks[2].quantity_base, 10.0 * 6010.0 / 31525.5);
        assert_eq!(orderbook.asks[2].quantity_quote, 10.0 * 6010.0);
        assert_eq!(orderbook.asks[2].quantity_contract.unwrap(), 6010.0);
    }

    #[test]
    fn option() {
        let raw_msg = r#"{"jsonrpc":"2.0","method":"subscription","params":{"channel":"book.BTC-30SEP22-60000-C.none.20.100ms","data":{"timestamp":1653983742265,"instrument_name":"BTC-30SEP22-60000-C","change_id":45176637818,"bids":[[0.011,15.4],[0.0105,42.2],[0.01,4.1]],"asks":[[0.012,10.2],[0.0125,16.6],[0.013,44.4]]}}}"#;
        let orderbook =
            &parse_l2_topk(EXCHANGE_NAME, MarketType::EuropeanOption, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 3);
        assert_eq!(orderbook.bids.len(), 3);
        assert!(orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            EXCHANGE_NAME,
            MarketType::EuropeanOption,
            MessageType::L2TopK,
            "BTC/BTC".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::EuropeanOption, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );
        assert_eq!(
            1653983742265,
            extract_timestamp(EXCHANGE_NAME, MarketType::EuropeanOption, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1653983742265);
        assert_eq!(orderbook.seq_id, Some(45176637818));
        assert_eq!(orderbook.prev_seq_id, None);

        assert_eq!(orderbook.bids[0].price, 0.011);
        assert_eq!(orderbook.bids[0].quantity_base, 15.4);
        assert_eq!(orderbook.bids[0].quantity_quote, 0.011 * 15.4);
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 15.4);

        assert_eq!(orderbook.bids[2].price, 0.01);
        assert_eq!(orderbook.bids[2].quantity_base, 4.1);
        assert_eq!(orderbook.bids[2].quantity_quote, 0.01 * 4.1);
        assert_eq!(orderbook.bids[2].quantity_contract.unwrap(), 4.1);

        assert_eq!(orderbook.asks[0].price, 0.012);
        assert_eq!(orderbook.asks[0].quantity_base, 10.2);
        assert_eq!(orderbook.asks[0].quantity_quote, 0.012 * 10.2);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 10.2);

        assert_eq!(orderbook.asks[2].price, 0.013);
        assert_eq!(orderbook.asks[2].quantity_base, 44.4);
        assert_eq!(orderbook.asks[2].quantity_quote, 0.013 * 44.4);
        assert_eq!(orderbook.asks[2].quantity_contract.unwrap(), 44.4);
    }
}

#[cfg(test)]
mod bbo {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, extract_timestamp};

    #[test]
    fn inverse_future() {
        let raw_msg = r#"{"jsonrpc":"2.0","method":"subscription","params":{"channel":"quote.BTC-30SEP22","data":{"timestamp":1654012570801,"instrument_name":"BTC-30SEP22","best_bid_price":32499.0,"best_bid_amount":2370.0,"best_ask_price":32503.5,"best_ask_amount":2400.0}}}"#;

        assert_eq!(
            1654012570801,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "BTC-30SEP22",
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );
    }

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"jsonrpc":"2.0","method":"subscription","params":{"channel":"quote.BTC-PERPETUAL","data":{"timestamp":1654012882984,"instrument_name":"BTC-PERPETUAL","best_bid_price":32143.0,"best_bid_amount":179960.0,"best_ask_price":32143.5,"best_ask_amount":20.0}}}"#;

        assert_eq!(
            1654012882984,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "BTC-PERPETUAL",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );
    }
}

#[cfg(test)]
mod candlestick {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, extract_timestamp};

    #[test]
    fn inverse_future() {
        let raw_msg = r#"{"jsonrpc":"2.0","method":"subscription","params":{"channel":"chart.trades.BTC-30SEP22.1","data":{"volume":0.0,"tick":1654078920000,"open":31949.0,"low":31949.0,"high":31949.0,"cost":0.0,"close":31949.0}}}"#;

        assert_eq!(
            1654078920000,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "BTC-30SEP22",
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );
    }

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"jsonrpc":"2.0","method":"subscription","params":{"channel":"chart.trades.BTC-PERPETUAL.1","data":{"volume":0.02120555,"tick":1654079340000,"open":31595.5,"low":31595.5,"high":31595.5,"cost":670.0,"close":31595.5}}}"#;

        assert_eq!(
            1654079340000,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "BTC-PERPETUAL",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );
    }

    #[test]
    fn option() {
        let raw_msg = r#"{"jsonrpc":"2.0","method":"subscription","params":{"channel":"chart.trades.BTC-30SEP22-60000-C.1","data":{"volume":0.0,"tick":1654079400000,"open":0.0115,"low":0.0115,"high":0.0115,"cost":0.0,"close":0.0115}}}"#;

        assert_eq!(
            1654079400000,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "BTC-30SEP22-60000-C",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );
    }
}
