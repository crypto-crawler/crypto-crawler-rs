#[macro_use]
mod utils;

#[cfg(test)]
mod huobi_spot {
    use crypto_crawler::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_crawl_trade() {
        gen_test_code!(
            crawl_trade,
            "Huobi",
            MarketType::Spot,
            "btcusdt",
            MessageType::Trade
        )
    }

    #[test]
    fn test_crawl_l2_event() {
        gen_test_code!(
            crawl_l2_event,
            "Huobi",
            MarketType::Spot,
            "btcusdt",
            MessageType::L2Event
        )
    }

    #[test]
    fn test_crawl_l2_snapshot() {
        gen_test_code!(
            crawl_l2_snapshot,
            "Huobi",
            MarketType::Spot,
            "btcusdt",
            MessageType::L2Snapshot
        )
    }
}

#[cfg(test)]
mod huobi_future {
    use crypto_crawler::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_crawl_trade() {
        gen_test_code!(
            crawl_trade,
            "Huobi",
            MarketType::Future,
            "BTC_CQ",
            MessageType::Trade
        )
    }

    #[test]
    fn test_crawl_l2_event() {
        gen_test_code!(
            crawl_l2_event,
            "Huobi",
            MarketType::Future,
            "BTC_CQ",
            MessageType::L2Event
        )
    }

    #[test]
    fn test_crawl_l2_snapshot() {
        gen_test_code!(
            crawl_l2_snapshot,
            "Huobi",
            MarketType::Future,
            "BTC_CQ",
            MessageType::L2Snapshot
        )
    }
}

#[cfg(test)]
mod huobi_linear_swap {
    use crypto_crawler::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_crawl_trade() {
        gen_test_code!(
            crawl_trade,
            "Huobi",
            MarketType::Swap,
            "BTC-USDT",
            MessageType::Trade
        )
    }

    #[test]
    fn test_crawl_l2_event() {
        gen_test_code!(
            crawl_l2_event,
            "Huobi",
            MarketType::Swap,
            "BTC-USDT",
            MessageType::L2Event
        )
    }

    #[test]
    fn test_crawl_l2_snapshot() {
        gen_test_code!(
            crawl_l2_snapshot,
            "Huobi",
            MarketType::Swap,
            "BTC-USDT",
            MessageType::L2Snapshot
        )
    }
}

#[cfg(test)]
mod huobi_inverse_swap {
    use crypto_crawler::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_crawl_trade() {
        gen_test_code!(
            crawl_trade,
            "Huobi",
            MarketType::Swap,
            "BTC-USD",
            MessageType::Trade
        )
    }

    #[test]
    fn test_crawl_l2_event() {
        gen_test_code!(
            crawl_l2_event,
            "Huobi",
            MarketType::Swap,
            "BTC-USD",
            MessageType::L2Event
        )
    }

    #[test]
    fn test_crawl_l2_snapshot() {
        gen_test_code!(
            crawl_l2_snapshot,
            "Huobi",
            MarketType::Swap,
            "BTC-USD",
            MessageType::L2Snapshot
        )
    }
}

#[cfg(test)]
mod huobi_option {
    use crypto_crawler::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_crawl_trade() {
        gen_test_code!(
            crawl_trade,
            "Huobi",
            MarketType::Option,
            "BTC-USDT-210326-C-32000",
            MessageType::Trade
        )
    }

    #[test]
    fn test_crawl_l2_event() {
        gen_test_code!(
            crawl_l2_event,
            "Huobi",
            MarketType::Option,
            "BTC-USDT-210326-C-32000",
            MessageType::L2Event
        )
    }

    #[test]
    fn test_crawl_l2_snapshot() {
        gen_test_code!(
            crawl_l2_snapshot,
            "Huobi",
            MarketType::Option,
            "BTC-USDT-210326-C-32000",
            MessageType::L2Snapshot
        )
    }
}
