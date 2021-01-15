#[macro_use]
mod utils;

#[cfg(test)]
mod bitmex_inverse_swap {
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
            "bitmex",
            MarketType::InverseSwap,
            "XBTUSD",
            MessageType::Trade
        )
    }

    #[test]
    fn test_crawl_l2_event() {
        gen_test_code!(
            crawl_l2_event,
            "bitmex",
            MarketType::InverseSwap,
            "XBTUSD",
            MessageType::L2Event
        )
    }

    #[test]
    fn test_crawl_l2_snapshot() {
        gen_test_snapshot_code!(
            crawl_l2_snapshot,
            "bitmex",
            MarketType::InverseSwap,
            "XBTUSD",
            MessageType::L2Snapshot
        )
    }
}

#[cfg(test)]
mod bitmex_inverse_future {
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
            "bitmex",
            MarketType::InverseFuture,
            "XBTM21",
            MessageType::Trade
        )
    }

    #[test]
    fn test_crawl_l2_event() {
        gen_test_code!(
            crawl_l2_event,
            "bitmex",
            MarketType::InverseFuture,
            "XBTM21",
            MessageType::L2Event
        )
    }

    #[test]
    fn test_crawl_l2_snapshot() {
        gen_test_snapshot_code!(
            crawl_l2_snapshot,
            "bitmex",
            MarketType::InverseFuture,
            "XBTM21",
            MessageType::L2Snapshot
        )
    }
}
