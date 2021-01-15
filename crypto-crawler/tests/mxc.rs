#[macro_use]
mod utils;

#[cfg(test)]
mod mxc_spot {
    use crypto_crawler::*;
    use crypto_markets::MarketType;
    use std::thread_local;
    use std::{
        cell::RefCell,
        sync::{Arc, Mutex},
    };

    #[test]
    fn test_crawl_trade() {
        gen_test_code!(
            crawl_trade,
            "mxc",
            MarketType::Spot,
            "BTC_USDT",
            MessageType::Trade
        )
    }

    #[test]
    fn test_crawl_l2_event() {
        gen_test_code!(
            crawl_l2_event,
            "mxc",
            MarketType::Spot,
            "BTC_USDT",
            MessageType::L2Event
        )
    }

    #[test]
    #[ignore]
    fn test_crawl_l2_snapshot() {
        gen_test_snapshot_code!(
            crawl_l2_snapshot,
            "mxc",
            MarketType::Spot,
            "BTC_USDT",
            MessageType::L2Snapshot
        )
    }
}

#[cfg(test)]
mod mxc_linear_swap {
    use crypto_crawler::*;
    use crypto_markets::MarketType;
    use std::thread_local;
    use std::{
        cell::RefCell,
        sync::{Arc, Mutex},
    };

    #[test]
    fn test_crawl_trade() {
        gen_test_code!(
            crawl_trade,
            "mxc",
            MarketType::LinearSwap,
            "BTC_USDT",
            MessageType::Trade
        )
    }

    #[test]
    fn test_crawl_l2_event() {
        gen_test_code!(
            crawl_l2_event,
            "mxc",
            MarketType::LinearSwap,
            "BTC_USDT",
            MessageType::L2Event
        )
    }

    #[test]
    fn test_crawl_l2_snapshot() {
        gen_test_snapshot_code!(
            crawl_l2_snapshot,
            "mxc",
            MarketType::LinearSwap,
            "BTC_USDT",
            MessageType::L2Snapshot
        )
    }
}
