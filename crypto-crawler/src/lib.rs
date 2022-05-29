//! A rock-solid cryprocurrency crawler.
//!
//! ## Crawl realtime trades
//!
//! ```rust
//! use crypto_crawler::{crawl_trade, MarketType};
//!
//! #[tokio::main(flavor = "multi_thread")]
//! async fn main() {
//!     let (tx, rx) = std::sync::mpsc::channel();
//!     tokio::task::spawn(async move {
//!         // Crawl realtime trades for all symbols of binance inverse_swap markets
//!         crawl_trade("binance", MarketType::InverseSwap, None, tx).await;
//!     });
//!
//!     let mut messages = Vec::new();
//!     for msg in rx {
//!         messages.push(msg);
//!         break;
//!     }
//!     assert!(!messages.is_empty());
//! }
//! ```
//!
//! ## Crawl realtime level2 orderbook incremental updates
//!
//! ```rust
//! use crypto_crawler::{crawl_l2_event, MarketType};
//!
//! #[tokio::main(flavor = "multi_thread")]
//! async fn main() {
//!     let (tx, rx) = std::sync::mpsc::channel();
//!     tokio::task::spawn(async move {
//!         // Crawl realtime level2 incremental updates for all symbols of binance inverse_swap markets
//!         crawl_l2_event("binance", MarketType::InverseSwap, None, tx).await;
//!     });
//!
//!     let mut messages = Vec::new();
//!     for msg in rx {
//!         messages.push(msg);
//!         break;
//!     }
//!     assert!(!messages.is_empty());
//! }
//! ```
//!
//! ## Crawl level2 orderbook full snapshots from RESTful API
//!
//! ```rust
//! use crypto_crawler::{crawl_l2_snapshot, MarketType};
//!
//! let (tx, rx) = std::sync::mpsc::channel();
//! std::thread::spawn(move || {
//!     // Crawl level2 full snapshots for all symbols of binance inverse_swap markets
//!     crawl_l2_snapshot("binance", MarketType::InverseSwap, None, tx);
//! });
//!
//! let mut messages = Vec::new();
//! for msg in rx {
//!     messages.push(msg);
//!     break;
//! }
//! assert!(!messages.is_empty());
//! ```
//!
//! ## Crawl realtime level2 orderbook top-K snapshots
//!
//! ```rust
//! use crypto_crawler::{crawl_l2_topk, MarketType};
//!
//! #[tokio::main(flavor = "multi_thread")]
//! async fn main() {
//!     let (tx, rx) = std::sync::mpsc::channel();
//!     tokio::task::spawn(async move {
//!         // Crawl realtime level2 top-k snapshots for all symbols of binance inverse_swap markets
//!         crawl_l2_topk("binance", MarketType::InverseSwap, None, tx).await;
//!     });
//!
//!     let mut messages = Vec::new();
//!     for msg in rx {
//!         messages.push(msg);
//!         break;
//!     }
//!     assert!(!messages.is_empty());
//! }
//! ```
//!
//! ## Crawl realtime level3 orderbook incremental updates
//!
//! ```rust
//! use crypto_crawler::{crawl_l3_event, MarketType};
//!
//! #[tokio::main(flavor = "multi_thread")]
//! async fn main() {
//!     let (tx, rx) = std::sync::mpsc::channel();
//!     tokio::task::spawn(async move {
//!         // Crawl realtime level3 updates for all symbols of CoinbasePro spot market
//!         crawl_l3_event("coinbase_pro", MarketType::Spot, None, tx).await;
//!     });
//!
//!     let mut messages = Vec::new();
//!     for msg in rx {
//!         messages.push(msg);
//!         break;
//!     }
//!     assert!(!messages.is_empty());
//! }
//! ```
//!
//! ## Crawl level3 orderbook full snapshots from RESTful API
//!
//! ```rust
//! use crypto_crawler::{crawl_l3_snapshot, MarketType};
//!
//! let (tx, rx) = std::sync::mpsc::channel();
//! std::thread::spawn(move || {
//!     // Crawl level3 orderbook full snapshots for all symbols of CoinbasePro spot markets
//!     crawl_l3_snapshot("coinbase_pro", MarketType::Spot, None, tx);
//! });
//!
//! let mut messages = Vec::new();
//! for msg in rx {
//!     messages.push(msg);
//!     break;
//! }
//! assert!(!messages.is_empty());
//! ```
//!
//! ## Crawl realtime BBO
//!
//! ```rust
//! use crypto_crawler::{crawl_bbo, MarketType};
//!
//! #[tokio::main(flavor = "multi_thread")]
//! async fn main() {
//!     let (tx, rx) = std::sync::mpsc::channel();
//!     tokio::task::spawn(async move {
//!         // Crawl realtime best bid and ask messages for all symbols of binance COIN-margined perpetual markets
//!         crawl_bbo("binance", MarketType::InverseSwap, None, tx).await;
//!     });
//!
//!     let mut messages = Vec::new();
//!     for msg in rx {
//!         messages.push(msg);
//!         break;
//!     }
//!     assert!(!messages.is_empty());
//! }
//! ```
//!
//! ## Crawl 24hr rolling window tickers
//!
//! ```rust
//! use crypto_crawler::{crawl_ticker, MarketType};
//!
//! #[tokio::main(flavor = "multi_thread")]
//! async fn main() {
//!     let (tx, rx) = std::sync::mpsc::channel();
//!     tokio::task::spawn(async move {
//!         // Crawl 24hr rolling window tickers for all symbols of binance COIN-margined perpetual markets
//!         crawl_ticker("binance", MarketType::InverseSwap, None, tx).await;
//!     });
//!
//!     let mut messages = Vec::new();
//!     for msg in rx {
//!         messages.push(msg);
//!         break;
//!     }
//!     assert!(!messages.is_empty());
//! }
//! ```
//!
//! ## Crawl candlesticks(i.e., OHLCV)
//!
//! ```rust
//! use crypto_crawler::{crawl_candlestick, MarketType};
//!
//! #[tokio::main(flavor = "multi_thread")]
//! async fn main() {
//!     let (tx, rx) = std::sync::mpsc::channel();
//!     tokio::task::spawn(async move {
//!         // Crawl candlesticks from 1 minute to 3 minutes for all symbols of binance COIN-margined perpetual markets
//!         crawl_candlestick("binance", MarketType::InverseSwap, None, tx).await;
//!     });
//!
//!     let mut messages = Vec::new();
//!     for msg in rx {
//!         messages.push(msg);
//!         break;
//!     }
//!     assert!(!messages.is_empty());
//! }
//! ```
//!
//! ## Crawl funding rates
//!
//! ```rust
//! use crypto_crawler::{crawl_funding_rate, MarketType};
//!
//! #[tokio::main(flavor = "multi_thread")]
//! async fn main() {
//!     let (tx, rx) = std::sync::mpsc::channel();
//!     tokio::task::spawn(async move {
//!         // Crawl funding rates for all symbols of binance COIN-margined perpetual markets
//!         crawl_funding_rate("binance", MarketType::InverseSwap, None, tx).await;
//!     });
//!
//!     let mut messages = Vec::new();
//!     for msg in rx {
//!         messages.push(msg);
//!         break;
//!     }
//!     assert!(!messages.is_empty());
//! }
//! ```
mod crawlers;
mod msg;
mod utils;

