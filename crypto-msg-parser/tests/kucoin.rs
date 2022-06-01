mod utils;

const EXCHANGE_NAME: &str = "kucoin";

#[cfg(test)]
mod trade {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, extract_timestamp, parse_trade, TradeSide};

    #[test]
    fn spot() {
        let raw_msg = r#"{"data":{"symbol":"BTC-USDT","sequence":"1614503482134","side":"buy","size":"0.00013064","price":"57659.6","takerOrderId":"6057bb821220fc00060f26bf","time":"1616362370760468781","type":"match","makerOrderId":"6057bb81b5ab390006532c9d","tradeId":"6057bb822e113d292396c272"},"subject":"trade.l3match","topic":"/market/match:BTC-USDT","type":"message"}"#;
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
            1616362370760,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(trade.quantity_base, 0.00013064);
        assert_eq!(trade.quantity_contract, None);
        assert_eq!(trade.side, TradeSide::Buy);
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"data":{"makerUserId":"5e568500eb029b0008715785","symbol":"XBTUSDTM","sequence":8267947,"side":"buy","size":16,"price":57850,"takerOrderId":"6057bc95660a7d0006dc1171","makerOrderId":"6057bc92652ce800067e841a","takerUserId":"601f35b4d42fad0006b2df21","tradeId":"6057bc953c7feb667195bac9","ts":1616362645429686578},"subject":"match","topic":"/contractMarket/execution:XBTUSDTM","type":"message"}"#;
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
            1616362645429,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(trade.quantity_base, 0.001 * 16.0);
        assert_eq!(trade.quantity_quote, 0.001 * 16.0 * 57850.0);
        assert_eq!(trade.quantity_contract, Some(16.0));
        assert_eq!(trade.side, TradeSide::Buy);
    }

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"data":{"makerUserId":"5d85a240c788c62738732dd9","symbol":"XBTUSDM","sequence":5174061,"side":"buy","size":5000,"price":57798,"takerOrderId":"6057bc692cfab900061f8b11","makerOrderId":"6057bc4df4b11f0006a7743b","takerUserId":"5dba895d134ab72ce156079a","tradeId":"6057bc693c7feb6705f9a248","ts":1616362601277456186},"subject":"match","topic":"/contractMarket/execution:XBTUSDM","type":"message"}"#;
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
            1616362601277,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(trade.quantity_base, 5000.0 / 57798.0);
        assert_eq!(trade.quantity_quote, 5000.0);
        assert_eq!(trade.quantity_contract, Some(5000.0));
        assert_eq!(trade.side, TradeSide::Buy);
    }

    #[test]
    fn inverse_future() {
        let raw_msg = r#"{"data":{"makerUserId":"5f802947889eb80006a1ba0f","symbol":"XBTMH21","sequence":31319,"side":"sell","size":1510,"price":57963.0,"takerOrderId":"6057be2685c6a0000610a89a","makerOrderId":"6057be11652ce800067fafb9","takerUserId":"5f802947889eb80006a1ba0f","tradeId":"6057be2677a0c431d1d1f5b6","ts":1616363046546528915},"subject":"match","topic":"/contractMarket/execution:XBTMH21","type":"message"}"#;
        let trades = &parse_trade(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg).unwrap();

        assert_eq!(trades.len(), 1);
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
            1616363046546,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(trade.quantity_base, 1510.0 / 57963.0);
        assert_eq!(trade.quantity_quote, 1510.0);
        assert_eq!(trade.quantity_contract, Some(1510.0));
        assert_eq!(trade.side, TradeSide::Sell);
    }
}

