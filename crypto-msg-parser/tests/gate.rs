mod utils;

const EXCHANGE_NAME: &str = "gate";

#[cfg(test)]
mod trade {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_message::TradeSide;
    use crypto_msg_parser::{extract_symbol, extract_timestamp, parse_trade, round};

    #[test]
    fn spot_20210916() {
        let raw_msg = r#"{"method": "trades.update", "params": ["BTC_USDT", [{"id": 643716793, "time": 1616327474.6243241, "price": "56173.28", "amount": "0.0037", "type": "sell"}]], "id": null}"#;
        let trades = &parse_trade(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap();

        assert_eq!(trades.len(), 1);
        let trade = &trades[0];

        crate::utils::check_trade_fields(
            EXCHANGE_NAME,
            MarketType::Spot,
            "BTC/USDT".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap(),
            trade,
            raw_msg,
        );
        assert_eq!(
            1616327474624,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(trades[0].quantity_base, 0.0037);
        assert_eq!(trades[0].quantity_quote, 0.0037 * 56173.28);
        assert_eq!(trades[0].quantity_contract, None);
        assert_eq!(trades[0].side, TradeSide::Sell);
    }

    #[test]
    fn spot() {
        let raw_msg = r#"{"time":1631824310,"channel":"spot.trades","event":"update","result":{"id":1638417041,"create_time":1631824310,"create_time_ms":"1631824310261.896","side":"buy","currency_pair":"BTC_USDT","amount":"0.00052","price":"47395.009"}}"#;
        let trades = &parse_trade(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap();

        assert_eq!(trades.len(), 1);
        let trade = &trades[0];

        crate::utils::check_trade_fields(
            EXCHANGE_NAME,
            MarketType::Spot,
            "BTC/USDT".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap(),
            trade,
            raw_msg,
        );
        assert_eq!(
            1631824310261,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(trades[0].price, 47395.009);
        assert_eq!(trades[0].quantity_base, 0.00052);
        assert_eq!(trades[0].quantity_quote, 0.00052 * 47395.009);
        assert_eq!(trades[0].quantity_contract, None);
        assert_eq!(trades[0].side, TradeSide::Buy);
    }

    #[test]
    fn inverse_future() {
        let raw_msg = r#"{"time":1653808101,"channel":"futures.trades","event":"update","error":null,"result":[{"size":-7,"id":376991,"create_time":1653808101,"price":"29009.9","contract":"BTC_USD_20220603"},{"size":-9,"id":376992,"create_time":1653808101,"price":"29008.7","contract":"BTC_USD_20220603"}]}"#;
        let trades = &parse_trade(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg).unwrap();

        assert_eq!(trades.len(), 2);
        let trade = &trades[0];

        crate::utils::check_trade_fields(
            EXCHANGE_NAME,
            MarketType::InverseFuture,
            "BTC/USD".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg).unwrap(),
            trade,
            raw_msg,
        );
        assert_eq!(
            1653808101000,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(trade.quantity_base, 7.0 / 29009.9);
        assert_eq!(trade.quantity_quote, 7.0);
        assert_eq!(trade.quantity_contract, Some(7.0));
        assert_eq!(trade.side, TradeSide::Sell);
    }

    #[test]
    fn linear_future() {
        let raw_msg = r#"{"time":1615253386,"channel":"futures.trades","event":"update","error":null,"result":[{"size":-19,"id":48081,"create_time":1615253386,"price":"53560.5","contract":"BTC_USDT_20210326"}]}"#;
        let trades = &parse_trade(EXCHANGE_NAME, MarketType::LinearFuture, raw_msg).unwrap();

        assert_eq!(trades.len(), 1);
        let trade = &trades[0];

        crate::utils::check_trade_fields(
            EXCHANGE_NAME,
            MarketType::LinearFuture,
            "BTC/USDT".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::LinearFuture, raw_msg).unwrap(),
            trade,
            raw_msg,
        );
        assert_eq!(
            1615253386000,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearFuture, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(trade.quantity_base, 19.0 * 0.0001);
        assert_eq!(trade.quantity_quote, 0.0019 * 53560.5);
        assert_eq!(trade.quantity_contract, Some(19.0));
        assert_eq!(trade.side, TradeSide::Sell);
    }

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"time":1616327545,"channel":"futures.trades","event":"update","error":null,"result":[{"size":7,"id":19910126,"create_time":1616327545,"create_time_ms":1616327545436,"price":"56155.2","contract":"BTC_USD"}]}"#;
        let trades = &parse_trade(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap();

        assert_eq!(trades.len(), 1);
        let trade = &trades[0];

        crate::utils::check_trade_fields(
            EXCHANGE_NAME,
            MarketType::InverseSwap,
            "BTC/USD".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap(),
            trade,
            raw_msg,
        );
        assert_eq!(
            1616327545436,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(trade.quantity_base, 7.0 / 56155.2);
        assert_eq!(trade.quantity_quote, 7.0);
        assert_eq!(trade.quantity_contract, Some(7.0));
        assert_eq!(trade.side, TradeSide::Buy);
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"time":1616327563,"channel":"futures.trades","event":"update","error":null,"result":[{"size":50,"id":15366793,"create_time":1616327563,"create_time_ms":1616327563918,"price":"56233.3","contract":"BTC_USDT"}]}"#;
        let trades = &parse_trade(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap();

        assert_eq!(trades.len(), 1);
        let trade = &trades[0];

        crate::utils::check_trade_fields(
            EXCHANGE_NAME,
            MarketType::LinearSwap,
            "BTC/USDT".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap(),
            trade,
            raw_msg,
        );
        assert_eq!(
            1616327563918,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(trade.quantity_base, 0.0001 * 50.0);
        assert_eq!(trade.quantity_quote, round(0.005 * 56233.3));
        assert_eq!(trade.quantity_contract, Some(50.0));
        assert_eq!(trade.side, TradeSide::Buy);
    }
}

#[cfg(test)]
mod l2_event {
    use super::EXCHANGE_NAME;
    use chrono::prelude::*;
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, extract_timestamp, parse_l2, round};
    use crypto_msg_type::MessageType;

    #[test]
    fn spot_snapshot_20200916() {
        let raw_msg = r#"{"method": "depth.update", "params": [true, {"asks": [["37483.21", "0.048"], ["37483.89", "0.0739"], ["37486.86", "0.1639"]], "bids": [["37483.19", "0.01"], ["37480.69", "0.0183"], ["37479.16", "0.0292"]], "id": 3166483561}, "BTC_USDT"], "id": null}"#;
        let received_at = Utc::now().timestamp_millis();
        let orderbook =
            &parse_l2(EXCHANGE_NAME, MarketType::Spot, raw_msg, Some(received_at)).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 3);
        assert_eq!(orderbook.bids.len(), 3);
        assert!(orderbook.snapshot);

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
            None,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );

        assert_eq!(orderbook.asks[0].price, 37483.21);
        assert_eq!(orderbook.asks[0].quantity_base, 0.048);
        assert_eq!(orderbook.asks[0].quantity_quote, 37483.21 * 0.048);

        assert_eq!(orderbook.asks[2].price, 37486.86);
        assert_eq!(orderbook.asks[2].quantity_base, 0.1639);
        assert_eq!(orderbook.asks[2].quantity_quote, 37486.86 * 0.1639);

        assert_eq!(orderbook.bids[0].price, 37483.19);
        assert_eq!(orderbook.bids[0].quantity_base, 0.01);
        assert_eq!(orderbook.bids[0].quantity_quote, 37483.19 * 0.01);

        assert_eq!(orderbook.bids[2].price, 37479.16);
        assert_eq!(orderbook.bids[2].quantity_base, 0.0292);
        assert_eq!(orderbook.bids[2].quantity_quote, 37479.16 * 0.0292);
    }

    #[test]
    fn spot_snapshot() {
        let raw_msg = r#"{"time":1631845776,"channel":"spot.order_book","event":"update","result":{"t":1631845775906,"lastUpdateId":4622752959,"s":"BTC_USDT","bids":[["47815.97","0.0608"],["47815.07","0.0367"],["47815.01","0.0001"]],"asks":[["47815.98","0.004"],["47815.99","0.00290642"],["47816","0.001"]]}}"#;
        let orderbook = &parse_l2(
            EXCHANGE_NAME,
            MarketType::Spot,
            raw_msg,
            Some(Utc::now().timestamp_millis()),
        )
        .unwrap()[0];

        assert_eq!(orderbook.asks.len(), 3);
        assert_eq!(orderbook.bids.len(), 3);
        assert!(orderbook.snapshot);

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
            1631845775906,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.asks[0].price, 47815.98);
        assert_eq!(orderbook.asks[0].quantity_base, 0.004);
        assert_eq!(orderbook.asks[0].quantity_quote, 47815.98 * 0.004);
        assert_eq!(orderbook.asks[0].quantity_contract, None);

        assert_eq!(orderbook.asks[2].price, 47816.0);
        assert_eq!(orderbook.asks[2].quantity_base, 0.001);
        assert_eq!(orderbook.asks[2].quantity_quote, 47816.0 * 0.001);
        assert_eq!(orderbook.asks[2].quantity_contract, None);

        assert_eq!(orderbook.bids[0].price, 47815.97);
        assert_eq!(orderbook.bids[0].quantity_base, 0.0608);
        assert_eq!(orderbook.bids[0].quantity_quote, 47815.97 * 0.0608);
        assert_eq!(orderbook.bids[0].quantity_contract, None);

        assert_eq!(orderbook.bids[2].price, 47815.01);
        assert_eq!(orderbook.bids[2].quantity_base, 0.0001);
        assert_eq!(orderbook.bids[2].quantity_quote, 47815.01 * 0.0001);
        assert_eq!(orderbook.bids[2].quantity_contract, None);
    }

    #[test]
    fn spot_update_20210916() {
        let raw_msg = r#"{"method": "depth.update", "params": [false, {"asks": [["37483.89", "0"]], "bids": [["37479.16", "0"], ["37478.79", "0.0554"]]}, "BTC_USDT"], "id": null}"#;
        let received_at = Utc::now().timestamp_millis();
        let orderbook =
            &parse_l2(EXCHANGE_NAME, MarketType::Spot, raw_msg, Some(received_at)).unwrap()[0];

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
            None,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );
        assert_eq!(received_at, orderbook.timestamp);

        assert_eq!(orderbook.asks[0].price, 37483.89);
        assert_eq!(orderbook.asks[0].quantity_base, 0.0);
        assert_eq!(orderbook.asks[0].quantity_quote, 0.0);

        assert_eq!(orderbook.bids[0].price, 37479.16);
        assert_eq!(orderbook.bids[0].quantity_base, 0.0);
        assert_eq!(orderbook.bids[0].quantity_quote, 0.0);

        assert_eq!(orderbook.bids[1].price, 37478.79);
        assert_eq!(orderbook.bids[1].quantity_base, 0.0554);
        assert_eq!(orderbook.bids[1].quantity_quote, 37478.79 * 0.0554);
    }

