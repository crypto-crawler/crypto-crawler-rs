# crypto-crawler

[![](https://img.shields.io/github/workflow/status/soulmachine/crypto-crawler-rs/CI/main)](https://github.com/soulmachine/crypto-crawler-rs/actions?query=branch%3Amain)
[![](https://img.shields.io/crates/v/crypto-crawler.svg)](https://crates.io/crates/crypto-crawler)
[![](https://docs.rs/crypto-crawler/badge.svg)](https://docs.rs/crypto-crawler)
==========

A rock-solid cryprocurrency crawler.

## Crawl realtime trades

```rust
use std::sync::{Arc, Mutex};
use crypto_crawler::{crawl_trade, MarketType, Message};

fn main() {
    let on_msg = Arc::new(Mutex::new(|msg: Message| {
        println!("{}", msg);
    }));

    // Crawl BitMEX inverse_swap market, for all symbols
    crawl_trade("bitmex", MarketType::InverseSwap, None, on_msg, None);
}
```

## Crawl level2 orderbook update events

```rust
use std::sync::{Arc, Mutex};
use crypto_crawler::{crawl_l2_event, MarketType, Message};

fn main() {
    let on_msg = Arc::new(Mutex::new(|msg: Message| {
        println!("{}", msg);
    }));

    // Crawl BitMEX inverse_swap market, for all symbols
    crawl_l2_event("bitmex", MarketType::InverseSwap, None, on_msg, None);
}
```

## Crawl level2 orderbook snapshots

```rust
use std::sync::{Arc, Mutex};
use crypto_crawler::{crawl_l2_snapshot, MarketType, Message};

fn main() {
    let on_msg = Arc::new(Mutex::new(|msg: Message| {
        println!("{}", msg);
    }));

    // Crawl BitMEX inverse_swap market level2 orderbook snapshots every 60 seconds, for all symbols
    crawl_l2_snapshot("bitmex", MarketType::InverseSwap, None, on_msg, Some(60), None);
}
```

## Crawl level3 orderbook update events

```rust
use std::sync::{Arc, Mutex};
use crypto_crawler::{crawl_l3_event, MarketType, Message};

fn main() {
    let on_msg = Arc::new(Mutex::new(|msg: Message| {
        println!("{}", msg);
    }));

    // Crawl CoinbasePro spot market, for all symbols
    crawl_l3_event("coinbase_pro", MarketType::Spot, None, on_msg, None);
}
```

## Crawl level3 orderbook snapshots

```rust
use std::sync::{Arc, Mutex};
use crypto_crawler::{crawl_l3_snapshot, MarketType, Message};

fn main() {
    let on_msg = Arc::new(Mutex::new(|msg: Message| {
        println!("{}", msg);
    }));

    // Crawl CoinbasePro spot market level2 orderbook snapshots every 60 seconds, for all symbols
    crawl_l3_snapshot("coinbase_pro", MarketType::Spot, None, on_msg, Some(60), None);
}
```
