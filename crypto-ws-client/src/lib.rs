//! A versatile websocket client that supports many cryptocurrency exchanges.
//!
//! ## Example
//!
//! ```
//! use crypto_ws_client::{BinanceSpotWSClient, WSClient};
//!
//! #[tokio::main]
//! async fn main() {
//!     let (tx, rx) = std::sync::mpsc::channel();
//!     tokio::task::spawn(async move {
//!         let symbols = vec!["BTCUSDT".to_string(), "ETHUSDT".to_string()];
//!         let ws_client = BinanceSpotWSClient::new(tx, None).await;
//!         ws_client.subscribe_trade(&symbols).await;
//!         // run for 5 seconds
//!         let _ = tokio::time::timeout(std::time::Duration::from_secs(5), ws_client.run()).await;
//!         ws_client.close();
//!     });
//!
//!     let mut messages = Vec::new();
//!     for msg in rx {
//!         messages.push(msg);
//!     }
//!     assert!(!messages.is_empty());
//! }
//! ```
//! ## High Level APIs
//!
//! The following APIs are high-level APIs with ease of use:
//!
//! * `subscribe_trade(&self, symbols: &[String])`
//! * `subscribe_bbo(&self, symbols: &[String])`
//! * `subscribe_orderbook(&self, symbols: &[String])`
//! * `subscribe_ticker(&self, symbols: &[String])`
//! * `subscribe_candlestick(&self, symbol_interval_list: &[(String, usize)])`
//!
//! They are easier to use and cover most user scenarios.
//!
//! ## Low Level APIs
//!
//! Sometimes high-level APIs can NOT meet users' requirements, this package provides three
//! low-level APIs:
//!
//! * `subscribe(&self, topics: &[(String, String)])`
//! * `unsubscribe(&self, topics: &[(String, String)])`
//! * `send(&self, commands: &[String])`
//!
//! ## OrderBook Data Categories
//!
//! Each orderbook has three properties: `aggregation`, `frequency` and `depth`.
//!
//! |                      | Aggregated        | Non-Aggregated |
//! | -------------------- | ----------------- | -------------- |
//! | Updated per Tick     | Inremental Level2 | Level3         |
//! | Updated per Interval | Snapshot Level2   | Not Usefull    |
//!
//! * Level1 data is non-aggregated, updated per tick, top 1 bid & ask from the original orderbook.
//! * Level2 data is aggregated by price level, updated per tick.
//! * Level3 data is the original orderbook, which is not aggregated.

mod clients;
mod common;

pub use common::ws_client::WSClient;

pub use clients::{
    binance::*, binance_option::*, bitfinex::*, bitget::*, bithumb::*, bitmex::*, bitstamp::*,
    bitz::*, bybit::*, coinbase_pro::*, deribit::*, dydx::*, ftx::*, gate::*, huobi::*, kraken::*,
    kucoin::*, mexc::*, okx::*, zb::*, zbg::*,
};