#[cfg(test)]
mod l2_event {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, extract_timestamp, parse_l2};
    use crypto_msg_type::MessageType;

    #[test]
    fn spot_update() {
        let raw_msg = r#"{"data":{"sequenceStart":1617071937790,"symbol":"BTC-USDT","changes":{"asks":[],"bids":[["39272","0.0530867","1617071937790"]]},"sequenceEnd":1617071937790},"subject":"trade.l2update","topic":"/market/level2:BTC-USDT","type":"message"}"#;
        let received_at = 1625097804231_i64;
        let orderbook =
            &parse_l2(EXCHANGE_NAME, MarketType::Spot, raw_msg, Some(received_at)).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 0);
        assert_eq!(orderbook.bids.len(), 1);
        assert!(!orderbook.snapshot);
        assert_eq!(orderbook.timestamp, received_at);
        assert_eq!(orderbook.seq_id, Some(1617071937790));

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

        assert_eq!(orderbook.bids[0].price, 39272.0);
        assert_eq!(orderbook.bids[0].quantity_base, 0.0530867);
        assert_eq!(orderbook.bids[0].quantity_quote, 39272.0 * 0.0530867);
    }

    #[test]
    fn inverse_swap_update() {
        let raw_msg = r#"{"data":{"sequence":1617852459594,"change":"39069.0,buy,23960","timestamp":1622718985044},"subject":"level2","topic":"/contractMarket/level2:XBTUSDM","type":"message"}"#;
        let orderbook =
            &parse_l2(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 0);
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
            1622718985044,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1622718985044);
        assert_eq!(orderbook.seq_id, Some(1617852459594));

        assert_eq!(orderbook.bids[0].price, 39069.0);
        assert_eq!(orderbook.bids[0].quantity_base, 23960.0 / 39069.0);
        assert_eq!(orderbook.bids[0].quantity_quote, 23960.0);
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 23960.0);
    }

    #[test]
    fn linear_swap_update() {
        let raw_msg = r#"{"data":{"sequence":1618232029293,"change":"38962.0,buy,4374","timestamp":1622719195286},"subject":"level2","topic":"/contractMarket/level2:XBTUSDTM","type":"message"}"#;
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
            1622719195286,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1622719195286);
        assert_eq!(orderbook.seq_id, Some(1618232029293));

        assert_eq!(orderbook.bids[0].price, 38962.0);
        assert_eq!(orderbook.bids[0].quantity_base, 4.374);
        assert_eq!(orderbook.bids[0].quantity_quote, 38962.0 * 4.374);
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 4374.0);
    }

    #[test]
    fn inverse_future_update() {
        let raw_msg = r#"{"data":{"sequence":1616827077941,"change":"39006.0,sell,11450","timestamp":1622719594867},"subject":"level2","topic":"/contractMarket/level2:XBTMM21","type":"message"}"#;
        let orderbook =
            &parse_l2(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 1);
        assert_eq!(orderbook.bids.len(), 0);
        assert!(!orderbook.snapshot);

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
            1622719594867,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1622719594867);
        assert_eq!(orderbook.seq_id, Some(1616827077941));

        assert_eq!(orderbook.asks[0].price, 39006.0);
        assert_eq!(orderbook.asks[0].quantity_base, 11450.0 / 39006.0);
        assert_eq!(orderbook.asks[0].quantity_quote, 11450.0);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 11450.0);
    }
}

