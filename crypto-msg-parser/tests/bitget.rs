mod utils;

#[cfg(test)]
mod trade {
    use crypto_msg_parser::{parse_trade, MarketType, TradeSide};
    use float_cmp::approx_eq;

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"data":[{"instrument_id":"btcusd","price":"58722.0","side":"sell","size":"158","timestamp":"1616236107276"},{"instrument_id":"btcusd","price":"58722.0","side":"sell","size":"450","timestamp":"1616236107276"},{"instrument_id":"btcusd","price":"58722.0","side":"sell","size":"762","timestamp":"1616236107276"}],"table":"swap/trade"}"#;
        let trades = &parse_trade("bitget", MarketType::InverseSwap, raw_msg).unwrap();

        assert_eq!(trades.len(), 3);

        for trade in trades.iter() {
            crate::utils::check_trade_fields(
                "bitget",
                MarketType::InverseSwap,
                "BTC/USD".to_string(),
                trade,
            );
            assert_eq!(trade.side, TradeSide::Sell);
        }

        assert_eq!(trades[0].quantity_base, 158.0 / 58722.0);
        assert_eq!(trades[0].quantity_quote, 158.0);
        assert_eq!(trades[0].quantity_contract, Some(158.0));

        assert_eq!(trades[1].quantity_base, 450.0 / 58722.0);
        assert_eq!(trades[1].quantity_quote, 450.0);
        assert_eq!(trades[1].quantity_contract, Some(450.0));

        assert_eq!(trades[2].quantity_base, 762.0 / 58722.0);
        assert_eq!(trades[2].quantity_quote, 762.0);
        assert_eq!(trades[2].quantity_contract, Some(762.0));
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"data":[{"instrument_id":"cmt_btcusdt","price":"58784.0","side":"sell","size":"1265","timestamp":"1616236212569"},{"instrument_id":"cmt_btcusdt","price":"58784.0","side":"sell","size":"25","timestamp":"1616236212569"},{"instrument_id":"cmt_btcusdt","price":"58784.0","side":"sell","size":"181","timestamp":"1616236212569"}],"table":"swap/trade"}"#;
        let trades = &parse_trade("bitget", MarketType::LinearSwap, raw_msg).unwrap();

        assert_eq!(trades.len(), 3);

        for trade in trades.iter() {
            crate::utils::check_trade_fields(
                "bitget",
                MarketType::LinearSwap,
                "BTC/USDT".to_string(),
                trade,
            );

            assert_eq!(trade.side, TradeSide::Sell);
        }
        assert!(approx_eq!(
            f64,
            trades[0].quantity_base,
            1265.0 * 0.001,
            epsilon = 0.0000001
        ));
        assert!(approx_eq!(
            f64,
            trades[1].quantity_base,
            25.0 * 0.001,
            epsilon = 0.0000001
        ));
        assert!(approx_eq!(
            f64,
            trades[2].quantity_base,
            181.0 * 0.001,
            epsilon = 0.0000001
        ));
    }
}

#[cfg(test)]
mod funding_rate {
    use crypto_msg_parser::{parse_funding_rate, MarketType};

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"data":[{"funding_rate":"0.000258514264","funding_time":"1617346800000","instrument_id":"btcusd"}],"table":"swap/funding_rate"}"#;
        let funding_rates =
            &parse_funding_rate("bitget", MarketType::InverseSwap, raw_msg).unwrap();

        assert_eq!(funding_rates.len(), 1);

        for rate in funding_rates.iter() {
            crate::utils::check_funding_rate_fields("bitget", MarketType::InverseSwap, rate);
        }

        assert_eq!(funding_rates[0].pair, "BTC/USD".to_string());
        assert_eq!(funding_rates[0].funding_rate, 0.000258514264);
        assert_eq!(funding_rates[0].funding_time, 1617346800000);
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"data":[{"funding_rate":"0.000106539854","funding_time":"1617346800000","instrument_id":"cmt_btcusdt"}],"table":"swap/funding_rate"}"#;
        let funding_rates = &parse_funding_rate("bitget", MarketType::LinearSwap, raw_msg).unwrap();

        assert_eq!(funding_rates.len(), 1);

        for rate in funding_rates.iter() {
            crate::utils::check_funding_rate_fields("bitget", MarketType::LinearSwap, rate);
        }

        assert_eq!(funding_rates[0].pair, "BTC/USDT".to_string());
        assert_eq!(funding_rates[0].funding_rate, 0.000106539854);
        assert_eq!(funding_rates[0].funding_time, 1617346800000);
    }
}
