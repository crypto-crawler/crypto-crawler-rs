mod utils;

const EXCHANGE_NAME: &str = "bitmex";

#[cfg(test)]
mod trade {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_message::TradeSide;
    use crypto_msg_parser::{extract_symbol, extract_timestamp, parse_trade};

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"table":"trade","action":"insert","data":[{"timestamp":"2021-03-12T02:00:04.608Z","symbol":"XBTUSD","side":"Sell","size":900,"price":56927,"tickDirection":"MinusTick","trdMatchID":"d1b82d61-d902-349c-936c-2588b8204aff","grossValue":1581300,"homeNotional":0.015813,"foreignNotional":900}]}"#;
        let trade = &parse_trade(EXCHANGE_NAME, MarketType::Unknown, raw_msg).unwrap()[0];

        crate::utils::check_trade_fields(
            EXCHANGE_NAME,
            MarketType::InverseSwap,
            "BTC/USD".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap(),
            trade,
            raw_msg,
        );
        assert_eq!(
            1615514404608,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(trade.price, 56927.0);
        assert_eq!(trade.quantity_base, 0.015813);
        assert_eq!(trade.quantity_quote, 900.0);
        assert_eq!(trade.quantity_contract, Some(900.0));
        assert_eq!(trade.side, TradeSide::Sell);
    }

    #[test]
    fn quanto_swap() {
        let raw_msg = r#"{"table":"trade","action":"partial","data":[{"timestamp":"2021-03-21T00:22:09.258Z","symbol":"ETHUSD","side":"Buy","size":1,"price":1811.6,"tickDirection":"ZeroPlusTick","trdMatchID":"46fcd532-c20e-ac2c-eaed-392f2d599487","grossValue":181160,"homeNotional":0.058513750731421885,"foreignNotional":106.00351082504389}]}"#;
        let trade = &parse_trade(EXCHANGE_NAME, MarketType::Unknown, raw_msg).unwrap()[0];

        crate::utils::check_trade_fields(
            EXCHANGE_NAME,
            MarketType::QuantoSwap,
            "ETH/USD".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::QuantoSwap, raw_msg).unwrap(),
            trade,
            raw_msg,
        );
        assert_eq!(
            1616286129258,
            extract_timestamp(EXCHANGE_NAME, MarketType::QuantoSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(trade.price, 1811.6);
        assert_eq!(trade.quantity_base, 0.058513750731421885);
        assert_eq!(trade.quantity_quote, 106.00351082504388); // TODO: It's weird that foreignNotional is parsed as 106.00351082504388
        assert_eq!(trade.quantity_contract, Some(1.0));
        assert_eq!(trade.side, TradeSide::Buy);
    }

    #[test]
    fn inverse_future() {
        let raw_msg = r#"{"table":"trade","action":"partial","data":[{"timestamp":"2021-03-21T01:12:42.361Z","symbol":"XBTM21","side":"Sell","size":8000,"price":62695.5,"tickDirection":"ZeroPlusTick","trdMatchID":"68624a99-e949-33cd-d7e9-63307cf15cfc","grossValue":12760000,"homeNotional":0.1276,"foreignNotional":8000}]}"#;
        let trade = &parse_trade(EXCHANGE_NAME, MarketType::Unknown, raw_msg).unwrap()[0];

        crate::utils::check_trade_fields(
            EXCHANGE_NAME,
            MarketType::InverseFuture,
            "BTC/USD".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg).unwrap(),
            trade,
            raw_msg,
        );
        assert_eq!(
            1616289162361,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(trade.price, 62695.5);
        assert_eq!(trade.quantity_base, 0.1276);
        assert_eq!(trade.quantity_quote, 8000.0);
        assert_eq!(trade.quantity_contract, Some(8000.0));
        assert_eq!(trade.side, TradeSide::Sell);
    }

    #[test]
    fn linear_future() {
        let raw_msg = r#"{"table":"trade","action":"insert","data":[{"timestamp":"2021-03-12T01:46:03.886Z","symbol":"ETHH21","side":"Buy","size":1,"price":0.03191,"tickDirection":"PlusTick","trdMatchID":"a9371640-78d6-53d9-c9e4-31f7b7afb06d","grossValue":3191000,"homeNotional":1,"foreignNotional":0.03191}]}"#;
        let trade = &parse_trade(EXCHANGE_NAME, MarketType::Unknown, raw_msg).unwrap()[0];

        crate::utils::check_trade_fields(
            EXCHANGE_NAME,
            MarketType::LinearFuture,
            "ETH/BTC".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::LinearFuture, raw_msg).unwrap(),
            trade,
            raw_msg,
        );
        assert_eq!(
            1615513563886,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearFuture, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(trade.price, 0.03191);
        assert_eq!(trade.quantity_base, 1.0);
        assert_eq!(trade.quantity_quote, 0.03191);
        assert_eq!(trade.quantity_contract, Some(1.0));
        assert_eq!(trade.side, TradeSide::Buy);
    }

    #[test]
    fn quanto_future() {
        let raw_msg = r#"{"table":"trade","action":"insert","data":[{"timestamp":"2021-03-12T02:13:43.222Z","symbol":"ETHUSDH21","side":"Sell","size":12,"price":1892.8,"tickDirection":"PlusTick","trdMatchID":"14c7d828-80c4-2c91-ad9e-1662081aeaec","grossValue":2271360,"homeNotional":0.6814310051107325,"foreignNotional":1289.8126064735945}]}"#;
        let trade = &parse_trade(EXCHANGE_NAME, MarketType::Unknown, raw_msg).unwrap()[0];

        crate::utils::check_trade_fields(
            EXCHANGE_NAME,
            MarketType::QuantoFuture,
            "ETH/USD".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::QuantoFuture, raw_msg).unwrap(),
            trade,
            raw_msg,
        );
        assert_eq!(
            1615515223222,
            extract_timestamp(EXCHANGE_NAME, MarketType::QuantoFuture, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(trade.price, 1892.8);
        assert_eq!(trade.quantity_base, 0.6814310051107325);
        assert_eq!(trade.quantity_quote, 1289.8126064735943);
        assert_eq!(trade.quantity_contract, Some(12.0));
        assert_eq!(trade.side, TradeSide::Sell);
    }
}

#[cfg(test)]
mod funding_rate {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, extract_timestamp, parse_funding_rate};

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"table":"funding","action":"partial","data":[{"timestamp":"2021-04-01T20:00:00.000Z","symbol":"XBTUSD","fundingInterval":"2000-01-01T08:00:00.000Z","fundingRate":0.000817,"fundingRateDaily":0.002451}]}"#;
        let received_at = 1615515223227;
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
            "XBTUSD",
            extract_symbol(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap()
        );
        assert_eq!(
            None,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap()
        );

        assert_eq!(funding_rates[0].pair, "BTC/USD".to_string());
        assert_eq!(funding_rates[0].funding_rate, 0.000817);
        assert_eq!(funding_rates[0].funding_time, 1617307200000);
        assert_eq!(funding_rates[0].timestamp, received_at);
    }

    #[test]
    fn quanto_swap() {
        let raw_msg = r#"{"table":"funding","action":"partial","data":[{"timestamp":"2021-04-01T20:00:00.000Z","symbol":"ETHUSD","fundingInterval":"2000-01-01T08:00:00.000Z","fundingRate":0.002142,"fundingRateDaily":0.006425999999999999}]}"#;
        let received_at = 1615515223227;
        let funding_rates = &parse_funding_rate(
            EXCHANGE_NAME,
            MarketType::QuantoSwap,
            raw_msg,
            Some(received_at),
        )
        .unwrap();

        assert_eq!(funding_rates.len(), 1);

        for rate in funding_rates.iter() {
            crate::utils::check_funding_rate_fields(
                EXCHANGE_NAME,
                MarketType::QuantoSwap,
                rate,
                raw_msg,
            );
        }
        assert_eq!(
            "ETHUSD",
            extract_symbol(EXCHANGE_NAME, MarketType::QuantoSwap, raw_msg).unwrap()
        );
        assert_eq!(
            None,
            extract_timestamp(EXCHANGE_NAME, MarketType::QuantoSwap, raw_msg).unwrap()
        );

        assert_eq!(funding_rates[0].pair, "ETH/USD".to_string());
        assert_eq!(funding_rates[0].funding_rate, 0.002142);
        assert_eq!(funding_rates[0].funding_time, 1617307200000);
        assert_eq!(funding_rates[0].timestamp, received_at);
    }

    #[test]
    fn all() {
        let raw_msg = r#"{"table":"funding","action":"partial","data":[{"timestamp":"2021-11-02T12:00:00.000Z","symbol":"AAVEUSDT","fundingInterval":"2000-01-01T08:00:00.000Z","fundingRate":0.001941,"fundingRateDaily":0.005823},{"timestamp":"2022-06-02T12:00:00.000Z","symbol":"XBTUSDT","fundingInterval":"2000-01-01T08:00:00.000Z","fundingRate":0.000075,"fundingRateDaily":0.000225}]}"#;
        let received_at = 1615515223227;
        let funding_rates = &parse_funding_rate(
            EXCHANGE_NAME,
            MarketType::Unknown,
            raw_msg,
            Some(received_at),
        )
        .unwrap();

        assert_eq!(funding_rates.len(), 2);

        for rate in funding_rates.iter() {
            crate::utils::check_funding_rate_fields(
                EXCHANGE_NAME,
                MarketType::LinearSwap,
                rate,
                raw_msg,
            );
        }
        assert_eq!(
            "ALL",
            extract_symbol(EXCHANGE_NAME, MarketType::Unknown, raw_msg).unwrap()
        );
        assert_eq!(
            None,
            extract_timestamp(EXCHANGE_NAME, MarketType::Unknown, raw_msg).unwrap()
        );

        assert_eq!(funding_rates[0].pair, "AAVE/USDT".to_string());
        assert_eq!(funding_rates[0].funding_rate, 0.001941);
        assert_eq!(funding_rates[0].funding_time, 1635854400000);
        assert_eq!(funding_rates[0].timestamp, received_at);

        assert_eq!(funding_rates[1].pair, "BTC/USDT".to_string());
        assert_eq!(funding_rates[1].funding_rate, 0.000075);
        assert_eq!(funding_rates[1].funding_time, 1654171200000);
        assert_eq!(funding_rates[1].timestamp, received_at);
    }
}

