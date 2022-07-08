mod utils;

const EXCHANGE_NAME: &str = "okx"; // V5 API

#[cfg(test)]
mod trade {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_message::TradeSide;
    use crypto_msg_parser::{extract_symbol, extract_timestamp, parse_trade, round};

    #[test]
    fn spot() {
        let raw_msg = r#"{"arg":{"channel":"trades","instId":"BTC-USDT"},"data":[{"instId":"BTC-USDT","tradeId":"314161276","px":"43474.1","sz":"0.00373695","side":"buy","ts":"1646311839593"}]}"#;
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
            1646311839593,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(trade.timestamp, 1646311839593);
        assert_eq!(trade.quantity_base, 0.00373695);
        assert_eq!(trade.price, 43474.1);
        assert_eq!(trade.quantity_contract, None);
        assert_eq!(trade.side, TradeSide::Buy);
    }

    #[test]
    fn linear_future() {
        let raw_msg = r#"{"arg":{"channel":"trades","instId":"BTC-USDT-220325"},"data":[{"instId":"BTC-USDT-220325","tradeId":"17400303","px":"43535.3","sz":"2","side":"sell","ts":"1646311972504"}]}"#;
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
            1646311972504,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearFuture, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(trade.timestamp, 1646311972504);
        assert_eq!(trade.price, 43535.3);
        assert_eq!(trade.quantity_contract, Some(2.0));
        assert_eq!(trade.quantity_base, 2.0 * 0.01);
        assert_eq!(trade.quantity_quote, round(2.0 * 0.01 * 43535.3));
        assert_eq!(trade.side, TradeSide::Sell);
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"arg":{"channel":"trades","instId":"BTC-USDT-SWAP"},"data":[{"instId":"BTC-USDT-SWAP","tradeId":"219066264","px":"43568.8","sz":"7","side":"buy","ts":"1646312440645"}]}"#;
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
            1646312440645,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(trade.timestamp, 1646312440645);
        assert_eq!(trade.price, 43568.8);
        assert_eq!(trade.quantity_contract, Some(7.0));
        assert_eq!(trade.quantity_base, 7.0 * 0.01);
        assert_eq!(trade.quantity_quote, round(7.0 * 0.01 * 43568.8));
        assert_eq!(trade.side, TradeSide::Buy);
    }

    #[test]
    fn inverse_future() {
        let raw_msg = r#"{"arg":{"channel":"trades","instId":"BTC-USD-220325"},"data":[{"instId":"BTC-USD-220325","tradeId":"18928717","px":"43568.7","sz":"1","side":"sell","ts":"1646312543604"}]}"#;
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
            1646312543604,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(trade.timestamp, 1646312543604);
        assert_eq!(trade.price, 43568.7);
        assert_eq!(trade.quantity_contract, Some(1.0));
        assert_eq!(trade.quantity_quote, 100.0 * 1.0);
        assert_eq!(trade.quantity_base, 100.0 * 1.0 / 43568.7);
        assert_eq!(trade.side, TradeSide::Sell);
    }

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"arg":{"channel":"trades","instId":"BTC-USD-SWAP"},"data":[{"instId":"BTC-USD-SWAP","tradeId":"173543957","px":"43574.9","sz":"1","side":"sell","ts":"1646312664791"}]}"#;
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
            1646312664791,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(trade.timestamp, 1646312664791);
        assert_eq!(trade.price, 43574.9);
        assert_eq!(trade.quantity_contract, Some(1.0));
        assert_eq!(trade.quantity_quote, 100.0 * 1.0);
        assert_eq!(trade.quantity_base, 100.0 * 1.0 / 43574.9);
        assert_eq!(trade.side, TradeSide::Sell);
    }

    #[test]
    fn option() {
        let raw_msg = r#"{"arg":{"channel":"trades","instId":"BTC-USD-220304-32000-P"},"data":[{"instId":"BTC-USD-220304-32000-P","tradeId":"81","px":"0.001","sz":"85","side":"buy","ts":"1646138219181"}]}"#;
        let trades = &parse_trade(EXCHANGE_NAME, MarketType::EuropeanOption, raw_msg).unwrap();

        assert_eq!(trades.len(), 1);
        let trade = &trades[0];

        crate::utils::check_trade_fields(
            EXCHANGE_NAME,
            MarketType::EuropeanOption,
            "BTC/USD".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::EuropeanOption, raw_msg).unwrap(),
            trade,
            raw_msg,
        );
        assert_eq!(
            "BTC-USD-220304-32000-P",
            extract_symbol(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap()
        );
        assert_eq!(
            1646138219181,
            extract_timestamp(EXCHANGE_NAME, MarketType::EuropeanOption, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(trade.timestamp, 1646138219181);
        assert_eq!(trade.price, 0.001);
        assert_eq!(trade.quantity_contract, Some(85.0));
        assert_eq!(trade.quantity_base, 85.0);
        assert_eq!(trade.quantity_quote, 85.0 * 0.001);
        assert_eq!(trade.side, TradeSide::Buy);
    }
}

#[cfg(test)]
mod funding_rate {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, extract_timestamp, parse_funding_rate};

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"arg":{"channel":"funding-rate","instId":"BTC-USD-SWAP"},"data":[{"fundingRate":"0.0000734174532791","fundingTime":"1646323200000","instId":"BTC-USD-SWAP","instType":"SWAP","nextFundingRate":"0.0001163723201487"}]}"#;
        let received_at = 1646323212345;
        let funding_rates = &parse_funding_rate(
            EXCHANGE_NAME,
            MarketType::InverseSwap,
            raw_msg,
            Some(received_at),
        )
        .unwrap();

        assert_eq!(funding_rates.len(), 1);

        for rate in funding_rates.iter() {
            crate::utils::check_funding_rate_fields(
                EXCHANGE_NAME,
                MarketType::InverseSwap,
                rate,
                raw_msg,
            );
        }
        assert_eq!(
            "BTC-USD-SWAP",
            extract_symbol(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap()
        );
        assert_eq!(
            None,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap()
        );

        assert_eq!(funding_rates[0].pair, "BTC/USD".to_string());
        assert_eq!(funding_rates[0].funding_rate, 0.0000734174532791);
        assert_eq!(funding_rates[0].estimated_rate, Some(0.0001163723201487));
        assert_eq!(funding_rates[0].funding_time, 1646323200000);
        assert_eq!(funding_rates[0].timestamp, received_at);
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"arg":{"channel":"funding-rate","instId":"BTC-USDT-SWAP"},"data":[{"fundingRate":"0.0001534702159002","fundingTime":"1646323200000","instId":"BTC-USDT-SWAP","instType":"SWAP","nextFundingRate":"0.0001542145319804"}]}"#;
        let received_at = 1646323212345;
        let funding_rates = &parse_funding_rate(
            EXCHANGE_NAME,
            MarketType::InverseSwap,
            raw_msg,
            Some(received_at),
        )
        .unwrap();

        assert_eq!(funding_rates.len(), 1);

        for rate in funding_rates.iter() {
            crate::utils::check_funding_rate_fields(
                EXCHANGE_NAME,
                MarketType::InverseSwap,
                rate,
                raw_msg,
            );
        }
        assert_eq!(
            "BTC-USDT-SWAP",
            extract_symbol(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap()
        );
        assert_eq!(
            None,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap()
        );

        assert_eq!(funding_rates[0].pair, "BTC/USDT".to_string());
        assert_eq!(funding_rates[0].funding_rate, 0.0001534702159002);
        assert_eq!(funding_rates[0].estimated_rate, Some(0.0001542145319804));
        assert_eq!(funding_rates[0].funding_time, 1646323200000);
        assert_eq!(funding_rates[0].timestamp, received_at);
    }
}

