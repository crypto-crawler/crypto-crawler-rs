#![allow(clippy::unnecessary_wraps)]
//! Get all trading pairs of a cryptocurrency exchange.
//!
//! ## Example
//!
//! ```
//! use crypto_markets::fetch_markets;
//! use crypto_market_type::MarketType;
//!
//! let markets = fetch_markets("binance", MarketType::Spot).unwrap();
//! println!("{}", serde_json::to_string_pretty(&markets).unwrap())
//! ```

mod error;
mod exchanges;
mod market;

use crypto_market_type::MarketType;
pub use error::Error;
pub use market::{Fees, Market, Precision, QuantityLimit};

use error::Result;

/// Fetch trading symbols.
pub fn fetch_symbols(exchange: &str, market_type: MarketType) -> Result<Vec<String>> {
    match exchange {
        "binance" => exchanges::binance::fetch_symbols(market_type),
        "bitfinex" => exchanges::bitfinex::fetch_symbols(market_type),
        "bitget" => exchanges::bitget::fetch_symbols(market_type),
        "bithumb" => exchanges::bithumb::fetch_symbols(market_type),
        "bitmex" => exchanges::bitmex::fetch_symbols(market_type),
        "bitstamp" => exchanges::bitstamp::fetch_symbols(market_type),
        "bitz" => exchanges::bitz::fetch_symbols(market_type),
        "bybit" => exchanges::bybit::fetch_symbols(market_type),
        "coinbase_pro" => exchanges::coinbase_pro::fetch_symbols(market_type),
        "deribit" => exchanges::deribit::fetch_symbols(market_type),
        "dydx" => exchanges::dydx::fetch_symbols(market_type),
        "ftx" => exchanges::ftx::fetch_symbols(market_type),
        "gate" => exchanges::gate::fetch_symbols(market_type),
        "huobi" => exchanges::huobi::fetch_symbols(market_type),
        "kraken" => exchanges::kraken::fetch_symbols(market_type),
        "kraken_futures" => exchanges::kraken_futures::fetch_symbols(market_type),
        "kucoin" => exchanges::kucoin::fetch_symbols(market_type),
        "mxc" => exchanges::mxc::fetch_symbols(market_type),
        "okex" => exchanges::okex::fetch_symbols(market_type),
        "zbg" => exchanges::zbg::fetch_symbols(market_type),
        _ => panic!("Unsupported exchange {}", exchange),
    }
}

/// Fetch trading markets of a cryptocurrency exchange.
///
/// # Arguments
///
/// * `exchange` - The exchange name
/// * `market_type` - The market type
///
/// # Example
///
/// ```
/// use crypto_markets::fetch_markets;
/// use crypto_market_type::MarketType;
/// let markets = fetch_markets("binance", MarketType::Spot).unwrap();
/// assert!(!markets.is_empty());
/// println!("{}", serde_json::to_string_pretty(&markets).unwrap())
/// ```
pub fn fetch_markets(exchange: &str, market_type: MarketType) -> Result<Vec<Market>> {
    match exchange {
        "binance" => exchanges::binance::fetch_markets(market_type),
        "bitfinex" => exchanges::bitfinex::fetch_markets(market_type),
        "bitget" => exchanges::bitget::fetch_markets(market_type),
        "bithumb" => exchanges::bithumb::fetch_markets(market_type),
        "bitmex" => exchanges::bitmex::fetch_markets(market_type),
        "bitstamp" => exchanges::bitstamp::fetch_markets(market_type),
        "bitz" => exchanges::bitz::fetch_markets(market_type),
        "bybit" => exchanges::bybit::fetch_markets(market_type),
        "coinbase_pro" => exchanges::coinbase_pro::fetch_markets(market_type),
        "deribit" => exchanges::deribit::fetch_markets(market_type),
        "dydx" => exchanges::dydx::fetch_markets(market_type),
        "ftx" => exchanges::ftx::fetch_markets(market_type),
        "gate" => exchanges::gate::fetch_markets(market_type),
        "huobi" => exchanges::huobi::fetch_markets(market_type),
        "kraken" => exchanges::kraken::fetch_markets(market_type),
        "kraken_futures" => exchanges::kraken_futures::fetch_markets(market_type),
        "kucoin" => exchanges::kucoin::fetch_markets(market_type),
        "mxc" => exchanges::mxc::fetch_markets(market_type),
        "okex" => exchanges::okex::fetch_markets(market_type),
        "zbg" => exchanges::zbg::fetch_markets(market_type),
        _ => panic!("Unsupported exchange {}", exchange),
    }
}
