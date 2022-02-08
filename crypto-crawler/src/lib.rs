//! A rock-solid cryprocurrency crawler.
//!
//! ## Crawl realtime trades
//!
//! ```rust
//! use crypto_crawler::{crawl_trade, MarketType, Message};
//!
//! let (tx, rx) = std::sync::mpsc::channel();
//! std::thread::spawn(move || {
//!     for msg in rx {
//!         println!("{}", msg);
//!     }
//! });
//!
//! // Crawl realtime trades for all symbols of binance inverse_swap markets, only run for 5 seconds
//! crawl_trade("binance", MarketType::InverseSwap, None, tx, Some(5));
//! ```
//!
//! ## Crawl realtime level2 orderbook incremental updates
//!
//! ```rust
//! use crypto_crawler::{crawl_l2_event, MarketType, Message};
//!
//! let (tx, rx) = std::sync::mpsc::channel();
//! std::thread::spawn(move || {
//!     for msg in rx {
//!         println!("{}", msg);
//!     }
//! });
//!
//! // Crawl realtime level2 incremental updates for all symbols of binance inverse_swap markets, only run for 5 seconds
//! crawl_l2_event("binance", MarketType::InverseSwap, None, tx, Some(5));
//! ```
//!
//! ## Crawl level2 orderbook full snapshots from RESTful API
//!
//! ```rust
//! use crypto_crawler::{crawl_l2_snapshot, MarketType, Message};
//!
//! let (tx, rx) = std::sync::mpsc::channel();
//! std::thread::spawn(move || {
//!     for msg in rx {
//!         println!("{}", msg);
//!     }
//! });
//!
//! // Crawl level2 full snapshots for all symbols of binance inverse_swap markets, only run for 5 seconds
//! crawl_l2_snapshot("binance", MarketType::InverseSwap, None, tx, Some(5));
//! ```
//!
//! ## Crawl realtime level2 orderbook top-K snapshots
//!
//! ```rust
//! use crypto_crawler::{crawl_l2_topk, MarketType, Message};
//!
//! let (tx, rx) = std::sync::mpsc::channel();
//! std::thread::spawn(move || {
//!     for msg in rx {
//!         println!("{}", msg);
//!     }
//! });
//!
//! // Crawl realtime level2 top-k snapshots for all symbols of binance inverse_swap markets, only run for 5 seconds
//! crawl_l2_topk("binance", MarketType::InverseSwap, None, tx, Some(5));
//! ```
//!
//! ## Crawl realtime level3 orderbook incremental updates
//!
//! ```rust
//! use crypto_crawler::{crawl_l3_event, MarketType, Message};
//!
//! let (tx, rx) = std::sync::mpsc::channel();
//! std::thread::spawn(move || {
//!     for msg in rx {
//!         println!("{}", msg);
//!     }
//! });
//!
//! // Crawl realtime level3 updates for all symbols of CoinbasePro spot market, only run for 5 seconds
//! crawl_l3_event("coinbase_pro", MarketType::Spot, None, tx, Some(5));
//! ```
//!
//! ## Crawl level3 orderbook full snapshots from RESTful API
//!
//! ```rust
//! use crypto_crawler::{crawl_l3_snapshot, MarketType, Message};
//!
//! let (tx, rx) = std::sync::mpsc::channel();
//! std::thread::spawn(move || {
//!     for msg in rx {
//!         println!("{}", msg);
//!     }
//! });
//!
//! // Crawl level3 orderbook full snapshots for all symbols of CoinbasePro spot markets, only run for 5 seconds
//! crawl_l3_snapshot("coinbase_pro", MarketType::Spot, None, tx, Some(5));
//! ```
//!
//! ## Crawl realtime BBO
//!
//! ```rust
//! use crypto_crawler::{crawl_bbo, MarketType, Message};
//!
//! let (tx, rx) = std::sync::mpsc::channel();
//! std::thread::spawn(move || {
//!     for msg in rx {
//!         println!("{}", msg);
//!     }
//! });
//!
//! // Crawl realtime best bid and ask messages for all symbols of binance COIN-margined perpetual markets, only run for 5 seconds
//! crawl_bbo("binance", MarketType::InverseSwap, None, tx, Some(5));
//! ```
//!
//! ## Crawl 24hr rolling window tickers
//!
//! ```rust
//! use crypto_crawler::{crawl_ticker, MarketType, Message};
//!
//! let (tx, rx) = std::sync::mpsc::channel();
//! std::thread::spawn(move || {
//!     for msg in rx {
//!         println!("{}", msg);
//!     }
//! });
//!
//! // Crawl 24hr rolling window tickers for all symbols of binance COIN-margined perpetual markets, only run for 5 seconds
//! crawl_ticker("binance", MarketType::InverseSwap, None, tx, Some(5));
//! ```
//!
//! ## Crawl candlesticks(i.e., OHLCV)
//!
//! ```rust
//! use crypto_crawler::{crawl_candlestick, MarketType, Message};
//!
//! let (tx, rx) = std::sync::mpsc::channel();
//! std::thread::spawn(move || {
//!     for msg in rx {
//!         println!("{}", msg);
//!     }
//! });
//!
//! // Crawl candlesticks from 1 minute to 3 minutes for all symbols of binance COIN-margined perpetual markets, only run for 5 seconds
//! crawl_candlestick("binance", MarketType::InverseSwap, None, tx, Some(5));
//! ```
//!
//! ## Crawl funding rates
//!
//! ```rust
//! use crypto_crawler::{crawl_funding_rate, MarketType, Message};
//!
//! let (tx, rx) = std::sync::mpsc::channel();
//! std::thread::spawn(move || {
//!     for msg in rx {
//!         println!("{}", msg);
//!     }
//! });
//!
//! // Crawl funding rates for all symbols of binance COIN-margined perpetual markets, only run for 5 seconds
//! crawl_funding_rate("binance", MarketType::InverseSwap, None, tx, Some(5));
//! ```
mod crawlers;
mod msg;
mod utils;

