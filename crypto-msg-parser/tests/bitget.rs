mod utils;

const EXCHANGE_NAME: &str = "bitget";

#[cfg(test)]
mod trade {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_message::TradeSide;
    use crypto_msg_parser::{extract_symbol, extract_timestamp, parse_trade};

    #[test]
    fn spot() {
        let raw_msg = r#"{"action":"update","arg":{"instType":"sp","channel":"trade","instId":"BTCUSDT"},"data":[["1653873778747","29443.24","0.4134","buy"]]}"#;
        let trade = &parse_trade(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()[0];

        crate::utils::check_trade_fields(
            EXCHANGE_NAME,
            MarketType::Spot,
            "BTC/USDT".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap(),
            trade,
            raw_msg,
        );
        assert_eq!(
            1653873778747,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(trade.timestamp, 1653873778747);
        assert_eq!(trade.price, 29443.24);
        assert_eq!(trade.quantity_base, 0.4134);
        assert_eq!(trade.quantity_quote, 29443.24 * 0.4134);
        assert_eq!(trade.quantity_contract, None);
        assert_eq!(trade.side, TradeSide::Buy);
    }

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"action":"update","arg":{"instType":"mc","channel":"trade","instId":"BTCUSD"},"data":[["1653881896935","30285","0.024","buy"]]}"#;
        let trade = &parse_trade(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap()[0];

        crate::utils::check_trade_fields(
            EXCHANGE_NAME,
            MarketType::InverseSwap,
            "BTC/USD".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap(),
            trade,
            raw_msg,
        );
        assert_eq!(
            1653881896935,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(trade.timestamp, 1653881896935);
        assert_eq!(trade.price, 30285.0);
        assert_eq!(trade.quantity_base, 0.024);
        assert_eq!(trade.quantity_quote, 30285.0 * 0.024);
        assert_eq!(trade.quantity_contract, Some(0.024));
        assert_eq!(trade.side, TradeSide::Buy);
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"action":"update","arg":{"instType":"mc","channel":"trade","instId":"BTCUSDT"},"data":[["1653882567817","30322.5","1.117","buy"],["1653882567817","30322","1.566","buy"]]}"#;
        let trades = &parse_trade(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap();
        assert_eq!(2, trades.len());
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
            1653882567817,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(trade.timestamp, 1653882567817);
        assert_eq!(trade.price, 30322.5);
        assert_eq!(trade.quantity_base, 1.117);
        assert_eq!(trade.quantity_quote, 30322.5 * 1.117);
        assert_eq!(trade.quantity_contract, Some(1.117));
        assert_eq!(trade.side, TradeSide::Buy);
    }
}

#[cfg(test)]
mod l2_event {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, extract_timestamp, parse_l2};
    use crypto_msg_type::MessageType;

    #[test]
    fn spot() {
        let raw_msg = r#"{"action":"update","arg":{"instType":"sp","channel":"books","instId":"BTCUSDT"},"data":[{"asks":[["30266.73","0.0109"],["30266.77","0.0117"],["30266.94","2.5135"]],"bids":[["30266.57","0.0119"],["30266.53","0.0130"],["30265.49","0.0140"] ],"checksum":1732241839,"ts":"1653885248245"}]}"#;
        let orderbook = &parse_l2(EXCHANGE_NAME, MarketType::Spot, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 3);
        assert_eq!(orderbook.bids.len(), 3);
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
            1653885248245,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1653885248245);
        assert_eq!(orderbook.seq_id, None);
        assert_eq!(orderbook.prev_seq_id, None);

        assert_eq!(orderbook.bids[0].price, 30266.57);
        assert_eq!(orderbook.bids[0].quantity_base, 0.0119);
        assert_eq!(orderbook.bids[0].quantity_quote, 30266.57 * 0.0119);
        assert_eq!(orderbook.bids[0].quantity_contract, None);

        assert_eq!(orderbook.bids[2].price, 30265.49);
        assert_eq!(orderbook.bids[2].quantity_base, 0.0140);
        assert_eq!(orderbook.bids[2].quantity_quote, 30265.49 * 0.0140);
        assert_eq!(orderbook.bids[0].quantity_contract, None);

        assert_eq!(orderbook.asks[0].price, 30266.73);
        assert_eq!(orderbook.asks[0].quantity_base, 0.0109);
        assert_eq!(orderbook.asks[0].quantity_quote, 30266.73 * 0.0109);
        assert_eq!(orderbook.asks[0].quantity_contract, None);

        assert_eq!(orderbook.asks[2].price, 30266.94);
        assert_eq!(orderbook.asks[2].quantity_base, 2.5135);
        assert_eq!(orderbook.asks[2].quantity_quote, 30266.94 * 2.5135);
        assert_eq!(orderbook.asks[2].quantity_contract, None);
    }

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"action":"update","arg":{"instType":"mc","channel":"books","instId":"BTCUSD"},"data":[{"asks":[["30693.5","0.073"],["30694.0","0.064"],["30695.0","18.601"]],"bids":[["30678.0","12.693"],["30675.5","0.091"],["30674.0","22.504"]],"checksum":1033568482,"ts":"1653935348839"}]}"#;
        let orderbook =
            &parse_l2(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 3);
        assert_eq!(orderbook.bids.len(), 3);
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
            1653935348839,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1653935348839);
        assert_eq!(orderbook.seq_id, None);
        assert_eq!(orderbook.prev_seq_id, None);

        assert_eq!(orderbook.bids[0].price, 30678.0);
        assert_eq!(orderbook.bids[0].quantity_base, 12.693);
        assert_eq!(orderbook.bids[0].quantity_quote, 30678.0 * 12.693);
        assert_eq!(orderbook.bids[0].quantity_contract, Some(12.693));

        assert_eq!(orderbook.bids[2].price, 30674.0);
        assert_eq!(orderbook.bids[2].quantity_base, 22.504);
        assert_eq!(orderbook.bids[2].quantity_quote, 30674.0 * 22.504);
        assert_eq!(orderbook.bids[2].quantity_contract, Some(22.504));

        assert_eq!(orderbook.asks[0].price, 30693.5);
        assert_eq!(orderbook.asks[0].quantity_base, 0.073);
        assert_eq!(orderbook.asks[0].quantity_quote, 30693.5 * 0.073);
        assert_eq!(orderbook.asks[0].quantity_contract, Some(0.073));

        assert_eq!(orderbook.asks[2].price, 30695.0);
        assert_eq!(orderbook.asks[2].quantity_base, 18.601);
        assert_eq!(orderbook.asks[2].quantity_quote, 30695.0 * 18.601);
        assert_eq!(orderbook.asks[2].quantity_contract, Some(18.601));
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"action":"update","arg":{"instType":"mc","channel":"books","instId":"BTCUSDT"},"data":[{"asks":[["30677.5","17.098"],["30678.0","62.033"],["30679.0","5.129"]],"bids":[["30673.5","5.264"],["30673.0","18.938"],["30672.5","10.378"]],"checksum":-1093370704,"ts":"1653935972126"}]}"#;
        let orderbook = &parse_l2(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 3);
        assert_eq!(orderbook.bids.len(), 3);
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
            1653935972126,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1653935972126);
        assert_eq!(orderbook.seq_id, None);
        assert_eq!(orderbook.prev_seq_id, None);

        assert_eq!(orderbook.bids[0].price, 30673.5);
        assert_eq!(orderbook.bids[0].quantity_base, 5.264);
        assert_eq!(orderbook.bids[0].quantity_quote, 30673.5 * 5.264);
        assert_eq!(orderbook.bids[0].quantity_contract, Some(5.264));

        assert_eq!(orderbook.bids[2].price, 30672.5);
        assert_eq!(orderbook.bids[2].quantity_base, 10.378);
        assert_eq!(orderbook.bids[2].quantity_quote, 30672.5 * 10.378);
        assert_eq!(orderbook.bids[2].quantity_contract, Some(10.378));

        assert_eq!(orderbook.asks[0].price, 30677.5);
        assert_eq!(orderbook.asks[0].quantity_base, 17.098);
        assert_eq!(orderbook.asks[0].quantity_quote, 30677.5 * 17.098);
        assert_eq!(orderbook.asks[0].quantity_contract, Some(17.098));

        assert_eq!(orderbook.asks[2].price, 30679.0);
        assert_eq!(orderbook.asks[2].quantity_base, 5.129);
        assert_eq!(orderbook.asks[2].quantity_quote, 30679.0 * 5.129);
        assert_eq!(orderbook.asks[2].quantity_contract, Some(5.129));
    }
}