    #[test]
    fn spot_update() {
        let raw_msg = r#"{"time":1631836142,"channel":"spot.order_book_update","event":"update","result":{"t":1631836142325,"e":"depthUpdate","E":1631836142,"s":"BTC_USDT","U":4622074361,"u":4622074364,"b":[["47737.89","0.002"],["47741.35","0"]],"a":[["47813.04","0.0355"],["47978.86","0"]]}}"#;
        let orderbook = &parse_l2(
            EXCHANGE_NAME,
            MarketType::Spot,
            raw_msg,
            Some(Utc::now().timestamp_millis()),
        )
        .unwrap()[0];

        assert_eq!(orderbook.asks.len(), 2);
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
            1631836142325,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.seq_id, Some(4622074364));

        assert_eq!(orderbook.asks[0].price, 47813.04);
        assert_eq!(orderbook.asks[0].quantity_base, 0.0355);
        assert_eq!(orderbook.asks[0].quantity_quote, 0.0355 * 47813.04);
        assert_eq!(orderbook.asks[0].quantity_contract, None);

        assert_eq!(orderbook.asks[1].price, 47978.86);
        assert_eq!(orderbook.asks[1].quantity_base, 0.0);
        assert_eq!(orderbook.asks[1].quantity_quote, 0.0);
        assert_eq!(orderbook.asks[1].quantity_contract, None);

