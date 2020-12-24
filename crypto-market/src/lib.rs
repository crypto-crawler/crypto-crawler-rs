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
pub fn fetch_markets(exchange: &str, market_type: MarketType) -> Vec<Market> {
    match exchange {
        "Binance" => exchanges::binance::fetch_markets(market_type),
        "BitMEX" => exchanges::bitmex::fetch_markets(market_type),
        "Huobi" => exchanges::huobi::fetch_markets(market_type),
        "OKEx" => exchanges::okex::fetch_markets(market_type),
        _ => panic!("Unknown exchange: {}", exchange),
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
