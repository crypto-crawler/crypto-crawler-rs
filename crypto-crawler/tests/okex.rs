#[macro_use]
mod utils;

#[cfg(test)]
mod okex_spot {
    use crypto_crawler::*;

    #[test]
    fn test_crawl_trade() {
        gen_test_code!(
            crawl_trade,
            "OKEx",
            MarketType::Spot,
            "BTC-USDT",
            MessageType::Trade
        )
    }

    #[test]
    fn test_crawl_l2_event() {
        gen_test_code!(
            crawl_l2_event,
            "OKEx",
            MarketType::Spot,
            "BTC-USDT",
            MessageType::L2Event
        )
    }

    #[test]
    fn test_crawl_l2_snapshot() {
        gen_test_code!(
            crawl_l2_snapshot,
            "OKEx",
            MarketType::Spot,
            "BTC-USDT",
            MessageType::L2Snapshot
        )
    }
}

#[cfg(test)]
mod okex_future {
    use crypto_crawler::*;

    #[test]
    fn test_crawl_trade() {
        gen_test_code!(
            crawl_trade,
            "OKEx",
            MarketType::Future,
            "BTC-USDT-210625",
            MessageType::Trade
        )
    }

    #[test]
    fn test_crawl_l2_event() {
        gen_test_code!(
            crawl_l2_event,
            "OKEx",
            MarketType::Future,
            "BTC-USDT-210625",
            MessageType::L2Event
        )
    }

    #[test]
    fn test_crawl_l2_snapshot() {
        gen_test_code!(
            crawl_l2_snapshot,
            "OKEx",
            MarketType::Future,
            "BTC-USDT-210625",
            MessageType::L2Snapshot
        )
    }
}

#[cfg(test)]
mod okex_swap {
    use crypto_crawler::*;

    #[test]
    fn test_crawl_trade() {
        gen_test_code!(
            crawl_trade,
            "OKEx",
            MarketType::Swap,
            "BTC-USDT-SWAP",
            MessageType::Trade
        )
    }

    #[test]
    fn test_crawl_l2_event() {
        gen_test_code!(
            crawl_l2_event,
            "OKEx",
            MarketType::Swap,
            "BTC-USDT-SWAP",
            MessageType::L2Event
        )
    }

    #[test]
    fn test_crawl_l2_snapshot() {
        gen_test_code!(
            crawl_l2_snapshot,
            "OKEx",
            MarketType::Swap,
            "BTC-USDT-SWAP",
            MessageType::L2Snapshot
        )
    }
}

#[cfg(test)]
mod okex_option {
    use crypto_crawler::*;

    #[test]
    fn test_crawl_trade() {
        gen_test_code!(
            crawl_trade,
            "OKEx",
            MarketType::Option,
            "BTC-USD-210625-72000-C",
            MessageType::Trade
        )
    }

    #[test]
    fn test_crawl_l2_event() {
        gen_test_code!(
            crawl_l2_event,
            "OKEx",
            MarketType::Option,
            "BTC-USD-210625-72000-C",
            MessageType::L2Event
        )
    }

    #[test]
    fn test_crawl_l2_snapshot() {
        gen_test_code!(
            crawl_l2_snapshot,
            "OKEx",
            MarketType::Option,
            "BTC-USD-210625-72000-C",
            MessageType::L2Snapshot
        )
    }
}
