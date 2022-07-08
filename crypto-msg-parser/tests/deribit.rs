mod utils;

const EXCHANGE_NAME: &str = "deribit";

#[cfg(test)]
mod trade {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_message::TradeSide;
    use crypto_msg_parser::{extract_symbol, extract_timestamp, parse_trade};

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
    use crypto_msg_parser::{extract_symbol, extract_timestamp, parse_bbo};
    use crypto_msg_type::MessageType;

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

        let received_at = 1654012882984;
        let bbo_msg = parse_bbo(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg, Some(received_at)).unwrap();

        assert_eq!(MessageType::BBO, bbo_msg.msg_type);
        assert_eq!("BTC-PERPETUAL", bbo_msg.symbol);
        assert_eq!(1654012882984, bbo_msg.timestamp);
        assert_eq!(None, bbo_msg.id);

        assert_eq!(32143.5, bbo_msg.ask_price);
        assert_eq!(0.006222097780266617, bbo_msg.ask_quantity_base);
        assert_eq!(32143.5 * 0.006222097780266617, bbo_msg.ask_quantity_quote);
        assert_eq!(Some(20.0), bbo_msg.ask_quantity_contract);

        assert_eq!(32143.0, bbo_msg.bid_price);
        assert_eq!(55.98730672308123, bbo_msg.bid_quantity_base);
        assert_eq!(32143.0 * 55.98730672308123, bbo_msg.bid_quantity_quote);
        assert_eq!(Some(179960.0), bbo_msg.bid_quantity_contract);
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

#[cfg(test)]
mod ticker {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, extract_timestamp};

