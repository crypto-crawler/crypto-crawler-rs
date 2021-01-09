#[macro_use]
mod utils;

#[cfg(test)]
mod bitmex_swap {
    use crypto_crawler::*;
    use std::{cell::RefCell, rc::Rc};

    #[test]
    fn test_crawl_trade() {
        gen_test_code!(
            crawl_trade,
            "BitMEX",
            MarketType::Swap,
            "XBTUSD",
            MessageType::Trade
        )
    }

    #[test]
    fn test_crawl_l2_event() {
        gen_test_code!(
            crawl_l2_event,
            "BitMEX",
            MarketType::Swap,
            "XBTUSD",
            MessageType::L2Event
        )
    }

    #[test]
    fn test_crawl_l2_snapshot() {
        gen_test_code!(
            crawl_l2_snapshot,
            "BitMEX",
            MarketType::Swap,
            "XBTUSD",
            MessageType::L2Snapshot
        )
    }
}

#[cfg(test)]
mod bitmex_future {
    use crypto_crawler::*;
    use std::{cell::RefCell, rc::Rc};

    #[test]
    fn test_crawl_trade() {
        gen_test_code!(
            crawl_trade,
            "BitMEX",
            MarketType::Future,
            "XBTM21",
            MessageType::Trade
        )
    }

    #[test]
    fn test_crawl_l2_event() {
        gen_test_code!(
            crawl_l2_event,
            "BitMEX",
            MarketType::Future,
            "XBTM21",
            MessageType::L2Event
        )
    }

    #[test]
    fn test_crawl_l2_snapshot() {
        gen_test_code!(
            crawl_l2_snapshot,
            "BitMEX",
            MarketType::Future,
            "XBTM21",
            MessageType::L2Snapshot
        )
    }
}
