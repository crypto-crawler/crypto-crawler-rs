mod utils;

const EXCHANGE_NAME: &str = "bitfinex";

#[cfg(test)]
mod trade {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_message::TradeSide;
    use crypto_msg_parser::{extract_symbol, extract_timestamp, parse_trade, round};

    #[test]
    fn spot_te() {
        let raw_msg = r#"[{"symbol":"tBTCUST","channel":"trades"},"te",[637771130,1615232733897,0.11546588,51350]]"#;
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
            1615232733897,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(trade.quantity_base, 0.11546588);
        assert_eq!(trade.quantity_quote, round(0.11546588 * 51350.0));
        assert_eq!(trade.quantity_contract, None);

        assert_eq!(trade.side, TradeSide::Buy);
    }

    #[test]
    fn spot_tu() {
        let raw_msg = r#"[{"symbol":"tBTCUST","channel":"trades"},"tu",[637771130,1615232733897,0.11546588,51350]]"#;
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
            1615232733897,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(trade.quantity_base, 0.11546588);
        assert_eq!(trade.quantity_quote, round(0.11546588 * 51350.0));
        assert_eq!(trade.quantity_contract, None);

        assert_eq!(trade.side, TradeSide::Buy);
    }

    #[test]
    fn spot_snapshot() {
        let raw_msg = r#"[{"channel":"trades","symbol":"tBTCUST"},[[647229117,1616217509543,0.0033,58239],[647229114,1616217326462,0.05605347,58296],[647229113,1616217326462,0.00102018,58296]]]"#;
        let trades = &parse_trade(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap();

        assert_eq!(trades.len(), 3);

        for trade in trades.iter() {
            crate::utils::check_trade_fields(
                EXCHANGE_NAME,
                MarketType::Spot,
                "BTC/USDT".to_string(),
                extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap(),
                trade,
                raw_msg,
            );
        }
        assert_eq!(
            1616217509543,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );
    }

    #[test]
    fn swap_te() {
        let raw_msg = r#"[{"channel":"trades","symbol":"tBTCF0:USTF0"},"te",[647256282,1616219711336,0.00020449,58244]]"#;
        let trade = &parse_trade(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()[0];

        crate::utils::check_trade_fields(
            EXCHANGE_NAME,
            MarketType::LinearSwap,
            "BTC/USDT".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap(),
            trade,
            raw_msg,
        );
        assert_eq!(
            1616219711336,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(trade.quantity_base, 0.00020449);
        assert_eq!(trade.quantity_quote, round(0.00020449 * 58244.0));
        assert_eq!(trade.quantity_contract, Some(0.00020449));

        assert_eq!(trade.side, TradeSide::Buy);
    }

    #[test]
    fn swap_tu() {
        let raw_msg = r#"[{"channel":"trades","symbol":"tBTCF0:USTF0"},"tu",[647256282,1616219711336,0.00020449,58244]]"#;
        let trade = &parse_trade(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()[0];

        crate::utils::check_trade_fields(
            EXCHANGE_NAME,
            MarketType::LinearSwap,
            "BTC/USDT".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap(),
            trade,
            raw_msg,
        );
        assert_eq!(
            1616219711336,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(trade.quantity_base, 0.00020449);
        assert_eq!(trade.quantity_quote, round(0.00020449 * 58244.0));
        assert_eq!(trade.quantity_contract, Some(0.00020449));

        assert_eq!(trade.side, TradeSide::Buy);
    }

    #[test]
    fn swap_snapshot() {
        let raw_msg = r#"[{"channel":"trades","symbol":"tBTCF0:USTF0"},[[647256201,1616219105954,-0.06153795,58119],[647256191,1616219094921,0.0257,58138],[647256188,1616219088734,0.01679516,58138]]]"#;
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
        }
        assert_eq!(
            1616219105954,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );
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
    fn spot_snapshot() {
        let raw_msg = r#"[{"symbol":"tBTCUST","len":"25","freq":"F0","channel":"book","prec":"P0"},[[36167,1,0.48403686],[36162,2,0.22625024],[36161,1,0.43250047],[36158,1,0.209],[36155,2,0.48229814],[36171,1,-0.000006],[36172,1,-0.0002],[36173,1,-0.0002],[36174,2,-0.0102],[36175,1,-0.0002]]]"#;
        let received_at = Utc::now().timestamp_millis();
        let orderbook =
            &parse_l2(EXCHANGE_NAME, MarketType::Spot, raw_msg, Some(received_at)).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 5);
        assert_eq!(orderbook.bids.len(), 5);
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

        assert_eq!(orderbook.bids[0].price, 36167.0);
        assert_eq!(orderbook.bids[0].quantity_base, 0.48403686);
        assert_eq!(orderbook.bids[0].quantity_quote, 36167.0 * 0.48403686);

        assert_eq!(orderbook.bids[4].price, 36155.0);
        assert_eq!(orderbook.bids[4].quantity_base, 0.48229814);
        assert_eq!(orderbook.bids[4].quantity_quote, 36155.0 * 0.48229814);

        assert_eq!(orderbook.asks[0].price, 36171.0);
        assert_eq!(orderbook.asks[0].quantity_base, 0.000006);
        assert_eq!(orderbook.asks[0].quantity_quote, 36171.0 * 0.000006);

        assert_eq!(orderbook.asks[4].price, 36175.0);
        assert_eq!(orderbook.asks[4].quantity_base, 0.0002);
        assert_eq!(orderbook.asks[4].quantity_quote, 36175.0 * 0.0002);
    }

    #[test]
    fn spot_update() {
        let raw_msg = r#"[{"symbol":"tBTCUST","channel":"book","len":"25","freq":"F0","prec":"P0"},[34668,1,-0.00813136]]"#;
        let received_at = Utc::now().timestamp_millis();
        let orderbook =
            &parse_l2(EXCHANGE_NAME, MarketType::Spot, raw_msg, Some(received_at)).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 1);
        assert_eq!(orderbook.bids.len(), 0);
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

        assert_eq!(orderbook.asks[0].price, 34668.0);
        assert_eq!(orderbook.asks[0].quantity_base, 0.00813136);
        assert_eq!(
            orderbook.asks[0].quantity_quote,
            round(34668.0 * 0.00813136)
        );
    }

    #[test]
    fn linear_swap_snapshot() {
        let raw_msg = r#"[{"freq":"F0","channel":"book","prec":"P0","len":"25","symbol":"tBTCF0:USTF0"},[[34840,2,0.20047952],[34837,1,0.17573],[34829,1,0.0857],[34828,1,0.17155],[34826,2,0.25510833],[34841,1,-0.00034929],[34843,4,-0.70368583],[34844,1,-0.51672161],[34845,2,-0.78960194],[34846,1,-1.0339621]]]"#;
        let received_at = Utc::now().timestamp_millis();
        let orderbook = &parse_l2(
            EXCHANGE_NAME,
            MarketType::LinearSwap,
            raw_msg,
            Some(received_at),
        )
        .unwrap()[0];

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
            None,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );

        assert_eq!(orderbook.bids[0].price, 34840.0);
        assert_eq!(orderbook.bids[0].quantity_base, 0.20047952);
        assert_eq!(orderbook.bids[0].quantity_quote, 34840.0 * 0.20047952);
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 0.20047952);

        assert_eq!(orderbook.bids[4].price, 34826.0);
        assert_eq!(orderbook.bids[4].quantity_base, 0.25510833);
        assert_eq!(orderbook.bids[4].quantity_quote, 34826.0 * 0.25510833);
        assert_eq!(orderbook.bids[4].quantity_contract.unwrap(), 0.25510833);

        assert_eq!(orderbook.asks[0].price, 34841.0);
        assert_eq!(orderbook.asks[0].quantity_base, 0.00034929);
        assert_eq!(orderbook.asks[0].quantity_quote, 34841.0 * 0.00034929);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 0.00034929);

        assert_eq!(orderbook.asks[4].price, 34846.0);
        assert_eq!(orderbook.asks[4].quantity_base, 1.0339621);
        assert_eq!(orderbook.asks[4].quantity_quote, 34846.0 * 1.0339621);
        assert_eq!(orderbook.asks[4].quantity_contract.unwrap(), 1.0339621);
    }

