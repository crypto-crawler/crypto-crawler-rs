mod utils;

const EXCHANGE_NAME: &str = "mexc";

#[cfg(test)]
mod trade {
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, extract_timestamp, parse_trade, TradeSide};

    #[test]
    fn spot() {
        let raw_msg = r#"["push.symbol",{"symbol":"BTC_USDT","data":{"deals":[{"t":1616373554541,"p":"57005.89","q":"0.007811","T":1}]}}]"#;
        let trades = &parse_trade(super::EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap();

        assert_eq!(trades.len(), 1);
        let trade = &trades[0];

        crate::utils::check_trade_fields(
            super::EXCHANGE_NAME,
            MarketType::Spot,
            "BTC/USDT".to_string(),
            extract_symbol(super::EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap(),
            trade,
            raw_msg,
        );
        assert_eq!(
            1616373554541,
            extract_timestamp(super::EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(trade.quantity_base, 0.007811);
        assert_eq!(trade.side, TradeSide::Buy);
    }

    #[test]
    fn spot_20220311() {
        let raw_msg = r#"{"symbol":"BTC_USDT","data":{"deals":[{"t":1646996447307,"p":"39008.35","q":"0.003533","T":2}]},"channel":"push.deal"}"#;
        let trades = &parse_trade(super::EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap();

        assert_eq!(trades.len(), 1);
        let trade = &trades[0];

        crate::utils::check_trade_fields(
            super::EXCHANGE_NAME,
            MarketType::Spot,
            "BTC/USDT".to_string(),
            extract_symbol(super::EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap(),
            trade,
            raw_msg,
        );
        assert_eq!(
            1646996447307,
            extract_timestamp(super::EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(trade.timestamp, 1646996447307);
        assert_eq!(trade.price, 39008.35);
        assert_eq!(trade.quantity_base, 0.003533);
        assert_eq!(trade.side, TradeSide::Sell);
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"channel":"push.deal","data":{"M":1,"O":3,"T":2,"p":57602,"t":1616370338806,"v":14},"symbol":"BTC_USDT","ts":1616370338806}"#;
        let trades = &parse_trade(super::EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap();

        assert_eq!(trades.len(), 1);
        let trade = &trades[0];

        crate::utils::check_trade_fields(
            super::EXCHANGE_NAME,
            MarketType::LinearSwap,
            "BTC/USDT".to_string(),
            extract_symbol(super::EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap(),
            trade,
            raw_msg,
        );
        assert_eq!(
            1616370338806,
            extract_timestamp(super::EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(trade.timestamp, 1616370338806);
        assert_eq!(trade.price, 57602.0);
        assert_eq!(trade.quantity_contract, Some(14.0));
        assert_eq!(trade.quantity_base, 0.0001 * 14.0);
        assert_eq!(trade.quantity_quote, 0.0001 * 14.0 * 57602.0);
        assert_eq!(trade.side, TradeSide::Sell);
    }

    #[test]
    fn linear_swap_20220311() {
        let raw_msg = r#"{"channel":"push.deal","data":{"M":1,"O":3,"T":2,"p":39766.5,"t":1646999591755,"v":32},"symbol":"BTC_USDT","ts":1646999591755}"#;
        let trades = &parse_trade(super::EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap();

        assert_eq!(trades.len(), 1);
        let trade = &trades[0];

        crate::utils::check_trade_fields(
            super::EXCHANGE_NAME,
            MarketType::LinearSwap,
            "BTC/USDT".to_string(),
            extract_symbol(super::EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap(),
            trade,
            raw_msg,
        );
        assert_eq!(
            1646999591755,
            extract_timestamp(super::EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(trade.timestamp, 1646999591755);
        assert_eq!(trade.price, 39766.5);
        assert_eq!(trade.quantity_contract, Some(32.0));
        assert_eq!(trade.quantity_base, 0.0001 * 32.0);
        assert_eq!(trade.quantity_quote, 0.0001 * 32.0 * 39766.5);
        assert_eq!(trade.side, TradeSide::Sell);
    }

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"channel":"push.deal","data":{"M":1,"O":3,"T":1,"p":57476.5,"t":1616370470356,"v":79},"symbol":"BTC_USD","ts":1616370470356}"#;
        let trades = &parse_trade(super::EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap();

        assert_eq!(trades.len(), 1);
        let trade = &trades[0];

        crate::utils::check_trade_fields(
            super::EXCHANGE_NAME,
            MarketType::InverseSwap,
            "BTC/USD".to_string(),
            extract_symbol(super::EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap(),
            trade,
            raw_msg,
        );
        assert_eq!(
            1616370470356,
            extract_timestamp(super::EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(trade.timestamp, 1616370470356);
        assert_eq!(trade.price, 57476.5);
        assert_eq!(trade.quantity_contract, Some(79.0));
        assert_eq!(trade.quantity_quote, 100.0 * 79.0);
        assert_eq!(trade.quantity_base, 100.0 * 79.0 / 57476.5);
        assert_eq!(trade.side, TradeSide::Buy);
    }

    #[test]
    fn inverse_swap_20220311() {
        let raw_msg = r#"{"channel":"push.deal","data":{"M":1,"O":3,"T":2,"p":39885.5,"t":1647000043904,"v":8},"symbol":"BTC_USD","ts":1647000043904}"#;
        let trades = &parse_trade(super::EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap();

        assert_eq!(trades.len(), 1);
        let trade = &trades[0];

        crate::utils::check_trade_fields(
            super::EXCHANGE_NAME,
            MarketType::InverseSwap,
            "BTC/USD".to_string(),
            extract_symbol(super::EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap(),
            trade,
            raw_msg,
        );
        assert_eq!(
            1647000043904,
            extract_timestamp(super::EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(trade.timestamp, 1647000043904);
        assert_eq!(trade.price, 39885.5);
        assert_eq!(trade.quantity_contract, Some(8.0));
        assert_eq!(trade.quantity_quote, 100.0 * 8.0);
        assert_eq!(trade.quantity_base, 100.0 * 8.0 / 39885.5);
        assert_eq!(trade.side, TradeSide::Sell);
    }
}

#[cfg(test)]
mod l2_orderbook {
    use chrono::prelude::*;
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, extract_timestamp, parse_l2};
    use crypto_msg_type::MessageType;

    #[test]
    fn spot_update() {
        let raw_msg = r#"["push.symbol",{"symbol":"BTC_USDT","data":{"bids":[{"p":"38932.19","q":"0.049010","a":"1908.06663"},{"p":"38931.18","q":"0.038220","a":"1487.94969"}],"asks":[{"p":"38941.81","q":"0.000000","a":"0.00000000"},{"p":"38940.71","q":"0.000000","a":"0.00000000"}]}}]"#;
        let received_at = Utc::now().timestamp_millis();
        let orderbook = &parse_l2(
            super::EXCHANGE_NAME,
            MarketType::Spot,
            raw_msg,
            Some(received_at),
        )
        .unwrap()[0];

        assert_eq!(orderbook.asks.len(), 2);
        assert_eq!(orderbook.bids.len(), 2);
        assert!(!orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            super::EXCHANGE_NAME,
            MarketType::Spot,
            MessageType::L2Event,
            "BTC/USDT".to_string(),
            extract_symbol(super::EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );
        assert_eq!(
            None,
            extract_timestamp(super::EXCHANGE_NAME, MarketType::Spot, raw_msg,).unwrap()
        );

        assert_eq!(orderbook.bids[0].price, 38932.19);
        assert_eq!(orderbook.bids[0].quantity_base, 0.04901);
        assert_eq!(orderbook.bids[0].quantity_quote, 1908.06663);
    }

    #[test]
    fn spot_20220311() {
        let raw_msg = r#"{"symbol":"BTC_USDT","data":{"version":"672257402","bids":[{"p":"39763.35","q":"0.054069","a":"2149.96457"}]},"channel":"push.depth"}"#;
        let received_at = Utc::now().timestamp_millis();
        let orderbook = &parse_l2(
            super::EXCHANGE_NAME,
            MarketType::Spot,
            raw_msg,
            Some(received_at),
        )
        .unwrap()[0];

        assert_eq!(orderbook.asks.len(), 0);
        assert_eq!(orderbook.bids.len(), 1);
        assert!(!orderbook.snapshot);
        assert_eq!(orderbook.seq_id, Some(672257402));

        crate::utils::check_orderbook_fields(
            super::EXCHANGE_NAME,
            MarketType::Spot,
            MessageType::L2Event,
            "BTC/USDT".to_string(),
            extract_symbol(super::EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );
        assert_eq!(
            None,
            extract_timestamp(super::EXCHANGE_NAME, MarketType::Spot, raw_msg,).unwrap()
        );

        assert_eq!(orderbook.bids[0].price, 39763.35);
        assert_eq!(orderbook.bids[0].quantity_base, 0.054069);
    }

    #[test]
    fn linear_swap_update() {
        let raw_msg = r#"{"channel":"push.depth","data":{"asks":[[38704.5,138686,1]],"bids":[],"version":2427341830},"symbol":"BTC_USDT","ts":1622722473816}"#;
        let orderbook =
            &parse_l2(super::EXCHANGE_NAME, MarketType::LinearSwap, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 1);
        assert_eq!(orderbook.bids.len(), 0);
        assert!(!orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            super::EXCHANGE_NAME,
            MarketType::LinearSwap,
            MessageType::L2Event,
            "BTC/USDT".to_string(),
            extract_symbol(super::EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );
        assert_eq!(
            1622722473816,
            extract_timestamp(super::EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1622722473816);
        assert_eq!(orderbook.seq_id, Some(2427341830));

        assert_eq!(orderbook.asks[0].price, 38704.5);
        assert_eq!(orderbook.asks[0].quantity_base, 13.8686);
        assert_eq!(orderbook.asks[0].quantity_quote, 38704.5 * 13.8686);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 138686.0);
    }

    #[test]
    fn linear_swap_update_20220311() {
        let raw_msg = r#"{"channel":"push.depth","data":{"asks":[[39961,0,0],[39961.5,0,0]],"bids":[[39962.5,58272,1]],"version":4702740808},"symbol":"BTC_USDT","ts":1647000258746}"#;
        let orderbook =
            &parse_l2(super::EXCHANGE_NAME, MarketType::LinearSwap, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 2);
        assert_eq!(orderbook.bids.len(), 1);
        assert!(!orderbook.snapshot);
        assert_eq!(orderbook.timestamp, 1647000258746);
        assert_eq!(orderbook.seq_id, Some(4702740808));

        crate::utils::check_orderbook_fields(
            super::EXCHANGE_NAME,
            MarketType::LinearSwap,
            MessageType::L2Event,
            "BTC/USDT".to_string(),
            extract_symbol(super::EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );
        assert_eq!(
            1647000258746,
            extract_timestamp(super::EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.asks[0].price, 39961.0);
        assert_eq!(orderbook.asks[0].quantity_contract, Some(0.0));
        assert_eq!(orderbook.asks[0].quantity_base, 0.0);
        assert_eq!(orderbook.asks[0].quantity_quote, 0.0);

        assert_eq!(orderbook.bids[0].price, 39962.5);
        assert_eq!(orderbook.bids[0].quantity_contract, Some(58272.0));
        assert_eq!(orderbook.bids[0].quantity_base, 0.0001 * 58272.0);
        assert_eq!(orderbook.bids[0].quantity_quote, 0.0001 * 58272.0 * 39962.5);
    }

    #[test]
    fn inverse_swap_update() {
        let raw_msg = r#"{"channel":"push.depth","data":{"asks":[[38758.5,4172,2]],"bids":[],"version":1151578213},"symbol":"BTC_USD","ts":1622723010000}"#;
        let orderbook =
            &parse_l2(super::EXCHANGE_NAME, MarketType::InverseSwap, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 1);
        assert_eq!(orderbook.bids.len(), 0);
        assert!(!orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            super::EXCHANGE_NAME,
            MarketType::InverseSwap,
            MessageType::L2Event,
            "BTC/USD".to_string(),
            extract_symbol(super::EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );
        assert_eq!(
            1622723010000,
            extract_timestamp(super::EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1622723010000);
        assert_eq!(orderbook.seq_id, Some(1151578213));

        assert_eq!(orderbook.asks[0].price, 38758.5);
        assert_eq!(orderbook.asks[0].quantity_base, 417200.0 / 38758.5);
        assert_eq!(orderbook.asks[0].quantity_quote, 417200.0);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 4172.0);
    }

    #[test]
    fn inverse_swap_update_20220311() {
        let raw_msg = r#"{"channel":"push.depth","data":{"asks":[],"bids":[[39944,943,1]],"version":2768205529},"symbol":"BTC_USD","ts":1647000870946}"#;
        let orderbook =
            &parse_l2(super::EXCHANGE_NAME, MarketType::InverseSwap, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 0);
        assert_eq!(orderbook.bids.len(), 1);
        assert!(!orderbook.snapshot);
        assert_eq!(orderbook.timestamp, 1647000870946);
        assert_eq!(orderbook.seq_id, Some(2768205529));

        crate::utils::check_orderbook_fields(
            super::EXCHANGE_NAME,
            MarketType::InverseSwap,
            MessageType::L2Event,
            "BTC/USD".to_string(),
            extract_symbol(super::EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );
        assert_eq!(
            1647000870946,
            extract_timestamp(super::EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.bids[0].price, 39944.0);
        assert_eq!(orderbook.bids[0].quantity_contract, Some(943.0));
        assert_eq!(orderbook.bids[0].quantity_quote, 100.0 * 943.0);
        assert_eq!(orderbook.bids[0].quantity_base, 100.0 * 943.0 / 39944.0);
    }
}
