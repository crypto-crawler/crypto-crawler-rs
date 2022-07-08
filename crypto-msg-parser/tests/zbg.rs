mod utils;

const EXCHANGE_NAME: &str = "zbg";

#[cfg(test)]
mod trade {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_message::TradeSide;
    use crypto_msg_parser::{extract_symbol, extract_timestamp, parse_trade};

    #[test]
    fn spot() {
        let raw_msg = r#"[["T","329","1616384937","BTC_USDT","bid","57347.4","0.048800"]]"#;
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
            1616384937000,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(trade.quantity_base, 0.048800);
        assert_eq!(trade.side, TradeSide::Buy);

        let raw_msg = r#"["T","329","1616486457","BTC_USDT","ask","54139.4","0.654172"]"#;
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
            1616486457000,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(trade.quantity_base, 0.654172);
        assert_eq!(trade.side, TradeSide::Sell);
    }

    #[test]
    fn linear_swap() {
        let raw_msg =
            r#"["future_tick",{"contractId":1000000,"trades":[1616385064674265,"57326","31",-1]}]"#;
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
            1616385064674,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(trade.quantity_base, 0.01 * 31.0);
        assert_eq!(trade.quantity_quote, 0.01 * 31.0 * 57326.0);
        assert_eq!(trade.quantity_contract, Some(31.0));
        assert_eq!(trade.side, TradeSide::Sell);
    }

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"["future_tick",{"contractId":1000001,"trades":[1616385036580662,"57370","188",-1]}]"#;
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
            1616385036580,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(trade.quantity_base, 188.0 / 57370.0);
        assert_eq!(trade.quantity_quote, 188.0);
        assert_eq!(trade.quantity_contract, Some(188.0));
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
    fn spot_snapshot_1() {
        let raw_msg = r#"[["AE","329","BTC_USDT","1622729950",{"asks":[["38394.8","0.01917"],["38394.2","0.195885"]]},{"bids":[["38388.7","0.146025"],["38388.1","0.155175"]]}]]"#;
        let orderbook = &parse_l2(EXCHANGE_NAME, MarketType::Spot, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 2);
        assert_eq!(orderbook.bids.len(), 2);
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
            1622729950000,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1622729950000);

        assert_eq!(orderbook.bids[0].price, 38388.7);
        assert_eq!(orderbook.bids[0].quantity_base, 0.146025);
        assert_eq!(orderbook.bids[0].quantity_quote, 38388.7 * 0.146025);

