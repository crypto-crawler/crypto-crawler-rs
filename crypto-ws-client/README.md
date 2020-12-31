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
    let mut ws_client = BinanceSpotWSClient::new(Box::new(|msg| println!("{}", msg)), None);
    let channels = vec!["btcusdt@aggTrade".to_string(), "btcusdt@depth".to_string(),];
    ws_client.subscribe(&channels);
    ws_client.run(None);
    ws_client.close();
}
```

## Contribution

### How to add support for a new exchange

#### 1. Add a new file under `src/clients/`

Define a struct in the file, with the same name as the file.

Define a customized `serialize_command()`.

Use `define_client!` macro to implement the `WSClient` trait.

#### 2. Add exchange specific handling logic in `handle_msg()` of `ws_client_internal.rs`

#### 3. Add a new file under `tests/`

Add a new file under `tests/` and put some integration tests in it.
