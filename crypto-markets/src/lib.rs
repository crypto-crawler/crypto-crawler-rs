mod exchanges;
mod market;
mod utils;

pub use crate::market::{Market, MarketType};

/// Fetch trading markets of a crypto exchange.
///
/// # Arguments
///
/// * `exchange` - The exchange name
/// * `market_type` - The market type
///
/// # Example
///
/// ```
/// use crypto_markets::{fetch_markets,MarketType};
/// let markets = fetch_markets("Binance", MarketType::Spot).unwrap();
/// assert!(!markets.is_empty());
/// println!("{}", serde_json::to_string_pretty(&markets).unwrap())
/// ```
pub fn fetch_markets(
    exchange: &str,
    market_type: MarketType,
) -> Result<Vec<Market>, reqwest::Error> {
    match exchange {
        "Binance" => exchanges::binance::fetch_markets(market_type),
        "BitMEX" => exchanges::bitmex::fetch_markets(market_type),
        "Huobi" => exchanges::huobi::fetch_markets(market_type),
        "OKEx" => exchanges::okex::fetch_markets(market_type),
        _ => panic!("Unsupported exchange {}", exchange),
    }
}

pub const SUPPORTED_EXCHANGES: [&str; 4] = ["Binance", "BitMEX", "Huobi", "OKEx"];

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
