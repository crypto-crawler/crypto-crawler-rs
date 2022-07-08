mod utils;

const EXCHANGE_NAME: &str = "zb";

#[cfg(test)]
mod trade {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_message::TradeSide;
    use crypto_msg_parser::{extract_symbol, extract_timestamp, parse_trade, round};

    #[test]
    fn spot() {
        let raw_msg = r#"{"data":[{"date":1653774784,"amount":"0.0380","price":"29029.5","trade_type":"ask","type":"sell","tid":2796890056},{"date":1653774785,"amount":"0.0001","price":"29041.49","trade_type":"bid","type":"buy","tid":2796890057}],"dataType":"trades","channel":"btcusdt_trades"}"#;

        let trades = &parse_trade(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap();
        assert_eq!(2, trades.len());
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
            1653774785000,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(1653774784000, trade.timestamp);

        assert_eq!(trade.price, 29029.5);
        assert_eq!(trade.quantity_base, 0.038);
        assert_eq!(trade.quantity_quote, 29029.5 * 0.038);
        assert_eq!(trade.quantity_contract, None);
        assert_eq!(trade.side, TradeSide::Sell);
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"channel":"BTC_USDT.Trade","data":[[29011.85,0.441,1,1653774742]]}"#;

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
            1653774742000,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(trade.timestamp, 1653774742000);
        assert_eq!(trade.price, 29011.85);
        assert_eq!(trade.quantity_contract, Some(0.441));
        assert_eq!(trade.quantity_base, 0.441);
        assert_eq!(trade.quantity_quote, round(0.441 * 29011.85));
        assert_eq!(trade.side, TradeSide::Buy);
    }
}

