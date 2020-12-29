# crypto-pair

[![](https://img.shields.io/github/workflow/status/soulmachine/crypto-crawler-rs/CI/main)](https://github.com/soulmachine/crypto-crawler-rs/actions?query=branch%3Amain)
[![](https://img.shields.io/crates/v/crypto-pair.svg)](https://crates.io/crates/crypto-pair)
[![](https://docs.rs/crypto-pair/badge.svg)](https://docs.rs/crypto-pair)
==========

Get all trading pairs of a cryptocurrency exchange.

## Usage

```rust
use crypto_pair::fetch_markets;

fn main() {
    assert_eq!(Some("BTC_USD".to_string()), normalize_pair("XBTH21", "BitMEX"));
}
```
