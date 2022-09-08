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

mod l2_topk {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, extract_timestamp, parse_l2_topk, round};
    use crypto_msg_type::MessageType;

    #[test]
    fn spot() {
        let raw_msg = r#"{"time":1662630117,"channel":"spot.order_book","event":"update","result":{"t":1662630117190,"lastUpdateId":8093065166,"s":"BTC_USDT","bids":[["19201.39","0.0005"],["19201.18","0.0042"],["19199.93","0.0117"],["19199.65","0.0208"],["19199.39","0.0416"],["19197.84","0.1041"],["19195.78","0.0051"],["19195.77","0.2083"],["19195.61","0.2487"],["19195.12","0.03"],["19195","0.06"],["19194.84","0.09"],["19194.62","0.0104"],["19194.14","0.0301"],["19193.36","0.4168"],["19192.92","0.0544"],["19191.8","1.0078"],["19190.68","0.4168"],["19190.11","0.6821"],["19189.87","0.0155"]],"asks":[["19201.4","1.0963"],["19201.71","2.5184"],["19201.72","0.1301"],["19201.75","0.026"],["19202.38","0.2603"],["19203.3","0.1689"],["19203.72","0.5207"],["19204.03","0.03"],["19204.13","0.06"],["19204.58","0.09"],["19204.78","0.1409"],["19205.12","0.0103"],["19205.42","0.3999"],["19205.96","0.0001"],["19206.02","0.0155"],["19206.06","0.5206"],["19206.7","0.0023"],["19208.96","0.0002"],["19209.73","0.001802"],["19209.9","1.0078"]]}}"#;
        let orderbook = &parse_l2_topk(EXCHANGE_NAME, MarketType::Spot, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 20);
        assert_eq!(orderbook.bids.len(), 20);
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
            "BTC_USDT",
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );
        assert_eq!(
            1662630117190,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1662630117190);
        assert_eq!(orderbook.seq_id, Some(8093065166));
        assert_eq!(orderbook.prev_seq_id, None);

        assert_eq!(orderbook.bids[0].price, 19201.39);
        assert_eq!(orderbook.bids[0].quantity_base, 0.0005);
        assert_eq!(orderbook.bids[0].quantity_quote, 19201.39 * 0.0005);
        assert_eq!(orderbook.bids[0].quantity_contract, None);

        assert_eq!(orderbook.bids[19].price, 19189.87);
        assert_eq!(orderbook.bids[19].quantity_base, 0.0155);
        assert_eq!(orderbook.bids[19].quantity_quote, 19189.87 * 0.0155);
        assert_eq!(orderbook.bids[19].quantity_contract, None);

        assert_eq!(orderbook.asks[0].price, 19201.4);
        assert_eq!(orderbook.asks[0].quantity_base, 1.0963);
        assert_eq!(orderbook.asks[0].quantity_quote, 19201.4 * 1.0963);
        assert_eq!(orderbook.asks[0].quantity_contract, None);

        assert_eq!(orderbook.asks[19].price, 19209.9);
        assert_eq!(orderbook.asks[19].quantity_base, 1.0078);
        assert_eq!(orderbook.asks[19].quantity_quote, 19209.9 * 1.0078);
        assert_eq!(orderbook.asks[19].quantity_contract, None);
    }

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"time":1662631128,"channel":"futures.order_book","event":"all","result":{"t":1662631128855,"id":3779267315,"contract":"BTC_USD","asks":[{"p":"19162.4","s":12},{"p":"19162.5","s":12465},{"p":"19163.4","s":4},{"p":"19165.4","s":9600},{"p":"19167.3","s":3270},{"p":"19171.1","s":4},{"p":"19171.4","s":9600},{"p":"19172.3","s":4003},{"p":"19172.9","s":14505},{"p":"19173.4","s":298},{"p":"19178.2","s":20000},{"p":"19178.5","s":28984},{"p":"19179.2","s":42899},{"p":"19185.2","s":6965},{"p":"19190.9","s":83333},{"p":"19191.1","s":11932},{"p":"19192.4","s":58289},{"p":"19192.7","s":100},{"p":"19197","s":6965},{"p":"19204.1","s":60000}],"bids":[{"p":"19158.8","s":5503},{"p":"19155.6","s":5782},{"p":"19153.5","s":265},{"p":"19152.5","s":1439},{"p":"19151.6","s":3455},{"p":"19147.9","s":14505},{"p":"19146.3","s":72},{"p":"19145.6","s":5782},{"p":"19134.4","s":20000},{"p":"19133.3","s":28984},{"p":"19132.9","s":5782},{"p":"19127.1","s":83333},{"p":"19123.8","s":42899},{"p":"19122.8","s":5782},{"p":"19114.2","s":58289},{"p":"19113.2","s":5782},{"p":"19112","s":166667},{"p":"19109.3","s":60000},{"p":"19100","s":55},{"p":"19089","s":2500}]}}"#;
        let orderbook =
            &parse_l2_topk(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 20);
        assert_eq!(orderbook.bids.len(), 20);
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
            "BTC_USD",
            extract_symbol(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap()
        );
        assert_eq!(
            1662631128855,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1662631128855);
        assert_eq!(orderbook.seq_id, Some(3779267315));
        assert_eq!(orderbook.prev_seq_id, None);

        assert_eq!(orderbook.asks[0].price, 19162.4);
        assert_eq!(orderbook.asks[0].quantity_base, 12.0 / 19162.4);
        assert_eq!(orderbook.asks[0].quantity_quote, 12.0);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 12.0);

        assert_eq!(orderbook.asks[19].price, 19204.1);
        assert_eq!(orderbook.asks[19].quantity_base, 60000.0 / 19204.1);
        assert_eq!(orderbook.asks[19].quantity_quote, 60000.0);
        assert_eq!(orderbook.asks[19].quantity_contract.unwrap(), 60000.0);

        assert_eq!(orderbook.bids[0].price, 19158.8);
        assert_eq!(orderbook.bids[0].quantity_base, 5503.0 / 19158.8);
        assert_eq!(orderbook.bids[0].quantity_quote, 5503.0);
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 5503.0);

        assert_eq!(orderbook.bids[19].price, 19089.0);
        assert_eq!(orderbook.bids[19].quantity_base, 2500.0 / 19089.0);
        assert_eq!(orderbook.bids[19].quantity_quote, 2500.0);
        assert_eq!(orderbook.bids[19].quantity_contract.unwrap(), 2500.0);
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"time":1662631387,"channel":"futures.order_book","event":"all","result":{"t":1662631387399,"id":17819554462,"contract":"BTC_USDT","asks":[{"p":"19183.6","s":60530},{"p":"19183.8","s":1721},{"p":"19184.1","s":939},{"p":"19184.3","s":3440},{"p":"19185","s":94},{"p":"19185.1","s":1898},{"p":"19185.2","s":1},{"p":"19185.4","s":1926},{"p":"19185.5","s":1},{"p":"19185.6","s":539},{"p":"19185.8","s":635},{"p":"19185.9","s":2085},{"p":"19186","s":5643},{"p":"19186.1","s":2582},{"p":"19186.2","s":5991},{"p":"19186.3","s":4470},{"p":"19186.4","s":11218},{"p":"19186.5","s":1271},{"p":"19186.6","s":1},{"p":"19186.8","s":1639}],"bids":[{"p":"19183.5","s":64362},{"p":"19183.4","s":34555},{"p":"19183","s":3540},{"p":"19182.7","s":7287},{"p":"19182","s":1074},{"p":"19181.9","s":22519},{"p":"19181.8","s":3911},{"p":"19181.5","s":3752},{"p":"19181.4","s":96701},{"p":"19181.2","s":68053},{"p":"19181","s":72685},{"p":"19180.9","s":103403},{"p":"19180.7","s":75646},{"p":"19180.6","s":84712},{"p":"19180.5","s":81173},{"p":"19180.4","s":75646},{"p":"19180.3","s":57295},{"p":"19180.2","s":24472},{"p":"19180.1","s":4000},{"p":"19180","s":2086}]}}"#;
        let orderbook =
            &parse_l2_topk(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 20);
        assert_eq!(orderbook.bids.len(), 20);
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
            "BTC_USDT",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );
        assert_eq!(
            1662631387399,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1662631387399);
        assert_eq!(orderbook.seq_id, Some(17819554462));
        assert_eq!(orderbook.prev_seq_id, None);

        assert_eq!(orderbook.asks[0].price, 19183.6);
        assert_eq!(orderbook.asks[0].quantity_base, 6.053);
        assert_eq!(orderbook.asks[0].quantity_quote, round(19183.6 * 6.053));
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 60530.0);

        assert_eq!(orderbook.asks[19].price, 19186.8);
        assert_eq!(orderbook.asks[19].quantity_base, 0.1639);
        assert_eq!(orderbook.asks[19].quantity_quote, round(19186.8 * 0.1639));
        assert_eq!(orderbook.asks[19].quantity_contract.unwrap(), 1639.0);

        assert_eq!(orderbook.bids[0].price, 19183.5);
        assert_eq!(orderbook.bids[0].quantity_base, 6.4362);
        assert_eq!(orderbook.bids[0].quantity_quote, round(19183.5 * 6.4362));
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 64362.0);

        assert_eq!(orderbook.bids[19].price, 19180.0);
        assert_eq!(orderbook.bids[19].quantity_base, 0.2086);
        assert_eq!(orderbook.bids[19].quantity_quote, round(19180.0 * 0.2086));
        assert_eq!(orderbook.bids[19].quantity_contract.unwrap(), 2086.0);
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

        let bbo_msg = &parse_bbo(EXCHANGE_NAME, MarketType::Spot, raw_msg, None).unwrap()[0];

        assert_eq!(MessageType::BBO, bbo_msg.msg_type);
        assert_eq!("BTC_USDT", bbo_msg.symbol);
        assert_eq!(1654029559473, bbo_msg.timestamp);
        assert_eq!(None, bbo_msg.id);

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

        let bbo_msg = &parse_bbo(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg, None).unwrap()[0];

        assert_eq!(MessageType::BBO, bbo_msg.msg_type);
        assert_eq!("BTC_USD", bbo_msg.symbol);
        assert_eq!(1654029908840, bbo_msg.timestamp);
        assert_eq!(None, bbo_msg.id);

        assert_eq!(31654.0, bbo_msg.ask_price);
        assert_eq!(99.0 / 31654.0, bbo_msg.ask_quantity_base);
        assert_eq!(99.0, bbo_msg.ask_quantity_quote);
        assert_eq!(Some(99.0), bbo_msg.ask_quantity_contract);

        assert_eq!(31653.9, bbo_msg.bid_price);
        assert_eq!(19485.0 / 31653.9, bbo_msg.bid_quantity_base);
        assert_eq!(19485.0, bbo_msg.bid_quantity_quote);
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

        let bbo_msg = &parse_bbo(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg, None).unwrap()[0];

        assert_eq!(MessageType::BBO, bbo_msg.msg_type);
        assert_eq!("BTC_USDT", bbo_msg.symbol);
        assert_eq!(1654030293769, bbo_msg.timestamp);
        assert_eq!(None, bbo_msg.id);

        assert_eq!(31709.3, bbo_msg.ask_price);
        assert_eq!(5.6231, bbo_msg.ask_quantity_base);
        assert_eq!(31709.3 * 5.6231, bbo_msg.ask_quantity_quote);
        assert_eq!(Some(56231.0), bbo_msg.ask_quantity_contract);

        assert_eq!(31709.2, bbo_msg.bid_price);
        assert_eq!(11.9926, bbo_msg.bid_quantity_base);
        assert_eq!(31709.2 * 11.9926, bbo_msg.bid_quantity_quote);
        assert_eq!(Some(119926.0), bbo_msg.bid_quantity_contract);
    }
}

#[cfg(test)]
mod candlestick {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, extract_timestamp};

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
    fn spot_sample_2() {
        let raw_msg = r#"{"method": "ticker.update", "params": ["BTC_USDT", {"period": 86400, "open": "45366", "close": "44681", "high": "46433.56", "low": "44336.17", "last": "44681", "change": "-1.54", "quoteVolume": "780.195181207", "baseVolume": "35527977.73407791947245739827"}], "id": null}"#;

        assert_eq!(
            None,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
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
