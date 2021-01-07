#[macro_use]
mod utils;

#[cfg(test)]
mod binance_spot {
    use crypto_crawler::*;

    #[test]
    fn test_crawl_trade() {
        gen_test_code!(
            crawl_trade,
            "Binance",
            MarketType::Spot,
            "btcusdt",
            MessageType::Trade
        )
    }

    #[test]
    fn test_crawl_l2_event() {
        gen_test_code!(
            crawl_l2_event,
            "Binance",
            MarketType::Spot,
            "btcusdt",
            MessageType::L2Event
        )
    }

    #[test]
    fn test_crawl_l2_snapshot() {
        gen_test_code!(
            crawl_l2_snapshot,
            "Binance",
            MarketType::Spot,
            "BTCUSDT",
            MessageType::L2Snapshot
        )
    }
}
