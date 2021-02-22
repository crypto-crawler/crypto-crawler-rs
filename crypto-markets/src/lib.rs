//! Get all trading pairs of a cryptocurrency exchange.
//!
//! ## Example
//!
//! ```
//! use crypto_markets::{fetch_markets, MarketType};
//!
//! let markets = fetch_markets("binance", MarketType::Spot).unwrap();
//! println!("{}", serde_json::to_string_pretty(&markets).unwrap())
//! ```

mod error;
mod exchanges;
mod market;
mod market_type;
mod utils;

pub use error::Error;
pub use market::Market;
pub use market_type::{get_market_types, MarketType};

use error::Result;

/// Fetch trading symbols.
pub fn fetch_symbols(exchange: &str, market_type: MarketType) -> Result<Vec<String>> {
    match exchange {
        "binance" => exchanges::binance::fetch_symbols(market_type),
        "bitfinex" => exchanges::bitfinex::fetch_symbols(market_type),
        "bitget" => exchanges::bitget::fetch_symbols(market_type),
        "bitmex" => exchanges::bitmex::fetch_symbols(market_type),
        "bitstamp" => exchanges::bitstamp::fetch_symbols(market_type),
        "bitz" => exchanges::bitz::fetch_symbols(market_type),
        "bybit" => exchanges::bybit::fetch_symbols(market_type),
        "coinbase_pro" => exchanges::coinbase_pro::fetch_symbols(market_type),
        "deribit" => exchanges::deribit::fetch_symbols(market_type),
        "ftx" => exchanges::ftx::fetch_symbols(market_type),
        "huobi" => exchanges::huobi::fetch_symbols(market_type),
        "kraken" => exchanges::kraken::fetch_symbols(market_type),
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
/// use crypto_markets::{fetch_markets, MarketType};
/// let markets = fetch_markets("binance", MarketType::Spot).unwrap();
/// assert!(!markets.is_empty());
/// println!("{}", serde_json::to_string_pretty(&markets).unwrap())
/// ```
pub fn fetch_markets(exchange: &str, market_type: MarketType) -> Result<Vec<Market>> {
    match exchange {
        "binance" => exchanges::binance::fetch_markets(market_type),
        "bitfinex" => exchanges::bitfinex::fetch_markets(market_type),
        "bitget" => exchanges::bitget::fetch_markets(market_type),
        "bitmex" => exchanges::bitmex::fetch_markets(market_type),
        "bitstamp" => exchanges::bitstamp::fetch_markets(market_type),
        "bitz" => exchanges::bitz::fetch_markets(market_type),
        "bybit" => exchanges::bybit::fetch_markets(market_type),
        "coinbase_pro" => exchanges::coinbase_pro::fetch_markets(market_type),
        "deribit" => exchanges::deribit::fetch_markets(market_type),
        "ftx" => exchanges::ftx::fetch_markets(market_type),
        "huobi" => exchanges::huobi::fetch_markets(market_type),
        "kraken" => exchanges::kraken::fetch_markets(market_type),
        "mxc" => exchanges::mxc::fetch_markets(market_type),
        "okex" => exchanges::okex::fetch_markets(market_type),
        "zbg" => exchanges::zbg::fetch_markets(market_type),
        _ => panic!("Unsupported exchange {}", exchange),
    }
}