#[cfg(test)]
mod l2_topk {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, extract_timestamp, parse_l2_topk};
    use crypto_msg_type::MessageType;

    #[test]
    fn spot() {
        let raw_msg = r#"{"action":"snapshot","arg":{"instType":"sp","channel":"books5","instId":"BTCUSDT"},"data":[{"asks":[["30682.29","0.0119"],["30682.33","0.0127"],["30682.37","0.0213"],["30682.41","0.0560"],["30682.45","0.1474"]],"bids":[["30682.15","0.0122"],["30682.11","0.0132"],["30682.07","0.0114"],["30682.03","0.0122"],["30681.99","0.0118"]],"ts":"1653936946292"}]}"#;
        let orderbook = &parse_l2_topk(EXCHANGE_NAME, MarketType::Spot, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 5);
        assert_eq!(orderbook.bids.len(), 5);
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
            1653936946292,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1653936946292);
        assert_eq!(orderbook.seq_id, None);
        assert_eq!(orderbook.prev_seq_id, None);

        assert_eq!(orderbook.bids[0].price, 30682.15);
        assert_eq!(orderbook.bids[0].quantity_base, 0.0122);
        assert_eq!(orderbook.bids[0].quantity_quote, 30682.15 * 0.0122);
        assert_eq!(orderbook.bids[0].quantity_contract, None);

        assert_eq!(orderbook.bids[4].price, 30681.99);
        assert_eq!(orderbook.bids[4].quantity_base, 0.0118);
        assert_eq!(orderbook.bids[4].quantity_quote, 30681.99 * 0.0118);
        assert_eq!(orderbook.bids[4].quantity_contract, None);

        assert_eq!(orderbook.asks[0].price, 30682.29);
        assert_eq!(orderbook.asks[0].quantity_base, 0.0119);
        assert_eq!(orderbook.asks[0].quantity_quote, 30682.29 * 0.0119);
        assert_eq!(orderbook.asks[0].quantity_contract, None);

        assert_eq!(orderbook.asks[4].price, 30682.45);
        assert_eq!(orderbook.asks[4].quantity_base, 0.1474);
        assert_eq!(orderbook.asks[4].quantity_quote, 30682.45 * 0.1474);
        assert_eq!(orderbook.asks[4].quantity_contract, None);
    }

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"action":"snapshot","arg":{"instType":"mc","channel":"books5","instId":"BTCUSD"},"data":[{"asks":[["30669.0","0.763"],["30669.5","3.036"],["30670.0","0.103"],["30670.5","1.955"],["30671.5","9.537"]],"bids":[["30667.5","0.093"],["30667.0","25.104"],["30666.5","20.913"],["30666.0","20.223"],["30665.5","0.695"]],"ts":"1653937135034"}]}"#;
        let orderbook =
            &parse_l2_topk(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 5);
        assert_eq!(orderbook.bids.len(), 5);
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
            1653937135034,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1653937135034);
        assert_eq!(orderbook.seq_id, None);
        assert_eq!(orderbook.prev_seq_id, None);

        assert_eq!(orderbook.bids[0].price, 30667.5);
        assert_eq!(orderbook.bids[0].quantity_base, 0.093);
        assert_eq!(orderbook.bids[0].quantity_quote, 30667.5 * 0.093);
        assert_eq!(orderbook.bids[0].quantity_contract, Some(0.093));

        assert_eq!(orderbook.bids[4].price, 30665.5);
        assert_eq!(orderbook.bids[4].quantity_base, 0.695);
        assert_eq!(orderbook.bids[4].quantity_quote, 30665.5 * 0.695);
        assert_eq!(orderbook.bids[4].quantity_contract, Some(0.695));

        assert_eq!(orderbook.asks[0].price, 30669.0);
        assert_eq!(orderbook.asks[0].quantity_base, 0.763);
        assert_eq!(orderbook.asks[0].quantity_quote, 30669.0 * 0.763);
        assert_eq!(orderbook.asks[0].quantity_contract, Some(0.763));

        assert_eq!(orderbook.asks[4].price, 30671.5);
        assert_eq!(orderbook.asks[4].quantity_base, 9.537);
        assert_eq!(orderbook.asks[4].quantity_quote, 30671.5 * 9.537);
        assert_eq!(orderbook.asks[4].quantity_contract, Some(9.537));
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"action":"snapshot","arg":{"instType":"mc","channel":"books5","instId":"BTCUSDT"},"data":[{"asks":[["30678.0","0.500"],["30679.0","56.116"],["30679.5","7.024"],["30680.0","2.916"],["30680.5","3.098"]],"bids":[["30677.5","0.953"],["30677.0","4.152"],["30676.5","2.030"],["30676.0","24.110"],["30675.5","44.509"]],"ts":"1653937451315"}]}"#;
        let orderbook =
            &parse_l2_topk(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 5);
        assert_eq!(orderbook.bids.len(), 5);
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
            1653937451315,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1653937451315);
        assert_eq!(orderbook.seq_id, None);
        assert_eq!(orderbook.prev_seq_id, None);

        assert_eq!(orderbook.bids[0].price, 30677.5);
        assert_eq!(orderbook.bids[0].quantity_base, 0.953);
        assert_eq!(orderbook.bids[0].quantity_quote, 30677.5 * 0.953);
        assert_eq!(orderbook.bids[0].quantity_contract, Some(0.953));

        assert_eq!(orderbook.bids[4].price, 30675.5);
        assert_eq!(orderbook.bids[4].quantity_base, 44.509);
        assert_eq!(orderbook.bids[4].quantity_quote, 30675.5 * 44.509);
        assert_eq!(orderbook.bids[4].quantity_contract, Some(44.509));

        assert_eq!(orderbook.asks[0].price, 30678.0);
        assert_eq!(orderbook.asks[0].quantity_base, 0.500);
        assert_eq!(orderbook.asks[0].quantity_quote, 30678.0 * 0.500);
        assert_eq!(orderbook.asks[0].quantity_contract, Some(0.500));

        assert_eq!(orderbook.asks[4].price, 30680.5);
        assert_eq!(orderbook.asks[4].quantity_base, 3.098);
        assert_eq!(orderbook.asks[4].quantity_quote, 30680.5 * 3.098);
        assert_eq!(orderbook.asks[4].quantity_contract, Some(3.098));
    }
}