#[cfg(test)]
mod l2_event {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, extract_timestamp, parse_l2, round};
    use crypto_msg_type::MessageType;

    #[test]
    fn spot_snapshot() {
        let raw_msg = r#"{"arg":{"channel":"books-l2-tbt","instId":"BTC-USDT"},"action":"snapshot","data":[{"asks":[["43666.1","1.09431286","0","15"],["43666.3","0.01","0","1"],["43668.1","0.00102036","0","1"]],"bids":[["43666","0.00278174","0","5"],["43664","0.00245053","0","2"],["43662","0.00245065","0","2"]],"ts":"1646313944551","checksum":144433427}]}"#;
        let orderbook = &parse_l2(EXCHANGE_NAME, MarketType::Spot, raw_msg, None).unwrap()[0];

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
            1646313944551,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1646313944551);

        assert_eq!(orderbook.bids[0].price, 43666.0);
        assert_eq!(orderbook.bids[0].quantity_base, 0.00278174);
        assert_eq!(orderbook.bids[0].quantity_quote, 43666.0 * 0.00278174);

        assert_eq!(orderbook.asks[0].price, 43666.1);
        assert_eq!(orderbook.asks[0].quantity_base, 1.09431286);
        assert_eq!(orderbook.asks[0].quantity_quote, 1.09431286 * 43666.1);
    }

    #[test]
    fn spot_update() {
        let raw_msg = r#"{"arg":{"channel":"books-l2-tbt","instId":"BTC-USDT"},"action":"update","data":[{"asks":[["43736.2","0.1358","0","2"]],"bids":[["43675.6","0.05","0","1"]],"ts":"1646314295200","checksum":796530682}]}"#;
        let orderbook = &parse_l2(EXCHANGE_NAME, MarketType::Spot, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 1);
        assert_eq!(orderbook.bids.len(), 1);
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
            1646314295200,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1646314295200);

        assert_eq!(orderbook.bids[0].price, 43675.6);
        assert_eq!(orderbook.bids[0].quantity_base, 0.05);
        assert_eq!(orderbook.bids[0].quantity_quote, 43675.6 * 0.05);

        assert_eq!(orderbook.asks[0].price, 43736.2);
        assert_eq!(orderbook.asks[0].quantity_base, 0.1358);
        assert_eq!(orderbook.asks[0].quantity_quote, round(43736.2 * 0.1358));
    }

    #[test]
    fn linear_future_snapshot() {
        let raw_msg = r#"{"arg":{"channel":"books-l2-tbt","instId":"BTC-USDT-220325"},"action":"snapshot","data":[{"asks":[["43741.9","4","0","1"],["43743.4","1","0","1"],["43743.5","4","0","1"]],"bids":[["43741.8","2","0","1"],["43739.3","4","0","1"],["43738","34","0","1"]],"ts":"1646314548269","checksum":2127111983}]}"#;
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
            1646314548269,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearFuture, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1646314548269);

        assert_eq!(orderbook.asks[0].price, 43741.9);
        assert_eq!(orderbook.asks[0].quantity_contract, Some(4.0));
        assert_eq!(orderbook.asks[0].quantity_base, 4.0 * 0.01);
        assert_eq!(
            orderbook.asks[0].quantity_quote,
            round(4.0 * 0.01 * 43741.9)
        );

        assert_eq!(orderbook.bids[0].price, 43741.8);
        assert_eq!(orderbook.bids[0].quantity_contract, Some(2.0));
        assert_eq!(orderbook.bids[0].quantity_base, 2.0 * 0.01);
        assert_eq!(
            orderbook.bids[0].quantity_quote,
            round(2.0 * 0.01 * 43741.8)
        );
    }

    #[test]
    fn inverse_swap_snapshot() {
        let raw_msg = r#"{"arg":{"channel":"books-l2-tbt","instId":"BTC-USD-SWAP"},"action":"snapshot","data":[{"asks":[["43726.4","145","0","5"],["43730.5","10","0","1"],["43733.1","45","0","1"]],"bids":[["43726.3","131","0","1"],["43726","10","0","1"],["43722","16","0","1"]],"ts":"1646314888087","checksum":-1817371130}]}"#;
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
            1646314888087,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1646314888087);

        assert_eq!(orderbook.asks[0].price, 43726.4);
        assert_eq!(orderbook.asks[0].quantity_contract, Some(145.0));
        assert_eq!(orderbook.asks[0].quantity_quote, 100.0 * 145.0);
        assert_eq!(orderbook.asks[0].quantity_base, 100.0 * 145.0 / 43726.4);

        assert_eq!(orderbook.bids[0].price, 43726.3);
        assert_eq!(orderbook.bids[0].quantity_contract, Some(131.0));
        assert_eq!(orderbook.bids[0].quantity_quote, 100.0 * 131.0);
        assert_eq!(orderbook.bids[0].quantity_base, 100.0 * 131.0 / 43726.3);
    }

    #[test]
    fn option_snapshot() {
        let raw_msg = r#"{"arg":{"channel":"books-l2-tbt","instId":"BTC-USD-220304-32000-P"},"action":"snapshot","data":[{"asks":[["0.0005","305","0","1"],["0.001","550","0","2"]],"bids":[],"ts":"1646315100798","checksum":971343753}]}"#;
        let orderbook =
            &parse_l2(EXCHANGE_NAME, MarketType::EuropeanOption, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 2);
        assert_eq!(orderbook.bids.len(), 0);
        assert!(orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            EXCHANGE_NAME,
            MarketType::EuropeanOption,
            MessageType::L2Event,
            "BTC/USD".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::EuropeanOption, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );
        assert_eq!(
            1646315100798,
            extract_timestamp(EXCHANGE_NAME, MarketType::EuropeanOption, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1646315100798);

        assert_eq!(orderbook.asks[0].price, 0.0005);
        assert_eq!(orderbook.asks[0].quantity_contract, Some(305.0));
        assert_eq!(orderbook.asks[0].quantity_base, 305.0);
        assert_eq!(orderbook.asks[0].quantity_quote, 305.0 * 0.0005);
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
        let raw_msg = r#"{"arg":{"channel":"books5","instId":"BTC-USDT"},"data":[{"asks":[["30221.8","0.00439","0","2"],["30223.5","1.12","0","1"],["30223.7","1.16000647","0","3"],["30224.6","1.22","0","1"],["30225.6","1.64553107","0","2"]],"bids":[["30221.7","0.30608367","0","6"],["30220.9","0.01321829","0","1"],["30219.6","1.06226719","0","2"],["30219.5","0.0130546","0","1"],["30219.4","0.41","0","2"]],"instId":"BTC-USDT","ts":"1652671418459"}]}"#;
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
            1652671418459,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1652671418459);
        assert_eq!(orderbook.seq_id, None);
        assert_eq!(orderbook.prev_seq_id, None);

        assert_eq!(orderbook.bids[0].price, 30221.7);
        assert_eq!(orderbook.bids[0].quantity_base, 0.30608367);
        assert_eq!(
            orderbook.bids[0].quantity_quote,
            round(30221.7 * 0.30608367)
        );
        assert_eq!(orderbook.bids[0].quantity_contract, None);

        assert_eq!(orderbook.bids[4].price, 30219.4);
        assert_eq!(orderbook.bids[4].quantity_base, 0.41);
        assert_eq!(orderbook.bids[4].quantity_quote, 30219.4 * 0.41);
        assert_eq!(orderbook.bids[4].quantity_contract, None);

        assert_eq!(orderbook.asks[0].price, 30221.8);
        assert_eq!(orderbook.asks[0].quantity_base, 0.00439);
        assert_eq!(orderbook.asks[0].quantity_quote, 0.00439 * 30221.8);
        assert_eq!(orderbook.asks[0].quantity_contract, None);

        assert_eq!(orderbook.asks[4].price, 30225.6);
        assert_eq!(orderbook.asks[4].quantity_base, 1.64553107);
        assert_eq!(orderbook.asks[4].quantity_quote, 1.64553107 * 30225.6);
        assert_eq!(orderbook.asks[4].quantity_contract, None);
    }

    #[test]
    fn spot_2() {
        let raw_msg = r#"{"table":"spot/depth5","data":[{"asks":[["0.9788","0.001","1"],["0.9789","0.944259","7"],["0.979","1.956785","1"],["0.9792","0.9401","1"],["0.98","31.459918","8"]],"bids":[["0.9768","0.605755","1"],["0.976","1.0532","1"],["0.9757","0.002","1"],["0.9752","0.720555","1"],["0.9748","0.001","1"]],"instrument_id":"BETH-ETH","timestamp":"2022-02-25T00:00:01.032Z"}]}"#;
        let orderbook = &parse_l2_topk(EXCHANGE_NAME, MarketType::Spot, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 5);
        assert_eq!(orderbook.bids.len(), 5);
        assert!(orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            EXCHANGE_NAME,
            MarketType::Spot,
            MessageType::L2TopK,
            "BETH/ETH".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );
        assert_eq!(
            "BETH-ETH",
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );
        assert_eq!(
            1645747201032,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1645747201032);
        assert_eq!(orderbook.seq_id, None);
        assert_eq!(orderbook.prev_seq_id, None);

        assert_eq!(orderbook.bids[0].price, 0.9768);
        assert_eq!(orderbook.bids[0].quantity_base, 0.605755);
        assert_eq!(orderbook.bids[0].quantity_quote, 0.9768 * 0.605755);
        assert_eq!(orderbook.bids[0].quantity_contract, None);

        assert_eq!(orderbook.bids[4].price, 0.9748);
        assert_eq!(orderbook.bids[4].quantity_base, 0.001);
        assert_eq!(orderbook.bids[4].quantity_quote, round(0.9748 * 0.001));
        assert_eq!(orderbook.bids[4].quantity_contract, None);

        assert_eq!(orderbook.asks[0].price, 0.9788);
        assert_eq!(orderbook.asks[0].quantity_base, 0.001);
        assert_eq!(orderbook.asks[0].quantity_quote, 0.001 * 0.9788);
        assert_eq!(orderbook.asks[0].quantity_contract, None);

        assert_eq!(orderbook.asks[4].price, 0.98);
        assert_eq!(orderbook.asks[4].quantity_base, 31.459918);
        assert_eq!(orderbook.asks[4].quantity_quote, round(31.459918 * 0.98));
        assert_eq!(orderbook.asks[4].quantity_contract, None);
    }

    #[test]
    fn inverse_future() {
        let raw_msg = r#"{"arg":{"channel":"books5","instId":"BTC-USD-220624"},"data":[{"asks":[["31835.7","690","0","2"],["31841.2","5","0","1"],["31841.5","148","0","1"],["31841.8","5","0","1"],["31843.4","10","0","1"]],"bids":[["31835.6","6","0","2"],["31834","1","0","1"],["31833.2","23","0","1"],["31833.1","403","0","2"],["31832.9","5","0","1"]],"instId":"BTC-USD-220624","ts":"1653997473120"}]}"#;
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
            1653997473120,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1653997473120);
        assert_eq!(orderbook.seq_id, None);
        assert_eq!(orderbook.prev_seq_id, None);

        assert_eq!(orderbook.asks[0].price, 31835.7);
        assert_eq!(orderbook.asks[0].quantity_quote, 100.0 * 690.0);
        assert_eq!(orderbook.asks[0].quantity_base, 100.0 * 690.0 / 31835.7);
        assert_eq!(orderbook.asks[0].quantity_contract, Some(690.0));

        assert_eq!(orderbook.asks[4].price, 31843.4);
        assert_eq!(orderbook.asks[4].quantity_quote, 100.0 * 10.0);
        assert_eq!(orderbook.asks[4].quantity_base, 100.0 * 10.0 / 31843.4);
        assert_eq!(orderbook.asks[4].quantity_contract, Some(10.0));

        assert_eq!(orderbook.bids[0].price, 31835.6);
        assert_eq!(orderbook.bids[0].quantity_quote, 100.0 * 6.0);
        assert_eq!(orderbook.bids[0].quantity_base, 100.0 * 6.0 / 31835.6);
        assert_eq!(orderbook.bids[0].quantity_contract, Some(6.0));

        assert_eq!(orderbook.bids[4].price, 31832.9);
        assert_eq!(orderbook.bids[4].quantity_quote, 100.0 * 5.0);
        assert_eq!(orderbook.bids[4].quantity_base, 100.0 * 5.0 / 31832.9);
        assert_eq!(orderbook.bids[4].quantity_contract, Some(5.0));
    }

    #[test]
    fn linear_future() {
        let raw_msg = r#"{"arg":{"channel":"books5","instId":"BTC-USDT-220624"},"data":[{"asks":[["30351.5","18","0","2"],["30355.2","6","0","1"],["30355.3","30","0","2"],["30356.5","1","0","1"],["30358","1","0","1"]],"bids":[["30346.1","1","0","1"],["30344.5","8","0","1"],["30343.6","1","0","1"],["30343.4","1","0","1"],["30340.6","45","0","1"]],"instId":"BTC-USDT-220624","ts":"1652672165391"}]}"#;
        let orderbook =
            &parse_l2_topk(EXCHANGE_NAME, MarketType::LinearFuture, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 5);
        assert_eq!(orderbook.bids.len(), 5);
        assert!(orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            EXCHANGE_NAME,
            MarketType::LinearFuture,
            MessageType::L2TopK,
            "BTC/USDT".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::LinearFuture, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );
        assert_eq!(
            1652672165391,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearFuture, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1652672165391);
        assert_eq!(orderbook.seq_id, None);
        assert_eq!(orderbook.prev_seq_id, None);

        assert_eq!(orderbook.asks[0].price, 30351.5);
        assert_eq!(orderbook.asks[0].quantity_base, 18.0 * 0.01);
        assert_eq!(
            orderbook.asks[0].quantity_quote,
            round(18.0 * 0.01 * 30351.5)
        );
        assert_eq!(orderbook.asks[0].quantity_contract, Some(18.0));

        assert_eq!(orderbook.asks[4].price, 30358.0);
        assert_eq!(orderbook.asks[4].quantity_base, 1.0 * 0.01);
        assert_eq!(orderbook.asks[4].quantity_quote, 1.0 * 0.01 * 30358.0);
        assert_eq!(orderbook.asks[4].quantity_contract, Some(1.0));

        assert_eq!(orderbook.bids[0].price, 30346.1);
        assert_eq!(orderbook.bids[0].quantity_base, 1.0 * 0.01);
        assert_eq!(orderbook.bids[0].quantity_quote, 1.0 * 0.01 * 30346.1);
        assert_eq!(orderbook.bids[0].quantity_contract, Some(1.0));

        assert_eq!(orderbook.bids[4].price, 30340.6);
        assert_eq!(orderbook.bids[4].quantity_base, 45.0 * 0.01);
        assert_eq!(orderbook.bids[4].quantity_quote, 45.0 * 0.01 * 30340.6);
        assert_eq!(orderbook.bids[4].quantity_contract, Some(45.0));
    }

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"arg":{"channel":"books5","instId":"BTC-USD-SWAP"},"data":[{"asks":[["29502","350","0","19"],["29502.2","42","0","2"],["29502.3","62","0","1"],["29502.7","5","0","1"],["29505","3","0","1"]],"bids":[["29501.9","77","0","1"],["29500.7","194","0","1"],["29499.5","1","0","1"],["29496.6","2","0","1"],["29495.1","1","0","1"]],"instId":"BTC-USD-SWAP","ts":"1652686260965"}]}"#;
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
            1652686260965,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1652686260965);
        assert_eq!(orderbook.seq_id, None);
        assert_eq!(orderbook.prev_seq_id, None);

        assert_eq!(orderbook.asks[0].price, 29502.0);
        assert_eq!(orderbook.asks[0].quantity_quote, 100.0 * 350.0);
        assert_eq!(orderbook.asks[0].quantity_base, 100.0 * 350.0 / 29502.0);
        assert_eq!(orderbook.asks[0].quantity_contract, Some(350.0));

        assert_eq!(orderbook.asks[4].price, 29505.0);
        assert_eq!(orderbook.asks[4].quantity_quote, 100.0 * 3.0);
        assert_eq!(orderbook.asks[4].quantity_base, 100.0 * 3.0 / 29505.0);
        assert_eq!(orderbook.asks[4].quantity_contract, Some(3.0));

        assert_eq!(orderbook.bids[0].price, 29501.9);
        assert_eq!(orderbook.bids[0].quantity_quote, 100.0 * 77.0);
        assert_eq!(orderbook.bids[0].quantity_base, 100.0 * 77.0 / 29501.9);
        assert_eq!(orderbook.bids[0].quantity_contract, Some(77.0));

        assert_eq!(orderbook.bids[4].price, 29495.1);
        assert_eq!(orderbook.bids[4].quantity_quote, 100.0 * 1.0);
        assert_eq!(orderbook.bids[4].quantity_base, 100.0 * 1.0 / 29495.1);
        assert_eq!(orderbook.bids[4].quantity_contract, Some(1.0));
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"arg":{"channel":"books5","instId":"BTC-USDT-SWAP"},"data":[{"asks":[["31806.6","159","0","6"],["31807.1","9","0","3"],["31807.5","32","0","1"],["31807.9","28","0","1"],["31808.3","1","0","1"]],"bids":[["31806.5","54","0","3"],["31806","1","0","1"],["31805.7","5","0","1"],["31805.6","4","0","1"],["31805","10","0","2"]],"instId":"BTC-USDT-SWAP","ts":"1653997254735"}]}"#;
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
            1653997254735,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1653997254735);
        assert_eq!(orderbook.seq_id, None);
        assert_eq!(orderbook.prev_seq_id, None);

        assert_eq!(orderbook.asks[0].price, 31806.6);
        assert_eq!(orderbook.asks[0].quantity_base, 159.0 * 0.01);
        assert_eq!(orderbook.asks[0].quantity_quote, 159.0 * 0.01 * 31806.6);
        assert_eq!(orderbook.asks[0].quantity_contract, Some(159.0));

        assert_eq!(orderbook.asks[4].price, 31808.3);
        assert_eq!(orderbook.asks[4].quantity_base, 1.0 * 0.01);
        assert_eq!(orderbook.asks[4].quantity_quote, 1.0 * 0.01 * 31808.3);
        assert_eq!(orderbook.asks[4].quantity_contract, Some(1.0));

        assert_eq!(orderbook.bids[0].price, 31806.5);
        assert_eq!(orderbook.bids[0].quantity_base, 54.0 * 0.01);
        assert_eq!(
            orderbook.bids[0].quantity_quote,
            round(54.0 * 0.01 * 31806.5)
        );
        assert_eq!(orderbook.bids[0].quantity_contract, Some(54.0));

        assert_eq!(orderbook.bids[4].price, 31805.0);
        assert_eq!(orderbook.bids[4].quantity_base, 10.0 * 0.01);
        assert_eq!(orderbook.bids[4].quantity_quote, 10.0 * 0.01 * 31805.0);
        assert_eq!(orderbook.bids[4].quantity_contract, Some(10.0));
    }

    #[test]
    fn option() {
        let raw_msg = r#"{"arg":{"channel":"books5","instId":"BTC-USD-220624-50000-C"},"data":[{"asks":[["0.001","606","0","2"],["0.0015","330","0","2"],["0.002","10","0","1"]],"bids":[],"instId":"BTC-USD-220624-50000-C","ts":"1654387553361"}]}"#;

        assert_eq!(
            1654387553361,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "BTC-USD-220624-50000-C",
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );
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
        let raw_msg = r#"{"arg":{"channel":"bbo-tbt","instId":"BTC-USDT"},"data":[{"asks":[["31774.7","0.14368878","0","3"]],"bids":[["31774.6","0.3392211","0","3"]],"ts":"1654032991947"}]}"#;
        
        let bbo_msg = &parse_bbo(EXCHANGE_NAME, MarketType::Spot, raw_msg, None).unwrap();

        assert_eq!(
            1654032991947,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "BTC-USDT",
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );

        let received_at = 1654032991947;
        assert_eq!(MessageType::BBO, bbo_msg.msg_type);
        assert_eq!("BTC-USDT", bbo_msg.symbol);
        assert_eq!(received_at, bbo_msg.timestamp);

        assert_eq!(31774.7, bbo_msg.ask_price);
        assert_eq!(0.14368878, bbo_msg.ask_quantity_base);
        assert_eq!(round(31774.7 * 0.14368878), bbo_msg.ask_quantity_quote);
        assert_eq!(None, bbo_msg.ask_quantity_contract);

        assert_eq!(31774.6, bbo_msg.bid_price);
        assert_eq!(0.3392211, bbo_msg.bid_quantity_base);
        assert_eq!(round(31774.6 * 0.3392211), bbo_msg.bid_quantity_quote);
        assert_eq!(None, bbo_msg.bid_quantity_contract);

    }

    #[test]
    fn inverse_future() {
        let raw_msg = r#"{"arg":{"channel":"bbo-tbt","instId":"BTC-USD-220624"},"data":[{"asks":[["31769.9","15","0","1"]],"bids":[["31769.8","38","0","6"]],"ts":"1654033096078"}]}"#;

        assert_eq!(
            1654033096078,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "BTC-USD-220624",
            extract_symbol(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg).unwrap()
        );

        let bbo_msg = &parse_bbo(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg, None).unwrap();
        
        assert_eq!(MessageType::BBO, bbo_msg.msg_type);
        assert_eq!("BTC-USD-220624", bbo_msg.symbol);
        assert_eq!(1654033096078, bbo_msg.timestamp);

        assert_eq!(31769.9, bbo_msg.ask_price);
        assert_eq!(15.0 * 100.0 / 31769.9, bbo_msg.ask_quantity_base);
        assert_eq!(15.0 * 100.0, bbo_msg.ask_quantity_quote);
        assert_eq!(Some(15.0), bbo_msg.ask_quantity_contract);

        assert_eq!(31769.8, bbo_msg.bid_price);
        assert_eq!(38.0 * 100.0 / 31769.8, bbo_msg.bid_quantity_base);
        assert_eq!(38.0 * 100.0, bbo_msg.bid_quantity_quote);
        assert_eq!(Some(38.0), bbo_msg.bid_quantity_contract);

    }

    #[test]
    fn linear_future() {
        let raw_msg = r#"{"arg":{"channel":"bbo-tbt","instId":"BTC-USDT-220624"},"data":[{"asks":[["31854.6","4","0","1"]],"bids":[["31850.4","2","0","1"]],"ts":"1654033073837"}]}"#;

        assert_eq!(
            1654033073837,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearFuture, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "BTC-USDT-220624",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearFuture, raw_msg).unwrap()
        );

        let bbo_msg = &parse_bbo(EXCHANGE_NAME, MarketType::LinearFuture, raw_msg, None).unwrap();
        assert_eq!(MessageType::BBO, bbo_msg.msg_type);
        assert_eq!("BTC-USDT-220624", bbo_msg.symbol);
        assert_eq!(1654033073837, bbo_msg.timestamp);

        assert_eq!(31854.6, bbo_msg.ask_price);
        assert_eq!(round(4.0 * 0.01), bbo_msg.ask_quantity_base);
        assert_eq!(round(0.01 * 4.0 * 31854.6), bbo_msg.ask_quantity_quote);
        assert_eq!(Some(4.0), bbo_msg.ask_quantity_contract);

        assert_eq!(31850.4, bbo_msg.bid_price);
        assert_eq!(round(2.0 * 0.01), bbo_msg.bid_quantity_base);
        assert_eq!(round(0.01 * 2.0 * 31850.4), bbo_msg.bid_quantity_quote);
        assert_eq!(Some(2.0), bbo_msg.bid_quantity_contract);

    }

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"arg":{"channel":"tickers","instId":"BTC-USD-SWAP"},"data":[{"instType":"SWAP","instId":"BTC-USD-SWAP","last":"31771.6","lastSz":"16","askPx":"31771.6","askSz":"16","bidPx":"31771.5","bidSz":"1967","open24h":"31648.1","high24h":"32398.1","low24h":"31202.4","sodUtc0":"31717.3","sodUtc8":"32038.6","volCcy24h":"13760.6923","vol24h":"4364424","ts":"1654033212805"}]}"#;

        assert_eq!(
            1654033212805,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "BTC-USD-SWAP",
            extract_symbol(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap()
        );

        let bbo_msg = parse_bbo(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg, None).unwrap();

        assert_eq!(MessageType::BBO, bbo_msg.msg_type);
        assert_eq!("BTC-USD-SWAP", bbo_msg.symbol);
        assert_eq!(1654033212805, bbo_msg.timestamp);

        assert_eq!(31771.6, bbo_msg.ask_price);
        assert_eq!(16.0 * 100.0 / 31771.6, bbo_msg.ask_quantity_base);
        assert_eq!(16.0 * 100.0, bbo_msg.ask_quantity_quote);
        assert_eq!(Some(16.0), bbo_msg.ask_quantity_contract);

        assert_eq!(31771.5, bbo_msg.bid_price);
        assert_eq!(1967.0 * 100.0 / 31771.5, bbo_msg.bid_quantity_base);
        assert_eq!(1967.0 * 100.0, bbo_msg.bid_quantity_quote);
        assert_eq!(Some(1967.0), bbo_msg.bid_quantity_contract);

    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"arg":{"channel":"tickers","instId":"BTC-USDT-SWAP"},"data":[{"instType":"SWAP","instId":"BTC-USDT-SWAP","last":"31807.4","lastSz":"3","askPx":"31807.4","askSz":"4","bidPx":"31807.3","bidSz":"482","open24h":"31671.1","high24h":"32450","low24h":"31234.8","sodUtc0":"31743.6","sodUtc8":"32052.7","volCcy24h":"122143.88","vol24h":"12214388","ts":"1654033232461"}]}"#;

        assert_eq!(
            1654033232461,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "BTC-USDT-SWAP",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );

        let bbo_msg = parse_bbo(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg, None).unwrap();

        assert_eq!(MessageType::BBO, bbo_msg.msg_type);
        assert_eq!("BTC-USDT-SWAP", bbo_msg.symbol);
        assert_eq!(1654033232461, bbo_msg.timestamp);

        assert_eq!(31807.4, bbo_msg.ask_price);
        assert_eq!(4.0 * 0.01, bbo_msg.ask_quantity_base);
        assert_eq!(round(4.0 * 0.01 * 31807.4), bbo_msg.ask_quantity_quote);
        assert_eq!(Some(4.0), bbo_msg.ask_quantity_contract);

        assert_eq!(31807.3, bbo_msg.bid_price);
        assert_eq!(482.0 * 0.01, bbo_msg.bid_quantity_base);
        assert_eq!(round(482.0 * 0.01 * 31807.3), bbo_msg.bid_quantity_quote);
        assert_eq!(Some(482.0), bbo_msg.bid_quantity_contract);
    }

    #[test]
    fn option() {
        let raw_msg = r#"{"arg":{"channel":"bbo-tbt","instId":"BTC-USD-220624-50000-C"},"data":[{"asks":[["0.0015","8","0","1"]],"bids":[],"ts":"1654033343415"}]}"#;

        assert_eq!(
            1654033343415,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "BTC-USD-220624-50000-C",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );
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
        let raw_msg = r#"{"arg":{"channel":"candle1m","instId":"BTC-USDT"},"data":[["1654154580000","29930.7","29936.3","29930.7","29936.3","0.0111536","333.86246417"]]}"#;

        assert_eq!(
            1654154580000,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "BTC-USDT",
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );

        let data = parse_candlestick(EXCHANGE_NAME, MarketType::Spot, raw_msg, MessageType::L2TopK).unwrap();

        assert_eq!(1654154580000, data.timestamp);
        assert_eq!("1m", data.period);
    }

    #[test]
    fn inverse_future() {
        let raw_msg = r#"{"arg":{"channel":"candle1m","instId":"BTC-USD-220624"},"data":[["1654154580000","29901.6","29921.2","29901.6","29921.2","166","0.554"]]}"#;

        assert_eq!(
            1654154580000,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "BTC-USD-220624",
            extract_symbol(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap()
        );
    }

    #[test]
    fn linear_future() {
        let raw_msg = r#"{"arg":{"channel":"candle1m","instId":"BTC-USDT-220624"},"data":[["1654154520000","29963.4","29971.2","29958.8","29970.3","133","1.33"]]}"#;

        assert_eq!(
            1654154520000,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "BTC-USDT-220624",
            extract_symbol(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap()
        );
    }

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"arg":{"channel":"tickers","instId":"BTC-USD-SWAP"},"data":[{"instType":"SWAP","instId":"BTC-USD-SWAP","last":"31771.6","lastSz":"16","askPx":"31771.6","askSz":"16","bidPx":"31771.5","bidSz":"1967","open24h":"31648.1","high24h":"32398.1","low24h":"31202.4","sodUtc0":"31717.3","sodUtc8":"32038.6","volCcy24h":"13760.6923","vol24h":"4364424","ts":"1654033212805"}]}"#;

        assert_eq!(
            1654033212805,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "BTC-USD-SWAP",
            extract_symbol(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap()
        );
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"arg":{"channel":"tickers","instId":"BTC-USDT-SWAP"},"data":[{"instType":"SWAP","instId":"BTC-USDT-SWAP","last":"31807.4","lastSz":"3","askPx":"31807.4","askSz":"4","bidPx":"31807.3","bidSz":"482","open24h":"31671.1","high24h":"32450","low24h":"31234.8","sodUtc0":"31743.6","sodUtc8":"32052.7","volCcy24h":"122143.88","vol24h":"12214388","ts":"1654033232461"}]}"#;

        assert_eq!(
            1654033232461,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "BTC-USDT-SWAP",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );
    }

    #[test]
    fn option() {
        let raw_msg = r#"{"arg":{"channel":"candle1m","instId":"BTC-USD-220624-50000-C"},"data":[["1654155480000","0.0005","0.0005","0.0005","0.0005","0","0"]]}"#;

        assert_eq!(
            1654155480000,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "BTC-USD-220624-50000-C",
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
        let raw_msg = r#"{"arg":{"channel":"tickers","instId":"BTC-USDT"},"data":[{"instType":"SPOT","instId":"BTC-USDT","last":"29934","lastSz":"0.00001468","askPx":"29934.1","askSz":"0.30918743","bidPx":"29934","bidSz":"0.05181315","open24h":"31588.3","high24h":"31899.8","low24h":"29318.7","sodUtc0":"29806.5","sodUtc8":"30747","volCcy24h":"302140058.01215828","vol24h":"9935.14546361","ts":"1654166073135"}]}"#;

        assert_eq!(
            1654166073135,
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
        let raw_msg = r#"{"arg":{"channel":"tickers","instId":"BTC-USD-220624"},"data":[{"instType":"FUTURES","instId":"BTC-USD-220624","last":"29883.4","lastSz":"2","askPx":"29885.7","askSz":"3","bidPx":"29882","bidSz":"15","open24h":"31579.7","high24h":"31899.9","low24h":"29243.9","sodUtc0":"29748.4","sodUtc8":"30699.4","volCcy24h":"6540.5687","vol24h":"1985047","ts":"1654166101413"}]}"#;

        assert_eq!(
            1654166101413,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "BTC-USD-220624",
            extract_symbol(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap()
        );
    }

    #[test]
    fn linear_future() {
        let raw_msg = r#"{"arg":{"channel":"tickers","instId":"BTC-USDT-220624"},"data":[{"instType":"FUTURES","instId":"BTC-USDT-220624","last":"29973.6","lastSz":"1","askPx":"29973.6","askSz":"1","bidPx":"29973.5","bidSz":"34","open24h":"31666.9","high24h":"31978.1","low24h":"29336.7","sodUtc0":"29839","sodUtc8":"30788.7","volCcy24h":"3764.42","vol24h":"376442","ts":"1654166126913"}]}"#;

        assert_eq!(
            1654166126913,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "BTC-USDT-220624",
            extract_symbol(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap()
        );
    }

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"arg":{"channel":"tickers","instId":"BTC-USD-SWAP"},"data":[{"instType":"SWAP","instId":"BTC-USD-SWAP","last":"29937.6","lastSz":"2","askPx":"29933","askSz":"34","bidPx":"29932.9","bidSz":"66","open24h":"31572.7","high24h":"31885.6","low24h":"29277.5","sodUtc0":"29777.9","sodUtc8":"30715","volCcy24h":"17799.3208","vol24h":"5412510","ts":"1654166220361"}]}"#;

        assert_eq!(
            1654166220361,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "BTC-USD-SWAP",
            extract_symbol(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap()
        );
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"arg":{"channel":"tickers","instId":"BTC-USDT-SWAP"},"data":[{"instType":"SWAP","instId":"BTC-USDT-SWAP","last":"29963","lastSz":"2","askPx":"29960","askSz":"12","bidPx":"29959.9","bidSz":"431","open24h":"31589.3","high24h":"31907.2","low24h":"29302","sodUtc0":"29805","sodUtc8":"30753.7","volCcy24h":"108540.53","vol24h":"10854053","ts":"1654166233877"}]}"#;

        assert_eq!(
            1654166233877,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "BTC-USDT-SWAP",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );
    }

    #[test]
    fn option() {
        let raw_msg = r#"{"arg":{"channel":"tickers","instId":"BTC-USD-220624-50000-C"},"data":[{"instType":"OPTION","instId":"BTC-USD-220624-50000-C","last":"0.0005","lastSz":"6","askPx":"0.0015","askSz":"330","bidPx":"","bidSz":"","open24h":"0.0005","high24h":"0.0005","low24h":"0.0005","sodUtc0":"0.0005","sodUtc8":"0.0005","volCcy24h":"0","vol24h":"0","ts":"1654166283149"}]}"#;

        assert_eq!(
            1654166283149,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "BTC-USD-220624-50000-C",
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
        let raw_msg = r#"{"code":"0","msg":"","data":[{"asks":[["29692.5","0.10951348","0","5"],["29692.6","0.00631557","0","1"],["29692.8","0.10662911","0","1"]],"bids":[["29692.4","0.43986254","0","2"],["29691.2","0.33792","0","1"],["29690","0.39370474","0","2"]],"ts":"1654328918680"}]}"#;

        assert_eq!(
            1654328918680,
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
        let raw_msg = r#"{"code":"0","msg":"","data":[{"asks":[["29678.9","11","0","1"],["29679","5","0","1"],["29679.3","14","0","2"]],"bids":[["29673.5","8","0","1"],["29673.4","14","0","1"],["29673.1","285","0","1"]],"ts":"1654329601749"}]}"#;

        assert_eq!(
            1654329601749,
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
        let raw_msg = r#"{"code":"0","msg":"","data":[{"asks":[["29756.3","5","0","1"],["29756.4","90","0","1"],["29761.1","7","0","1"]],"bids":[["29755.4","95","0","1"],["29755.3","124","0","1"],["29755","128","0","1"]],"ts":"1654329646278"}]}"#;

        assert_eq!(
            1654329646278,
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
    fn inverse_swap() {
        let raw_msg = r#"{"code":"0","msg":"","data":[{"asks":[["29671.3","12","0","3"],["29675.7","10","0","1"],["29676.9","38","0","1"]],"bids":[["29671.2","759","0","9"],["29670.8","10","0","1"],["29670.2","4","0","1"]],"ts":"1654329603386"}]}"#;

        assert_eq!(
            1654329603386,
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
        let raw_msg = r#"{"code":"0","msg":"","data":[{"asks":[["29701.7","2","0","1"],["29702.9","19","0","1"],["29703.4","20","0","4"]],"bids":[["29701.6","5","0","1"],["29701.5","2","0","1"],["29700.4","2","0","2"]],"ts":"1654329607417"}]}"#;

        assert_eq!(
            1654329607417,
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
    fn option() {
        let raw_msg = r#"{"code":"0","msg":"","data":[{"asks":[["0.0155","305","0","1"]],"bids":[["0.011","305","0","1"]],"ts":"1654329628580"}]}"#;

        assert_eq!(
            1654329628580,
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
    fn inverse_future() {
        let raw_msg = r#"{"code":"0","data":[{"instId":"BTC-USD-220520","instType":"FUTURES","oi":"0","oiCcy":"0","ts":"1654348172899"},{"instId":"BTC-USD-220527","instType":"FUTURES","oi":"0","oiCcy":"0","ts":"1654348172899"}],"msg":""}"#;

        assert_eq!(
            1654348172899,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "ALL",
            extract_symbol(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap()
        );

        let raw_msg = r#"{"code":"0","data":[{"instId":"BTC-USD-220520","instType":"FUTURES","oi":"0","oiCcy":"0","ts":"1654348172899"}],"msg":""}"#;

        assert_eq!(
            1654348172899,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "BTC-USD-220520",
            extract_symbol(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap()
        );
    }

    #[test]
    fn linear_future() {
        let raw_msg = r#"{"code":"0","data":[{"instId":"BTC-USD-220520","instType":"FUTURES","oi":"0","oiCcy":"0","ts":"1654348596037"},{"instId":"BTC-USD-220527","instType":"FUTURES","oi":"0","oiCcy":"0","ts":"1654348596037"}],"msg":""}"#;

        assert_eq!(
            1654348596037,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "ALL",
            extract_symbol(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap()
        );
    }

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"code":"0","data":[{"instId":"BTC-USD-SWAP","instType":"SWAP","oi":"4092973","oiCcy":"13840.048827662696883","ts":"1654348683853"},{"instId":"ETH-USD-SWAP","instType":"SWAP","oi":"21795246","oiCcy":"123562.1205163528751467","ts":"1654348683853"}],"msg":""}"#;

        assert_eq!(
            1654348683853,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "ALL",
            extract_symbol(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap()
        );
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"code":"0","data":[{"instId":"BTC-USD-SWAP","instType":"SWAP","oi":"4093058","oiCcy":"13842.2084993270069734","ts":"1654348739678"},{"instId":"ETH-USD-SWAP","instType":"SWAP","oi":"21797983","oiCcy":"123701.0640410861731408","ts":"1654348739678"}],"msg":""}"#;

        assert_eq!(
            1654348739678,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "ALL",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );
    }

    #[test]
    fn option() {
        let raw_msg = r#"{"code":"0","data":[{"instId":"BTC-USD-220520-18000-C","instType":"OPTION","oi":"0","oiCcy":"0","ts":"1654348880968"},{"instId":"BTC-USD-220520-20000-C","instType":"OPTION","oi":"0","oiCcy":"0","ts":"1654348906381"}],"msg":""}"#;

        assert_eq!(
            1654348906381,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "ALL",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );
    }
}
