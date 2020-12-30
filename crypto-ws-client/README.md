# crypto-ws-client

[![](https://img.shields.io/github/workflow/status/soulmachine/crypto-crawler-rs/CI/main)](https://github.com/soulmachine/crypto-crawler-rs/actions?query=branch%3Amain)
[![](https://img.shields.io/crates/v/crypto-ws-client.svg)](https://crates.io/crates/crypto-ws-client)
[![](https://docs.rs/crypto-ws-client/badge.svg)](https://docs.rs/crypto-ws-client)
==========

A versatile websocket client that supports many cryptocurrency exchanges.

## Usage

```rust
use crypto_ws_client::{BinanceSpotWSClient, WSClient};

fn main() {
    let mut ws_client = BinanceSpotWSClient::init(|msg| println!("{}", msg), None);
    let channels = vec!["btcusdt@aggTrade".to_string(), "btcusdt@depth".to_string(),];
    ws_client.subscribe(&channels);
    ws_client.run(None);
    ws_client.close();
}
```