    #[test]
    fn linear_swap_update() {
        let raw_msg = r#"[{"freq":"F0","symbol":"tBTCF0:USTF0","channel":"book","len":"25","prec":"P0"},[34442,2,2.27726294]]"#;
        let received_at = Utc::now().timestamp_millis();
        let orderbook = &parse_l2(
            EXCHANGE_NAME,
            MarketType::LinearSwap,
            raw_msg,
            Some(received_at),
        )
        .unwrap()[0];

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
            None,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );

        assert_eq!(orderbook.bids[0].price, 34442.0);
        assert_eq!(orderbook.bids[0].quantity_base, 2.27726294);
        assert_eq!(orderbook.bids[0].quantity_quote, 34442.0 * 2.27726294);
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 2.27726294);
    }
}

#[cfg(test)]
mod bbo {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_msg_type::MessageType;
    use crypto_msg_parser::{extract_symbol, extract_timestamp, parse_bbo};

    #[test]
    fn spot_snapshot() {
        let raw_msg = r#"[{"len":"1","prec":"R0","freq":"F0","symbol":"tBTCUST","channel":"book"},[[96064678342,31630,0.0007111],[96063747304,31643,-0.01576]]]"#;

        assert_eq!(
            None,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );
        assert_eq!(
            "tBTCUST",
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );

        let received_at = 1651122265862;
        let bbo_msg =
            parse_bbo(EXCHANGE_NAME, MarketType::Spot, raw_msg, Some(received_at)).unwrap();

        assert_eq!(MessageType::BBO, bbo_msg.msg_type);
        assert_eq!("tBTCUST", bbo_msg.symbol);
        assert_eq!(received_at, bbo_msg.timestamp);
        assert_eq!(None, bbo_msg.id);

        assert_eq!(96064678342.0, bbo_msg.ask_price);
        assert_eq!(31630.0, bbo_msg.ask_quantity_base);
        assert_eq!(96064678342.0 * 31630.0, bbo_msg.ask_quantity_quote);
        assert_eq!(None, bbo_msg.ask_quantity_contract);

        assert_eq!(96063747304.0, bbo_msg.bid_price);
        assert_eq!(31643.0, bbo_msg.bid_quantity_base);
        assert_eq!(96063747304.0 * 31643.0, bbo_msg.bid_quantity_quote);
        assert_eq!(None, bbo_msg.bid_quantity_contract);
    }

