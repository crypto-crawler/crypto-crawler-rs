# crypto-markets

[![](https://img.shields.io/github/workflow/status/soulmachine/crypto-crawler-rs/CI/main)](https://github.com/soulmachine/crypto-crawler-rs/actions?query=branch%3Amain)
[![](https://img.shields.io/crates/v/crypto-markets.svg)](https://crates.io/crates/crypto-markets)
[![](https://docs.rs/crypto-markets/badge.svg)](https://docs.rs/crypto-markets)
==========

Get all trading pairs of a cryptocurrency exchange.

## Example

```rust
use crypto_markets::{fetch_markets, MarketType};

fn main() {
    let markets = fetch_markets("Binance", MarketType::Spot).unwrap();
    println!("{}", serde_json::to_string_pretty(&markets).unwrap())
}
```
