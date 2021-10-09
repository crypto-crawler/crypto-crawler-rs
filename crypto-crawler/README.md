# crypto-crawler

[![](https://img.shields.io/github/workflow/status/soulmachine/crypto-crawler-rs/CI/main)](https://github.com/soulmachine/crypto-crawler-rs/actions?query=branch%3Amain)
[![](https://img.shields.io/crates/v/crypto-crawler.svg)](https://crates.io/crates/crypto-crawler)
[![](https://docs.rs/crypto-crawler/badge.svg)](https://docs.rs/crypto-crawler)
==========

A rock-solid cryprocurrency crawler.

## Crawl realtime trades

```rust
use crypto_crawler::{crawl_trade, MarketType, Message};

let (tx, rx) = std::sync::mpsc::channel();
std::thread::spawn(move || {
    for msg in rx {
        println!("{}", msg);
    }
});

// Crawl realtime trades for all symbols of binance inverse_swap markets
crawl_trade("binance", MarketType::InverseSwap, None, tx, None);
```

## Crawl realtime level2 orderbook incremental updates

```rust
use crypto_crawler::{crawl_l2_event, MarketType, Message};

let (tx, rx) = std::sync::mpsc::channel();
std::thread::spawn(move || {
    for msg in rx {
        println!("{}", msg);
    }
});

// Crawl realtime level2 incremental updates for all symbols of binance inverse_swap markets
crawl_l2_event("binance", MarketType::InverseSwap, None, tx, None);
```

## Crawl level2 orderbook full snapshots from RESTful API

```rust
use crypto_crawler::{crawl_l2_snapshot, MarketType, Message};

let (tx, rx) = std::sync::mpsc::channel();
std::thread::spawn(move || {
    for msg in rx {
        println!("{}", msg);
    }
});

// Crawl level2 full snapshots for all symbols of binance inverse_swap markets
crawl_l2_snapshot("binance", MarketType::InverseSwap, None, tx, None);
```

## Crawl realtime level2 orderbook top-K snapshots

```rust
use crypto_crawler::{crawl_l2_topk, MarketType, Message};

let (tx, rx) = std::sync::mpsc::channel();
std::thread::spawn(move || {
    for msg in rx {
        println!("{}", msg);
    }
});

// Crawl realtime level2 top-k snapshots for all symbols of binance inverse_swap markets
crawl_l2_topk("binance", MarketType::InverseSwap, None, tx, None);
```

## Crawl realtime level3 orderbook incremental updates

```rust
use crypto_crawler::{crawl_l3_event, MarketType, Message};

let (tx, rx) = std::sync::mpsc::channel();
std::thread::spawn(move || {
    for msg in rx {
        println!("{}", msg);
    }
});

// Crawl realtime level3 updates for all symbols of CoinbasePro spot market
crawl_l3_event("coinbase_pro", MarketType::Spot, None, tx, None);
```

## Crawl level3 orderbook full snapshots from RESTful API

```rust
use crypto_crawler::{crawl_l3_snapshot, MarketType, Message};

let (tx, rx) = std::sync::mpsc::channel();
std::thread::spawn(move || {
    for msg in rx {
        println!("{}", msg);
    }
});

// Crawl level3 orderbook full snapshots for all symbols of CoinbasePro spot markets
crawl_l3_snapshot("coinbase_pro", MarketType::Spot, None, tx, None);
```

## Crawl realtime BBO

```rust
use crypto_crawler::{crawl_bbo, MarketType, Message};

let (tx, rx) = std::sync::mpsc::channel();
std::thread::spawn(move || {
    for msg in rx {
        println!("{}", msg);
    }
});

// Crawl realtime best bid and ask messages for all symbols of binance COIN-margined perpetual markets
crawl_bbo("binance", MarketType::InverseSwap, None, tx, None);
```

## Crawl 24hr rolling window tickers

```rust
use crypto_crawler::{crawl_ticker, MarketType, Message};

let (tx, rx) = std::sync::mpsc::channel();
std::thread::spawn(move || {
    for msg in rx {
        println!("{}", msg);
    }
});

// Crawl 24hr rolling window tickers for all symbols of binance COIN-margined perpetual markets
crawl_ticker("binance", MarketType::InverseSwap, None, tx, None);
```

## Crawl candlesticks(i.e., OHLCV)

```rust
use crypto_crawler::{crawl_candlestick, MarketType, Message};

let (tx, rx) = std::sync::mpsc::channel();
std::thread::spawn(move || {
    for msg in rx {
        println!("{}", msg);
    }
});

// Crawl candlesticks from 1 minute to 3 minutes for all symbols of binance COIN-margined perpetual markets
crawl_candlestick("binance", MarketType::InverseSwap, None, tx, None);
```

## Crawl funding rates

```rust
use crypto_crawler::{crawl_funding_rate, MarketType, Message};

let (tx, rx) = std::sync::mpsc::channel();
std::thread::spawn(move || {
    for msg in rx {
        println!("{}", msg);
    }
});

// Crawl funding rates for all symbols of binance COIN-margined perpetual markets
crawl_funding_rate("binance", MarketType::InverseSwap, None, tx, None);
```
