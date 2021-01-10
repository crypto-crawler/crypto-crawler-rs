#[macro_use]
mod utils;

#[cfg(test)]
mod bitfinex_spot {
    use crypto_crawler::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_crawl_trade() {
        gen_test_code!(
            crawl_trade,
            "Bitfinex",
            MarketType::Spot,
            "tBTCUSD",
            MessageType::Trade
        )
    }

    #[test]
    fn test_crawl_l2_event() {
        gen_test_code!(
            crawl_l2_event,
            "Bitfinex",
            MarketType::Spot,
            "tBTCUSD",
            MessageType::L2Event
        )
    }

    #[test]
    fn test_crawl_l2_snapshot() {
        gen_test_code!(
            crawl_l2_snapshot,
            "Bitfinex",
            MarketType::Spot,
            "tBTCUSD",
            MessageType::L2Snapshot
        )
    }

    #[test]
    fn test_crawl_l3_event() {
        gen_test_code!(
            crawl_l3_event,
            "Bitfinex",
            MarketType::Spot,
            "tBTCUSD",
            MessageType::L3Event
        )
    }

    #[test]
    fn test_crawl_l3_snapshot() {
        gen_test_code!(
            crawl_l3_snapshot,
            "Bitfinex",
            MarketType::Spot,
            "tBTCUSD",
            MessageType::L3Snapshot
        )
    }
}

#[cfg(test)]
mod bitfinex_swap {
    use crypto_crawler::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_crawl_trade() {
        gen_test_code!(
            crawl_trade,
            "Bitfinex",
            MarketType::Swap,
            "tBTCF0:USTF0",
            MessageType::Trade
        )
    }

    #[test]
    fn test_crawl_l2_event() {
        gen_test_code!(
            crawl_l2_event,
            "Bitfinex",
            MarketType::Swap,
            "tBTCF0:USTF0",
            MessageType::L2Event
        )
    }

    #[test]
    fn test_crawl_l2_snapshot() {
        gen_test_code!(
            crawl_l2_snapshot,
            "Bitfinex",
            MarketType::Swap,
            "tBTCF0:USTF0",
            MessageType::L2Snapshot
        )
    }

    #[test]
    fn test_crawl_l3_event() {
        gen_test_code!(
            crawl_l3_event,
            "Bitfinex",
            MarketType::Swap,
            "tBTCF0:USTF0",
            MessageType::L3Event
        )
    }

    #[test]
    fn test_crawl_l3_snapshot() {
        gen_test_code!(
            crawl_l3_snapshot,
            "Bitfinex",
            MarketType::Swap,
            "tBTCF0:USTF0",
            MessageType::L3Snapshot
        )
    }
}
