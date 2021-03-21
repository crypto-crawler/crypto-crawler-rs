mod utils;

#[cfg(test)]
mod trade {
    use crypto_msg_parser::{parse_trade, MarketType, TradeSide};

    #[test]
    fn spot() {
        let raw_msg = r#"{"data":{"symbol":"BTC-USDT","sequence":"1614503482134","side":"buy","size":"0.00013064","price":"57659.6","takerOrderId":"6057bb821220fc00060f26bf","time":"1616362370760468781","type":"match","makerOrderId":"6057bb81b5ab390006532c9d","tradeId":"6057bb822e113d292396c272"},"subject":"trade.l3match","topic":"/market/match:BTC-USDT","type":"message"}"#;
        let trades = &parse_trade("kucoin", MarketType::Spot, raw_msg).unwrap();

        assert_eq!(trades.len(), 1);
        let trade = &trades[0];

        crate::utils::check_trade_fields("kucoin", MarketType::Spot, "BTC/USDT".to_string(), trade);

        assert_eq!(trade.volume, trade.price * trade.quantity);
        assert_eq!(trade.side, TradeSide::Buy);
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"data":{"makerUserId":"5e568500eb029b0008715785","symbol":"XBTUSDTM","sequence":8267947,"side":"buy","size":16,"price":57850,"takerOrderId":"6057bc95660a7d0006dc1171","makerOrderId":"6057bc92652ce800067e841a","takerUserId":"601f35b4d42fad0006b2df21","tradeId":"6057bc953c7feb667195bac9","ts":1616362645429686578},"subject":"match","topic":"/contractMarket/execution:XBTUSDTM","type":"message"}"#;
        let trades = &parse_trade("kucoin", MarketType::LinearSwap, raw_msg).unwrap();

        assert_eq!(trades.len(), 1);
        let trade = &trades[0];

        crate::utils::check_trade_fields(
            "kucoin",
            MarketType::LinearSwap,
            "BTC/USDT".to_string(),
            trade,
        );

        assert_eq!(trade.volume, trade.price * trade.quantity);
        assert_eq!(trade.quantity, 0.016);
        assert_eq!(trade.side, TradeSide::Buy);
    }

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"data":{"makerUserId":"5d85a240c788c62738732dd9","symbol":"XBTUSDM","sequence":5174061,"side":"buy","size":5000,"price":57798,"takerOrderId":"6057bc692cfab900061f8b11","makerOrderId":"6057bc4df4b11f0006a7743b","takerUserId":"5dba895d134ab72ce156079a","tradeId":"6057bc693c7feb6705f9a248","ts":1616362601277456186},"subject":"match","topic":"/contractMarket/execution:XBTUSDM","type":"message"}"#;
        let trades = &parse_trade("kucoin", MarketType::InverseSwap, raw_msg).unwrap();

        assert_eq!(trades.len(), 1);
        let trade = &trades[0];

        crate::utils::check_trade_fields(
            "kucoin",
            MarketType::InverseSwap,
            "BTC/USD".to_string(),
            trade,
        );

        assert_eq!(trade.volume, trade.price * trade.quantity);
        assert_eq!(trade.volume, 5000.0);
        assert_eq!(trade.side, TradeSide::Buy);
    }

    #[test]
    fn inverse_future() {
        let raw_msg = r#"{"data":{"makerUserId":"5f802947889eb80006a1ba0f","symbol":"XBTMH21","sequence":31319,"side":"sell","size":1510,"price":57963.0,"takerOrderId":"6057be2685c6a0000610a89a","makerOrderId":"6057be11652ce800067fafb9","takerUserId":"5f802947889eb80006a1ba0f","tradeId":"6057be2677a0c431d1d1f5b6","ts":1616363046546528915},"subject":"match","topic":"/contractMarket/execution:XBTMH21","type":"message"}"#;
        let trades = &parse_trade("kucoin", MarketType::InverseFuture, raw_msg).unwrap();

        assert_eq!(trades.len(), 1);
        let trade = &trades[0];

        crate::utils::check_trade_fields(
            "kucoin",
            MarketType::InverseFuture,
            "BTC/USD".to_string(),
            trade,
        );

        assert_eq!(trade.volume, trade.price * trade.quantity);
        assert_eq!(trade.volume, 1510.0);
        assert_eq!(trade.side, TradeSide::Sell);
    }
}