    #[test]
    fn spot_update() {
        let raw_msg = r#"[{"len":"1","prec":"R0","freq":"F0","symbol":"tBTCUST","channel":"book"},[96064730405,31631,0.01581346]]"#;

        assert_eq!(
            None,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );
        assert_eq!(
            "tBTCUST",
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );
    }

    #[test]
    fn linear_swap_snapshot() {
        let raw_msg = r#"[{"channel":"book","freq":"F0","symbol":"tBTCF0:USTF0","prec":"R0","len":"1"},[[96065326882,31606,0.00339553],[96065369152,31609,-0.40545978]]]"#;

        assert_eq!(
            None,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );
        assert_eq!(
            "tBTCF0:USTF0",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );
        let received_at = 1651122265862;
        let bbo_msg =
            parse_bbo(EXCHANGE_NAME, MarketType::Spot, raw_msg, Some(received_at)).unwrap();

        assert_eq!(MessageType::BBO, bbo_msg.msg_type);
        assert_eq!("tBTCF0:USTF0", bbo_msg.symbol);
        assert_eq!(received_at, bbo_msg.timestamp);
        assert_eq!(None, bbo_msg.id);

        assert_eq!(96065326882.0, bbo_msg.ask_price);
        assert_eq!(31606.0, bbo_msg.ask_quantity_base);
        assert_eq!(96065326882.0 * 31606.0, bbo_msg.ask_quantity_quote);
        assert_eq!(None, bbo_msg.ask_quantity_contract);

        assert_eq!(96065369152.0, bbo_msg.bid_price);
        assert_eq!(31609.0, bbo_msg.bid_quantity_base);
        assert_eq!(96065369152.0 * 31609.0, bbo_msg.bid_quantity_quote);
        assert_eq!(None, bbo_msg.bid_quantity_contract);
    }