use std::sync::mpsc::Sender;

pub use crawlers::fetch_symbols_retry;
use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;
pub use msg::*;
pub use utils::get_hot_spot_symbols;

/// Crawl realtime trades.
///
/// If `symbols` is None or empty, this API will crawl realtime trades for all symbols in the `market_type`
/// market, and launch a thread to discover new symbols every hour. And so forth for all other APIs.
pub fn crawl_trade(
    exchange: &str,
    market_type: MarketType,
    symbols: Option<&[String]>,
    tx: Sender<Message>,
    duration: Option<u64>,
) {
    match exchange {
        "binance" => crawlers::binance::crawl_trade(market_type, symbols, tx, duration),
        "bitmex" => crawlers::bitmex::crawl_trade(market_type, symbols, tx, duration),
        "deribit" => crawlers::deribit::crawl_trade(market_type, symbols, tx, duration),
        "okex" => crawlers::okex::crawl_trade(market_type, symbols, tx, duration),
        "bitfinex" | "bitget" | "bithumb" | "bitstamp" | "bitz" | "bybit" | "coinbase_pro"
        | "dydx" | "ftx" | "gate" | "huobi" | "kraken" | "kucoin" | "mxc" | "zbg" => {
            crawlers::crawl_event(
                exchange,
                MessageType::Trade,
                market_type,
                symbols,
                tx,
                duration,
            )
        }
        _ => panic!("{} does NOT have the trade websocket channel", exchange),
    }
}

