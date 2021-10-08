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
//! crawl_l2_snapshot("bitmex", MarketType::InverseSwap, None, on_msg, Some(5));
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
//! crawl_l3_snapshot("coinbase_pro", MarketType::Spot, None, on_msg, Some(5));
//! ```
mod crawlers;
mod msg;
mod utils;

use std::sync::{Arc, Mutex};

pub use crawlers::fetch_symbols_retry;
pub use crypto_market_type::MarketType;
pub use msg::*;
pub use utils::get_hot_spot_symbols;

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
    match exchange {
        "binance" => crawlers::binance::crawl_trade(market_type, symbols, on_msg, duration),
        "bitmex" => crawlers::bitmex::crawl_trade(market_type, symbols, on_msg, duration),
        "deribit" => crawlers::deribit::crawl_trade(market_type, symbols, on_msg, duration),
        "okex" => crawlers::okex::crawl_trade(market_type, symbols, on_msg, duration),
        "bitfinex" | "bitget" | "bithumb" | "bitstamp" | "bitz" | "bybit" | "coinbase_pro"
        | "ftx" | "gate" | "huobi" | "kraken" | "kucoin" | "mxc" | "zbg" => crawlers::crawl_event(
            exchange,
            MessageType::Trade,
            market_type,
            symbols,
            on_msg,
            duration,
        ),
        _ => panic!("{} does NOT have the trade websocket channel", exchange),
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
    match exchange {
        "binance" => crawlers::binance::crawl_l2_event(market_type, symbols, on_msg, duration),
        "bitmex" => crawlers::bitmex::crawl_l2_event(market_type, symbols, on_msg, duration),
        "huobi" => crawlers::huobi::crawl_l2_event(market_type, symbols, on_msg, duration),
        "bitfinex" | "bitget" | "bithumb" | "bitstamp" | "bitz" | "bybit" | "coinbase_pro"
        | "deribit" | "ftx" | "gate" | "kraken" | "kucoin" | "mxc" | "okex" | "zbg" => {
            crawlers::crawl_event(
                exchange,
                MessageType::L2Event,
                market_type,
                symbols,
                on_msg,
                duration,
            )
        }
        _ => panic!(
            "{} does NOT have the incremental level2 websocket channel",
            exchange
        ),
    }
}

/// Crawl level3 orderbook update events.
pub fn crawl_l3_event(
    exchange: &str,
    market_type: MarketType,
    symbols: Option<&[String]>,
    on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
    duration: Option<u64>,
) {
    match exchange {
        "bitfinex" | "bitstamp" | "coinbase_pro" | "kucoin" => crawlers::crawl_event(
            exchange,
            MessageType::L3Event,
            market_type,
            symbols,
            on_msg,
            duration,
        ),
        _ => panic!(
            "{} does NOT have the incremental level3 websocket channel",
            exchange
        ),
    }
}

/// Crawl level2 orderbook snapshots through RESTful APIs.
pub fn crawl_l2_snapshot(
    exchange: &str,
    market_type: MarketType,
    symbols: Option<&[String]>,
    on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
    duration: Option<u64>,
) {
    crawlers::crawl_snapshot(
        exchange,
        market_type,
        MessageType::L2Snapshot,
        symbols,
        on_msg,
        duration,
    )
}

/// Crawl best bid and ask.
pub fn crawl_bbo(
    exchange: &str,
    market_type: MarketType,
    symbols: Option<&[String]>,
    on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
    duration: Option<u64>,
) {
    match exchange {
        "binance" => crawlers::binance::crawl_bbo(market_type, symbols, on_msg, duration),
        "bitmex" => crawlers::bitmex::crawl_bbo(market_type, symbols, on_msg, duration),
        "kucoin" => crawlers::kucoin::crawl_bbo(market_type, symbols, on_msg, duration),
        "bitfinex" | "deribit" | "ftx" | "gate" | "huobi" | "kraken" | "okex" => {
            crawlers::crawl_event(
                exchange,
                MessageType::BBO,
                market_type,
                symbols,
                on_msg,
                duration,
            )
        }
        _ => panic!("{} does NOT have BBO websocket channel", exchange),
    }
}

