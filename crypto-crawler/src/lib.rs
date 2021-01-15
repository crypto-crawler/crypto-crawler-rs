mod crawlers;
mod msg;

use crypto_markets::MarketType;
use std::sync::{Arc, Mutex};

pub use msg::*;

/// Crawl trades.
///
/// If `symbols` is None, this function will crawl all trading symbols of in the `market_type`,
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
        "bitmex" => crawlers::bitmex::crawl_trade,
        "bitstamp" => crawlers::bitstamp::crawl_trade,
        "coinbase_pro" => crawlers::coinbase_pro::crawl_trade,
        "huobi" => crawlers::huobi::crawl_trade,
        "kraken" => crawlers::kraken::crawl_trade,
        "mxc" => crawlers::mxc::crawl_trade,
        "okex" => crawlers::okex::crawl_trade,
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
        "bitmex" => crawlers::bitmex::crawl_l2_event,
        "bitstamp" => crawlers::bitstamp::crawl_l2_event,
        "coinbase_pro" => crawlers::coinbase_pro::crawl_l2_event,
        "huobi" => crawlers::huobi::crawl_l2_event,
        "kraken" => crawlers::kraken::crawl_l2_event,
        "mxc" => crawlers::mxc::crawl_l2_event,
        "okex" => crawlers::okex::crawl_l2_event,
        _ => panic!("Unknown exchange {}", exchange),
    };
    let handle = func(market_type, symbols, on_msg, duration);
    if let Some(h) = handle {
        h.join().expect("The thread panicked");
    }
}

/// Crawl level2 orderbook snapshot through REST API.
pub fn crawl_l2_snapshot(
    exchange: &str,
    market_type: MarketType,
    symbols: Option<&[String]>,
    on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
) {
    let func = match exchange {
        "binance" => crawlers::binance::crawl_l2_snapshot,
        "bitfinex" => crawlers::bitfinex::crawl_l2_snapshot,
        "bitmex" => crawlers::bitmex::crawl_l2_snapshot,
        "bitstamp" => crawlers::bitstamp::crawl_l2_snapshot,
        "coinbase_pro" => crawlers::coinbase_pro::crawl_l2_snapshot,
        "huobi" => crawlers::huobi::crawl_l2_snapshot,
        "kraken" => crawlers::kraken::crawl_l2_snapshot,
        "mxc" => crawlers::mxc::crawl_l2_snapshot,
        "okex" => crawlers::okex::crawl_l2_snapshot,
        _ => panic!("Unknown exchange {}", exchange),
    };
    func(market_type, symbols, on_msg);
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

/// Crawl level3 orderbook snapshot through REST API.
pub fn crawl_l3_snapshot(
    exchange: &str,
    market_type: MarketType,
    symbols: Option<&[String]>,
    on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
) {
    let func = match exchange {
        "binance" => panic!("Binance does NOT provide level3 orderbook data"),
        "bitfinex" => crawlers::bitfinex::crawl_l3_snapshot,
        "bitstamp" => crawlers::bitstamp::crawl_l3_snapshot,
        "coinbase_pro" => crawlers::coinbase_pro::crawl_l3_snapshot,
        _ => panic!("Unknown exchange {}", exchange),
    };
    func(market_type, symbols, on_msg);
}