#[cfg(test)]
mod order_book_l2_25 {
    use super::EXCHANGE_NAME;
    use chrono::prelude::*;
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{
        exchanges::bitmex::price_to_id, extract_symbol, extract_timestamp, parse_l2,
    };
    use crypto_msg_type::MessageType;

    #[test]
    fn inverse_swap_snapshot() {
        let raw_msg = r#"{"table":"orderBookL2_25","action":"partial","data":[{"symbol":"XBTUSD","id":8796381000,"side":"Sell","size":49900,"price":36190},{"symbol":"XBTUSD","id":8796381050,"side":"Sell","size":125714,"price":36189.5},{"symbol":"XBTUSD","id":8796381100,"side":"Sell","size":34600,"price":36189},{"symbol":"XBTUSD","id":8796385500,"side":"Buy","size":136,"price":36145},{"symbol":"XBTUSD","id":8796385600,"side":"Buy","size":26,"price":36144},{"symbol":"XBTUSD","id":8796385800,"side":"Buy","size":18067,"price":36142}]}"#;
        let received_at = Utc::now().timestamp_millis();
        let orderbook = &parse_l2(
            EXCHANGE_NAME,
            MarketType::Unknown,
            raw_msg,
            Some(received_at),
        )
        .unwrap()[0];

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
            None,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg,).unwrap()
        );

        assert_eq!(orderbook.bids[0].price, 36145.0);
        assert_eq!(8796385500, price_to_id("XBTUSD", 36145.0));
        assert_eq!(orderbook.bids[0].quantity_base, 136.0 / 36145.0);
        assert_eq!(orderbook.bids[0].quantity_quote, 136.0);
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 136.0);

        assert_eq!(orderbook.bids[2].price, 36142.0);
        assert_eq!(8796385800, price_to_id("XBTUSD", 36142.0));
        assert_eq!(orderbook.bids[2].quantity_base, 18067.0 / 36142.0);
        assert_eq!(orderbook.bids[2].quantity_quote, 18067.0);
        assert_eq!(orderbook.bids[2].quantity_contract.unwrap(), 18067.0);

        assert_eq!(orderbook.asks[2].price, 36190.0);
        assert_eq!(8796381000, price_to_id("XBTUSD", 36190.0));
        assert_eq!(orderbook.asks[2].quantity_base, 49900.0 / 36190.0);
        assert_eq!(orderbook.asks[2].quantity_quote, 49900.0);
        assert_eq!(orderbook.asks[2].quantity_contract.unwrap(), 49900.0);

        assert_eq!(orderbook.asks[0].price, 36189.0);
        assert_eq!(8796381100, price_to_id("XBTUSD", 36189.0));
        assert_eq!(orderbook.asks[0].quantity_base, 34600.0 / 36189.0);
        assert_eq!(orderbook.asks[0].quantity_quote, 34600.0);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 34600.0);
    }

    #[test]
    fn inverse_swap_update() {
        let insert_msg = r#"{"table":"orderBookL2_25","action":"insert","data":[{"symbol":"XBTUSD","id":8796323950,"side":"Sell","size":38760,"price":36760.5}]}"#;
        let received_at = Utc::now().timestamp_millis();
        let _ = parse_l2(
            EXCHANGE_NAME,
            MarketType::Unknown,
            insert_msg,
            Some(received_at),
        );
        let update_msg = r#"{"table":"orderBookL2_25","action":"update","data":[{"symbol":"XBTUSD","id":8796323950,"side":"Sell","size":36760}]}"#;
        let orderbook = &parse_l2(
            EXCHANGE_NAME,
            MarketType::InverseSwap,
            update_msg,
            Some(Utc::now().timestamp_millis()),
        )
        .unwrap()[0];

        assert_eq!(orderbook.asks.len(), 1);
        assert_eq!(orderbook.bids.len(), 0);
        assert!(!orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            EXCHANGE_NAME,
            MarketType::InverseSwap,
            MessageType::L2Event,
            "BTC/USD".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::InverseSwap, update_msg).unwrap(),
            orderbook,
            update_msg,
        );
        assert_eq!(
            None,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, update_msg,).unwrap()
        );

        assert_eq!(orderbook.asks[0].price, 36760.5);
        assert_eq!(orderbook.asks[0].quantity_base, 36760.0 / 36760.5);
        assert_eq!(orderbook.asks[0].quantity_quote, 36760.0);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 36760.0);

        let delete_msg = r#"{"table":"orderBookL2_25","action":"delete","data":[{"symbol":"XBTUSD","id":8796323950,"side":"Sell"}]}"#;
        let orderbook = &parse_l2(
            EXCHANGE_NAME,
            MarketType::InverseSwap,
            delete_msg,
            Some(Utc::now().timestamp_millis()),
        )
        .unwrap()[0];

        assert_eq!(orderbook.asks.len(), 1);
        assert_eq!(orderbook.bids.len(), 0);
        assert!(!orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            EXCHANGE_NAME,
            MarketType::InverseSwap,
            MessageType::L2Event,
            "BTC/USD".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::InverseSwap, delete_msg).unwrap(),
            orderbook,
            delete_msg,
        );

        assert_eq!(orderbook.asks[0].price, 36760.5);
        assert_eq!(orderbook.asks[0].quantity_base, 0.0);
        assert_eq!(orderbook.asks[0].quantity_quote, 0.0);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 0.0);
    }

    #[test]
    fn linear_future_snapshot() {
        let raw_msg = r#"{"table":"orderBookL2_25","action":"partial","data":[{"symbol":"ETHH22","id":75899993108,"side":"Sell","size":50000,"price":0.06892,"timestamp":"2022-03-01T01:55:45.088Z"},{"symbol":"ETHH22","id":75899993113,"side":"Sell","size":125000,"price":0.06887,"timestamp":"2022-03-01T01:55:45.088Z"},{"symbol":"ETHH22","id":75899993250,"side":"Buy","size":3000,"price":0.0675,"timestamp":"2022-03-01T01:55:45.088Z"},{"symbol":"ETHH22","id":75899993260,"side":"Buy","size":117000,"price":0.0674,"timestamp":"2022-03-01T01:55:45.088Z"}]}"#;
        let received_at = Utc::now().timestamp_millis();
        let orderbook = &parse_l2(
            EXCHANGE_NAME,
            MarketType::Unknown,
            raw_msg,
            Some(received_at),
        )
        .unwrap()[0];

        assert_eq!(orderbook.asks.len(), 2);
        assert_eq!(orderbook.bids.len(), 2);
        assert!(orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            EXCHANGE_NAME,
            MarketType::LinearFuture,
            MessageType::L2Event,
            "ETH/BTC".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::LinearFuture, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );
        assert_eq!(
            1646099745088,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearFuture, raw_msg,)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.bids[0].price, 0.0675);
        assert_eq!(orderbook.bids[0].quantity_base, 0.03);
        assert_eq!(orderbook.bids[0].quantity_quote, 0.03 * 0.0675);
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 3000.0);

        assert_eq!(orderbook.bids[1].price, 0.0674);
        assert_eq!(orderbook.bids[1].quantity_base, 1.170);
        assert_eq!(orderbook.bids[1].quantity_quote, 1.17 * 0.0674);
        assert_eq!(orderbook.bids[1].quantity_contract.unwrap(), 117000.0);

        assert_eq!(orderbook.asks[0].price, 0.06887);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 125000.0);
        assert_eq!(orderbook.asks[0].quantity_base, 1.25);
        assert_eq!(orderbook.asks[0].quantity_quote, 0.06887 * 1.25);

        assert_eq!(orderbook.asks[1].price, 0.06892);
        assert_eq!(orderbook.asks[1].quantity_contract.unwrap(), 50000.0);
        assert_eq!(orderbook.asks[1].quantity_base, 0.5);
        assert_eq!(orderbook.asks[1].quantity_quote, 0.06892 * 0.5);
    }

    #[test]
    fn linear_future_delete() {
        let raw_msg = r#"{"table":"orderBookL2_25","action":"delete","data":[{"symbol":"ETHZ21","id":63399993018,"side":"Buy"}]}"#;
        let received_at = Utc::now().timestamp_millis();
        let orderbook = &parse_l2(
            EXCHANGE_NAME,
            MarketType::Unknown,
            raw_msg,
            Some(received_at),
        )
        .unwrap()[0];

        assert_eq!(orderbook.asks.len(), 0);
        assert_eq!(orderbook.bids.len(), 1);
        assert!(!orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            EXCHANGE_NAME,
            MarketType::LinearFuture,
            MessageType::L2Event,
            "ETH/BTC".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::LinearFuture, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );
        assert_eq!(
            None,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearFuture, raw_msg,).unwrap()
        );

        assert_eq!(orderbook.bids[0].price, 0.06982);
        assert_eq!(orderbook.bids[0].quantity_base, 0.0);
        assert_eq!(orderbook.bids[0].quantity_quote, 0.0);
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 0.0);
    }

    #[test]
    fn quanto_swap() {
        let raw_msg = r#"{"table":"orderBookL2","action":"insert","data":[{"symbol":"ETHUSD","id":29699964036,"side":"Buy","size":93,"price":1798.2,"timestamp":"2022-06-04T23:34:52.603Z"}]}"#;

        assert_eq!(
            "ETHUSD",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );
        assert_eq!(
            1654385692603,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
    }

    #[test]
    fn quanto_future() {
        let raw_msg = r#"{"table":"orderBookL2","action":"insert","data":[{"symbol":"ETHUSDM22","id":81499963133,"side":"Sell","size":5,"price":1843.35,"timestamp":"2022-06-04T23:46:20.175Z"}]}"#;

        assert_eq!(
            "ETHUSDM22",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );
        assert_eq!(
            1654386380175,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
    }
}

#[cfg(test)]
mod l2_topk {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, extract_timestamp, parse_l2_topk, round};
    use crypto_msg_type::MessageType;

    #[test]
    fn inverse_future() {
        let raw_msg = r#"{"table":"orderBook10","action":"update","data":[{"symbol":"XBTM22","bids":[[31530.5,1800],[31530,7000],[31529,1700],[31528.5,6300],[31525,1400],[31524.5,5800],[31524,15900],[31523.5,300],[31522.5,2100],[31522,12200]],"timestamp":"2022-05-30T22:19:48.301Z","asks":[[31570.5,7000],[31571,19900],[31571.5,5000],[31573,233200],[31582.5,1900],[31587,174500],[31590,142000],[31591,41500],[31599.5,2000],[31601.5,429900]]}]}"#;
        let orderbook =
            &parse_l2_topk(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 10);
        assert_eq!(orderbook.bids.len(), 10);
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
            1653949188301,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseFuture, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1653949188301);
        assert_eq!(orderbook.seq_id, None);
        assert_eq!(orderbook.prev_seq_id, None);

        assert_eq!(orderbook.bids[0].price, 31530.5);
        assert_eq!(orderbook.bids[0].quantity_base, 1800.0 / 31530.5);
        assert_eq!(orderbook.bids[0].quantity_quote, 1800.0);
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 1800.0);

        assert_eq!(orderbook.bids[9].price, 31522.0);
        assert_eq!(orderbook.bids[9].quantity_base, 12200.0 / 31522.0);
        assert_eq!(orderbook.bids[9].quantity_quote, 12200.0);
        assert_eq!(orderbook.bids[9].quantity_contract.unwrap(), 12200.0);

        assert_eq!(orderbook.asks[0].price, 31570.5);
        assert_eq!(orderbook.asks[0].quantity_base, 7000.0 / 31570.5);
        assert_eq!(orderbook.asks[0].quantity_quote, 7000.0);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 7000.0);

        assert_eq!(orderbook.asks[9].price, 31601.5);
        assert_eq!(orderbook.asks[9].quantity_base, 429900.0 / 31601.5);
        assert_eq!(orderbook.asks[9].quantity_quote, 429900.0);
        assert_eq!(orderbook.asks[9].quantity_contract.unwrap(), 429900.0);
    }

    #[test]
    fn linear_future() {
        let raw_msg = r#"{"table":"orderBook10","action":"partial","data":[{"symbol":"ETHM22","bids":[[0.06233,256000],[0.06232,1000000],[0.06231,9000],[0.0623,8000],[0.06229,10000],[0.06228,9000],[0.06227,8000],[0.06226,10000],[0.06225,9000],[0.06224,9000]],"asks":[[0.06263,131000],[0.06264,480000],[0.06266,9000],[0.06267,106000],[0.06268,10000],[0.06269,27000],[0.06274,9000],[0.06275,9000],[0.06276,5000000],[0.0628,12000]],"timestamp":"2022-05-30T21:33:22.996Z"}]}"#;
        let orderbook =
            &parse_l2_topk(EXCHANGE_NAME, MarketType::LinearFuture, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 10);
        assert_eq!(orderbook.bids.len(), 10);
        assert!(orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            EXCHANGE_NAME,
            MarketType::LinearFuture,
            MessageType::L2TopK,
            "ETH/BTC".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::LinearFuture, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );
        assert_eq!(
            1653946402996,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearFuture, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1653946402996);
        assert_eq!(orderbook.seq_id, None);
        assert_eq!(orderbook.prev_seq_id, None);

        assert_eq!(orderbook.bids[0].price, 0.06233);
        assert_eq!(orderbook.bids[0].quantity_base, 2.56);
        assert_eq!(orderbook.bids[0].quantity_quote, 2.56 * 0.06233);
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 256000.0);

        assert_eq!(orderbook.bids[9].price, 0.06224);
        assert_eq!(orderbook.bids[9].quantity_base, 0.09);
        assert_eq!(orderbook.bids[9].quantity_quote, round(0.06224 * 0.09));
        assert_eq!(orderbook.bids[9].quantity_contract.unwrap(), 9000.0);

        assert_eq!(orderbook.asks[0].price, 0.06263);
        assert_eq!(orderbook.asks[0].quantity_base, 1.31);
        assert_eq!(orderbook.asks[0].quantity_quote, round(0.06263 * 1.31));
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 131000.0);

        assert_eq!(orderbook.asks[9].price, 0.0628);
        assert_eq!(orderbook.asks[9].quantity_base, 0.12);
        assert_eq!(orderbook.asks[9].quantity_quote, round(0.0628 * 0.12));
        assert_eq!(orderbook.asks[9].quantity_contract.unwrap(), 12000.0);
    }

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"table":"orderBook10","action":"update","data":[{"symbol":"XBTUSD","bids":[[30715.5,217100],[30713,3000],[30711.5,30500],[30711,120100],[30710.5,131200],[30710,7200],[30709,6100],[30707.5,60000],[30707,36800],[30706.5,142100]],"timestamp":"2022-05-30T19:20:46.586Z","asks":[[30716,537700],[30716.5,32200],[30717,400],[30720,7200],[30723.5,7900],[30725,100],[30727,100],[30727.5,3600],[30728,12400],[30728.5,19200]]}]}"#;
        let orderbook =
            &parse_l2_topk(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 10);
        assert_eq!(orderbook.bids.len(), 10);
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
            1653938446586,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1653938446586);
        assert_eq!(orderbook.seq_id, None);
        assert_eq!(orderbook.prev_seq_id, None);

        assert_eq!(orderbook.bids[0].price, 30715.5);
        assert_eq!(orderbook.bids[0].quantity_base, 217100.0 / 30715.5);
        assert_eq!(orderbook.bids[0].quantity_quote, 217100.0);
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 217100.0);

        assert_eq!(orderbook.bids[9].price, 30706.5);
        assert_eq!(orderbook.bids[9].quantity_base, 142100.0 / 30706.5);
        assert_eq!(orderbook.bids[9].quantity_quote, 142100.0);
        assert_eq!(orderbook.bids[9].quantity_contract.unwrap(), 142100.0);

        assert_eq!(orderbook.asks[0].price, 30716.0);
        assert_eq!(orderbook.asks[0].quantity_base, 537700.0 / 30716.0);
        assert_eq!(orderbook.asks[0].quantity_quote, 537700.0);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 537700.0);

        assert_eq!(orderbook.asks[9].price, 30728.5);
        assert_eq!(orderbook.asks[9].quantity_base, 19200.0 / 30728.5);
        assert_eq!(orderbook.asks[9].quantity_quote, 19200.0);
        assert_eq!(orderbook.asks[9].quantity_contract.unwrap(), 19200.0);
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"table":"orderBook10","action":"update","data":[{"symbol":"XBTUSDT","asks":[[31650.5,4000],[31656.5,900000],[31657,316000],[31664,1220000],[31665,1500000],[31666,4072000],[31672,33000],[31676,1054000],[31678.5,344000],[31679,443000]],"timestamp":"2022-05-30T22:24:58.013Z","bids":[[31626.5,242000],[31626,1620000],[31620.5,316000],[31620,800000],[31616.5,4000],[31615,818000],[31614.5,834000],[31614,1611000],[31613.5,6416000],[31606,349000]]}]}"#;
        let orderbook =
            &parse_l2_topk(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 10);
        assert_eq!(orderbook.bids.len(), 10);
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
            1653949498013,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1653949498013);
        assert_eq!(orderbook.seq_id, None);
        assert_eq!(orderbook.prev_seq_id, None);

        assert_eq!(orderbook.bids[0].price, 31626.5);
        assert_eq!(orderbook.bids[0].quantity_base, 0.00242);
        assert_eq!(orderbook.bids[0].quantity_quote, 31626.5 * 0.00242);
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 242000.0);

        assert_eq!(orderbook.bids[9].price, 31606.0);
        assert_eq!(orderbook.bids[9].quantity_base, 0.00349);
        assert_eq!(orderbook.bids[9].quantity_quote, 0.00349 * 31606.0);
        assert_eq!(orderbook.bids[9].quantity_contract.unwrap(), 349000.0);

        assert_eq!(orderbook.asks[0].price, 31650.5);
        assert_eq!(orderbook.asks[0].quantity_base, 0.00004);
        assert_eq!(orderbook.asks[0].quantity_quote, round(0.00004 * 31650.5));
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 4000.0);

        assert_eq!(orderbook.asks[9].price, 31679.0);
        assert_eq!(orderbook.asks[9].quantity_base, 0.00443);
        assert_eq!(orderbook.asks[9].quantity_quote, round(31679.0 * 0.00443));
        assert_eq!(orderbook.asks[9].quantity_contract.unwrap(), 443000.0);
    }

    #[test]
    fn quanto_swap() {
        let raw_msg = r#"{"table":"orderBook10","action":"update","data":[{"symbol":"ETHUSD","asks":[[1801.2,75],[1801.35,600],[1801.65,94],[1801.7,600],[1801.75,50],[1801.8,50],[1801.95,50],[1802.5,3534],[1802.8,4],[1802.9,360]],"timestamp":"2022-06-04T23:48:43.562Z","bids":[[1801.15,10],[1800.95,85],[1800.55,93],[1800.5,148],[1799.95,14],[1799.85,473],[1799.65,102],[1799.6,50],[1799.55,227],[1799.5,560]]}]}"#;

        assert_eq!(
            "ETHUSD",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );
        assert_eq!(
            1654386523562,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
    }

    #[test]
    fn quanto_future() {
        let raw_msg = r#"{"table":"orderBook10","action":"update","data":[{"symbol":"ETHUSDM22","bids":[[1842.1,10],[1841.45,5],[1841.4,522],[1841.35,1300],[1840.55,4],[1840.25,5],[1840.2,904],[1840.15,2260],[1838.85,1144],[1838.8,3380]],"timestamp":"2022-06-04T23:49:27.444Z","asks":[[1844.9,19],[1844.95,1277],[1845,10],[1846.15,91],[1846.2,1965],[1846.85,6],[1847.7,1908],[1847.85,1],[1849.2,42],[1849.25,3261]]}]}"#;

        assert_eq!(
            "ETHUSDM22",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );
        assert_eq!(
            1654386567444,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
    }
}

#[cfg(test)]
mod bbo {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, extract_timestamp};

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"table":"quote","action":"insert","data":[{"timestamp":"2022-05-31T15:21:51.493Z","symbol":"XBTUSD","bidSize":200,"bidPrice":31583.5,"askPrice":31584,"askSize":156000}]}"#;

        assert_eq!(
            1654010511493,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "XBTUSD",
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"table":"orderBookL2","action":"update","data":[{"symbol":"XBTUSDT","id":73199935756,"side":"Buy","size":203000,"timestamp":"2022-05-31T15:53:31.605Z"}]}"#;

        assert_eq!(
            1654012411605,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "XBTUSDT",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );
    }
}

