mod utils;

const EXCHANGE_NAME: &str = "ftx";

#[cfg(test)]
mod trade {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_message::TradeSide;
    use crypto_msg_parser::{extract_symbol, extract_timestamp, parse_trade, round};

    #[test]
    fn spot() {
        let raw_msg = r#"{"channel": "trades", "market": "BTC/USD", "type": "update", "data": [{"id": 632052557, "price": 56335.0, "size": 0.0444, "side": "buy", "liquidation": false, "time": "2021-03-21T10:24:37.319680+00:00"}]}"#;
        let trades = &parse_trade(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap();

        assert_eq!(trades.len(), 1);

        for trade in trades.iter() {
            crate::utils::check_trade_fields(
                EXCHANGE_NAME,
                MarketType::Spot,
                "BTC/USD".to_string(),
                extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap(),
                trade,
                raw_msg,
            );

            assert_eq!(trade.side, TradeSide::Buy);
        }
        assert_eq!(
            1616322277319,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(trades[0].quantity_base, 0.0444);
    }

    #[test]
    fn linear_futre() {
        let raw_msg = r#"{"channel": "trades", "market": "BTC-0326", "type": "update", "data": [{"id": 632137285, "price": 56244.0, "size": 0.0043, "side": "sell", "liquidation": false, "time": "2021-03-21T10:58:26.498464+00:00"}]}"#;
        let trades = &parse_trade(EXCHANGE_NAME, MarketType::LinearFuture, raw_msg).unwrap();

        assert_eq!(trades.len(), 1);

        for trade in trades.iter() {
            crate::utils::check_trade_fields(
                EXCHANGE_NAME,
                MarketType::LinearFuture,
                "BTC/USD".to_string(),
                extract_symbol(EXCHANGE_NAME, MarketType::LinearFuture, raw_msg).unwrap(),
                trade,
                raw_msg,
            );
        }
        assert_eq!(
            1616324306498,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearFuture, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(trades[0].quantity_base, 0.0043);
        assert_eq!(trades[0].quantity_quote, 0.0043 * 56244.0);
        assert_eq!(trades[0].quantity_contract, Some(0.0043));
        assert_eq!(trades[0].side, TradeSide::Sell);
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"channel": "trades", "market": "BTC-PERP", "type": "update", "data": [{"id": 632141274, "price": 56115.0, "size": 0.005, "side": "buy", "liquidation": false, "time": "2021-03-21T11:00:38.933676+00:00"}]}"#;
        let trades = &parse_trade(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap();

        assert_eq!(trades.len(), 1);

        for trade in trades.iter() {
            crate::utils::check_trade_fields(
                EXCHANGE_NAME,
                MarketType::LinearSwap,
                "BTC/USD".to_string(),
                extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap(),
                trade,
                raw_msg,
            );
        }
        assert_eq!(
            1616324438933,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(trades[0].quantity_base, 0.005);
        assert_eq!(trades[0].quantity_quote, 0.005 * 56115.0);
        assert_eq!(trades[0].quantity_contract, Some(0.005));
        assert_eq!(trades[0].side, TradeSide::Buy);
    }

    #[test]
    fn volatility_move() {
        let raw_msg = r#"{"channel": "trades", "market": "BTC-MOVE-WK-0402", "type": "update", "data": [{"id": 619750489, "price": 5862.0, "size": 0.1136, "side": "buy", "liquidation": false, "time": "2021-03-18T17:47:50.727425+00:00"}]}"#;
        let trades = &parse_trade(EXCHANGE_NAME, MarketType::Move, raw_msg).unwrap();

        assert_eq!(trades.len(), 1);

        for trade in trades.iter() {
            crate::utils::check_trade_fields(
                EXCHANGE_NAME,
                MarketType::Move,
                "BTC/USD".to_string(),
                extract_symbol(EXCHANGE_NAME, MarketType::Move, raw_msg).unwrap(),
                trade,
                raw_msg,
            );
        }
        assert_eq!(
            1616089670727,
            extract_timestamp(EXCHANGE_NAME, MarketType::Move, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(trades[0].quantity_base, 0.1136);
        assert_eq!(trades[0].quantity_quote, round(0.1136 * 5862.0));
        assert_eq!(trades[0].quantity_contract, Some(0.1136));
        assert_eq!(trades[0].side, TradeSide::Buy);
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
        let raw_msg = r#"{"channel": "orderbook", "market": "BTC/USD", "type": "partial", "data": {"time": 1622668801.966823, "checksum": 4093133381, "bids": [[37875.0, 0.4537], [37874.0, 0.5673], [37872.0, 0.328]], "asks": [[37876.0, 0.1749], [37877.0, 0.0001], [37878.0, 0.5]], "action": "partial"}}"#;
        let orderbook = &parse_l2(EXCHANGE_NAME, MarketType::Spot, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 3);
        assert_eq!(orderbook.bids.len(), 3);
        assert!(orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            EXCHANGE_NAME,
            MarketType::Spot,
            MessageType::L2Event,
            "BTC/USD".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );
        assert_eq!(
            1622668801966,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1622668801966);

        assert_eq!(orderbook.bids[0].price, 37875.0);
        assert_eq!(orderbook.bids[0].quantity_base, 0.4537);
        assert_eq!(orderbook.bids[0].quantity_quote, 37875.0 * 0.4537);

        assert_eq!(orderbook.bids[2].price, 37872.0);
        assert_eq!(orderbook.bids[2].quantity_base, 0.328);
        assert_eq!(orderbook.bids[2].quantity_quote, 37872.0 * 0.328);

        assert_eq!(orderbook.asks[0].price, 37876.0);
        assert_eq!(orderbook.asks[0].quantity_base, 0.1749);
        assert_eq!(orderbook.asks[0].quantity_quote, 37876.0 * 0.1749);

        assert_eq!(orderbook.asks[2].price, 37878.0);
        assert_eq!(orderbook.asks[2].quantity_base, 0.5);
        assert_eq!(orderbook.asks[2].quantity_quote, 37878.0 * 0.5);
    }

    #[test]
    fn spot_update() {
        let raw_msg = r#"{"channel": "orderbook", "market": "BTC/USD", "type": "update", "data": {"time": 1622668802.0262146, "checksum": 2044263315, "bids": [[37875.0, 0.446]], "asks": [[37886.0, 5.2109], [37889.0, 0.8493]], "action": "update"}}"#;
        let orderbook = &parse_l2(EXCHANGE_NAME, MarketType::Spot, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 2);
        assert_eq!(orderbook.bids.len(), 1);
        assert!(!orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            EXCHANGE_NAME,
            MarketType::Spot,
            MessageType::L2Event,
            "BTC/USD".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );
        assert_eq!(
            1622668802026,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1622668802026);

        assert_eq!(orderbook.bids[0].price, 37875.0);
        assert_eq!(orderbook.bids[0].quantity_base, 0.446);
        assert_eq!(orderbook.bids[0].quantity_quote, 37875.0 * 0.446);

        assert_eq!(orderbook.asks[0].price, 37886.0);
        assert_eq!(orderbook.asks[0].quantity_base, 5.2109);
        assert_eq!(orderbook.asks[0].quantity_quote, 37886.0 * 5.2109);

        assert_eq!(orderbook.asks[1].price, 37889.0);
        assert_eq!(orderbook.asks[1].quantity_base, 0.8493);
        assert_eq!(orderbook.asks[1].quantity_quote, 37889.0 * 0.8493);
    }

    #[test]
    fn linear_future_snapshot() {
        let raw_msg = r#"{"channel": "orderbook", "market": "BTC-0625", "type": "partial", "data": {"time": 1622669504.8200636, "checksum": 1739399809, "bids": [[37965.0, 2.7939], [37961.0, 0.005], [37960.0, 11.4351]], "asks": [[37980.0, 0.2474], [37987.0, 0.0957], [37991.0, 0.0005]], "action": "partial"}}"#;
        let orderbook =
            &parse_l2(EXCHANGE_NAME, MarketType::LinearFuture, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 3);
        assert_eq!(orderbook.bids.len(), 3);
        assert!(orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            EXCHANGE_NAME,
            MarketType::LinearFuture,
            MessageType::L2Event,
            "BTC/USD".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::LinearFuture, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );
        assert_eq!(
            1622669504820,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearFuture, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1622669504820);

        assert_eq!(orderbook.bids[0].price, 37965.0);
        assert_eq!(orderbook.bids[0].quantity_base, 2.7939);
        assert_eq!(orderbook.bids[0].quantity_quote, 37965.0 * 2.7939);
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 2.7939);

        assert_eq!(orderbook.bids[2].price, 37960.0);
        assert_eq!(orderbook.bids[2].quantity_base, 11.4351);
        assert_eq!(orderbook.bids[2].quantity_quote, 37960.0 * 11.4351);
        assert_eq!(orderbook.bids[2].quantity_contract.unwrap(), 11.4351);

        assert_eq!(orderbook.asks[0].price, 37980.0);
        assert_eq!(orderbook.asks[0].quantity_base, 0.2474);
        assert_eq!(orderbook.asks[0].quantity_quote, 37980.0 * 0.2474);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 0.2474);

        assert_eq!(orderbook.asks[2].price, 37991.0);
        assert_eq!(orderbook.asks[2].quantity_base, 0.0005);
        assert_eq!(orderbook.asks[2].quantity_quote, 37991.0 * 0.0005);
        assert_eq!(orderbook.asks[2].quantity_contract.unwrap(), 0.0005);
    }

    #[test]
    fn linear_future_update() {
        let raw_msg = r#"{"channel": "orderbook", "market": "BTC-0625", "type": "update", "data": {"time": 1622669504.8437843, "checksum": 1584262478, "bids": [], "asks": [[37999.0, 0.0], [38440.0, 0.0026]], "action": "update"}}"#;
        let orderbook =
            &parse_l2(EXCHANGE_NAME, MarketType::LinearFuture, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 2);
        assert_eq!(orderbook.bids.len(), 0);
        assert!(!orderbook.snapshot);

        crate::utils::check_orderbook_fields(
            EXCHANGE_NAME,
            MarketType::LinearFuture,
            MessageType::L2Event,
            "BTC/USD".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::LinearFuture, raw_msg).unwrap(),
            orderbook,
            raw_msg,
        );
        assert_eq!(
            1622669504843,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearFuture, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1622669504843);

        assert_eq!(orderbook.asks[0].price, 37999.0);
        assert_eq!(orderbook.asks[0].quantity_base, 0.0);
        assert_eq!(orderbook.asks[0].quantity_quote, 0.0);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 0.0);

        assert_eq!(orderbook.asks[1].price, 38440.0);
        assert_eq!(orderbook.asks[1].quantity_base, 0.0026);
        assert_eq!(orderbook.asks[1].quantity_quote, round(38440.0 * 0.0026));
        assert_eq!(orderbook.asks[1].quantity_contract.unwrap(), 0.0026);
    }

    #[test]
    fn linear_swap_snapshot() {
        let raw_msg = r#"{"channel": "orderbook", "market": "BTC-PERP", "type": "partial", "data": {"time": 1622660997.436228, "checksum": 1855139817, "bids": [[37955.0, 0.2212], [37954.0, 0.0025], [37953.0, 0.0025]], "asks": [[37956.0, 4.8852], [37957.0, 0.022], [37958.0, 0.4818]], "action": "partial"}}"#;
        let orderbook = &parse_l2(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 3);
        assert_eq!(orderbook.bids.len(), 3);
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
            1622660997436,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1622660997436);

        assert_eq!(orderbook.bids[0].price, 37955.0);
        assert_eq!(orderbook.bids[0].quantity_base, 0.2212);
        assert_eq!(orderbook.bids[0].quantity_quote, 37955.0 * 0.2212);
        assert_eq!(orderbook.bids[0].quantity_contract.unwrap(), 0.2212);

        assert_eq!(orderbook.bids[2].price, 37953.0);
        assert_eq!(orderbook.bids[2].quantity_base, 0.0025);
        assert_eq!(orderbook.bids[2].quantity_quote, round(37953.0 * 0.0025));
        assert_eq!(orderbook.bids[2].quantity_contract.unwrap(), 0.0025);

        assert_eq!(orderbook.asks[0].price, 37956.0);
        assert_eq!(orderbook.asks[0].quantity_base, 4.8852);
        assert_eq!(orderbook.asks[0].quantity_quote, round(37956.0 * 4.8852));
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 4.8852);

        assert_eq!(orderbook.asks[2].price, 37958.0);
        assert_eq!(orderbook.asks[2].quantity_base, 0.4818);
        assert_eq!(orderbook.asks[2].quantity_quote, 37958.0 * 0.4818);
        assert_eq!(orderbook.asks[2].quantity_contract.unwrap(), 0.4818);
    }