        assert_eq!(orderbook.bids[0].price, 47737.89);
        assert_eq!(orderbook.bids[0].quantity_base, 0.002);
        assert_eq!(orderbook.bids[0].quantity_quote, 0.002 * 47737.89);
        assert_eq!(orderbook.bids[0].quantity_contract, None);

        assert_eq!(orderbook.bids[1].price, 47741.35);
        assert_eq!(orderbook.bids[1].quantity_base, 0.0);
        assert_eq!(orderbook.bids[1].quantity_quote, 0.0);
        assert_eq!(orderbook.bids[1].quantity_contract, None);
    }

    #[test]
    fn inverse_swap_update() {
        let raw_msg = r#"{"id":null,"time":1632793098,"channel":"futures.order_book_update","event":"update","error":null,"result":{"t":1632793098358,"s":"BTC_USD","U":3136077080,"u":3136077083,"b":[],"a":[{"p":"42372.9","s":0},{"p":"42367.1","s":738}]}}"#;
        let orderbook =
            &parse_l2(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 2);
        assert_eq!(orderbook.bids.len(), 0);
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
            1632793098358,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1632793098358);
        assert_eq!(orderbook.seq_id, Some(3136077083));

        assert_eq!(orderbook.asks[0].price, 42372.9);
        assert_eq!(orderbook.asks[0].quantity_base, 0.0);
        assert_eq!(orderbook.asks[0].quantity_quote, 0.0);
        assert_eq!(orderbook.asks[0].quantity_contract, Some(0.0));
    }

    #[test]
    fn inverse_swap_snapshot() {
        let raw_msg = r#"{"id":null,"time":1622682306,"channel":"futures.order_book","event":"all","error":null,"result":{"t":1622682306315,"id":2861474582,"contract":"BTC_USD","asks":[{"p":"37481.3","s":7766},{"p":"37484.7","s":1775},{"p":"37485.1","s":2004}],"bids":[{"p":"37481.2","s":51735},{"p":"37480.2","s":9111},{"p":"37479.1","s":2004}]}}"#;
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
            1622682306315,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1622682306315);

        assert_eq!(orderbook.asks[0].price, 37481.3);
        assert_eq!(orderbook.asks[0].quantity_base, 7766.0 / 37481.3);
        assert_eq!(orderbook.asks[0].quantity_quote, 7766.0);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 7766.0);

        assert_eq!(orderbook.asks[2].price, 37485.1);
        assert_eq!(orderbook.asks[2].quantity_base, 2004.0 / 37485.1);
        assert_eq!(orderbook.asks[2].quantity_quote, 2004.0);
        assert_eq!(orderbook.asks[2].quantity_contract.unwrap(), 2004.0);

        assert_eq!(orderbook.bids[0].price, 37481.2);
        assert_eq!(orderbook.bids[0].quantity_base, 51735.0 / 37481.2);
        assert_eq!(orderbook.bids[0].quantity_quote, 51735.0);
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 51735.0);

        assert_eq!(orderbook.bids[2].price, 37479.1);
        assert_eq!(orderbook.bids[2].quantity_base, 2004.0 / 37479.1);
        assert_eq!(orderbook.bids[2].quantity_quote, 2004.0);
        assert_eq!(orderbook.bids[2].quantity_contract.unwrap(), 2004.0);
    }

    #[test]
    fn linear_swap_update() {
        let raw_msg = r#"{"id":null,"time":1632799979,"channel":"futures.order_book_update","event":"update","error":null,"result":{"t":1632799979523,"s":"BTC_USDT","U":8179159885,"u":8179159933,"b":[{"p":"42459.2","s":73982}],"a":[]}}"#;
        let orderbook = &parse_l2(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 0);
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
            1632799979523,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1632799979523);
        assert_eq!(orderbook.seq_id, Some(8179159933));

        assert_eq!(orderbook.bids[0].price, 42459.2);
        assert_eq!(orderbook.bids[0].quantity_base, 7.3982);
        assert_eq!(orderbook.bids[0].quantity_quote, round(42459.2 * 7.3982));
        assert_eq!(orderbook.bids[0].quantity_contract, Some(73982.0));
    }

    #[test]
    fn linear_swap_snapshot() {
        let raw_msg = r#"{"id":null,"time":1622689062,"channel":"futures.order_book","event":"all","error":null,"result":{"t":1622689062072,"id":4906611559,"contract":"BTC_USDT","asks":[{"p":"37396.5","s":22137},{"p":"37397.3","s":500},{"p":"37401.2","s":790}],"bids":[{"p":"37396.4","s":8553},{"p":"37393.9","s":525},{"p":"37393.6","s":500}]}}"#;
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
            1622689062072,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1622689062072);

        assert_eq!(orderbook.asks[0].price, 37396.5);
        assert_eq!(orderbook.asks[0].quantity_base, 2.2137);
        assert_eq!(orderbook.asks[0].quantity_quote, round(37396.5 * 2.2137));
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 22137.0);

        assert_eq!(orderbook.asks[2].price, 37401.2);
        assert_eq!(orderbook.asks[2].quantity_base, 0.079);
        assert_eq!(orderbook.asks[2].quantity_quote, round(37401.2 * 0.079));
        assert_eq!(orderbook.asks[2].quantity_contract.unwrap(), 790.0);

        assert_eq!(orderbook.bids[0].price, 37396.4);
        assert_eq!(orderbook.bids[0].quantity_base, 0.8553);
        assert_eq!(orderbook.bids[0].quantity_quote, round(37396.4 * 0.8553));
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 8553.0);

        assert_eq!(orderbook.bids[2].price, 37393.6);
        assert_eq!(orderbook.bids[2].quantity_base, 0.05);
        assert_eq!(orderbook.bids[2].quantity_quote, 37393.6 * 0.05);
        assert_eq!(orderbook.bids[2].quantity_contract.unwrap(), 500.0);
    }

    #[test]
    fn inverse_future_snapshot() {
        let raw_msg = r#"{"time":1653810275,"channel":"futures.order_book","event":"all","error":null,"result":{"t":1653810274815,"id":79619326,"contract":"BTC_USD_20220624","asks":[{"p":"28988.9","s":620},{"p":"28991.8","s":535},{"p":"28997.6","s":513}],"bids":[{"p":"28941.5","s":564},{"p":"28938.6","s":535},{"p":"28932.8","s":513}]}}"#;
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
            1653810274815,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1653810274815);

        assert_eq!(orderbook.asks[0].price, 28988.9);
        assert_eq!(orderbook.asks[0].quantity_base, 620.0 / 28988.9);
        assert_eq!(orderbook.asks[0].quantity_quote, 620.0);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 620.0);

        assert_eq!(orderbook.asks[2].price, 28997.6);
        assert_eq!(orderbook.asks[2].quantity_base, 513.0 / 28997.6);
        assert_eq!(orderbook.asks[2].quantity_quote, 513.0);
        assert_eq!(orderbook.asks[2].quantity_contract.unwrap(), 513.0);

        assert_eq!(orderbook.bids[0].price, 28941.5);
        assert_eq!(orderbook.bids[0].quantity_base, 564.0 / 28941.5);
        assert_eq!(orderbook.bids[0].quantity_quote, 564.0);
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 564.0);

        assert_eq!(orderbook.bids[2].price, 28932.8);
        assert_eq!(orderbook.bids[2].quantity_base, 513.0 / 28932.8);
        assert_eq!(orderbook.bids[2].quantity_quote, 513.0);
        assert_eq!(orderbook.bids[2].quantity_contract.unwrap(), 513.0);
    }

    #[test]
    fn linear_future_snapshot() {
        let raw_msg = r#"{"time":1622697760,"channel":"futures.order_book","event":"all","error":null,"result":{"contract":"BTC_USDT_20210625","asks":[{"p":"38624.6","s":500},{"p":"38708.3","s":500},{"p":"38821","s":2000}],"bids":[{"p":"38538","s":500},{"p":"38460","s":500},{"p":"38373","s":2000}]}}"#;
        let orderbook =
            &parse_l2(EXCHANGE_NAME, MarketType::LinearFuture, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 3);
        assert_eq!(orderbook.bids.len(), 3);
        assert!(orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            EXCHANGE_NAME,
            MarketType::LinearFuture,
            MessageType::L2Event,
            "BTC/USDT".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::LinearFuture, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );
        assert_eq!(
            1622697760000,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearFuture, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1622697760000);

        assert_eq!(orderbook.asks[0].price, 38624.6);
        assert_eq!(orderbook.asks[0].quantity_base, 0.05);
        assert_eq!(orderbook.asks[0].quantity_quote, 38624.6 * 0.05);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 500.0);

        assert_eq!(orderbook.asks[2].price, 38821.0);
        assert_eq!(orderbook.asks[2].quantity_base, 0.2);
        assert_eq!(orderbook.asks[2].quantity_quote, round(38821.0 * 0.2));
        assert_eq!(orderbook.asks[2].quantity_contract.unwrap(), 2000.0);

        assert_eq!(orderbook.bids[0].price, 38538.0);
        assert_eq!(orderbook.bids[0].quantity_base, 0.05);
        assert_eq!(orderbook.bids[0].quantity_quote, 38538.0 * 0.05);
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 500.0);

        assert_eq!(orderbook.bids[2].price, 38373.0);
        assert_eq!(orderbook.bids[2].quantity_base, 0.2);
        assert_eq!(orderbook.bids[2].quantity_quote, 38373.0 * 0.2);
        assert_eq!(orderbook.bids[2].quantity_contract.unwrap(), 2000.0);
    }

    #[test]
    fn linear_future_update() {
        let raw_msg = r#"{"time":1622769533,"channel":"futures.order_book","event":"update","error":null,"result":[{"p":"38258.9","s":-500,"c":"BTC_USDT_20210625","id":90062644},{"p":"38258.9","s":0,"c":"BTC_USDT_20210625","id":90062645},{"p":"38013","s":500,"c":"BTC_USDT_20210625","id":90062646}]}"#;
        let orderbook =
            &parse_l2(EXCHANGE_NAME, MarketType::LinearFuture, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 2);
        assert_eq!(orderbook.bids.len(), 1);
        assert!(!orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            EXCHANGE_NAME,
            MarketType::LinearFuture,
            MessageType::L2Event,
            "BTC/USDT".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::LinearFuture, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );
        assert_eq!(
            1622769533000,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearFuture, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1622769533000);

        assert_eq!(orderbook.asks[0].price, 38258.9);
        assert_eq!(orderbook.asks[0].quantity_base, 0.05);
        assert_eq!(orderbook.asks[0].quantity_quote, round(38258.9 * 0.05));
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 500.0);

        assert_eq!(orderbook.asks[1].price, 38258.9);
        assert_eq!(orderbook.asks[1].quantity_base, 0.0);
        assert_eq!(orderbook.asks[1].quantity_quote, 0.0);
        assert_eq!(orderbook.asks[1].quantity_contract.unwrap(), 0.0);

        assert_eq!(orderbook.bids[0].price, 38013.0);
        assert_eq!(orderbook.bids[0].quantity_base, 0.05);
        assert_eq!(orderbook.bids[0].quantity_quote, 38013.0 * 0.05);
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 500.0);
    }
}

