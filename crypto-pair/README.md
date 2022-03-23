# crypto-pair

[![](https://img.shields.io/github/workflow/status/crypto-crawler/crypto-crawler-rs/CI/main)](https://github.com/crypto-crawler/crypto-crawler-rs/actions?query=branch%3Amain)
[![](https://img.shields.io/crates/v/crypto-pair.svg)](https://crates.io/crates/crypto-pair)
[![](https://docs.rs/crypto-pair/badge.svg)](https://docs.rs/crypto-pair)
==========

Parse exchange-specific symbols to unified format.

## Usage

```rust
use crypto_pair::fetch_markets;

fn main() {
    assert_eq!(Some("BTC/USD".to_string()), normalize_pair("XBTH21", "BitMEX"));
}
```