    #[test]
    fn linear_swap_update() {
        let raw_msg = r#"{"channel": "orderbook", "market": "BTC-PERP", "type": "update", "data": {"time": 1622660997.4591022, "checksum": 276300987, "bids": [], "asks": [[37965.0, 19.6097]], "action": "update"}}"#;
        let orderbook = &parse_l2(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg, None).unwrap()[0];

        assert_eq!(orderbook.asks.len(), 1);
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
        assert_eq!(
            1622660997459,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg)
                .unwrap()
                .unwrap()
        );

        assert_eq!(orderbook.timestamp, 1622660997459);

        assert_eq!(orderbook.asks[0].price, 37965.0);
        assert_eq!(orderbook.asks[0].quantity_base, 19.6097);
        assert_eq!(orderbook.asks[0].quantity_quote, 37965.0 * 19.6097);
        assert_eq!(orderbook.asks[0].quantity_contract.unwrap(), 19.6097);
    }
}

#[cfg(test)]
mod bbo {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, extract_timestamp};

    #[test]
    fn spot() {
        let raw_msg = r#"{"channel":"ticker","market":"BTC/USD","type":"update","data":{"bid":31679.0,"ask":31680.0,"bidSize":1.8434,"askSize":1.1266,"last":31679.0,"time":1654029182.6905813}}"#;

        assert_eq!(
            1654029182690,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "BTC/USD",
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"channel":"ticker","market":"BTC-PERP","type":"update","data":{"bid":31699.0,"ask":31700.0,"bidSize":14.9905,"askSize":4.6393,"last":31699.0,"time":1654029408.920583}}"#;

        assert_eq!(
            1654029408920,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "BTC-PERP",
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
        );
    }

    #[test]
    fn linear_future() {
        let raw_msg = r#"{"channel":"ticker","market":"BTC-0624","type":"update","data":{"bid":31746.0,"ask":31747.0,"bidSize":1.0727,"askSize":0.1,"last":31760.0,"time":1654029472.207374}}"#;

        assert_eq!(
            1654029472207,
            extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            "BTC-0624",
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
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
        let raw_msg = r#"{"success":true,"result":{"bids":[[1151.0,0.36],[1150.0,0.683],[1149.5,0.651],[1148.5,13.132],[1148.0,3.057]],"asks":[[1153.5,13.326],[1154.0,0.622],[1154.5,4.182],[1155.0,26.58],[1156.0,4.106]]}}"#;

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
        let raw_msg = r#"{"success":true,"result":{"bids":[[1151.0,0.36],[1150.0,0.683],[1149.5,0.651],[1148.5,13.132],[1148.0,3.057]],"asks":[[1153.5,13.326],[1154.0,0.622],[1154.5,4.182],[1155.0,26.58],[1156.0,4.106]]}}"#;

        assert_eq!(
            "NONE",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );

        assert_eq!(
            None,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearSwap, raw_msg).unwrap()
        );
    }

    #[test]
    fn linear_future() {
        let raw_msg = r#"{"success":true,"result":{"bids":[[30215.0,0.0004],[30210.0,0.1287],[30209.0,0.9605],[30207.0,0.0824],[30206.0,0.1721]],"asks":[[30225.0,0.6408],[30226.0,0.8258],[30227.0,2.6539],[30228.0,0.3609],[30229.0,0.2078]]}}"#;

        assert_eq!(
            "NONE",
            extract_symbol(EXCHANGE_NAME, MarketType::LinearFuture, raw_msg).unwrap()
        );

        assert_eq!(
            None,
            extract_timestamp(EXCHANGE_NAME, MarketType::LinearFuture, raw_msg).unwrap()
        );
    }
}