#[cfg(test)]
mod l2_topk {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, extract_timestamp, parse_l2_topk, round};
    use crypto_msg_type::MessageType;

    #[test]
    fn spot() {
        let raw_msg = r#"{"type":"message","topic":"/spotMarket/level2Depth5:BTC-USDT","subject":"level2","data":{"asks":[["31530.2","2.90121626"],["31530.4","0.00026686"],["31531.5","0.01934176"],["31531.6","0.425"],["31531.7","0.09467136"]],"bids":[["31530.1","0.74468602"],["31530","0.27077267"],["31529.9","0.48567212"],["31528.9","0.000849"],["31528.8","0.15853762"]],"timestamp":1653989906722}}"#;
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
            1653989906722,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1653989906722);
        assert_eq!(orderbook.seq_id, None);
        assert_eq!(orderbook.prev_seq_id, None);

        assert_eq!(orderbook.bids[0].price, 31530.1);
        assert_eq!(orderbook.bids[0].quantity_base, 0.74468602);
        assert_eq!(orderbook.bids[0].quantity_quote, 31530.1 * 0.74468602);
        assert_eq!(orderbook.bids[0].quantity_contract, None);

        assert_eq!(orderbook.bids[4].price, 31528.8);
        assert_eq!(orderbook.bids[4].quantity_base, 0.15853762);
        assert_eq!(orderbook.bids[4].quantity_quote, 31528.8 * 0.15853762);
        assert_eq!(orderbook.bids[4].quantity_contract, None);

        assert_eq!(orderbook.asks[0].price, 31530.2);
        assert_eq!(orderbook.asks[0].quantity_base, 2.90121626);
        assert_eq!(orderbook.asks[0].quantity_quote, 31530.2 * 2.90121626);
        assert_eq!(orderbook.asks[0].quantity_contract, None);

        assert_eq!(orderbook.asks[4].price, 31531.7);
        assert_eq!(orderbook.asks[4].quantity_base, 0.09467136);
        assert_eq!(orderbook.asks[4].quantity_quote, 31531.7 * 0.09467136);
        assert_eq!(orderbook.asks[4].quantity_contract, None);
    }

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"type":"message","topic":"/contractMarket/level2Depth5:XBTUSDM","subject":"level2","data":{"sequence":1638556032307,"asks":[[31529,12725],[31530,21214],[31531,6090],[31532,44385],[31538,85]],"bids":[[31528.00000000,2856],[31527.00000000,10034],[31525,6266],[31524,5043],[31521,85]],"ts":1653991142662,"timestamp":1653991142662}}"#;
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
            1653991142662,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1653991142662);
        assert_eq!(orderbook.seq_id, Some(1638556032307));
        assert_eq!(orderbook.prev_seq_id, None);

        assert_eq!(orderbook.bids[0].price, 31528.0);
        assert_eq!(orderbook.bids[0].quantity_base, 2856.0 / 31528.0);
        assert_eq!(orderbook.bids[0].quantity_quote, 2856.0);
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 2856.0);

        assert_eq!(orderbook.bids[4].price, 31521.0);
        assert_eq!(orderbook.bids[4].quantity_base, 85.0 / 31521.0);
        assert_eq!(orderbook.bids[4].quantity_quote, 85.0);
        assert_eq!(orderbook.bids[4].quantity_contract.unwrap(), 85.0);

        assert_eq!(orderbook.asks[0].price, 31529.0);
        assert_eq!(orderbook.asks[0].quantity_base, 12725.0 / 31529.0);
        assert_eq!(orderbook.asks[0].quantity_quote, 12725.0);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 12725.0);

        assert_eq!(orderbook.asks[4].price, 31538.0);
        assert_eq!(orderbook.asks[4].quantity_base, 85.0 / 31538.0);
        assert_eq!(orderbook.asks[4].quantity_quote, 85.0);
        assert_eq!(orderbook.asks[4].quantity_contract.unwrap(), 85.0);
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"type":"message","topic":"/contractMarket/level2Depth5:XBTUSDTM","subject":"level2","data":{"sequence":1643184485510,"asks":[[31608,32278],[31609,571],[31610,4456],[31611,10157],[31612,3185]],"bids":[[31607,16350],[31606.0,7183],[31605,17234],[31604,553],[31603,620]],"ts":1653992430005,"timestamp":1653992430005}}"#;
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
            1653992430005,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1653992430005);
        assert_eq!(orderbook.seq_id, Some(1643184485510));
        assert_eq!(orderbook.prev_seq_id, None);

        assert_eq!(orderbook.bids[0].price, 31607.0);
        assert_eq!(orderbook.bids[0].quantity_base, 16.350);
        assert_eq!(orderbook.bids[0].quantity_quote, round(31607.0 * 16.35));
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 16350.0);

        assert_eq!(orderbook.bids[4].price, 31603.0);
        assert_eq!(orderbook.bids[4].quantity_base, 0.620);
        assert_eq!(orderbook.bids[4].quantity_quote, 31603.0 * 0.620);
        assert_eq!(orderbook.bids[4].quantity_contract.unwrap(), 620.0);

        assert_eq!(orderbook.asks[0].price, 31608.0);
        assert_eq!(orderbook.asks[0].quantity_base, 32.278);
        assert_eq!(orderbook.asks[0].quantity_quote, round(31608.0 * 32.278));
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 32278.0);

        assert_eq!(orderbook.asks[4].price, 31612.0);
        assert_eq!(orderbook.asks[4].quantity_base, 3.185);
        assert_eq!(orderbook.asks[4].quantity_quote, 31612.0 * 3.185);
        assert_eq!(orderbook.asks[4].quantity_contract.unwrap(), 3185.0);
    }

    #[test]
    fn inverse_future() {
        let raw_msg = r#"{"type":"message","topic":"/contractMarket/level2Depth5:XBTMM22","subject":"level2","data":{"sequence":1647031214270,"asks":[[31648,1600],[31657,28],[31658,1800],[31672,1204],[31683,150]],"bids":[[31626,1628],[31625,5466],[31622,1266],[31609,1557],[31595,2943]],"ts":1653992298695,"timestamp":1653992298695}}"#;
        let orderbook =
            &parse_l2_topk(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 5);
        assert_eq!(orderbook.bids.len(), 5);
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
            1653992298695,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1653992298695);
        assert_eq!(orderbook.seq_id, Some(1647031214270));
        assert_eq!(orderbook.prev_seq_id, None);

        assert_eq!(orderbook.bids[0].price, 31626.0);
        assert_eq!(orderbook.bids[0].quantity_base, 1628.0 / 31626.0);
        assert_eq!(orderbook.bids[0].quantity_quote, 1628.0);
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 1628.0);

        assert_eq!(orderbook.bids[4].price, 31595.0);
        assert_eq!(orderbook.bids[4].quantity_base, 2943.0 / 31595.0);
        assert_eq!(orderbook.bids[4].quantity_quote, 2943.0);
        assert_eq!(orderbook.bids[4].quantity_contract.unwrap(), 2943.0);

        assert_eq!(orderbook.asks[0].price, 31648.0);
        assert_eq!(orderbook.asks[0].quantity_base, 1600.0 / 31648.0);
        assert_eq!(orderbook.asks[0].quantity_quote, 1600.0);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 1600.0);

        assert_eq!(orderbook.asks[4].price, 31683.0);
        assert_eq!(orderbook.asks[4].quantity_base, 150.0 / 31683.0);
        assert_eq!(orderbook.asks[4].quantity_quote, 150.0);
        assert_eq!(orderbook.asks[4].quantity_contract.unwrap(), 150.0);
    }
}