/// Crawl level2 orderbook top-k snapshots through websocket.
pub fn crawl_l2_topk(
    exchange: &str,
    market_type: MarketType,
    symbols: Option<&[String]>,
    on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
    duration: Option<u64>,
) {
    match exchange {
        "binance" => crawlers::binance::crawl_l2_topk(market_type, symbols, on_msg, duration),
        "bitmex" => crawlers::bitmex::crawl_l2_topk(market_type, symbols, on_msg, duration),
        "bitget" | "bybit" | "bitstamp" | "deribit" | "huobi" | "kucoin" | "mxc" | "okex" => {
            crawlers::crawl_event(
                exchange,
                MessageType::L2TopK,
                market_type,
                symbols,
                on_msg,
                duration,
            )
        }
        _ => panic!(
            "{} does NOT have the level2 top-k snapshot websocket channel",
            exchange
        ),
    }
}

/// Crawl level3 orderbook snapshots through RESTful APIs.
pub fn crawl_l3_snapshot(
    exchange: &str,
    market_type: MarketType,
    symbols: Option<&[String]>,
    on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
    duration: Option<u64>,
) {
    crawlers::crawl_snapshot(
        exchange,
        market_type,
        MessageType::L3Snapshot,
        symbols,
        on_msg,
        duration,
    )
}

/// Crawl 24hr rolling window ticker.
///
/// If `symbols` is None, it means all trading symbols in the `market_type`,
/// and updates the latest symbols every hour.
pub fn crawl_ticker(
    exchange: &str,
    market_type: MarketType,
    symbols: Option<&[String]>,
    on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
    duration: Option<u64>,
) {
    match exchange {
        "binance" => crawlers::binance::crawl_ticker(market_type, symbols, on_msg, duration),
        "bitfinex" | "bitget" | "bithumb" | "bitz" | "bybit" | "coinbase_pro" | "deribit"
        | "gate" | "huobi" | "kraken" | "kucoin" | "mxc" | "okex" | "zbg" => crawlers::crawl_event(
            exchange,
            MessageType::Ticker,
            market_type,
            symbols,
            on_msg,
            duration,
        ),
        _ => panic!("{} does NOT have the ticker websocket channel", exchange),
    }
}

/// Crawl perpetual swap funding rates.
pub fn crawl_funding_rate(
    exchange: &str,
    market_type: MarketType,
    symbols: Option<&[String]>,
    on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
    duration: Option<u64>,
) {
    let func = match exchange {
        "binance" => crawlers::binance::crawl_funding_rate,
        "bitget" => crawlers::bitget::crawl_funding_rate,
        "bitmex" => crawlers::bitmex::crawl_funding_rate,
        "huobi" => crawlers::huobi::crawl_funding_rate,
        "okex" => crawlers::okex::crawl_funding_rate,
        _ => panic!("{} does NOT have perpetual swap market", exchange),
    };
    func(market_type, symbols, on_msg, duration);
}

/// Crawl candlestick(i.e., OHLCV) data.
pub fn crawl_candlestick(
    exchange: &str,
    market_type: MarketType,
    symbol_interval_list: Option<&[(String, usize)]>,
    on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
    duration: Option<u64>,
) {
    match exchange {
        "binance" => crawlers::binance::crawl_candlestick(
            market_type,
            symbol_interval_list,
            on_msg,
            duration,
        ),
        "bitmex" => {
            crawlers::bitmex::crawl_candlestick(market_type, symbol_interval_list, on_msg, duration)
        }
        "bitfinex" | "bitget" | "bitz" | "bybit" | "deribit" | "gate" | "huobi" | "kraken"
        | "kucoin" | "mxc" | "okex" | "zbg" => crawlers::crawl_candlestick_ext(
            exchange,
            market_type,
            symbol_interval_list,
            on_msg,
            duration,
        ),
        _ => panic!(
            "{} does NOT have the candlestick websocket channel",
            exchange
        ),
    };
}