#[cfg(test)]
mod l2_event {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, extract_timestamp, parse_l2, round};
    use crypto_msg_type::MessageType;

    #[test]
    fn linear_swap_snapshot() {
        let raw_msg = r#"{"channel":"BTC_USDT.Depth","type":"Whole","data":{"asks":[[31676.32,0.06],[31676.36,0.106],[31676.49,0.03]],"bids":[[31602.01,0.06],[31601.98,0.106],[31593.12,0.276]],"time":"1654002963803"}}"#;
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
            1654002963803,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1654002963803);
        assert_eq!(orderbook.seq_id, None);
        assert_eq!(orderbook.prev_seq_id, None);

        assert_eq!(orderbook.asks[0].price, 31676.32);
        assert_eq!(orderbook.asks[0].quantity_base, 0.06);
        assert_eq!(orderbook.asks[0].quantity_quote, round(31676.32 * 0.06));
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 0.06);

        assert_eq!(orderbook.asks[2].price, 31676.49);
        assert_eq!(orderbook.asks[2].quantity_base, 0.03);
        assert_eq!(orderbook.asks[2].quantity_quote, 31676.49 * 0.03);
        assert_eq!(orderbook.asks[2].quantity_contract.unwrap(), 0.03);

        assert_eq!(orderbook.bids[0].price, 31602.01);
        assert_eq!(orderbook.bids[0].quantity_base, 0.06);
        assert_eq!(orderbook.bids[0].quantity_quote, round(31602.01 * 0.06));
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 0.06);

        assert_eq!(orderbook.bids[2].price, 31593.12);
        assert_eq!(orderbook.bids[2].quantity_base, 0.276);
        assert_eq!(orderbook.bids[2].quantity_quote, 31593.12 * 0.276);
        assert_eq!(orderbook.bids[2].quantity_contract.unwrap(), 0.276);
    }

    #[test]
    fn linear_swap_update() {
        let raw_msg = r#"{"channel":"BTC_USDT.Depth","data":{"bids":[[31526.35,0.176],[31526.45,0.006],[31551.55,0]],"asks":[[31765.4,0.332],[31765.59,0],[31765.6,0]],"time":"1654003817266"}}"#;
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
            1654003817266,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1654003817266);
        assert_eq!(orderbook.seq_id, None);
        assert_eq!(orderbook.prev_seq_id, None);

        assert_eq!(orderbook.asks[0].price, 31765.4);
        assert_eq!(orderbook.asks[0].quantity_base, 0.332);
        assert_eq!(orderbook.asks[0].quantity_quote, round(31765.4 * 0.332));
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 0.332);

        assert_eq!(orderbook.asks[2].price, 31765.6);
        assert_eq!(orderbook.asks[2].quantity_base, 0.0);
        assert_eq!(orderbook.asks[2].quantity_quote, 0.0);
        assert_eq!(orderbook.asks[2].quantity_contract.unwrap(), 0.0);

        assert_eq!(orderbook.bids[0].price, 31526.35);
        assert_eq!(orderbook.bids[0].quantity_base, 0.176);
        assert_eq!(orderbook.bids[0].quantity_quote, round(31526.35 * 0.176));
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 0.176);

        assert_eq!(orderbook.bids[2].price, 31551.55);
        assert_eq!(orderbook.bids[2].quantity_base, 0.0);
        assert_eq!(orderbook.bids[2].quantity_quote, 0.0);
        assert_eq!(orderbook.bids[2].quantity_contract.unwrap(), 0.0);
    }

    #[test]
    fn linear_swap_update_2() {
        let raw_msg = r#"{"channel":"1000LUNC_USDT.Depth","data":{"bids":[[0.11,1.0]],"time":"1654009289339"}}"#;
        let orderbook = &parse_l2(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 0);
        assert_eq!(orderbook.bids.len(), 1);
        assert!(!orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            EXCHANGE_NAME,
            MarketType::LinearSwap,
            MessageType::L2Event,
            "1000LUNC/USDT".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );
        assert_eq!(
            "1000LUNC_USDT",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );
        assert_eq!(
            1654009289339,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1654009289339);
        assert_eq!(orderbook.seq_id, None);
        assert_eq!(orderbook.prev_seq_id, None);

        assert_eq!(orderbook.bids[0].price, 0.11);
        assert_eq!(orderbook.bids[0].quantity_base, 1.0);
        assert_eq!(orderbook.bids[0].quantity_quote, round(0.11 * 1.0));
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 1.0);
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
        let raw_msg = r#"{"asks":[[32383.57,0.0062],[32333.34,0.0002],[32333.0,0.0350]],"dataType":"depth","bids":[[31753.03,0.1500],[31749.61,0.6260],[31742.88,0.3500]],"channel":"btcusdt_depth","timestamp":1653997711}"#;
        let orderbook = &parse_l2_topk(EXCHANGE_NAME, MarketType::Spot, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 3);
        assert_eq!(orderbook.bids.len(), 3);
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
            1653997711000,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg,)
                .unwrap()
                .unwrap()
        );

        assert_eq!(1653997711000, orderbook.timestamp);
        assert_eq!(orderbook.seq_id, None);
        assert_eq!(orderbook.prev_seq_id, None);

        assert_eq!(orderbook.bids[0].price, 31753.03);
        assert_eq!(orderbook.bids[0].quantity_base, 0.15);
        assert_eq!(orderbook.bids[0].quantity_quote, 31753.03 * 0.15);
        assert_eq!(orderbook.bids[0].quantity_contract, None);

        assert_eq!(orderbook.bids[2].price, 31742.88);
        assert_eq!(orderbook.bids[2].quantity_base, 0.35);
        assert_eq!(orderbook.bids[2].quantity_quote, 31742.88 * 0.35);
        assert_eq!(orderbook.bids[2].quantity_contract, None);

        assert_eq!(orderbook.asks[0].price, 32333.0);
        assert_eq!(orderbook.asks[0].quantity_base, 0.035);
        assert_eq!(orderbook.asks[0].quantity_quote, 32333.0 * 0.035);
        assert_eq!(orderbook.asks[0].quantity_contract, None);

        assert_eq!(orderbook.asks[2].price, 32383.57);
        assert_eq!(orderbook.asks[2].quantity_base, 0.0062);
        assert_eq!(orderbook.asks[2].quantity_quote, 32383.57 * 0.0062);
        assert_eq!(orderbook.asks[2].quantity_contract, None);
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"channel":"BTC_USDT.DepthWhole","data":{"asks":[[31625.55,0.03],[31625.57,0.06],[31676.49,0.03]],"bids":[[31620.75,0.05],[31615.26,0.03],[31607.92,0.04]],"time":"1654000236324"}}"#;
        let orderbook =
            &parse_l2_topk(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 3);
        assert_eq!(orderbook.bids.len(), 3);
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
            1654000236324,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1654000236324);
        assert_eq!(orderbook.seq_id, None);
        assert_eq!(orderbook.prev_seq_id, None);

        assert_eq!(orderbook.asks[0].price, 31625.55);
        assert_eq!(orderbook.asks[0].quantity_base, 0.03);
        assert_eq!(orderbook.asks[0].quantity_quote, 0.03 * 31625.55);
        assert_eq!(orderbook.asks[0].quantity_contract, Some(0.03));

        assert_eq!(orderbook.asks[2].price, 31676.49);
        assert_eq!(orderbook.asks[2].quantity_base, 0.03);
        assert_eq!(orderbook.asks[2].quantity_quote, 0.03 * 31676.49);
        assert_eq!(orderbook.asks[2].quantity_contract, Some(0.03));

        assert_eq!(orderbook.bids[0].price, 31620.75);
        assert_eq!(orderbook.bids[0].quantity_base, 0.05);
        assert_eq!(orderbook.bids[0].quantity_quote, round(0.05 * 31620.75));
        assert_eq!(orderbook.bids[0].quantity_contract, Some(0.05));

        assert_eq!(orderbook.bids[2].price, 31607.92);
        assert_eq!(orderbook.bids[2].quantity_base, 0.04);
        assert_eq!(orderbook.bids[2].quantity_quote, 0.04 * 31607.92);
        assert_eq!(orderbook.bids[2].quantity_contract, Some(0.04));
    }
}