#[cfg(test)]
mod bbo {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, extract_timestamp};

    #[test]
    fn spot() {
        let raw_msg = r#"{"type":"message","topic":"/market/ticker:BTC-USDT","subject":"trade.ticker","data":{"bestAsk":"31785.3","bestAskSize":"1.0455757","bestBid":"31785.2","bestBidSize":"0.4645037","price":"31785.2","sequence":"1630218274617","size":"0.03133705","time":1654032320677}}"#;

        assert_eq!(
            1654032320677,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "BTC-USDT",
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );
    }

    #[test]
    fn inverse_future() {
        let raw_msg = r#"{"type":"message","topic":"/contractMarket/tickerV2:XBTMM22","subject":"tickerV2","data":{"symbol":"XBTMM22","sequence":1647024019666,"bestBidSize":118,"bestBidPrice":"31741.0","bestAskPrice":"31776.0","ts":1654032575773272833,"bestAskSize":562}}"#;

        assert_eq!(
            1654032575773,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "XBTMM22",
            extract_symbol(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap()
        );
    }

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"type":"message","topic":"/contractMarket/tickerV2:XBTUSDM","subject":"tickerV2","data":{"symbol":"XBTUSDM","sequence":1638549733058,"bestBidSize":5543,"bestBidPrice":"31741.0","bestAskPrice":"31742.0","ts":1654032770009498293,"bestAskSize":500}}"#;

        assert_eq!(
            1654032770009,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "XBTUSDM",
            extract_symbol(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap()
        );
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"type":"message","topic":"/contractMarket/tickerV2:XBTUSDTM","subject":"tickerV2","data":{"symbol":"XBTUSDTM","sequence":1643185147205,"bestBidSize":20074,"bestBidPrice":"31766.0","bestAskPrice":"31767.0","ts":1654032807465857723,"bestAskSize":1187}}"#;

        assert_eq!(
            1654032807465,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "XBTUSDTM",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );
    }
}

#[cfg(test)]
mod l3_event {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, extract_timestamp};

    #[test]
    fn spot() {
        let raw_msg = r#"{"type":"message","topic":"/spotMarket/level3:BTC-USDT","subject":"received","data":{"symbol":"BTC-USDT","orderId":"629724de1f7e6b00015310cb","sequence":1630234429271,"clientOid":"d2b351d1-e185-11ec-aceb-068cc764f03f","ts":1654072542361747612}}"#;

        assert_eq!(
            1654072542361,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "BTC-USDT",
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );
    }

    #[test]
    fn inverse_future() {
        let raw_msg = r#"{"type":"message","topic":"/contractMarket/level3v2:XBTMM22","subject":"open","data":{"symbol":"XBTMM22","sequence":1647173843748,"side":"sell","orderTime":1654073248891988536,"size":"28","orderId":"629727a023aac2000194fc87","price":"31615.0","ts":1654073248910399802}}"#;

        assert_eq!(
            1654073248910,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "XBTMM22",
            extract_symbol(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap()
        );
    }

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"type":"message","topic":"/contractMarket/level3v2:XBTUSDM","subject":"open","data":{"symbol":"XBTUSDM","sequence":1639148481406,"side":"buy","orderTime":1654073289118060857,"size":"3671","orderId":"629727c9edde6b0001f422a7","price":"31570.0","ts":1654073289160921530}}"#;

        assert_eq!(
            1654073289160,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "XBTUSDM",
            extract_symbol(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap()
        );
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"type":"message","topic":"/contractMarket/level3v2:XBTUSDTM","subject":"received","data":{"symbol":"XBTUSDTM","sequence":1655525144741,"orderId":"629727ecdd16e300018810de","clientOid":"cabifr55rj7cmsu5a850","ts":1654073324184830142}}"#;

        assert_eq!(
            1654073324184,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "XBTUSDTM",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );
    }
}