use std::sync::mpsc::Sender;

pub use crawlers::fetch_symbols_retry;
pub use crypto_market_type::MarketType;
pub use crypto_msg_type::MessageType;
pub use msg::*;
pub use utils::get_hot_spot_symbols;

/// Crawl realtime trades.
///
/// If `symbols` is None or empty, this API will crawl realtime trades for all symbols in the `market_type`
/// market, and launch a thread to discover new symbols every hour. And so forth for all other APIs.
pub async fn crawl_trade(
    exchange: &str,
    market_type: MarketType,
    symbols: Option<&[String]>,
    tx: Sender<Message>,
) {
    match exchange {
        "binance" => crawlers::binance::crawl_trade(market_type, symbols, tx).await,
        "bitmex" => crawlers::bitmex::crawl_trade(market_type, symbols, tx).await,
        "deribit" => crawlers::deribit::crawl_trade(market_type, symbols, tx).await,
        "bitfinex" | "bitget" | "bithumb" | "bitstamp" | "bitz" | "bybit" | "coinbase_pro"
        | "dydx" | "ftx" | "gate" | "huobi" | "kraken" | "kucoin" | "mexc" | "okx" | "zb"
        | "zbg" => {
            crawlers::crawl_event(exchange, MessageType::Trade, market_type, symbols, tx).await
        }
        _ => panic!("{} does NOT have the trade websocket channel", exchange),
    }
}

