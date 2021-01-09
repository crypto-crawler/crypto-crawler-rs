#[macro_use]
mod utils;

#[cfg(test)]
mod binance_spot {
    use crypto_crawler::*;
    use std::{cell::RefCell, rc::Rc};

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

#[cfg(test)]
mod binance_future {
    use crypto_crawler::*;
    use std::{cell::RefCell, rc::Rc};

    #[test]
    fn test_crawl_trade() {
        gen_test_code!(
            crawl_trade,
            "Binance",
            MarketType::Future,
            "btcusd_210625",
            MessageType::Trade
        )
    }

    #[test]
    fn test_crawl_l2_event() {
        gen_test_code!(
            crawl_l2_event,
            "Binance",
            MarketType::Future,
            "btcusd_210625",
            MessageType::L2Event
        )
    }

    #[test]
    fn test_crawl_l2_snapshot() {
        gen_test_code!(
            crawl_l2_snapshot,
            "Binance",
            MarketType::Future,
            "BTCUSD_210625",
            MessageType::L2Snapshot
        )
    }
}

#[cfg(test)]
mod binance_linear_swap {
    use crypto_crawler::*;
    use std::{cell::RefCell, rc::Rc};

    #[test]
    fn test_crawl_trade() {
        gen_test_code!(
            crawl_trade,
            "Binance",
            MarketType::Swap,
            "btcusdt",
            MessageType::Trade
        )
    }

    #[test]
    fn test_crawl_l2_event() {
        gen_test_code!(
            crawl_l2_event,
            "Binance",
            MarketType::Swap,
            "btcusdt",
            MessageType::L2Event
        )
    }

    #[test]
    fn test_crawl_l2_snapshot() {
        gen_test_code!(
            crawl_l2_snapshot,
            "Binance",
            MarketType::Swap,
            "BTCUSDT",
            MessageType::L2Snapshot
        )
    }
}

#[cfg(test)]
mod binance_inverse_swap {
    use crypto_crawler::*;
    use std::{cell::RefCell, rc::Rc};

    #[test]
    fn test_crawl_trade() {
        gen_test_code!(
            crawl_trade,
            "Binance",
            MarketType::Swap,
            "btcusd_perp",
            MessageType::Trade
        )
    }

    #[test]
    fn test_crawl_l2_event() {
        gen_test_code!(
            crawl_l2_event,
            "Binance",
            MarketType::Swap,
            "btcusd_perp",
            MessageType::L2Event
        )
    }

    #[test]
    fn test_crawl_l2_snapshot() {
        gen_test_code!(
            crawl_l2_snapshot,
            "Binance",
            MarketType::Swap,
            "BTCUSD_PERP",
            MessageType::L2Snapshot
        )
    }
}

#[cfg(test)]
mod binance_option {
    use crypto_crawler::*;
    use std::{cell::RefCell, rc::Rc};

    #[test]
    #[ignore]
    fn test_crawl_trade() {
        gen_test_code!(
            crawl_trade,
            "Binance",
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
            "Binance",
            MarketType::Option,
            "BTC-210129-40000-C",
            MessageType::L2Event
        )
    }

    #[test]
    #[ignore]
    fn test_crawl_l2_snapshot() {
        gen_test_code!(
            crawl_l2_snapshot,
            "Binance",
            MarketType::Option,
            "BTC-210129-40000-C",
            MessageType::L2Snapshot
        )
    }
}