        assert_eq!(orderbook.asks[0].price, 38394.2);
        assert_eq!(orderbook.asks[0].quantity_base, 0.195885);
        assert_eq!(orderbook.asks[0].quantity_quote, 38394.2 * 0.195885);
    }

    #[test]
    fn spot_snapshot_2() {
        let raw_msg = r#"[["AE","5374","SOS_USDT","1648785278",{"asks":[[0.00000471,2033667.52],[0.000004664,10167976.22]]},{"bids":[[0.000001726,41991455.48],["6E-7",300000000.00]]}]]"#;
        let orderbook = &parse_l2(EXCHANGE_NAME, MarketType::Spot, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 2);
        assert_eq!(orderbook.bids.len(), 2);
        assert!(orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            EXCHANGE_NAME,
            MarketType::Spot,
            MessageType::L2Event,
            "SOS/USDT".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );
        assert_eq!(
            1648785278000,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1648785278000);

        assert_eq!(orderbook.bids[0].price, 0.000001726);
        assert_eq!(orderbook.bids[0].quantity_base, 41991455.48);
        assert_eq!(orderbook.bids[0].quantity_quote, 0.000001726 * 41991455.48);

        assert_eq!(orderbook.bids[1].price, 0.0000006);
        assert_eq!(orderbook.bids[1].quantity_base, 300000000.0);
        assert_eq!(orderbook.bids[1].quantity_quote, 0.0000006 * 300000000.0);

        assert_eq!(orderbook.asks[0].price, 0.000004664);
        assert_eq!(orderbook.asks[0].quantity_base, 10167976.22);
        assert_eq!(orderbook.asks[0].quantity_quote, 0.000004664 * 10167976.22);
    }

    #[test]
    fn spot_snapshot_null() {
        let raw_msg = r#"[["AE","5319","YFI_USDT",null,{"asks":null},{"bids":null}]]"#;
        let orderbooks = &parse_l2(EXCHANGE_NAME, MarketType::Spot, raw_msg, None).unwrap();
        assert!(orderbooks.is_empty());
        assert_eq!(
            "yfi_usdt",
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );
        assert_eq!(
            None,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );
    }

    #[test]
    fn spot_update() {
        let raw_msg = r#"["E","329","1622729958","BTC_USDT","BID","38382.3","0.1842"]"#;
        let orderbook = &parse_l2(EXCHANGE_NAME, MarketType::Spot, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 0);
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
            1622729958000,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1622729958000);

        assert_eq!(orderbook.bids[0].price, 38382.3);
        assert_eq!(orderbook.bids[0].quantity_base, 0.1842);
        assert_eq!(orderbook.bids[0].quantity_quote, 38382.3 * 0.1842);
    }

    #[test]
    fn linear_swap_update() {
        let raw_msg = r#"["future_snapshot_depth",{"asks":[["38704","2684"]],"contractId":1000000,"bids":[["38703","1606"],["38702.5","616"]],"tradeDate":20210603,"time":1622733219128160}]"#;
        let orderbook = &parse_l2(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg, None).unwrap()[0];

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
            1622733219128,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1622733219128);

        assert_eq!(orderbook.asks[0].price, 38704.0);
        assert_eq!(orderbook.asks[0].quantity_base, 26.84);
        assert_eq!(orderbook.asks[0].quantity_quote, 38704.0 * 26.84);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 2684.0);

        assert_eq!(orderbook.bids[0].price, 38703.0);
        assert_eq!(orderbook.bids[0].quantity_base, 16.06);
        assert_eq!(orderbook.bids[0].quantity_quote, 38703.0 * 16.06);
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 1606.0);
    }

    #[test]
    fn inverse_swap_update() {
        let raw_msg = r#"["future_snapshot_depth",{"asks":[["38547.5","4406"],["38548","11545"]],"contractId":1000001,"bids":[["38547","24345"],["38546.5","63623"]],"tradeDate":20210603,"time":1622734001831219}]"#;
        let orderbook =
            &parse_l2(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 2);
        assert_eq!(orderbook.bids.len(), 2);
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
            1622734001831,
            extract_timestamp(EXCHANGE_NAME, MarketType::InverseSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1622734001831);

        assert_eq!(orderbook.asks[0].price, 38547.5);
        assert_eq!(orderbook.asks[0].quantity_base, 4406.0 / 38547.5);
        assert_eq!(orderbook.asks[0].quantity_quote, 4406.0);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 4406.0);

        assert_eq!(orderbook.bids[0].price, 38547.0);
        assert_eq!(orderbook.bids[0].quantity_base, 24345.0 / 38547.0);
        assert_eq!(orderbook.bids[0].quantity_quote, 24345.0);
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 24345.0);
    }
}

