mod utils;

#[cfg(test)]
mod trade {
    use crypto_msg_parser::{parse_trade, MarketType};

    #[test]
    fn spot() {
        let raw_msg = r#"{"stream":"btcusdt@aggTrade","data":{"e":"aggTrade","E":1616176861895,"s":"BTCUSDT","a":640283266,"p":"58942.01000000","q":"0.00035600","f":716849523,"l":716849523,"T":1616176861893,"m":false,"M":true}}"#;
        let trade = &parse_trade("binance", MarketType::Spot, raw_msg).unwrap()[0];

        crate::utils::check_trade_fields(
            "binance",
            MarketType::Spot,
            "BTC/USDT".to_string(),
            trade,
        );

        assert!(!trade.side); // buyer is the taker
    }

    #[test]
    fn inverse_future() {
        let raw_msg = r#"{"stream":"btcusd_210625@aggTrade","data":{"e":"aggTrade","E":1616201787561,"a":5091038,"s":"BTCUSD_210625","p":"62838.0","q":"5","f":7621250,"l":7621250,"T":1616201787407,"m":true}}"#;
        let trade = &parse_trade("binance", MarketType::InverseFuture, raw_msg).unwrap()[0];

        crate::utils::check_trade_fields(
            "binance",
            MarketType::InverseFuture,
            "BTC/USD".to_string(),
            trade,
        );

        assert!(trade.side); // seller is the taker
    }

    #[test]
    fn linear_future() {
        let raw_msg = r#"{"stream":"btcusdt_210625@aggTrade","data":{"e":"aggTrade","E":1616201036113,"a":21021,"s":"BTCUSDT_210625","p":"62595.8","q":"0.094","f":21824,"l":21824,"T":1616201035958,"m":false}}"#;
        let trade = &parse_trade("binance", MarketType::LinearFuture, raw_msg).unwrap()[0];

        crate::utils::check_trade_fields(
            "binance",
            MarketType::LinearFuture,
            "BTC/USDT".to_string(),
            trade,
        );

        assert!(!trade.side); // buyer is the taker
    }

    #[test]
    fn inverse_swap() {
        let raw_msg = r#"{"stream":"btcusd_perp@aggTrade","data":{"e":"aggTrade","E":1616201883458,"a":41045788,"s":"BTCUSD_PERP","p":"58570.1","q":"58","f":91864326,"l":91864327,"T":1616201883304,"m":true}}"#;
        let trade = &parse_trade("binance", MarketType::InverseSwap, raw_msg).unwrap()[0];

        crate::utils::check_trade_fields(
            "binance",
            MarketType::InverseSwap,
            "BTC/USD".to_string(),
            trade,
        );

        assert!(trade.side); // seller is the taker
    }

    #[test]
    fn linear_swap() {
        let raw_msg = r#"{"stream":"btcusdt@aggTrade","data":{"e":"aggTrade","E":1616202009196,"a":389551486,"s":"BTCUSDT","p":"58665.00","q":"0.043","f":621622993,"l":621622993,"T":1616202009188,"m":false}}"#;
        let trade = &parse_trade("binance", MarketType::LinearSwap, raw_msg).unwrap()[0];

        crate::utils::check_trade_fields(
            "binance",
            MarketType::LinearSwap,
            "BTC/USDT".to_string(),
            trade,
        );

        assert!(!trade.side); // buyer is the taker
    }

    #[test]
    fn option() {
        let raw_msg = r#"{"stream":"BTCUSDT_C@TRADE_ALL","data":{"e":"trade_all","E":1616205287778,"s":"BTCUSDT_C","t":[{"t":"315","p":"4842.24","q":"0.0001","b":"4612047757752932782","a":"4612057653433061439","T":1616204382000,"s":"1","S":"BTC-210430-68000-C"},{"t":"805","p":"5616.36","q":"0.0001","b":"4612047757752932781","a":"4612057653433055969","T":1616204357000,"s":"1","S":"BTC-210430-64000-C"},{"t":"313","p":"7028.44","q":"0.0001","b":"4612015871915728334","a":"4612057653433051715","T":1616204344000,"s":"1","S":"BTC-210430-60000-C"}]}}"#;
        let trades = &parse_trade("binance", MarketType::Option, raw_msg).unwrap();

        assert_eq!(trades.len(), 3);

        for trade in trades.iter() {
            crate::utils::check_trade_fields(
                "binance",
                MarketType::Option,
                "BTC/USDT".to_string(),
                trade,
            );
        }
    }
}