    #[test]
    fn inverse_future() {
        let raw_msg = r#"{"jsonrpc":"2.0","method":"subscription","params":{"channel":"ticker.BTC-30SEP22.100ms","data":{"timestamp":1654161740658,"stats":{"volume_usd":23223070.0,"volume":754.95506101,"price_change":-5.5392,"low":29592.5,"high":32248.0},"state":"open","settlement_price":30225.47,"open_interest":230733270,"min_price":29766.5,"max_price":30673.5,"mark_price":30218.9,"last_price":30218.0,"instrument_name":"BTC-30SEP22","index_price":29939.87,"estimated_delivery_price":29939.87,"best_bid_price":30220.0,"best_bid_amount":2300.0,"best_ask_price":30222.0,"best_ask_amount":4370.0}}}"#;

        assert_eq!(
            1654161740658,
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
        let raw_msg = r#"{"jsonrpc":"2.0","method":"subscription","params":{"channel":"ticker.BTC-PERPETUAL.100ms","data":{"timestamp":1654161785818,"stats":{"volume_usd":545442610.0,"volume":17945.19644566,"price_change":-5.4014,"low":29265.5,"high":31903.5},"state":"open","settlement_price":29945.69,"open_interest":559791310,"min_price":29485.31,"max_price":30383.34,"mark_price":29932.79,"last_price":29931.0,"instrument_name":"BTC-PERPETUAL","index_price":29930.44,"funding_8h":0.00000255,"estimated_delivery_price":29930.44,"current_funding":0.0,"best_bid_price":29930.5,"best_bid_amount":149910.0,"best_ask_price":29931.0,"best_ask_amount":62850.0}}}"#;

        assert_eq!(
            1654161785818,
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
        let raw_msg = r#"{"jsonrpc":"2.0","method":"subscription","params":{"channel":"ticker.BTC-30SEP22-60000-C.100ms","data":{"underlying_price":30220.5,"underlying_index":"BTC-30SEP22","timestamp":1654161839367,"stats":{"volume":16.7,"price_change":-18.1818,"low":0.009,"high":0.011},"state":"open","settlement_price":0.01,"open_interest":1767.5,"min_price":0.0001,"max_price":0.038,"mark_price":0.0084,"mark_iv":67.7,"last_price":0.009,"interest_rate":0.0,"instrument_name":"BTC-30SEP22-60000-C","index_price":29939.43,"greeks":{"vega":20.05335,"theta":-5.65962,"rho":4.91491,"gamma":0.00001,"delta":0.05785},"estimated_delivery_price":29939.43,"bid_iv":67.16,"best_bid_price":0.008,"best_bid_amount":2.8,"best_ask_price":0.009,"best_ask_amount":18.5,"ask_iv":68.65}}}"#;

        assert_eq!(
            1654161839367,
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

#[cfg(test)]
mod l2_snapshot {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, extract_timestamp};

    #[test]
    fn inverse_future() {
        let raw_msg = r#"{"jsonrpc":"2.0","result":{"timestamp":1654245922403,"stats":{"volume_usd":10190920.0,"volume":335.62468116,"price_change":1.6501,"low":29846.0,"high":30976.5},"state":"open","settlement_price":30749.45,"open_interest":232318090,"min_price":30256.0,"max_price":31178.0,"mark_price":30717.33,"last_price":30709.5,"instrument_name":"BTC-30SEP22","index_price":30410.37,"estimated_delivery_price":30410.37,"change_id":45305026640,"bids":[[30718.5,2290.0],[30718.0,3000.0],[30714.0,5000.0]],"best_bid_price":30718.5,"best_bid_amount":2290.0,"best_ask_price":30723.0,"best_ask_amount":3000.0,"asks":[[30723.0,3000.0],[30723.5,29470.0],[30725.0,1380.0]]},"usIn":1654245922540414,"usOut":1654245922540910,"usDiff":496,"testnet":false}"#;

        assert_eq!(
            "BTC-30SEP22",
            extract_symbol(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg).unwrap()
        );

        assert_eq!(
            1654245922403,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg)
                .unwrap()
                .unwrap()
        );
    }

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"jsonrpc":"2.0","result":{"timestamp":1654246806027,"stats":{"volume_usd":278922050.0,"volume":9229.89241729,"price_change":1.721,"low":29583.5,"high":30729.0},"state":"open","settlement_price":30458.76,"open_interest":560520540,"min_price":29980.5,"max_price":30893.61,"mark_price":30436.94,"last_price":30439.5,"instrument_name":"BTC-PERPETUAL","index_price":30418.45,"funding_8h":0.00002085,"estimated_delivery_price":30418.45,"current_funding":0.00010785,"change_id":45305261539,"bids":[[30434.5,600.0],[30433.0,15000.0],[30431.0,15010.0]],"best_bid_price":30434.5,"best_bid_amount":600.0,"best_ask_price":30435.0,"best_ask_amount":198440.0,"asks":[[30435.0,198440.0],[30438.5,1000.0],[30439.5,188330.0]]},"usIn":1654246806051360,"usOut":1654246806055238,"usDiff":3878,"testnet":false}"#;

        assert_eq!(
            "BTC-PERPETUAL",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );

        assert_eq!(
            1654246806027,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
    }

    #[test]
    fn option() {
        let raw_msg = r#"{"jsonrpc":"2.0","result":{"underlying_price":1806.5,"underlying_index":"ETH-24JUN22","timestamp":1654247489923,"stats":{"volume":1117.0,"price_change":-8.4507,"low":0.031,"high":0.036},"state":"open","settlement_price":0.03,"open_interest":15009.0,"min_price":0.009,"max_price":0.0795,"mark_price":0.0336,"mark_iv":86.54,"last_price":0.0325,"interest_rate":0.0,"instrument_name":"ETH-24JUN22-1600-P","index_price":1804.41,"greeks":{"vega":1.36169,"theta":-2.81237,"rho":-0.28927,"gamma":0.00084,"delta":-0.24537},"estimated_delivery_price":1804.41,"change_id":24460060432,"bids":[[0.033,226.0],[0.0325,797.0],[0.032,355.0]],"bid_iv":85.74,"best_bid_price":0.033,"best_bid_amount":226.0,"best_ask_price":0.034,"best_ask_amount":595.0,"asks":[[0.034,595.0],[0.0345,779.0],[0.035,186.0]],"ask_iv":87.07},"usIn":1654247489999588,"usOut":1654247489999885,"usDiff":297,"testnet":false}"#;

        assert_eq!(
            1654247489923,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "ETH-24JUN22-1600-P",
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
        let raw_msg = r#"{"jsonrpc":"2.0","result":[{"volume_usd":12498220.0,"volume_notional":12498220.0,"volume":422.01,"quote_currency":"USD","price_change":-0.2148,"open_interest":272308070,"mid_price":29730.25,"mark_price":29727.97,"low":29280.5,"last":29724.5,"instrument_name":"BTC-24JUN22","high":29973.0,"estimated_delivery_price":29695.94,"creation_timestamp":1654341687529,"bid_price":29729.5,"base_currency":"BTC","ask_price":29731.0}],"usIn":1654341687528897,"usOut":1654341687529042,"usDiff":145,"testnet":false}"#;

        assert_eq!(
            "BTC-24JUN22",
            extract_symbol(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg).unwrap()
        );

        assert_eq!(
            1654341687529,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg)
                .unwrap()
                .unwrap()
        );
    }

    #[test]
    fn inverse_future_all() {
        let raw_msg = r#"{"jsonrpc":"2.0","result":[{"volume":175.6,"underlying_price":29755.49,"underlying_index":"BTC-24JUN22","quote_currency":"BTC","price_change":-13.2231,"open_interest":1124.7,"mid_price":0.053,"mark_price":0.05317014,"low":0.0515,"last":0.0525,"interest_rate":0.0,"instrument_name":"BTC-24JUN22-30000-C","high":0.0605,"estimated_delivery_price":29716.47,"creation_timestamp":1654338604906,"bid_price":0.0525,"base_currency":"BTC","ask_price":0.0535},{"volume":0.0,"underlying_price":30236.72,"underlying_index":"BTC-30DEC22","quote_currency":"BTC","price_change":null,"open_interest":8.1,"mid_price":null,"mark_price":1.98432769,"low":null,"last":1.099,"interest_rate":0.0,"instrument_name":"BTC-30DEC22-90000-P","high":null,"estimated_delivery_price":29716.47,"creation_timestamp":1654338604906,"bid_price":null,"base_currency":"BTC","ask_price":null}],"usIn":1654338604904465,"usOut":1654338604920779,"usDiff":16314,"testnet":false}"#;

        assert_eq!(
            "ALL",
            extract_symbol(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg).unwrap()
        );

        assert_eq!(
            1654338604906,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg)
                .unwrap()
                .unwrap()
        );
    }

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"jsonrpc":"2.0","result":[{"volume_usd":223275080.0,"volume_notional":223275080.0,"volume":7539.21,"quote_currency":"USD","price_change":-0.369,"open_interest":560105870,"mid_price":29700.75,"mark_price":29700.53,"low":29266.5,"last":29698.0,"instrument_name":"BTC-PERPETUAL","high":29934.0,"funding_8h":0.00000214,"estimated_delivery_price":29695.49,"current_funding":0.0,"creation_timestamp":1654340303741,"bid_price":29700.5,"base_currency":"BTC","ask_price":29701.0}],"usIn":1654340303741682,"usOut":1654340303741855,"usDiff":173,"testnet":false}"#;

        assert_eq!(
            "BTC-PERPETUAL",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );

        assert_eq!(
            1654340303741,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
    }

    #[test]
    fn option() {
        let raw_msg = r#"{"jsonrpc":"2.0","result":[{"volume":786.0,"underlying_price":1770.45,"underlying_index":"ETH-24JUN22","quote_currency":"ETH","price_change":-12.178,"open_interest":15135.0,"mid_price":0.037,"mark_price":0.037071,"low":0.0375,"last":0.0375,"interest_rate":0.0,"instrument_name":"ETH-24JUN22-1600-P","high":0.044,"estimated_delivery_price":1768.89,"creation_timestamp":1654341866165,"bid_price":0.0365,"base_currency":"ETH","ask_price":0.0375}],"usIn":1654341866165387,"usOut":1654341866165540,"usDiff":153,"testnet":false}"#;

        assert_eq!(
            1654341866165,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "ETH-24JUN22-1600-P",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );
    }
}
