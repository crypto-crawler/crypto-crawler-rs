# crypto-crawler

[![](https://img.shields.io/github/workflow/status/soulmachine/crypto-crawler-rs/CI/main)](https://github.com/soulmachine/crypto-crawler-rs/actions?query=branch%3Amain)
[![](https://img.shields.io/crates/v/crypto-markets.svg)](https://crates.io/crates/crypto-crawler)
[![](https://docs.rs/crypto-markets/badge.svg)](https://docs.rs/crypto-crawler)
==========

Crawl websocket messages from crypto exchanges.

## Example

```rust
use crypto_crawler::{BinanceCrawler, CryptoCrawler};

fn main() {
    let mut ws_client = BinanceCrawler::new(Box::new(|msg| println!("{}", msg)), None);
    let channels = vec!["btcusdt@aggTrade".to_string(), "btcusdt@depth".to_string(),];
    ws_client.subscribe(&channels);
    ws_client.run(None);
    ws_client.close();
}
```