#[cfg(test)]
mod candlestick {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, extract_timestamp};

    #[test]
    fn spot_snapshot() {
        let raw_msg = r#"[["K","329","btc_usdt","1654155660","30013.78","30017.31","30003.01","30014.64","0.0227","-0.2957","0","1M","false","0"],["K","329","btc_usdt","1654155600","30016.95","30019.49","29997.36","29997.36","0.3865","-0.2957","0","1M","false","0"]]"#;

        assert_eq!(
            "btc_usdt",
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );

        assert_eq!(
            1654155660000,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );
    }

    #[test]
    fn spot_update() {
        let raw_msg = r#"["K","329","btc_usdt","1654125240","29947.03","29976.14","29937.94","29939.95","0.6417","-0.2957","0","1M","false","0"]"#;

        assert_eq!(
            "btc_usdt",
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );

        assert_eq!(
            1654125240000,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );
    }

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"["future_kline",{"contractId":1000001,"range":"60000","lines":[[1652804280000,"30008.5","30015.5","29994.5","30005","16754"],[1652804340000,"30005","30005.5","29975.5","29976","6186"]]}]"#;

        assert_eq!(
            "BTC_USD-R",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );

        assert_eq!(
            1652804340000,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"["future_kline",{"contractId":1000000,"range":"180000","lines":[[1648876500000,"46535","46550.5","46505.5","46550","848"],[1648876680000,"46550","46615","46542","46613.5","1640"]]}]"#;

        assert_eq!(
            "BTC_USDT",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );

        assert_eq!(
            1648876680000,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
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
        let raw_msg = r#"{"trade_statistic":[["329","29980.15","31890.91","29316.96","3104.9576","-4.96","[]","29967.06","29981.99","0"]]}"#;

        assert_eq!(
            "btc_usdt",
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );

        assert_eq!(
            None,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );
    }

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"["future_snapshot_indicator",{"tt":"1335.2544","pp":"30117","lui":1621326625165456,"symbol":"BTC_USD-R","tv":"40286903","c24t":6083.218113,"lp":"29860.5","pv":"12762468","w24pc":"474","orderLimit":"150000","dp":"0","osp":"29860.5","uf":0,"indexPrice":"29883.127199","mq":"2218","mt":4,"ip":"29883.127199","ai":2,"tav":"0","w24pcr":"0.01612985554591394","basis":"0.08%","pcr24":0.0146,"hgp24":30732,"fb":"-0.00059234518798245","pfr":"-0.000059683363091459","pc24":431.0,"volumeUsd24h":"597787","tbv":"0","fr":"-0.000059683363091459","sb":"BTC/USD-R","currencyName":"btc","op24":29429.5,"sl":0,"contractUnit":"1","pcr":"-0.008533244791234332","op":"30117.5","hph":"69159","hpl":"0.5","ci":1000001,"ppi":"-0.000559683363091459","u24t":183544142.552353,"openInterestUSD":"12762468","cp":"29895.8998627","lwp24":29032,"td":20220518,"cs":2,"te":1652804557693495,"pc":"-257","ph":"30154","contractId":"1000001","pi":"-0.001792462854748013","pl":"29855","obp":"29837.5","ts":0,"commodityName":"usd","fundingRate":"-0.0059683363091459%"}]"#;

        assert_eq!(
            "BTC_USD-R",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );

        assert_eq!(
            1652804557693,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"["future_snapshot_indicator",{"tt":"5458565.758","pp":"30132.5","lui":1621326625165443,"symbol":"BTC_USDT","tv":"18157","c24t":121314.943076,"lp":"30043.5","pv":"500435","w24pc":"634","orderLimit":"1200","dp":"0","osp":"30044","uf":0,"indexPrice":"30056.142292","mq":"18","mt":4,"ip":"30056.142292","ai":2,"tav":"0","w24pcr":"0.021557659939815366","basis":"0.04%","pcr24":0.0206,"hgp24":30778,"fb":"-0.000431400800486624","pfr":"-0.000080086434965923","pc24":607.5,"volumeUsd24h":"5458565.758","tbv":"0","fr":"-0.000080086434965923","sb":"BTC/USDT","currencyName":"usdt","op24":29436,"sl":0,"contractUnit":"0.01","pcr":"-0.002903985928113903","op":"30131","hph":"69065","hpl":"1","ci":1000000,"ppi":"-0.000580086434965923","u24t":3662432145.195,"openInterestUSD":"150348189.22","cp":"30040.25","lwp24":29073.5,"td":20220518,"cs":2,"te":1652804313766584,"pc":"-87.5","ph":"30173","contractId":"1000000","pi":"-0.000439719378055049","pl":"29966","obp":"30043","ts":0,"commodityName":"btc","fundingRate":"-0.0080086434965923%"}]"#;

        assert_eq!(
            "BTC_USDT",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );

        assert_eq!(
            1652804313766,
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
        let raw_msg = r#"{"datas":{"asks":[[29763.69,"0.6260"],[29725.98,"0.1500"],[29723.01,"0.1500"]],"bids":[[29708.13,"0.1500"],[29705.16,"0.1500"],[29704.59,"0.4000"]],"timestamp":1654331401},"resMsg":{"message":"success !","method":null,"code":"1"}}"#;

        assert_eq!(
            "NONE",
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );

        assert_eq!(
            1654331401000,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );
    }

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"datas":{"bids":[["29834","121320"],["29833.5","35241"],["29831.5","30812"]],"asks":[["29837","67897"],["29837.5","18902"],["29840.5","26717"]],"timestamp":1654331401775},"resMsg":{"message":"success !","method":null,"code":"1"}}"#;

        assert_eq!(
            "NONE",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );

        assert_eq!(
            1654331401775,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"datas":{"bids":[["29860","5885"],["29858.5","1214"],["29856.5","1324"]],"asks":[["29861","3415"],["29863","811"],["29866.5","1216"]],"timestamp":1654332304984},"resMsg":{"message":"success !","method":null,"code":"1"}}"#;

        assert_eq!(
            "NONE",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );

        assert_eq!(
            1654332304984,
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
        let raw_msg = r#"{"datas":{"mt":4,"ai":2,"ci":1000001,"sb":"BTC_USD-R","td":20220518,"te":1652804560181491,"lp":"29834.5","mq":"3050","op":"30117.5","ph":"30154","pl":"29834.5","hph":"69159","hpl":"0.5","tt":"1335.4253","tv":"40292642","tbv":"0","tav":"0","pp":"30117","cp":"29888.591354933333333334","pv":"12762468","pcr":"-0.009396530256495393","pc":"-283","lui":1621326625165456,"cs":2,"dp":"0","fr":"-0.000063829677908321","pfr":"-0.000063829677908321","pi":"-0.002437963975130133","ppi":"-0.000563829677908321","fb":"-0.000612588122859156","ts":0,"sl":0,"ip":"29906.911974","w24pc":"448","w24pcr":"0.01524509553706634","u24t":0,"c24t":0,"op24":0,"pcr24":0,"pc24":29834.5,"lwp24":0,"hgp24":0,"bids":[["29834","121320"],["29833.5","35241"]],"asks":[["29837","67897"],["29837.5","18902"]],"volumeUsd24h":"601776","currencyName":"btc","commodityName":"usd","contractUnit":"1","orderLimit":"150000","openInterestUSD":"12762468","indexPrice":"29906.911974","basis":"0.24%","fundingRate":"-0.0063829677908321%","symbol":"BTC_USD-R","contractId":"1000001","ask":"29837","bid":"29834","spread":"0.0101%"},"resMsg":{"message":"success !","method":null,"code":"1"}}"#;

        assert_eq!(
            "BTC_USD-R",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );

        assert_eq!(
            1652804560181,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"datas":{"mt":4,"ai":2,"ci":1000000,"sb":"BTC_USDT","td":20220518,"te":1652804560188093,"lp":"29860.5","mq":"383","op":"30131","ph":"30173","pl":"29860.5","hph":"69065","hpl":"1","tt":"427896130.025","tv":"1417290","tbv":"0","tav":"0","pp":"30132.5","cp":"29871.561143633333333334","pv":"500142","pcr":"-0.008977465069197836","pc":"-270.5","lui":1621326625165443,"cs":2,"dp":"0","fr":"-0.000073356024299116","pfr":"-0.000073356024299116","pi":"-0.001001523440190902","ppi":"-0.000573356024299116","fb":"-0.000614732921872445","ts":0,"sl":0,"ip":"29889.935471","w24pc":"437.5","w24pcr":"0.01486931991979064","u24t":0,"c24t":0,"op24":0,"pcr24":0,"pc24":29860.5,"lwp24":0,"hgp24":0,"bids":[["29860","5885"],["29858.5","1214"]],"asks":[["29861","3415"],["29863","811"]],"volumeUsd24h":"6245135.92","currencyName":"usdt","commodityName":"btc","contractUnit":"0.01","orderLimit":"1200","openInterestUSD":"149344901.91","indexPrice":"29889.935471","basis":"0.10%","fundingRate":"-0.0073356024299116%","symbol":"BTC_USDT","contractId":"1000000","ask":"29861","bid":"29860","spread":"0.0033%"},"resMsg":{"message":"success !","method":null,"code":"1"}}"#;

        assert_eq!(
            "BTC_USDT",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );

        assert_eq!(
            1652804560188,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );
    }
}
