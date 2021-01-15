#[macro_use]
mod utils;

#[cfg(test)]
mod huobi_spot {
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
            "huobi",
            MarketType::Spot,
            "btcusdt",
            MessageType::Trade
        )
    }

    #[test]
    fn test_crawl_l2_event() {
        gen_test_code!(
            crawl_l2_event,
            "huobi",
            MarketType::Spot,
            "btcusdt",
            MessageType::L2Event
        )
    }

    #[test]
    fn test_crawl_l2_snapshot() {
        gen_test_snapshot_code!(
            crawl_l2_snapshot,
            "huobi",
            MarketType::Spot,
            "btcusdt",
            MessageType::L2Snapshot
        )
    }
}

#[cfg(test)]
mod huobi_inverse_future {
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
            "huobi",
            MarketType::InverseFuture,
            "BTC_CQ",
            MessageType::Trade
        )
    }

    #[test]
    fn test_crawl_l2_event() {
        gen_test_code!(
            crawl_l2_event,
            "huobi",
            MarketType::InverseFuture,
            "BTC_CQ",
            MessageType::L2Event
        )
    }

    #[test]
    fn test_crawl_l2_snapshot() {
        gen_test_snapshot_code!(
            crawl_l2_snapshot,
            "huobi",
            MarketType::InverseFuture,
            "BTC_CQ",
            MessageType::L2Snapshot
        )
    }
}

#[cfg(test)]
mod huobi_linear_swap {
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
            "huobi",
            MarketType::LinearSwap,
            "BTC-USDT",
            MessageType::Trade
        )
    }

    #[test]
    fn test_crawl_l2_event() {
        gen_test_code!(
            crawl_l2_event,
            "huobi",
            MarketType::LinearSwap,
            "BTC-USDT",
            MessageType::L2Event
        )
    }

    #[test]
    fn test_crawl_l2_snapshot() {
        gen_test_snapshot_code!(
            crawl_l2_snapshot,
            "huobi",
            MarketType::LinearSwap,
            "BTC-USDT",
            MessageType::L2Snapshot
        )
    }
}

#[cfg(test)]
mod huobi_inverse_swap {
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
            "huobi",
            MarketType::InverseSwap,
            "BTC-USD",
            MessageType::Trade
        )
    }

    #[test]
    fn test_crawl_l2_event() {
        gen_test_code!(
            crawl_l2_event,
            "huobi",
            MarketType::InverseSwap,
            "BTC-USD",
            MessageType::L2Event
        )
    }

    #[test]
    fn test_crawl_l2_snapshot() {
        gen_test_snapshot_code!(
            crawl_l2_snapshot,
            "huobi",
            MarketType::InverseSwap,
            "BTC-USD",
            MessageType::L2Snapshot
        )
    }
}

#[cfg(test)]
mod huobi_option {
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
            "huobi",
            MarketType::Option,
            "BTC-USDT-210326-C-32000",
            MessageType::Trade
        )
    }

    #[test]
    fn test_crawl_l2_event() {
        gen_test_code!(
            crawl_l2_event,
            "huobi",
            MarketType::Option,
            "BTC-USDT-210326-C-32000",
            MessageType::L2Event
        )
    }

    #[test]
    fn test_crawl_l2_snapshot() {
        gen_test_snapshot_code!(
            crawl_l2_snapshot,
            "huobi",
            MarketType::Option,
            "BTC-USDT-210326-C-32000",
            MessageType::L2Snapshot
        )
    }
}
