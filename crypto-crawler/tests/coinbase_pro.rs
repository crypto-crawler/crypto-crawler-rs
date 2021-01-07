use crypto_crawler::*;

#[macro_use]
mod utils;

#[test]
fn test_crawl_trade() {
    gen_test_code!(
        crawl_trade,
        "CoinbasePro",
        MarketType::Spot,
        "BTC-USD",
        MessageType::Trade
    )
}

#[test]
fn test_crawl_l2_event() {
    gen_test_code!(
        crawl_l2_event,
        "CoinbasePro",
        MarketType::Spot,
        "BTC-USD",
        MessageType::L2Event
    )
}

#[test]
fn test_crawl_l2_snapshot() {
    gen_test_code!(
        crawl_l2_snapshot,
        "CoinbasePro",
        MarketType::Spot,
        "BTC-USD",
        MessageType::L2Snapshot
    )
}

#[test]
fn test_crawl_l3_event() {
    gen_test_code!(
        crawl_l3_event,
        "CoinbasePro",
        MarketType::Spot,
        "BTC-USD",
        MessageType::L3Event
    )
}

#[test]
fn test_crawl_l3_snapshot() {
    gen_test_code!(
        crawl_l3_snapshot,
        "CoinbasePro",
        MarketType::Spot,
        "BTC-USD",
        MessageType::L3Snapshot
    )
}
