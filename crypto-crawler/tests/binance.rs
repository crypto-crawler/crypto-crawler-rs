#[cfg(test)]
mod binance_spot {
    use crypto_crawler::*;

    #[test]
    fn test_crawl_trade() {
        let mut messages = Vec::<Message>::new();

        let on_msg = |msg: Message| messages.push(msg);
        crawl_trade(
            "Binance",
            MarketType::Spot,
            &vec!["btcusdt".to_string()],
            Box::new(on_msg),
            Some(0),
        );

        assert!(!messages.is_empty());
        assert_eq!(messages[0].market_type, MarketType::Spot);
        assert_eq!(messages[0].symbol, "BTCUSDT".to_string());
        assert_eq!(messages[0].msg_type, MessageType::Trade);
    }

    #[test]
    fn test_crawl_l2_event() {
        let mut messages = Vec::<Message>::new();

        let on_msg = |msg: Message| messages.push(msg);
        crawl_l2_event(
            "Binance",
            MarketType::Spot,
            &vec!["btcusdt".to_string()],
            Box::new(on_msg),
            Some(0),
        );

        assert!(!messages.is_empty());
        assert_eq!(messages[0].market_type, MarketType::Spot);
        assert_eq!(messages[0].symbol, "BTCUSDT".to_string());
        assert_eq!(messages[0].msg_type, MessageType::L2Event);
    }

    #[test]
    fn test_crawl_l2_snapshot() {
        let mut messages = Vec::<Message>::new();

        let on_msg = |msg: Message| messages.push(msg);
        crawl_l2_snapshot(
            "Binance",
            MarketType::Spot,
            &vec!["BTCUSDT".to_string()],
            Box::new(on_msg),
            Some(0),
        );

        assert!(!messages.is_empty());
        assert_eq!(messages[0].market_type, MarketType::Spot);
        assert_eq!(messages[0].symbol, "BTCUSDT".to_string());
        assert_eq!(messages[0].msg_type, MessageType::L2Snapshot);
    }
}