#[cfg(test)]
mod funding_rate {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_msg_parser::parse_funding_rate;

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"data":[{"funding_rate":"0.000258514264","funding_time":"1617346800000","instrument_id":"btcusd"}],"table":"swap/funding_rate"}"#;
        let funding_rates =
            &parse_funding_rate(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg, None).unwrap();

        assert_eq!(funding_rates.len(), 1);

        for rate in funding_rates.iter() {
            crate::utils::check_funding_rate_fields(
                EXCHANGE_NAME,
                MarketType::InverseSwap,
                rate,
                raw_msg,
            );
        }

        assert_eq!(funding_rates[0].pair, "BTC/USD".to_string());
        assert_eq!(funding_rates[0].funding_rate, 0.000258514264);
        assert_eq!(funding_rates[0].funding_time, 1617346800000);
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"data":[{"funding_rate":"0.000106539854","funding_time":"1617346800000","instrument_id":"cmt_btcusdt"}],"table":"swap/funding_rate"}"#;
        let funding_rates =
            &parse_funding_rate(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg, None).unwrap();

        assert_eq!(funding_rates.len(), 1);

        for rate in funding_rates.iter() {
            crate::utils::check_funding_rate_fields(
                EXCHANGE_NAME,
                MarketType::LinearSwap,
                rate,
                raw_msg,
            );
        }

        assert_eq!(funding_rates[0].pair, "BTC/USDT".to_string());
        assert_eq!(funding_rates[0].funding_rate, 0.000106539854);
        assert_eq!(funding_rates[0].funding_time, 1617346800000);
    }
}