/// Crawl level2 orderbook update events.
pub fn crawl_l2_event(
    exchange: &str,
    market_type: MarketType,
    symbols: Option<&[String]>,
    tx: Sender<Message>,
    duration: Option<u64>,
) {
    match exchange {
        "binance" => crawlers::binance::crawl_l2_event(market_type, symbols, tx, duration),
        "bitmex" => crawlers::bitmex::crawl_l2_event(market_type, symbols, tx, duration),
        "huobi" => crawlers::huobi::crawl_l2_event(market_type, symbols, tx, duration),
        "bitfinex" | "bitget" | "bithumb" | "bitstamp" | "bitz" | "bybit" | "coinbase_pro"
        | "deribit" | "dydx" | "ftx" | "gate" | "kraken" | "kraken_futures" | "kucoin" | "mxc"
        | "okex" | "zbg" => crawlers::crawl_event(
            exchange,
            MessageType::L2Event,
            market_type,
            symbols,
            tx,
            duration,
        ),
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
    tx: Sender<Message>,
    duration: Option<u64>,
) {
    match exchange {
        "bitfinex" | "bitstamp" | "coinbase_pro" | "kucoin" => crawlers::crawl_event(
            exchange,
            MessageType::L3Event,
            market_type,
            symbols,
            tx,
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
    tx: Sender<Message>,
    duration: Option<u64>,
) {
    crawlers::crawl_snapshot(
        exchange,
        market_type,
        MessageType::L2Snapshot,
        symbols,
        tx,
        duration,
    )
}

/// Crawl best bid and ask.
pub fn crawl_bbo(
    exchange: &str,
    market_type: MarketType,
    symbols: Option<&[String]>,
    tx: Sender<Message>,
    duration: Option<u64>,
) {
    match exchange {
        "binance" => crawlers::binance::crawl_bbo(market_type, symbols, tx, duration),
        "bitmex" => crawlers::bitmex::crawl_bbo(market_type, symbols, tx, duration),
        "kucoin" => crawlers::kucoin::crawl_bbo(market_type, symbols, tx, duration),
        "bitfinex" | "deribit" | "ftx" | "gate" | "huobi" | "kraken" | "okex" => {
            crawlers::crawl_event(
                exchange,
                MessageType::BBO,
                market_type,
                symbols,
                tx,
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
    tx: Sender<Message>,
    duration: Option<u64>,
) {
    match exchange {
        "binance" => crawlers::binance::crawl_l2_topk(market_type, symbols, tx, duration),
        "bitmex" => crawlers::bitmex::crawl_l2_topk(market_type, symbols, tx, duration),
        "bitget" | "bybit" | "bitstamp" | "deribit" | "huobi" | "kucoin" | "mxc" | "okex" => {
            crawlers::crawl_event(
                exchange,
                MessageType::L2TopK,
                market_type,
                symbols,
                tx,
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
    tx: Sender<Message>,
    duration: Option<u64>,
) {
    crawlers::crawl_snapshot(
        exchange,
        market_type,
        MessageType::L3Snapshot,
        symbols,
        tx,
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
    tx: Sender<Message>,
    duration: Option<u64>,
) {
    match exchange {
        "binance" => crawlers::binance::crawl_ticker(market_type, symbols, tx, duration),
        "bitfinex" | "bitget" | "bithumb" | "bitz" | "bybit" | "coinbase_pro" | "deribit"
        | "gate" | "huobi" | "kraken" | "kucoin" | "mxc" | "okex" | "zbg" => crawlers::crawl_event(
            exchange,
            MessageType::Ticker,
            market_type,
            symbols,
            tx,
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
    tx: Sender<Message>,
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
    func(market_type, symbols, tx, duration);
}

/// Crawl candlestick(i.e., OHLCV) data.
///
/// If `symbol_interval_list` is None or empty, this API will crawl candlesticks from
/// 10 seconds to 3 minutes(if available) for all symbols.
pub fn crawl_candlestick(
    exchange: &str,
    market_type: MarketType,
    symbol_interval_list: Option<&[(String, usize)]>,
    tx: Sender<Message>,
    duration: Option<u64>,
) {
    match exchange {
        "binance" => {
            crawlers::binance::crawl_candlestick(market_type, symbol_interval_list, tx, duration)
        }
        "bitmex" => {
            crawlers::bitmex::crawl_candlestick(market_type, symbol_interval_list, tx, duration)
        }
        "bitfinex" | "bitget" | "bitz" | "bybit" | "deribit" | "gate" | "huobi" | "kraken"
        | "kucoin" | "mxc" | "okex" | "zbg" => crawlers::crawl_candlestick_ext(
            exchange,
            market_type,
            symbol_interval_list,
            tx,
            duration,
        ),
        _ => panic!(
            "{} does NOT have the candlestick websocket channel",
            exchange
        ),
    };
}

/// Crawl all open interest.
pub fn crawl_open_interest(
    exchange: &str,
    market_type: MarketType,
    tx: Sender<Message>,
    duration: Option<u64>,
) {
    crawlers::crawl_open_interest(exchange, market_type, tx, duration)
}

/// Subscribe to multiple message types of one symbol.
///
/// This API is suitable for client applications such as APP, website, etc.
///
/// String messages in `tx` are already parsed by `crypto-msg-parser`.
pub fn subscribe_symbol(
    exchange: &str,
    market_type: MarketType,
    symbol: &str,
    msg_types: &[MessageType],
    tx: Sender<String>,
    duration: Option<u64>,
) {
    let ws_client = crawlers::create_ws_client_symbol(exchange, market_type, tx);
    let symbols = vec![symbol.to_string()];
    let commands = crypto_msg_type::get_ws_commands(exchange, msg_types, &symbols, true, None);
    ws_client.subscribe(&commands);
    ws_client.run(duration);
}
