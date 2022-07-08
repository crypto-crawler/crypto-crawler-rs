mod utils;

const EXCHANGE_NAME: &str = "dydx";

#[cfg(test)]
mod trade {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_message::TradeSide;
    use crypto_msg_parser::{extract_symbol, extract_timestamp, parse_trade};

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"type":"channel_data","connection_id":"c685b690-168e-421d-bfd4-60aae426686d","message_id":2,"id":"BTC-USD","channel":"v3_trades","contents":{"trades":[{"size":"0.124","side":"BUY","price":"56503","createdAt":"2021-10-11T10:36:41.464Z"}]}}"#;
        let trade = &parse_trade(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()[0];

        crate::utils::check_trade_fields(
            EXCHANGE_NAME,
            MarketType::LinearSwap,
            "BTC/USD".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap(),
            trade,
            raw_msg,
        );
        assert_eq!(
            1633948601464,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(trade.quantity_base, 0.124);
        assert_eq!(trade.quantity_quote, 0.124 * 56503.0);
        assert_eq!(trade.quantity_contract, Some(0.124));

        assert_eq!(trade.side, TradeSide::Buy);
    }
}

#[cfg(test)]
mod l2_event {
    use super::EXCHANGE_NAME;
    use chrono::prelude::*;
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, extract_timestamp, parse_l2};
    use crypto_msg_type::MessageType;

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"type":"subscribed","connection_id":"f1e5eecb-7929-4033-8f47-47a2eb71af96","message_id":1,"channel":"v3_orderbook","id":"BTC-USD","contents":{"asks":[{"size":"1.7415","price":"56490"},{"size":"1.7718","price":"56493"}],"bids":[{"size":"1.7088","price":"56489"},{"size":"2.1594","price":"56488"}]}}"#;
        let received_at = Utc::now().timestamp_millis();
        let orderbook = &parse_l2(
            EXCHANGE_NAME,
            MarketType::LinearSwap,
            raw_msg,
            Some(received_at),
        )
        .unwrap()[0];

        assert_eq!(orderbook.asks.len(), 2);
        assert_eq!(orderbook.bids.len(), 2);
        assert!(orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            EXCHANGE_NAME,
            MarketType::LinearSwap,
            MessageType::L2Event,
            "BTC/USD".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );
        assert_eq!(
            None,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );

        assert_eq!(orderbook.timestamp, received_at);

        assert_eq!(orderbook.bids[0].price, 56489.0);
        assert_eq!(orderbook.bids[0].quantity_base, 1.7088);
        assert_eq!(orderbook.bids[0].quantity_quote, 56489.0 * 1.7088);
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 1.7088);

        assert_eq!(orderbook.bids[1].price, 56488.0);
        assert_eq!(orderbook.bids[1].quantity_base, 2.1594);
        assert_eq!(orderbook.bids[1].quantity_quote, 56488.0 * 2.1594);
        assert_eq!(orderbook.bids[1].quantity_contract.unwrap(), 2.1594);

        assert_eq!(orderbook.asks[0].price, 56490.0);
        assert_eq!(orderbook.asks[0].quantity_base, 1.7415);
        assert_eq!(orderbook.asks[0].quantity_quote, 56490.0 * 1.7415);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 1.7415);

        assert_eq!(orderbook.asks[1].price, 56493.0);
        assert_eq!(orderbook.asks[1].quantity_base, 1.7718);
        assert_eq!(orderbook.asks[1].quantity_quote, 56493.0 * 1.7718);
        assert_eq!(orderbook.asks[1].quantity_contract.unwrap(), 1.7718);

        let raw_msg = r#"{"type":"channel_data","connection_id":"f1e5eecb-7929-4033-8f47-47a2eb71af96","message_id":2,"id":"BTC-USD","channel":"v3_orderbook","contents":{"offset":"2060907065","bids":[],"asks":[["56525","0.4782"],["56527","0.0186"]]}}"#;
        let orderbook = &parse_l2(
            EXCHANGE_NAME,
            MarketType::LinearSwap,
            raw_msg,
            Some(1633951152106),
        )
        .unwrap()[0];

        assert_eq!(orderbook.asks.len(), 2);
        assert_eq!(orderbook.bids.len(), 0);
        assert!(!orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            EXCHANGE_NAME,
            MarketType::LinearSwap,
            MessageType::L2Event,
            "BTC/USD".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );

        assert_eq!(orderbook.timestamp, 1633951152106);

        assert_eq!(orderbook.asks[0].price, 56525.0);
        assert_eq!(orderbook.asks[0].quantity_base, 0.4782);
        assert_eq!(orderbook.asks[0].quantity_quote, 56525.0 * 0.4782);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 0.4782);
    }
}

#[cfg(test)]
mod l2_snapshot {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, extract_timestamp};

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"asks":[{"size":"0.595","price":"30315"},{"size":"1.56","price":"30317"},{"size":"0.345","price":"30318"}],"bids":[{"size":"1.3529","price":"30310"},{"size":"4.0488","price":"30308"},{"size":"0.033","price":"30306"}]}"#;

        assert_eq!(
            "NONE",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );

        assert_eq!(
            None,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );
    }
}

#[cfg(test)]
mod open_interest {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, extract_timestamp};

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"markets":{"BTC-USD":{"market":"BTC-USD","status":"ONLINE","baseAsset":"BTC","quoteAsset":"USD","stepSize":"0.0001","tickSize":"1","indexPrice":"29718.5600","oraclePrice":"29686.8200","priceChange24H":"-3.120000","nextFundingRate":"0.0000077940","nextFundingAt":"2022-06-04T12:00:00.000Z","minOrderSize":"0.001","type":"PERPETUAL","initialMarginFraction":"0.05","maintenanceMarginFraction":"0.03","volume24H":"172496250.187000","trades24H":"27100","openInterest":"7989.6723","incrementalInitialMarginFraction":"0.01","incrementalPositionSize":"1.5","maxPositionSize":"170","baselinePositionSize":"9","assetResolution":"10000000000","syntheticAssetId":"0x4254432d3130000000000000000000"},"AVAX-USD":{"market":"AVAX-USD","status":"ONLINE","baseAsset":"AVAX","quoteAsset":"USD","stepSize":"0.1","tickSize":"0.01","indexPrice":"23.1026","oraclePrice":"23.0900","priceChange24H":"0.442564","nextFundingRate":"0.0000073995","nextFundingAt":"2022-06-04T12:00:00.000Z","minOrderSize":"1","type":"PERPETUAL","initialMarginFraction":"0.10","maintenanceMarginFraction":"0.05","volume24H":"17097633.955000","trades24H":"6889","openInterest":"988005.4","incrementalInitialMarginFraction":"0.02","incrementalPositionSize":"1800","maxPositionSize":"91000","baselinePositionSize":"9000","assetResolution":"10000000","syntheticAssetId":"0x415641582d37000000000000000000"}}}"#;

        assert_eq!(
            "ALL",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );

        assert_eq!(
            None,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );
    }
}
