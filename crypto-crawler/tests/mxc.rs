#[macro_use]
mod utils;

#[cfg(test)]
mod mxc_spot {
    use crypto_crawler::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_crawl_trade() {
        gen_test_code!(
            crawl_trade,
            "MXC",
            MarketType::Spot,
            "BTC_USDT",
            MessageType::Trade
        )
    }

    #[test]
    fn test_crawl_l2_event() {
        gen_test_code!(
            crawl_l2_event,
            "MXC",
            MarketType::Spot,
            "BTC_USDT",
            MessageType::L2Event
        )
    }

    #[test]
    #[ignore]
    fn test_crawl_l2_snapshot() {
        gen_test_code!(
            crawl_l2_snapshot,
            "MXC",
            MarketType::Spot,
            "BTC_USDT",
            MessageType::L2Snapshot
        )
    }
}

#[cfg(test)]
mod mxc_swap {
    use crypto_crawler::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_crawl_trade() {
        gen_test_code!(
            crawl_trade,
            "MXC",
            MarketType::Swap,
            "BTC_USDT",
            MessageType::Trade
        )
    }

    #[test]
    fn test_crawl_l2_event() {
        gen_test_code!(
            crawl_l2_event,
            "MXC",
            MarketType::Swap,
            "BTC_USDT",
            MessageType::L2Event
        )
    }

    #[test]
    fn test_crawl_l2_snapshot() {
        gen_test_code!(
            crawl_l2_snapshot,
            "MXC",
            MarketType::Swap,
            "BTC_USDT",
            MessageType::L2Snapshot
        )
    }
}
