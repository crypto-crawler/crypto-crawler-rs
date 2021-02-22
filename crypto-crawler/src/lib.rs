//! A rock-solid cryprocurrency crawler.
//!
//! ## Crawl realtime trades
//!
//! ```rust
//! use std::sync::{Arc, Mutex};
//! use crypto_crawler::{crawl_trade, MarketType, Message};
//!
//! let on_msg = Arc::new(Mutex::new(|msg: Message| {
//!     println!("{}", msg);
//! }));
//!
//! // Crawl BitMEX inverse_swap market, for all symbols, only run for 5 seconds
//! crawl_trade("bitmex", MarketType::InverseSwap, None, on_msg, Some(5));
//! ```
//!
//! ## Crawl level2 orderbook update events
//!
//! ```rust
//! use std::sync::{Arc, Mutex};
//! use crypto_crawler::{crawl_l2_event, MarketType, Message};
//!
//! let on_msg = Arc::new(Mutex::new(|msg: Message| {
//!     println!("{}", msg);
//! }));
//!
//! // Crawl BitMEX inverse_swap market, for all symbols, only run for 5 seconds
//! crawl_l2_event("bitmex", MarketType::InverseSwap, None, on_msg, Some(5));
//! ```
//!
//! ## Crawl level2 orderbook snapshots
//!
//! ```rust
//! use std::sync::{Arc, Mutex};
//! use crypto_crawler::{crawl_l2_snapshot, MarketType, Message};
//!
//! let on_msg = Arc::new(Mutex::new(|msg: Message| {
//!     println!("{}", msg);
//! }));
//!
//! // Crawl BitMEX inverse_swap market level2 orderbook snapshots every 60 seconds, for all symbols, only run for 5 seconds
//! crawl_l2_snapshot("bitmex", MarketType::InverseSwap, None, on_msg, Some(60), Some(5));
//! ```
//!
//! ## Crawl level3 orderbook update events
//!
//! ```rust
//! use std::sync::{Arc, Mutex};
//! use crypto_crawler::{crawl_l3_event, MarketType, Message};
//!
//! let on_msg = Arc::new(Mutex::new(|msg: Message| {
//!     println!("{}", msg);
//! }));
//!
//! // Crawl CoinbasePro spot market, for all symbols, only run for 5 seconds
//! crawl_l3_event("coinbase_pro", MarketType::Spot, None, on_msg, Some(5));
//! ```
//!
//! ## Crawl level3 orderbook snapshots
//!
//! ```rust
//! use std::sync::{Arc, Mutex};
//! use crypto_crawler::{crawl_l3_snapshot, MarketType, Message};
//!
//! let on_msg = Arc::new(Mutex::new(|msg: Message| {
//!     println!("{}", msg);
//! }));
//!
//! // Crawl CoinbasePro spot market level2 orderbook snapshots every 60 seconds, for all symbols, only run for 5 seconds
//! crawl_l3_snapshot("coinbase_pro", MarketType::Spot, None, on_msg, Some(60), Some(5));
//! ```
mod crawlers;
mod msg;

pub use crypto_markets::MarketType;
use std::sync::{Arc, Mutex};

pub use msg::*;

/// Crawl realtime trades.
///
/// If `symbols` is None, this function will crawl all trading symbols in the `market_type`,
/// and fetch the latest symbols every hour.
pub fn crawl_trade(
    exchange: &str,
    market_type: MarketType,
    symbols: Option<&[String]>,
    on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
    duration: Option<u64>,
) {
    let func = match exchange {
        "binance" => crawlers::binance::crawl_trade,
        "bitfinex" => crawlers::bitfinex::crawl_trade,
        "bitget" => crawlers::bitget::crawl_trade,
        "bithumb" => crawlers::bithumb::crawl_trade,
        "bitmex" => crawlers::bitmex::crawl_trade,
        "bitstamp" => crawlers::bitstamp::crawl_trade,
        "bitz" => crawlers::bitz::crawl_trade,
        "bybit" => crawlers::bybit::crawl_trade,
        "coinbase_pro" => crawlers::coinbase_pro::crawl_trade,
        "deribit" => crawlers::deribit::crawl_trade,
        "ftx" => crawlers::ftx::crawl_trade,
        "huobi" => crawlers::huobi::crawl_trade,
        "kraken" => crawlers::kraken::crawl_trade,
        "mxc" => crawlers::mxc::crawl_trade,
        "okex" => crawlers::okex::crawl_trade,
        "zbg" => crawlers::zbg::crawl_trade,
        _ => panic!("Unknown exchange {}", exchange),
    };
    let handle = func(market_type, symbols, on_msg, duration);
    if let Some(h) = handle {
        h.join().expect("The thread panicked");
    }
}