#[cfg(test)]
mod bbo {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, extract_timestamp, parse_bbo, round};
    use crypto_msg_type::MessageType;

    #[test]
    fn spot() {
        let raw_msg = r#"{"time":1654029559,"channel":"spot.book_ticker","event":"update","result":{"t":1654029559473,"u":6765708346,"s":"BTC_USDT","b":"31738.93","B":"2.3039","a":"31738.94","A":"0.335"}}"#;

        assert_eq!(
            1654029559473,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "BTC_USDT",
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );
        let received_at = 1654029559473;
        let bbo_msg =
            parse_bbo(EXCHANGE_NAME, MarketType::Spot, raw_msg, Some(received_at)).unwrap();

        assert_eq!(MessageType::BBO, bbo_msg.msg_type);
        assert_eq!("BTC_USDT", bbo_msg.symbol);
        assert_eq!(received_at, bbo_msg.timestamp);
        assert_eq!(Some(6765708346), bbo_msg.id);

        assert_eq!(31738.94, bbo_msg.ask_price);
        assert_eq!(0.335, bbo_msg.ask_quantity_base);
        assert_eq!(31738.94 * 0.335, bbo_msg.ask_quantity_quote);
        assert_eq!(None, bbo_msg.ask_quantity_contract);

        assert_eq!(31738.93, bbo_msg.bid_price);
        assert_eq!(2.3039, bbo_msg.bid_quantity_base);
        assert_eq!(31738.93 * 2.3039, bbo_msg.bid_quantity_quote);
        assert_eq!(None, bbo_msg.bid_quantity_contract);
    }

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"time":1654029908,"channel":"futures.book_ticker","event":"update","result":{"t":1654029908840,"u":3613445820,"s":"BTC_USD","b":"31653.9","B":19485,"a":"31654","A":99}}"#;

        assert_eq!(
            1654029908840,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "BTC_USD",
            extract_symbol(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap()
        );
        
        let bbo_msg = parse_bbo(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg, None).unwrap();

        assert_eq!(MessageType::BBO, bbo_msg.msg_type);
        assert_eq!("BTC_USD", bbo_msg.symbol);
        assert_eq!(1654029908840, bbo_msg.timestamp);
        assert_eq!(Some(3613445820), bbo_msg.id);

        assert_eq!(31654.0, bbo_msg.ask_price);
        assert_eq!(99.0 * 1.0 / 31654.0, bbo_msg.ask_quantity_base);
        assert_eq!(99.0 * 1.0, bbo_msg.ask_quantity_quote);
        assert_eq!(Some(99.0), bbo_msg.ask_quantity_contract);

        assert_eq!(31653.9, bbo_msg.bid_price);
        assert_eq!(19485.0 *1.0 / 31653.9, bbo_msg.bid_quantity_base);
        assert_eq!(19485.0 * 1.0, bbo_msg.bid_quantity_quote);
        assert_eq!(Some(19485.0), bbo_msg.bid_quantity_contract);
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"id":null,"time":1654030293,"channel":"futures.book_ticker","event":"update","error":null,"result":{"t":1654030293769,"u":13980118150,"s":"BTC_USDT","b":"31709.2","B":119926,"a":"31709.3","A":56231}}"#;

        assert_eq!(
            1654030293769,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "BTC_USDT",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );

        let bbo_msg = parse_bbo(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg, None).unwrap();

        assert_eq!(MessageType::BBO, bbo_msg.msg_type);
        assert_eq!("BTC_USDT", bbo_msg.symbol);
        assert_eq!(1654030293769, bbo_msg.timestamp);
        assert_eq!(Some(13980118150), bbo_msg.id);

        assert_eq!(31709.3, bbo_msg.ask_price);
        assert_eq!(round(56231.0 * 0.0001), bbo_msg.ask_quantity_base);
        assert_eq!(round(56231.0 * 0.0001 * 31709.3), bbo_msg.ask_quantity_quote);
        assert_eq!(Some(56231.0), bbo_msg.ask_quantity_contract);

        assert_eq!(31709.2, bbo_msg.bid_price);
        assert_eq!(round(119926.0 * 0.0001), bbo_msg.bid_quantity_base);
        assert_eq!(round(119926.0 * 0.0001 * 31709.2), bbo_msg.bid_quantity_quote);
        assert_eq!(Some(119926.0), bbo_msg.bid_quantity_contract);
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
        let raw_msg = r#"{"time":1654080052,"channel":"spot.candlesticks","event":"update","result":{"t":"1654080050","v":"0","c":"31555.75","h":"31555.75","l":"31555.75","o":"31555.75","n":"10s_BTC_USDT","a":"0"}}"#;

        assert_eq!(
            1654080052000,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "BTC_USDT",
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );

        let data = parse_candlestick(EXCHANGE_NAME, MarketType::Spot, raw_msg, MessageType::L2TopK).unwrap();

        assert_eq!(1654080050, data.timestamp);
        assert_eq!("10s", data.period);
    }

    #[test]
    fn inverse_future() {
        let raw_msg = r#"{"time":1654080481,"channel":"futures.candlesticks","event":"update","error":null,"result":[{"t":1654080470,"v":12,"c":"31551.4","h":"31551.7","l":"31551.4","o":"31551.7","n":"10s_BTC_USD_20220624"},{"t":1654080480,"v":0,"c":"31551.4","h":"31551.4","l":"31551.4","o":"31551.4","n":"10s_BTC_USD_20220624"}]}"#;

        assert_eq!(
            1654080481000,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "BTC_USD_20220624",
            extract_symbol(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap()
        );
    }

    #[test]
    fn linear_future() {
        let raw_msg = r#"{"time":1654080831,"channel":"futures.candlesticks","event":"update","error":null,"result":[{"t":1654080810,"v":0,"c":"31638.9","h":"31638.9","l":"31638.9","o":"31638.9","n":"10s_BTC_USDT_20220624"},{"t":1654080820,"v":5,"c":"31640.3","h":"31640.3","l":"31640.3","o":"31640.3","n":"10s_BTC_USDT_20220624"},{"t":1654080830,"v":0,"c":"31640.3","h":"31640.3","l":"31640.3","o":"31640.3","n":"10s_BTC_USDT_20220624"}]}"#;

        assert_eq!(
            1654080831000,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "BTC_USDT_20220624",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );
    }

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"time":1654080889,"channel":"futures.candlesticks","event":"update","result":[{"t":1654080880,"v":0,"c":"31509.2","h":"31509.2","l":"31509.2","o":"31509.2","a":"0","n":"10s_BTC_USD"}]}"#;

        assert_eq!(
            1654080889000,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "BTC_USD",
            extract_symbol(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap()
        );

        let data = parse_candlestick(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg, MessageType::L2TopK).unwrap();

        assert_eq!(1654080880, data.timestamp);
        assert_eq!("10s", data.period);
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"id":null,"time":1654080940,"channel":"futures.candlesticks","event":"update","error":null,"result":[{"t":1654080930,"v":923,"c":"31533.1","h":"31533.1","l":"31531.5","o":"31531.5","n":"10s_BTC_USDT"},{"t":1654080940,"v":0,"c":"31533.1","h":"31533.1","l":"31533.1","o":"31533.1","n":"10s_BTC_USDT"}]}"#;

        assert_eq!(
            1654080940000,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "BTC_USDT",
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
    fn spot() {
        let raw_msg = r#"{"time":1654161931,"channel":"spot.tickers","event":"update","result":{"currency_pair":"BTC_USDT","last":"29968.31","lowest_ask":"29968.31","highest_bid":"29968.3","change_percentage":"-5.3731","base_volume":"10676.32785905","quote_volume":"324419781.600232","high_24h":"32399.99","low_24h":"29324.36"}}"#;

        assert_eq!(
            1654161931000,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "BTC_USDT",
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );
    }

    #[test]
    fn inverse_future() {
        let raw_msg = r#"{"time":1654162230,"channel":"futures.tickers","event":"update","error":null,"result":[{"total_size":"9999","volume_24h_quote":"265260","volume_24h_settle":"8","change_percentage":"-5.42","last":"29954.9","mark_price":"29955.64","volume_24h_base":"8","contract":"BTC_USD_20220624","volume_24h":"265260","settle_price":"0","basis_value":"-19.5","basis_rate":"-0.010791","high_24h":"31884.4","low_24h":"29336.9","index_price":"29975.14","quanto_base_rate":""}]}"#;

        assert_eq!(
            1654162230000,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "BTC_USD_20220624",
            extract_symbol(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap()
        );
    }

    #[test]
    fn linear_future() {
        let raw_msg = r#"{"time":1654162642,"channel":"futures.tickers","event":"update","error":null,"result":[{"total_size":"10037","volume_24h_quote":"83155","volume_24h_settle":"83155","change_percentage":"-5.68","last":"29960.5","mark_price":"29953.01","volume_24h_base":"2","contract":"BTC_USDT_20220624","volume_24h":"27755","settle_price":"0","basis_value":"27.12","basis_rate":"0.015036","high_24h":"31971.7","low_24h":"21380.4","index_price":"29925.89","quanto_base_rate":""}]}"#;

        assert_eq!(
            1654162642000,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "BTC_USDT_20220624",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );
    }

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"time":1654162687,"channel":"futures.tickers","event":"update","result":[{"contract":"BTC_USD","last":"29860.8","change_percentage":"-5.6438","total_size":"31659115","volume_24h":"15542254","volume_24h_base":"0","volume_24h_quote":"15542254","volume_24h_settle":"0.0000000000000006","mark_price":"29902.44","funding_rate":"0.0001","funding_rate_indicative":"-0.000292","index_price":"29900.05","quanto_base_rate":"","low_24h":"29259.1","high_24h":"31856.2","volume_24_usd":"15542254","volume_24_btc":"518.8480439694998983"}]}"#;

        assert_eq!(
            1654162687000,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "BTC_USD",
            extract_symbol(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap()
        );
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"id":null,"time":1654162715,"channel":"futures.tickers","event":"update","error":null,"result":[{"contract":"BTC_USDT","last":"29885.9","change_percentage":"-5.61","funding_rate":"-0.00004","mark_price":"29908","index_price":"29908.95","total_size":"754413619","volume_24h":"337595135","quanto_base_rate":"","low_24h":"29280","high_24h":"31880","funding_rate_indicative":"0.000056","volume_24h_quote":"1008933444","volume_24h_settle":"1008933444","volume_24h_base":"33759"}]}"#;

        assert_eq!(
            1654162715000,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "BTC_USDT",
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
        let raw_msg = r#"{"current":1654249533113,"update":1654249526591,"asks":[["30168.33","0.1824"],["30177.13","0.18"],["30178.62","0.2495"],["30178.63","1.7315"],["30179.94","0.0158"]],"bids":[["30168.32","0.5748"],["30165.14","0.0158"],["30164.8","0.035"],["30163.13","0.0023"],["30162.67","0.1252"]]}"#;

        assert_eq!(
            1654249533113,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "NONE",
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );
    }

    #[test]
    fn inverse_future() {
        let raw_msg = r#"{"current":1654249503.599,"asks":[{"s":564,"p":"30200.9"},{"s":535,"p":"30203.9"},{"s":564,"p":"30210"},{"s":497,"p":"30219"},{"s":487,"p":"30231.1"}],"bids":[{"s":564,"p":"30166.6"},{"s":535,"p":"30163.6"},{"s":513,"p":"30157.6"},{"s":546,"p":"30148.5"},{"s":487,"p":"30136.5"}],"update":1654249503.437}"#;

        assert_eq!(
            1654249503599,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "NONE",
            extract_symbol(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap()
        );
    }

    #[test]
    fn linear_future() {
        let raw_msg = r#"{"current":1654251300.95,"asks":[{"s":185,"p":"30199.7"},{"s":176,"p":"30202.7"},{"s":167,"p":"30208.8"},{"s":177,"p":"30217.8"},{"s":158,"p":"30229.9"}],"bids":[{"s":185,"p":"30174.5"},{"s":176,"p":"30171.5"},{"s":167,"p":"30165.4"},{"s":161,"p":"30156.4"},{"s":173,"p":"30144.3"}],"update":1654251300.797}"#;

        assert_eq!(
            1654251300950,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "NONE",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );
    }

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"current":1654251302.768,"asks":[{"s":475,"p":"30079.1"},{"s":4000,"p":"30079.2"},{"s":2408,"p":"30079.3"},{"s":10558,"p":"30079.6"},{"s":10,"p":"30090.8"}],"bids":[{"s":2,"p":"30061.6"},{"s":3,"p":"30061.5"},{"s":100,"p":"30056"},{"s":8036,"p":"30050.1"},{"s":500,"p":"30050"}],"update":1654251302.754}"#;

        assert_eq!(
            1654251302768,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "NONE",
            extract_symbol(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap()
        );
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"current":1654251438.92,"asks":[{"s":75703,"p":"30144.6"},{"s":30094,"p":"30144.7"},{"s":1750,"p":"30146.3"},{"s":1991,"p":"30146.4"},{"s":1658,"p":"30146.8"}],"bids":[{"s":324289,"p":"30144.5"},{"s":1369,"p":"30144"},{"s":1399,"p":"30143.9"},{"s":1376,"p":"30143.8"},{"s":1825,"p":"30143.4"}],"update":1654251438.902}"#;

        assert_eq!(
            1654251438920,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "NONE",
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
    fn inverse_swap() {
        let raw_msg = r#"[{"long_liq_size":0,"short_liq_size":0,"short_liq_usd":0,"lsr_account":1.8026315789474,"mark_price":29710.95,"top_lsr_size":0.93704798407213,"time":1654335000,"short_liq_amount":0,"long_liq_amount":0,"open_interest_usd":31828902,"top_lsr_account":1.5,"open_interest":31828902,"long_liq_usd":0,"lsr_taker":0.045587162654996},{"long_liq_size":0,"short_liq_size":0,"short_liq_usd":0,"lsr_account":1.8026315789474,"mark_price":29681.26,"top_lsr_size":0.93577945664931,"time":1654335300,"short_liq_amount":0,"long_liq_amount":0,"open_interest_usd":31883292,"top_lsr_account":1.5,"open_interest":31883292,"long_liq_usd":0,"lsr_taker":0.28336287667764}]"#;

        assert_eq!(
            None,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap()
        );
        assert_eq!(
            "NONE",
            extract_symbol(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap()
        );
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"[{"long_liq_size":0,"short_liq_size":0,"short_liq_usd":0,"lsr_account":1.7797816801139,"mark_price":29692.91,"top_lsr_size":0.91006677837509,"time":1654335300,"short_liq_amount":0,"long_liq_amount":0,"open_interest_usd":2226295750.1279,"top_lsr_account":1.1739130434783,"open_interest":749773515,"long_liq_usd":0,"lsr_taker":0.073035332215347},{"long_liq_size":0,"short_liq_size":0,"short_liq_usd":0,"lsr_account":1.7864954826438,"mark_price":29684.65,"top_lsr_size":0.90727688459152,"time":1654335600,"short_liq_amount":0,"long_liq_amount":0,"open_interest_usd":2225528156.4408,"top_lsr_account":1.1739130434783,"open_interest":749723563,"long_liq_usd":0,"lsr_taker":0.66780130171317}]"#;

        assert_eq!(
            None,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );
        assert_eq!(
            "NONE",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );
    }
}