#[cfg(test)]
mod open_interest {
    use super::EXCHANGE_NAME;
    use crypto_market_type::MarketType;
    use crypto_msg_parser::{extract_symbol, extract_timestamp};

    #[test]
    fn unknown() {
        let raw_msg = r#"{"success":true,"result":[{"name":"1INCH-PERP","underlying":"1INCH","description":"1INCH Token Perpetual Futures","type":"perpetual","expiry":null,"perpetual":true,"expired":false,"enabled":true,"postOnly":false,"priceIncrement":0.0001,"sizeIncrement":1.0,"last":0.8525,"bid":0.8526,"ask":0.853,"index":0.853847992062,"mark":0.8528,"imfFactor":0.0005,"lowerBound":0.8099,"upperBound":0.8965,"underlyingDescription":"1INCH Token","expiryDescription":"Perpetual","moveStart":null,"marginPrice":0.8528,"positionLimitWeight":20.0,"group":"perpetual","change1h":-0.009868802972251248,"change24h":-0.023474178403755867,"changeBod":-0.0217939894471209,"volumeUsd24h":6256487.5398,"volume":7237781.0,"openInterest":16277750.0,"openInterestUsd":13881665.2},{"name":"1INCH-0624","underlying":"1INCH","description":"1INCH Token June 2022 Futures","type":"future","expiry":"2022-06-24T03:00:00+00:00","perpetual":false,"expired":false,"enabled":true,"postOnly":false,"priceIncrement":0.0001,"sizeIncrement":1.0,"last":0.8292,"bid":0.8276,"ask":0.8292,"index":0.853847992062,"mark":0.8292,"imfFactor":0.0005,"lowerBound":0.7855,"upperBound":0.8965,"underlyingDescription":"1INCH Token","expiryDescription":"June 2022","moveStart":null,"marginPrice":0.8292,"positionLimitWeight":40.0,"group":"quarterly","change1h":-0.008133971291866028,"change24h":-0.01367907695967646,"changeBod":-0.011916110581506196,"volumeUsd24h":68694.8909,"volume":82273.0,"openInterest":849727.0,"openInterestUsd":704593.6284}]}"#;

        assert_eq!(
            "ALL",
            extract_symbol(EXCHANGE_NAME, MarketType::Unknown, raw_msg).unwrap()
        );

        assert_eq!(
            None,
            extract_timestamp(EXCHANGE_NAME, MarketType::Unknown, raw_msg).unwrap()
        );
    }
}
