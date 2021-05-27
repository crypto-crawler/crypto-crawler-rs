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
        let trades = &parse_trade("deribit", MarketType::Option, raw_msg).unwrap();

        assert_eq!(trades.len(), 1);

        for trade in trades.iter() {
            crate::utils::check_trade_fields(
                "deribit",
                MarketType::Option,
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