#[cfg(test)]
mod candlestick {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, extract_timestamp};

    #[test]
    fn spot_snapshot() {
        let raw_msg = r#"{"action":"snapshot","arg":{"instType":"sp","channel":"candle1m","instId":"BTCUSDT"},"data":[["1654017060000","32173.42","32173.42","32154.98","32154.98","6.7112"],["1654017120000","32154.98","32171.66","32154.83","32157.96","10.3505"]]}"#;

        assert_eq!(
            1654017120000,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "BTCUSDT_SPBL",
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );
    }

    #[test]
    fn spot_update() {
        let raw_msg = r#"{"action":"update","arg":{"instType":"sp","channel":"candle1m","instId":"BTCUSDT"},"data":[["1654077000000","31682.39","31683.63","31674.84","31676.58","20.3826"]]}"#;

        assert_eq!(
            1654077000000,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "BTCUSDT_SPBL",
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );
    }

    #[test]
    fn inverse_swap_snapshot() {
        let raw_msg = r#"{"action":"snapshot","arg":{"instType":"mc","channel":"candle1m","instId":"BTCUSD"},"data":[["1654017420000","31974","31992.5","31922","31935","9.197"],["1654017480000","31935","31988.5","31914.5","31938.5","7.004"]]}"#;

        assert_eq!(
            1654017480000,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "BTCUSD_DMCBL",
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );
    }

    #[test]
    fn inverse_swap_update() {
        let raw_msg = r#"{"action":"update","arg":{"instType":"mc","channel":"candle1m","instId":"BTCUSD"},"data":[["1654077360000","31652","31653.5","31651.5","31652","0.227"]]}"#;

        assert_eq!(
            1654077360000,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "BTCUSD_DMCBL",
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );
    }

    #[test]
    fn linear_swap_snapshot() {
        let raw_msg = r#"{"action":"snapshot","arg":{"instType":"mc","channel":"candle1m","instId":"BTCUSDT"},"data":[["1654017660000","31966.5","31966.5","31947","31952.5","111.769"],["1654017720000","31952.5","31974.5","31939","31940","109.557"]]}"#;

        assert_eq!(
            1654017720000,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "BTCUSDT_UMCBL",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );
    }

    #[test]
    fn linear_swap_update() {
        let raw_msg = r#"{"action":"update","arg":{"instType":"mc","channel":"candle1m","instId":"BTCUSDT"},"data":[["1654077600000","31676.5","31676.5","31671","31671","5.639"]]}"#;

        assert_eq!(
            1654077600000,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "BTCUSDT_UMCBL",
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
        let raw_msg = r#"{"action":"snapshot","arg":{"instType":"sp","channel":"ticker","instId":"BTCUSDT"},"data":[{"instId":"BTCUSDT","last":"29948.21","open24h":"30726.18","high24h":"30741.06","low24h":"29336.95","bestBid":"29944.390000","bestAsk":"29944.550000","baseVolume":"24658.6272","quoteVolume":"749224809.9118","ts":1654160360101,"labeId":0}]}"#;

        assert_eq!(
            1654160360101,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "BTCUSDT_SPBL",
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );
    }

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"action":"snapshot","arg":{"instType":"mc","channel":"ticker","instId":"BTCUSD"},"data":[{"instId":"BTCUSD","last":"29898.50","bestAsk":"29899","bestBid":"29898","high24h":"30706.50","low24h":"29270.50","priceChangePercent":"-0.02","capitalRate":"0.000100","nextSettleTime":1654182000000,"systemTime":1654160828664,"markPrice":"29897.79","indexPrice":"29906.49","holding":"6166.235","baseVolume":"6601.040","quoteVolume":"201847558.263"}]}"#;

        assert_eq!(
            1654160828664,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "BTCUSD_DMCBL",
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"action":"snapshot","arg":{"instType":"mc","channel":"ticker","instId":"BTCUSDT"},"data":[{"instId":"BTCUSDT","last":"29905.50","bestAsk":"29905.5","bestBid":"29904.5","high24h":"30731.50","low24h":"29293.00","priceChangePercent":"-0.02","capitalRate":"0.000100","nextSettleTime":1654182000000,"systemTime":1654160847314,"markPrice":"29906.42","indexPrice":"29928.32","holding":"87338.493","baseVolume":"214176.417","quoteVolume":"6536325903.110"}]}"#;

        assert_eq!(
            1654160847314,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "BTCUSDT_UMCBL",
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
        let raw_msg = r#"{"code":"00000","msg":"success","requestTime":1654232411024,"data":{"asks":[["30413.33","0.0141"],["30413.37","0.0112"],["30413.41","0.0179"]],"bids":[["30413.27","0.0296"],["30413.23","0.0484"],["30413.19","0.1272"]],"timestamp":"1654232411024"}}"#;

        assert_eq!(
            "NONE",
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );
        assert_eq!(
            1654232411024,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );
    }

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"code":"00000","msg":"success","requestTime":1654234201087,"data":{"asks":[["30521","1.661"],["30521.5","2.312"],["30522","0.139"]],"bids":[["3.052E+4","0.256"],["30519.5","0.482"],["30519","0.079"]],"timestamp":"1654234201087"}}"#;

        assert_eq!(
            "NONE",
            extract_symbol(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap()
        );
        assert_eq!(
            1654234201087,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"code":"00000","msg":"success","requestTime":1654234229348,"data":{"asks":[["30544","13.904"],["30544.5","2.678"],["30545","4.923"]],"bids":[["30543","0.756"],["30542.5","0.749"],["30542","15.438"]],"timestamp":"1654234229348"}}"#;

        assert_eq!(
            "NONE",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );
        assert_eq!(
            1654234229348,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
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
        let raw_msg = r#"{"code":"00000","msg":"success","requestTime":1654337704617,"data":{"symbol":"BTCUSD_DMCBL","amount":"5030.748","timestamp":"1654337704617"}}"#;

        assert_eq!(
            "BTCUSD_DMCBL",
            extract_symbol(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap()
        );
        assert_eq!(
            1654337704617,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"code":"00000","msg":"success","requestTime":1654337723059,"data":{"symbol":"BTCUSDT_UMCBL","amount":"89481.932","timestamp":"1654337723059"}}"#;

        assert_eq!(
            "BTCUSDT_UMCBL",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );
        assert_eq!(
            1654337723059,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
    }
}

#[cfg(test)]
mod before20220429 {
    #[cfg(test)]
    mod trade {
        use super::super::EXCHANGE_NAME;
        use crypto_market_type::MarketType;
        use crypto_message::TradeSide;
        use crypto_msg_parser::{extract_symbol, extract_timestamp, parse_trade, round};

        #[test]
        fn inverse_swap() {
            let raw_msg = r#"{"data":[{"instrument_id":"btcusd","price":"58722.0","side":"sell","size":"158","timestamp":"1616236107276"},{"instrument_id":"btcusd","price":"58722.0","side":"sell","size":"450","timestamp":"1616236107276"},{"instrument_id":"btcusd","price":"58722.0","side":"sell","size":"762","timestamp":"1616236107276"}],"table":"swap/trade"}"#;
            let trades = &parse_trade(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap();

            assert_eq!(trades.len(), 3);

            for trade in trades.iter() {
                crate::utils::check_trade_fields(
                    EXCHANGE_NAME,
                    MarketType::InverseSwap,
                    "BTC/USD".to_string(),
                    extract_symbol(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap(),
                    trade,
                    raw_msg,
                );
                assert_eq!(trade.side, TradeSide::Sell);
            }
            assert_eq!(
                1616236107276,
                extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                    .unwrap()
                    .unwrap()
            );

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
            let trades = &parse_trade(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap();

            assert_eq!(trades.len(), 3);

            for trade in trades.iter() {
                crate::utils::check_trade_fields(
                    EXCHANGE_NAME,
                    MarketType::LinearSwap,
                    "BTC/USDT".to_string(),
                    extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap(),
                    trade,
                    raw_msg,
                );

                assert_eq!(trade.side, TradeSide::Sell);
            }
            assert_eq!(
                1616236212569,
                extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                    .unwrap()
                    .unwrap()
            );

            assert_eq!(trades[0].quantity_base, round(1265.0 * 0.001));
            assert_eq!(trades[1].quantity_base, 25.0 * 0.001);
            assert_eq!(trades[2].quantity_base, 181.0 * 0.001);
        }
    }

    #[cfg(test)]
    mod funding_rate {
        use super::super::EXCHANGE_NAME;
        use crypto_market_type::MarketType;
        use crypto_msg_parser::parse_funding_rate;

        #[test]
        fn inverse_swap() {
            let raw_msg = r#"{"data":[{"funding_rate":"0.000258514264","funding_time":"1617346800000","instrument_id":"btcusd"}],"table":"swap/funding_rate"}"#;
            let funding_rates =
                &parse_funding_rate(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg, None).unwrap();

            assert_eq!(funding_rates.len(), 1);

            for rate in funding_rates.iter() {
                crate::utils::check_funding_rate_fields(
                    EXCHANGE_NAME,
                    MarketType::InverseSwap,
                    rate,
                    raw_msg,
                );
            }

            assert_eq!(funding_rates[0].pair, "BTC/USD".to_string());
            assert_eq!(funding_rates[0].funding_rate, 0.000258514264);
            assert_eq!(funding_rates[0].funding_time, 1617346800000);
        }

        #[test]
        fn linear_swap() {
            let raw_msg = r#"{"data":[{"funding_rate":"0.000106539854","funding_time":"1617346800000","instrument_id":"cmt_btcusdt"}],"table":"swap/funding_rate"}"#;
            let funding_rates =
                &parse_funding_rate(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg, None).unwrap();

            assert_eq!(funding_rates.len(), 1);

            for rate in funding_rates.iter() {
                crate::utils::check_funding_rate_fields(
                    EXCHANGE_NAME,
                    MarketType::LinearSwap,
                    rate,
                    raw_msg,
                );
            }

            assert_eq!(funding_rates[0].pair, "BTC/USDT".to_string());
            assert_eq!(funding_rates[0].funding_rate, 0.000106539854);
            assert_eq!(funding_rates[0].funding_time, 1617346800000);
        }
    }

    #[cfg(test)]
    mod l2_orderbook {
        use super::super::EXCHANGE_NAME;
        use crypto_market_type::MarketType;
        use crypto_msg_parser::{extract_symbol, extract_timestamp, parse_l2, round};
        use crypto_msg_type::MessageType;

        #[test]
        fn linear_swap_snapshot() {
            let raw_msg = r#"{"action":"partial","data":[{"asks":[["34589.0","507"],["34589.5","958"],["34590.0","6751"],["34590.5","898"],["34591.0","1987"]],"bids":[["34588.0","1199"],["34587.0","1339"],["34586.5","506"],["34586.0","4018"],["34585.0","1259"]],"instrument_id":"cmt_btcusdt","timestamp":"1622432420458"}],"table":"swap/depth"}"#;
            let orderbook =
                &parse_l2(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg, None).unwrap()[0];

            assert_eq!(orderbook.asks.len(), 5);
            assert_eq!(orderbook.bids.len(), 5);
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
                1622432420458,
                extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                    .unwrap()
                    .unwrap()
            );

            assert_eq!(orderbook.timestamp, 1622432420458);

            assert_eq!(orderbook.bids[0].price, 34588.0);
            assert_eq!(orderbook.bids[0].quantity_base, 1.199);
            assert_eq!(orderbook.bids[0].quantity_quote, 1.199 * 34588.0);
            assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 1199.0);

            assert_eq!(orderbook.bids[4].price, 34585.0);
            assert_eq!(orderbook.bids[4].quantity_base, 1.259);
            assert_eq!(orderbook.bids[4].quantity_quote, 1.259 * 34585.0);
            assert_eq!(orderbook.bids[4].quantity_contract.unwrap(), 1259.0);

            assert_eq!(orderbook.asks[0].price, 34589.0);
            assert_eq!(orderbook.asks[0].quantity_base, 0.507);
            assert_eq!(orderbook.asks[0].quantity_quote, 0.507 * 34589.0);
            assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 507.0);

            assert_eq!(orderbook.asks[4].price, 34591.0);
            assert_eq!(orderbook.asks[4].quantity_base, 1.987);
            assert_eq!(orderbook.asks[4].quantity_quote, round(1.987 * 34591.0));
            assert_eq!(orderbook.asks[4].quantity_contract.unwrap(), 1987.0);
        }

        #[test]
        fn linear_swap_update() {
            let raw_msg = r#"{"action":"update","data":[{"asks":[["34523","510"]],"bids":[["34522","9079"],["34521.5","31174"]],"instrument_id":"cmt_btcusdt","timestamp":"1622434075797"}],"table":"swap/depth"}"#;
            let orderbook =
                &parse_l2(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg, None).unwrap()[0];

            assert_eq!(orderbook.asks.len(), 1);
            assert_eq!(orderbook.bids.len(), 2);
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
                1622434075797,
                extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                    .unwrap()
                    .unwrap()
            );

            assert_eq!(orderbook.timestamp, 1622434075797);

            assert_eq!(orderbook.bids[0].price, 34522.0);
            assert_eq!(orderbook.bids[0].quantity_base, 9.079);
            assert_eq!(orderbook.bids[0].quantity_quote, 9.079 * 34522.0);
            assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 9079.0);

            assert_eq!(orderbook.bids[1].price, 34521.5);
            assert_eq!(orderbook.bids[1].quantity_base, 31.174);
            assert_eq!(orderbook.bids[1].quantity_quote, 31.174 * 34521.5);
            assert_eq!(orderbook.bids[1].quantity_contract.unwrap(), 31174.0);

            assert_eq!(orderbook.asks[0].price, 34523.0);
            assert_eq!(orderbook.asks[0].quantity_base, 0.51);
            assert_eq!(orderbook.asks[0].quantity_quote, 0.51 * 34523.0);
            assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 510.0);
        }

        #[test]
        fn inverse_swap_snapshot() {
            let raw_msg = r#"{"action":"partial","data":[{"asks":[["34880.5","506"],["34881.0","4496"],["34881.5","73280"],["34882.0","84782"],["34882.5","135651"]],"bids":[["34879.0","14946"],["34878.5","24386"],["34878.0","10048"],["34877.5","161361"],["34877.0","61292"]],"instrument_id":"btcusd","timestamp":"1622426574770"}],"table":"swap/depth"}"#;
            let orderbook =
                &parse_l2(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg, None).unwrap()[0];

            assert_eq!(orderbook.asks.len(), 5);
            assert_eq!(orderbook.bids.len(), 5);
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
                1622426574770,
                extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                    .unwrap()
                    .unwrap()
            );

            assert_eq!(orderbook.timestamp, 1622426574770);

            assert_eq!(orderbook.bids[0].price, 34879.0);
            assert_eq!(orderbook.bids[0].quantity_base, 14946.0 / 34879.0);
            assert_eq!(orderbook.bids[0].quantity_quote, 14946.0);
            assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 14946.0);

            assert_eq!(orderbook.bids[4].price, 34877.0);
            assert_eq!(orderbook.bids[4].quantity_base, 61292.0 / 34877.0);
            assert_eq!(orderbook.bids[4].quantity_quote, 61292.0);
            assert_eq!(orderbook.bids[4].quantity_contract.unwrap(), 61292.0);

            assert_eq!(orderbook.asks[0].price, 34880.5);
            assert_eq!(orderbook.asks[0].quantity_base, 506.0 / 34880.5);
            assert_eq!(orderbook.asks[0].quantity_quote, 506.0);
            assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 506.0);

            assert_eq!(orderbook.asks[4].price, 34882.5);
            assert_eq!(orderbook.asks[4].quantity_base, 135651.0 / 34882.5);
            assert_eq!(orderbook.asks[4].quantity_quote, 135651.0);
            assert_eq!(orderbook.asks[4].quantity_contract.unwrap(), 135651.0);
        }

        #[test]
        fn inverse_swap_update() {
            let raw_msg = r#"{"action":"update","data":[{"asks":[["34641.5","101367"],["34642","25822"]],"bids":[["34637","510"]],"instrument_id":"btcusd","timestamp":"1622431636806"}],"table":"swap/depth"}"#;
            let orderbook =
                &parse_l2(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg, None).unwrap()[0];

            assert_eq!(orderbook.asks.len(), 2);
            assert_eq!(orderbook.bids.len(), 1);
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
                1622431636806,
                extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                    .unwrap()
                    .unwrap()
            );

            assert_eq!(orderbook.timestamp, 1622431636806);

            assert_eq!(orderbook.bids[0].price, 34637.0);
            assert_eq!(orderbook.bids[0].quantity_base, 510.0 / 34637.0);
            assert_eq!(orderbook.bids[0].quantity_quote, 510.0);
            assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 510.0);

            assert_eq!(orderbook.asks[0].price, 34641.5);
            assert_eq!(orderbook.asks[0].quantity_base, 101367.0 / 34641.5);
            assert_eq!(orderbook.asks[0].quantity_quote, 101367.0);
            assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 101367.0);

            assert_eq!(orderbook.asks[1].price, 34642.0);
            assert_eq!(orderbook.asks[1].quantity_base, 25822.0 / 34642.0);
            assert_eq!(orderbook.asks[1].quantity_quote, 25822.0);
            assert_eq!(orderbook.asks[1].quantity_contract.unwrap(), 25822.0);
        }
    }

    #[cfg(test)]
    mod l2_topk {
        use super::super::EXCHANGE_NAME;
        use crypto_market_type::MarketType;
        use crypto_msg_parser::{extract_symbol, extract_timestamp, parse_l2_topk, round};
        use crypto_msg_type::MessageType;

        #[test]
        fn linear_swap() {
            let raw_msg = r#"{"data":[{"asks":[["371.18","307"],["371.19","171"],["371.20","111"],["371.21","454"],["371.22","414"]],"bids":[["370.87","1479"],["370.86","326"],["370.85","49"],["370.84","752"],["370.83","1415"]],"instrument_id":"cmt_bchusdt","timestamp":"1648785601210"}],"table":"swap/depth5"}"#;
            let orderbook =
                &parse_l2_topk(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg, None).unwrap()[0];

            assert_eq!(orderbook.asks.len(), 5);
            assert_eq!(orderbook.bids.len(), 5);
            assert!(orderbook.snapshot);

            crate::utils::check_orderbook_fields(
                EXCHANGE_NAME,
                MarketType::LinearSwap,
                MessageType::L2TopK,
                "BCH/USDT".to_string(),
                extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap(),
                orderbook,
                raw_msg,
            );
            assert_eq!(
                "cmt_bchusdt",
                extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
            );
            assert_eq!(
                1648785601210,
                extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                    .unwrap()
                    .unwrap()
            );

            assert_eq!(orderbook.timestamp, 1648785601210);

            assert_eq!(orderbook.bids[0].price, 370.87);
            assert_eq!(orderbook.bids[0].quantity_base, round(0.01 * 1479.0));
            assert_eq!(orderbook.bids[0].quantity_quote, 0.01 * 1479.0 * 370.87);
            assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 1479.0);

            assert_eq!(orderbook.bids[4].price, 370.83);
            assert_eq!(orderbook.bids[4].quantity_base, 0.01 * 1415.0);
            assert_eq!(orderbook.bids[4].quantity_quote, 0.01 * 1415.0 * 370.83);
            assert_eq!(orderbook.bids[4].quantity_contract.unwrap(), 1415.0);

            assert_eq!(orderbook.asks[0].price, 371.18);
            assert_eq!(orderbook.asks[0].quantity_base, round(0.01 * 307.0));
            assert_eq!(
                orderbook.asks[0].quantity_quote,
                round(0.01 * 307.0 * 371.18)
            );
            assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 307.0);

            assert_eq!(orderbook.asks[4].price, 371.22);
            assert_eq!(orderbook.asks[4].quantity_base, 0.01 * 414.0);
            assert_eq!(orderbook.asks[4].quantity_quote, 0.01 * 414.0 * 371.22);
            assert_eq!(orderbook.asks[4].quantity_contract.unwrap(), 414.0);
        }
    }

    #[cfg(test)]
    mod candlestick {
        use super::super::EXCHANGE_NAME;
        use crypto_market_type::MarketType;
        use crypto_msg_parser::{extract_symbol, extract_timestamp};

        #[test]
        fn inverse_swap() {
            let raw_msg = r#"{"data":{"candle":["1648801800000","45230.0","45230.0","45230.0","45230.0","0","0.000000000000"],"instrument_id":"btcusd"},"table":"swap/candle60s"}"#;

            assert_eq!(
                "btcusd",
                extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
            );
            assert_eq!(
                1648801800000,
                extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                    .unwrap()
                    .unwrap()
            );
        }

        #[test]
        fn linear_swap() {
            let raw_msg = r#"{"data":{"candle":["1648801800000","45298.5","45298.5","45274.0","45274.0","1273","57633.802000000000"],"instrument_id":"cmt_btcusdt"},"table":"swap/candle60s"}"#;

            assert_eq!(
                "cmt_btcusdt",
                extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
            );
            assert_eq!(
                1648801800000,
                extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                    .unwrap()
                    .unwrap()
            );
        }
    }
}