/// Crawl level2 orderbook update events.
pub fn crawl_l2_event(
    exchange: &str,
    market_type: MarketType,
    symbols: Option<&[String]>,
    on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
    duration: Option<u64>,
) {
    let func = match exchange {
        "binance" => crawlers::binance::crawl_l2_event,
        "bitfinex" => crawlers::bitfinex::crawl_l2_event,
        "bitget" => crawlers::bitget::crawl_l2_event,
        "bithumb" => crawlers::bithumb::crawl_l2_event,
        "bitmex" => crawlers::bitmex::crawl_l2_event,
        "bitstamp" => crawlers::bitstamp::crawl_l2_event,
        "bitz" => crawlers::bitz::crawl_l2_event,
        "bybit" => crawlers::bybit::crawl_l2_event,
        "coinbase_pro" => crawlers::coinbase_pro::crawl_l2_event,
        "deribit" => crawlers::deribit::crawl_l2_event,
        "ftx" => crawlers::ftx::crawl_l2_event,
        "huobi" => crawlers::huobi::crawl_l2_event,
        "kraken" => crawlers::kraken::crawl_l2_event,
        "mxc" => crawlers::mxc::crawl_l2_event,
        "okex" => crawlers::okex::crawl_l2_event,
        "zbg" => crawlers::zbg::crawl_l2_event,
        _ => panic!("Unknown exchange {}", exchange),
    };
    let handle = func(market_type, symbols, on_msg, duration);
    if let Some(h) = handle {
        h.join().expect("The thread panicked");
    }
}

/// Crawl level2 orderbook snapshots through RESTful APIs.
pub fn crawl_l2_snapshot(
    exchange: &str,
    market_type: MarketType,
    symbols: Option<&[String]>,
    on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
    interval: Option<u64>,
    duration: Option<u64>,
) {
    let func = match exchange {
        "binance" => crawlers::binance::crawl_l2_snapshot,
        "bitfinex" => crawlers::bitfinex::crawl_l2_snapshot,
        "bitget" => crawlers::bitget::crawl_l2_snapshot,
        "bithumb" => crawlers::bithumb::crawl_l2_snapshot,
        "bitmex" => crawlers::bitmex::crawl_l2_snapshot,
        "bitstamp" => crawlers::bitstamp::crawl_l2_snapshot,
        "bitz" => crawlers::bitz::crawl_l2_snapshot,
        "bybit" => crawlers::bybit::crawl_l2_snapshot,
        "coinbase_pro" => crawlers::coinbase_pro::crawl_l2_snapshot,
        "deribit" => crawlers::deribit::crawl_l2_snapshot,
        "ftx" => crawlers::ftx::crawl_l2_snapshot,
        "huobi" => crawlers::huobi::crawl_l2_snapshot,
        "kraken" => crawlers::kraken::crawl_l2_snapshot,
        "mxc" => crawlers::mxc::crawl_l2_snapshot,
        "okex" => crawlers::okex::crawl_l2_snapshot,
        "zbg" => crawlers::zbg::crawl_l2_snapshot,
        _ => panic!("Unknown exchange {}", exchange),
    };
    func(market_type, symbols, on_msg, interval, duration);
}

/// Crawl level3 orderbook update events.
pub fn crawl_l3_event(
    exchange: &str,
    market_type: MarketType,
    symbols: Option<&[String]>,
    on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
    duration: Option<u64>,
) {
    let func = match exchange {
        "binance" => panic!("Binance does NOT provide level3 orderbook data"),
        "bitfinex" => crawlers::bitfinex::crawl_l3_event,
        "bitstamp" => crawlers::bitstamp::crawl_l3_event,
        "coinbase_pro" => crawlers::coinbase_pro::crawl_l3_event,
        _ => panic!("Unknown exchange {}", exchange),
    };
    let handle = func(market_type, symbols, on_msg, duration);
    if let Some(h) = handle {
        h.join().expect("The thread panicked");
    }
}

/// Crawl level3 orderbook snapshots through RESTful APIs.
pub fn crawl_l3_snapshot(
    exchange: &str,
    market_type: MarketType,
    symbols: Option<&[String]>,
    on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
    interval: Option<u64>,
    duration: Option<u64>,
) {
    let func = match exchange {
        "binance" => panic!("Binance does NOT provide level3 orderbook data"),
        "bitfinex" => crawlers::bitfinex::crawl_l3_snapshot,
        "bitstamp" => crawlers::bitstamp::crawl_l3_snapshot,
        "coinbase_pro" => crawlers::coinbase_pro::crawl_l3_snapshot,
        _ => panic!("Unknown exchange {}", exchange),
    };
    func(market_type, symbols, on_msg, interval, duration);
}