    #[test]
    fn linear_swap_update() {
        let raw_msg = r#"[{"channel":"book","freq":"F0","symbol":"tBTCF0:USTF0","prec":"R0","len":"1"},[96065371804,31609,-0.40545978]]"#;

        assert_eq!(
            None,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );
        assert_eq!(
            "tBTCF0:USTF0",
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
    fn spot_snapshot() {
        let raw_msg = r#"[{"len":"250","symbol":"tBTCUST","channel":"book","prec":"R0","freq":"F0"},[[96124382782,31534,0.0285],[96124397723,31534,0.01],[96118584550,31532,0.01586],[96118584551,31544,-0.01585],[96124364148,31544,-0.27332593],[96124396297,31547,-0.6338]]]"#;

        assert_eq!(
            None,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );
        assert_eq!(
            "tBTCUST",
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );
    }

    #[test]
    fn spot_update() {
        let raw_msg = r#"[{"len":"250","symbol":"tBTCUST","channel":"book","prec":"R0","freq":"F0"},[96118584550,31535,0.01586]]"#;

        assert_eq!(
            None,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );
        assert_eq!(
            "tBTCUST",
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );
    }

    #[test]
    fn linear_swap_snapshot() {
        let raw_msg = r#"[{"freq":"F0","channel":"book","prec":"R0","symbol":"tBTCF0:USTF0","len":"250"},[[96124920207,31556,0.19100648],[96124877610,31555,0.031],[96124911151,31555,0.25466876],[96124920217,31557,-0.19103873],[96124919043,31558,-0.25474405],[96124858873,31560,-0.31772226]]]"#;

        assert_eq!(
            None,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );
        assert_eq!(
            "tBTCF0:USTF0",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );
    }

    #[test]
    fn linear_swap_update() {
        let raw_msg = r#"[{"freq":"F0","channel":"book","prec":"R0","symbol":"tBTCF0:USTF0","len":"250"},[96124877612,31555,0.039]]"#;

        assert_eq!(
            None,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );
        assert_eq!(
            "tBTCF0:USTF0",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );
    }
}

#[cfg(test)]
mod candlestick {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_msg_type::MessageType;
    use crypto_msg_parser::{extract_symbol, extract_timestamp, parse_candlestick};

    #[test]
    fn spot_snapshot() {
        let raw_msg = r#"[{"key":"trade:1m:tBTCUST","channel":"candles"},[[1654074480000,31636,31636,31636,31636,0.0001],[1654074420000,31633,31631,31640,31631,0.11289119],[1654074300000,31631,31626,31631,31626,0.00047848]]]"#;

        assert_eq!(
            1654074480000,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "tBTCUST",
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );
    }

    #[test]
    fn spot_update() {
        let raw_msg = r#"[{"channel":"candles","key":"trade:1m:tBTCUST"},[1654075080000,31619,31619,31619,31619,0.00843875]]"#;

        assert_eq!(
            1654075080000,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "tBTCUST",
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );
        let data = parse_candlestick(EXCHANGE_NAME, MarketType::Spot, raw_msg, MessageType::L2TopK).unwrap();

        assert_eq!(1654075080000, data.timestamp);
        assert_eq!("1m", data.period);
    }

    #[test]
    fn linear_swap_snapshot() {
        let raw_msg = r#"[{"channel":"candles","key":"trade:1m:tBTCF0:USTF0"},[[1654076100000,31672,31667,31672,31667,0.053312790000000006],[1654076040000,31672,31673,31673,31667,0.00118434],[1654075980000,31669,31672,31672,31669,0.0008369499999999999]]]"#;

        assert_eq!(
            1654076100000,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "tBTCF0:USTF0",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );
    }

    #[test]
    fn linear_swap_update() {
        let raw_msg = r#"[{"channel":"candles","key":"trade:1m:tBTCF0:USTF0"},[1654076040000,31672,31673,31673,31667,0.00118434]]"#;

        assert_eq!(
            1654076040000,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "tBTCF0:USTF0",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );
        let data = parse_candlestick(EXCHANGE_NAME, MarketType::Spot, raw_msg, MessageType::L2TopK).unwrap();

        assert_eq!(1654076040000, data.timestamp);
        assert_eq!("1m", data.period);
    }
}

#[cfg(test)]
mod ticker {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, extract_timestamp};

    #[test]
    fn spot() {
        let raw_msg = r#"[{"symbol":"tBTCUST","channel":"ticker"},[29967,8.32497516,29976,13.30144555,-1674,-0.0529,29966,488.04182112,31887,29335]]"#;

        assert_eq!(
            None,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );
        assert_eq!(
            "tBTCUST",
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"[{"symbol":"tBTCF0:USTF0","channel":"ticker"},[29936,25.086598379999998,29940,38.29793123,-1692,-0.0535,29940,3957.33360529,31878,29308]]"#;

        assert_eq!(
            None,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );
        assert_eq!(
            "tBTCF0:USTF0",
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
        let raw_msg = r#"[[30428,1,0.01],[30426,1,0.1],[30424,1,0.2954],[30423,1,0.3333],[30422,3,0.72231346],[30420,2,0.3349],[30416,2,0.29700845],[30415,3,0.482257],[30414,1,0.4],[30413,1,0.15439084]]"#;

        assert_eq!(
            "NONE",
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );
        assert_eq!(
            None,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"[[28293,1,0.0350506],[28291,1,0.0526735],[28289,2,0.1037385],[28287,1,0.1059222],[28285,1,0.1324028],[28284,1,0.1765371],[28282,1,0.2206713],[28280,1,0.2427385],[28277,1,0.2648056]]"#;

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
