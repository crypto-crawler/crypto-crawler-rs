#[macro_use]
mod utils;

#[cfg(test)]
mod okex_spot {
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
            "okex",
            MarketType::Spot,
            "BTC-USDT",
            MessageType::Trade
        )
    }

    #[test]
    fn test_crawl_l2_event() {
        gen_test_code!(
            crawl_l2_event,
            "okex",
            MarketType::Spot,
            "BTC-USDT",
            MessageType::L2Event
        )
    }

    #[test]
    fn test_crawl_l2_snapshot() {
        gen_test_snapshot_code!(
            crawl_l2_snapshot,
            "okex",
            MarketType::Spot,
            "BTC-USDT",
            MessageType::L2Snapshot
        )
    }
}

#[cfg(test)]
mod okex_linear_future {
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
            "okex",
            MarketType::LinearFuture,
            "BTC-USDT-210625",
            MessageType::Trade
        )
    }

    #[test]
    fn test_crawl_l2_event() {
        gen_test_code!(
            crawl_l2_event,
            "okex",
            MarketType::LinearFuture,
            "BTC-USDT-210625",
            MessageType::L2Event
        )
    }

    #[test]
    fn test_crawl_l2_snapshot() {
        gen_test_snapshot_code!(
            crawl_l2_snapshot,
            "okex",
            MarketType::LinearFuture,
            "BTC-USDT-210625",
            MessageType::L2Snapshot
        )
    }
}

#[cfg(test)]
mod okex_linear_swap {
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
            "okex",
            MarketType::LinearSwap,
            "BTC-USDT-SWAP",
            MessageType::Trade
        )
    }

    #[test]
    fn test_crawl_l2_event() {
        gen_test_code!(
            crawl_l2_event,
            "okex",
            MarketType::LinearSwap,
            "BTC-USDT-SWAP",
            MessageType::L2Event
        )
    }

    #[test]
    fn test_crawl_l2_snapshot() {
        gen_test_snapshot_code!(
            crawl_l2_snapshot,
            "okex",
            MarketType::LinearSwap,
            "BTC-USDT-SWAP",
            MessageType::L2Snapshot
        )
    }
}

#[cfg(test)]
mod okex_option {
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
            "okex",
            MarketType::Option,
            "BTC-USD-210625-72000-C",
            MessageType::Trade
        )
    }

    #[test]
    fn test_crawl_l2_event() {
        gen_test_code!(
            crawl_l2_event,
            "okex",
            MarketType::Option,
            "BTC-USD-210625-72000-C",
            MessageType::L2Event
        )
    }

    #[test]
    fn test_crawl_l2_snapshot() {
        gen_test_snapshot_code!(
            crawl_l2_snapshot,
            "okex",
            MarketType::Option,
            "BTC-USD-210625-72000-C",
            MessageType::L2Snapshot
        )
    }
}