#[cfg(test)]
mod candlestick {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, extract_timestamp};

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"table":"tradeBin1m","action":"insert","data":[{"timestamp":"2022-06-01T10:03:00.000Z","symbol":"XBTUSD","open":31639.5,"high":31639.5,"low":31635.5,"close":31635.5,"trades":6,"volume":101400,"vwap":31639.3619,"lastSize":200,"turnover":320487014,"homeNotional":3.2048701399999997,"foreignNotional":101400}]}"#;

        assert_eq!(
            1654077780000,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "XBTUSD",
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"table":"tradeBin1m","action":"insert","data":[{"timestamp":"2022-06-01T10:10:00.000Z","symbol":"XBTUSDT","open":31592.5,"high":31600,"low":31582.5,"close":31583.5,"trades":5,"volume":1634000,"vwap":31587.127,"lastSize":1233000,"turnover":51613365500,"homeNotional":1.6340000000000001,"foreignNotional":51613.3655}]}"#;

        assert_eq!(
            1654078200000,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "XBTUSDT",
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
    fn inverse_swap() {
        let raw_msg = r#"[{"symbol": "XBTUSD", "id": 8700000000, "side": "Sell", "size": 1007600, "price": 1000000}, {"symbol": "XBTUSD", "id": 8733748000, "side": "Sell", "size": 10000, "price": 662520}, {"symbol": "XBTUSD", "id": 8734110000, "side": "Sell", "size": 20000, "price": 658900}, {"symbol": "XBTUSD", "id": 8799999850, "side": "Buy", "size": 6000, "price": 1.5}, {"symbol": "XBTUSD", "id": 8799999900, "side": "Buy", "size": 500, "price": 1}, {"symbol": "XBTUSD", "id": 8799999950, "side": "Buy", "size": 1500, "price": 0.5}]"#;

        assert_eq!(
            "XBTUSD",
            extract_symbol(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap()
        );
        assert_eq!(
            None,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg).unwrap()
        );
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"[{"symbol": "XBTUSDT", "id": 73199872654, "side": "Sell", "size": 6000, "price": 63673}, {"symbol": "XBTUSDT", "id": 73199896000, "side": "Sell", "size": 5000, "price": 52000}, {"symbol": "XBTUSDT", "id": 73199899220, "side": "Sell", "size": 24000, "price": 50390}, {"symbol": "XBTUSDT", "id": 73199999997, "side": "Buy", "size": 16000000, "price": 1.5}, {"symbol": "XBTUSDT", "id": 73199999998, "side": "Buy", "size": 30000000, "price": 1}, {"symbol": "XBTUSDT", "id": 73199999999, "side": "Buy", "size": 85000000, "price": 0.5}]"#;

        assert_eq!(
            "XBTUSDT",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );
        assert_eq!(
            None,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );
    }
}