#[cfg(test)]
mod ticker {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, extract_timestamp};

    #[test]
    fn spot() {
        let raw_msg = r#"{"date":"1653781987181","ticker":{"high":"29249.63","vol":"4499.6492","last":"29046.17","low":"28527.54","buy":"29039.24","sell":"29056.69","turnover":"129792765.9200","open":"28598.8","riseRate":"1.57"},"dataType":"ticker","channel":"btcusdt_ticker"}"#;

        assert_eq!(
            "btcusdt",
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );

        assert_eq!(
            1653781987181,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"channel":"BTC_USDT.Ticker","data":[28669.4,29244.73,27980,29012.96,24264.005,1.2,1653783012,257344.9552]}"#;

        assert_eq!(
            "BTC_USDT",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );

        assert_eq!(
            1653783012000,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
    }

    #[test]
    fn linear_swap_all_ticker() {
        let raw_msg = r#"{"channel": "All.Ticker","data": {"ETH_USDT": [1739.34, 1807.79, 1721.41, 1790.14, 238051.871, 2.92, 1653783366, 15871.560254],"BTC_USDT": [28735.84, 29244.73, 27980, 28988.05, 24123.201, 0.88, 1653783365, 257010.950105]}}"#;

        assert_eq!(
            "All",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );

        assert_eq!(
            1653783366000,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
    }
}

#[cfg(test)]
mod candlestick {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, extract_timestamp};

    #[test]
    fn spot() {
        let raw_msg = r#"{"datas":{"data":[[1653782100000,29055.22,29055.22,29030.81,29032.9,19.3130],[1653782160000,29036.33,29036.33,29036.33,29036.33,0.0001]]},"channel":"btcusdt_kline_1min","isSuc":true}"#;

        assert_eq!(
            "btcusdt",
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );

        assert_eq!(
            1653782160000,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"channel":"BTC_USDT.KLine_1M","type":"Whole","data":[[28993.54,28996.39,28992.58,28994.78,0.921,1653783840]]}"#;

        assert_eq!(
            "BTC_USDT",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );

        assert_eq!(
            1653783840000,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
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
        let raw_msg = r#"{"asks":[[29704.57,0.0002],[29700.54,0.1500],[29695.72,0.1500]],"bids":[[29680.86,0.1500],[29677.89,0.1500],[29674.97,0.6260]],"timestamp":1654329612}"#;

        assert_eq!(
            "NONE",
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );

        assert_eq!(
            1654329612000,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"code":10000,"desc":"操作成功","data":{"asks":[[29663.89,0.03],[29668.69,0.04],[29676.09,0.04]],"bids":[[29659.12,0.06],[29658.84,0.03],[29652.77,0.04]],"time":1654330502522}}"#;

        assert_eq!(
            "NONE",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );

        assert_eq!(
            1654330502522,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
    }
}
