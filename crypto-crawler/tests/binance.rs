#[macro_use]
mod utils;

#[cfg(test)]
mod binance_spot {
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
            "binance",
            MarketType::Spot,
            "btcusdt",
            MessageType::Trade
        )
    }

    #[test]
    fn test_crawl_l2_event() {
        gen_test_code!(
            crawl_l2_event,
            "binance",
            MarketType::Spot,
            "btcusdt",
            MessageType::L2Event
        )
    }

    #[test]
    fn test_crawl_l2_snapshot() {
        gen_test_snapshot_code!(
            crawl_l2_snapshot,
            "binance",
            MarketType::Spot,
            "BTCUSDT",
            MessageType::L2Snapshot
        )
    }
}

#[cfg(test)]
mod binance_future {
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
            "binance",
            MarketType::InverseFuture,
            "btcusd_210625",
            MessageType::Trade
        )
    }

    #[test]
    fn test_crawl_l2_event() {
        gen_test_code!(
            crawl_l2_event,
            "binance",
            MarketType::InverseFuture,
            "btcusd_210625",
            MessageType::L2Event
        )
    }

    #[test]
    fn test_crawl_l2_snapshot() {
        gen_test_snapshot_code!(
            crawl_l2_snapshot,
            "binance",
            MarketType::InverseFuture,
            "BTCUSD_210625",
            MessageType::L2Snapshot
        )
    }
}

#[cfg(test)]
mod binance_linear_swap {
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
            "binance",
            MarketType::LinearSwap,
            "btcusdt",
            MessageType::Trade
        )
    }

    #[test]
    fn test_crawl_l2_event() {
        gen_test_code!(
            crawl_l2_event,
            "binance",
            MarketType::LinearSwap,
            "btcusdt",
            MessageType::L2Event
        )
    }

    #[test]
    fn test_crawl_l2_snapshot() {
        gen_test_snapshot_code!(
            crawl_l2_snapshot,
            "binance",
            MarketType::LinearSwap,
            "BTCUSDT",
            MessageType::L2Snapshot
        )
    }
}

#[cfg(test)]
mod binance_inverse_swap {
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
            "binance",
            MarketType::InverseSwap,
            "btcusd_perp",
            MessageType::Trade
        )
    }

    #[test]
    fn test_crawl_l2_event() {
        gen_test_code!(
            crawl_l2_event,
            "binance",
            MarketType::InverseSwap,
            "btcusd_perp",
            MessageType::L2Event
        )
    }

    #[test]
    fn test_crawl_l2_snapshot() {
        gen_test_snapshot_code!(
            crawl_l2_snapshot,
            "binance",
            MarketType::InverseSwap,
            "BTCUSD_PERP",
            MessageType::L2Snapshot
        )
    }
}

#[cfg(test)]
mod binance_option {
    use crypto_crawler::*;
    use crypto_markets::MarketType;
    use std::thread_local;
    use std::{
        cell::RefCell,
        sync::{Arc, Mutex},
    };

    #[test]
    #[ignore]
    fn test_crawl_trade() {
        gen_test_code!(
            crawl_trade,
            "binance",
            MarketType::Option,
            "BTC-210129-40000-C",
            MessageType::Trade
        )
    }

    #[test]
    #[ignore]
    fn test_crawl_l2_event() {
        gen_test_code!(
            crawl_l2_event,
            "binance",
            MarketType::Option,
            "BTC-210129-40000-C",
            MessageType::L2Event
        )
    }

    #[test]
    #[ignore]
    fn test_crawl_l2_snapshot() {
        gen_test_snapshot_code!(
            crawl_l2_snapshot,
            "binance",
            MarketType::Option,
            "BTC-210129-40000-C",
            MessageType::L2Snapshot
        )
    }
}