/// Crawl level2 orderbook update events.
pub async fn crawl_l2_event(
    exchange: &str,
    market_type: MarketType,
    symbols: Option<&[String]>,
    tx: Sender<Message>,
) {
    match exchange {
        "bitmex" => crawlers::bitmex::crawl_l2_event(market_type, symbols, tx).await,
        "huobi" => crawlers::huobi::crawl_l2_event(market_type, symbols, tx).await,
        "binance" | "bitfinex" | "bitget" | "bithumb" | "bitstamp" | "bitz" | "bybit"
        | "coinbase_pro" | "deribit" | "dydx" | "ftx" | "gate" | "kraken" | "kucoin" | "mexc"
        | "okx" | "zb" | "zbg" => {
            crawlers::crawl_event(exchange, MessageType::L2Event, market_type, symbols, tx).await
        }
        _ => panic!(
            "{} does NOT have the incremental level2 websocket channel",
            exchange
        ),
    }
}

/// Crawl level3 orderbook update events.
pub async fn crawl_l3_event(
    exchange: &str,
    market_type: MarketType,
    symbols: Option<&[String]>,
    tx: Sender<Message>,
) {
    match exchange {
        "bitfinex" | "bitstamp" | "coinbase_pro" | "kucoin" => {
            crawlers::crawl_event(exchange, MessageType::L3Event, market_type, symbols, tx).await
        }
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
    tx: Sender<Message>,
) {
    crawlers::crawl_snapshot(exchange, market_type, MessageType::L2Snapshot, symbols, tx);
}

/// Crawl best bid and ask.
pub async fn crawl_bbo(
    exchange: &str,
    market_type: MarketType,
    symbols: Option<&[String]>,
    tx: Sender<Message>,
) {
    match exchange {
        "binance" => crawlers::binance::crawl_bbo(market_type, symbols, tx).await,
        "bitmex" => crawlers::bitmex::crawl_bbo(market_type, symbols, tx).await,
        "kucoin" => crawlers::kucoin::crawl_bbo(market_type, symbols, tx).await,
        "bitfinex" | "deribit" | "ftx" | "gate" | "huobi" | "kraken" | "okx" => {
            crawlers::crawl_event(exchange, MessageType::BBO, market_type, symbols, tx).await
        }
        _ => panic!("{} does NOT have BBO websocket channel", exchange),
    }
}

/// Crawl level2 orderbook top-k snapshots through websocket.
pub async fn crawl_l2_topk(
    exchange: &str,
    market_type: MarketType,
    symbols: Option<&[String]>,
    tx: Sender<Message>,
) {
    match exchange {
        "bitmex" => crawlers::bitmex::crawl_l2_topk(market_type, symbols, tx).await,
        "binance" | "bitget" | "bybit" | "bitstamp" | "deribit" | "huobi" | "kucoin" | "mexc"
        | "okx" | "zb" => {
            crawlers::crawl_event(exchange, MessageType::L2TopK, market_type, symbols, tx).await
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
    tx: Sender<Message>,
) {
    crawlers::crawl_snapshot(exchange, market_type, MessageType::L3Snapshot, symbols, tx)
}

/// Crawl 24hr rolling window ticker.
///
/// If `symbols` is None, it means all trading symbols in the `market_type`,
/// and updates the latest symbols every hour.
pub async fn crawl_ticker(
    exchange: &str,
    market_type: MarketType,
    symbols: Option<&[String]>,
    tx: Sender<Message>,
) {
    match exchange {
        "binance" => crawlers::binance::crawl_ticker(market_type, symbols, tx).await,
        "bitfinex" | "bitget" | "bithumb" | "bitz" | "bybit" | "coinbase_pro" | "deribit"
        | "gate" | "huobi" | "kraken" | "kucoin" | "mexc" | "okx" | "zbg" => {
            crawlers::crawl_event(exchange, MessageType::Ticker, market_type, symbols, tx).await
        }
        "zb" => crawlers::zb::crawl_ticker(market_type, symbols, tx).await,
        _ => panic!("{} does NOT have the ticker websocket channel", exchange),
    }
}

/// Crawl perpetual swap funding rates.
pub async fn crawl_funding_rate(
    exchange: &str,
    market_type: MarketType,
    symbols: Option<&[String]>,
    tx: Sender<Message>,
) {
    match exchange {
        "binance" => crawlers::binance::crawl_funding_rate(market_type, symbols, tx).await,
        "bitmex" => crawlers::bitmex::crawl_funding_rate(market_type, symbols, tx).await,
        "huobi" => crawlers::huobi::crawl_funding_rate(market_type, symbols, tx).await,
        "okx" => crawlers::okx::crawl_funding_rate(market_type, symbols, tx).await,
        _ => panic!("{} does NOT have perpetual swap market", exchange),
    }
}

/// Crawl candlestick(i.e., OHLCV) data.
///
/// If `symbol_interval_list` is None or empty, this API will crawl candlesticks from
/// 10 seconds to 3 minutes(if available) for all symbols.
pub async fn crawl_candlestick(
    exchange: &str,
    market_type: MarketType,
    symbol_interval_list: Option<&[(String, usize)]>,
    tx: Sender<Message>,
) {
    match exchange {
        "bitmex" => {
            crawlers::bitmex::crawl_candlestick(market_type, symbol_interval_list, tx).await
        }
        "binance" | "bitfinex" | "bitget" | "bitz" | "bybit" | "deribit" | "gate" | "huobi"
        | "kraken" | "kucoin" | "mexc" | "okx" | "zb" | "zbg" => {
            crawlers::crawl_candlestick_ext(exchange, market_type, symbol_interval_list, tx).await
        }
        _ => panic!(
            "{} does NOT have the candlestick websocket channel",
            exchange
        ),
    };
}

/// Crawl all open interest.
pub fn crawl_open_interest(exchange: &str, market_type: MarketType, tx: Sender<Message>) {
    crawlers::crawl_open_interest(exchange, market_type, tx);
}

/// Subscribe to multiple message types of one symbol.
///
/// This API is suitable for client applications such as APP, website, etc.
///
/// String messages in `tx` are already parsed by `crypto-msg-parser`.
pub async fn subscribe_symbol(
    exchange: &str,
    market_type: MarketType,
    symbol: &str,
    msg_types: &[MessageType],
    tx: Sender<String>,
) {
    let ws_client = crawlers::create_ws_client_symbol(exchange, market_type, tx).await;
    let symbols = vec![symbol.to_string()];
    let commands = crypto_msg_type::get_ws_commands(exchange, msg_types, &symbols, true, None);
    ws_client.send(&commands).await;
    ws_client.run().await;
    ws_client.close();
}
