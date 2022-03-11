#![allow(clippy::unnecessary_wraps)]

use crypto_market_type::MarketType;
mod exchanges;

/// Normalize a trading currency.
///
/// # Arguments
///
/// * `currency` - The exchange-specific currency
/// * `exchange` - The normalized symbol
pub fn normalize_currency(currency: &str, exchange: &str) -> String {
    match exchange {
        "bitfinex" => exchanges::bitfinex::normalize_currency(currency),
        "bitmex" => exchanges::bitmex::normalize_currency(currency),
        "kraken" => exchanges::kraken::normalize_currency(currency),
        "kucoin" => exchanges::kucoin::normalize_currency(currency),
        _ => currency.to_uppercase(),
    }
}

/// Normalize a cryptocurrency trading symbol.
///
/// # Arguments
///
/// * `symbol` - The original pair of an exchange
/// * `exchange` - The exchange name
///
/// # Examples
///
/// ```
/// use crypto_pair::normalize_pair;
///
/// assert_eq!(Some("BTC/USD".to_string()), normalize_pair("XBTUSD", "bitmex"));
/// assert_eq!(Some("BTC/USD".to_string()), normalize_pair("XBTH21", "bitmex"));
/// assert_eq!(Some("BTC/USDT".to_string()), normalize_pair("BTCUSDT", "binance"));
/// assert_eq!(Some("BTC/USDT".to_string()), normalize_pair("btcusdt", "huobi"));
/// assert_eq!(Some("BTC/USDT".to_string()), normalize_pair("BTCUST", "bitfinex"));
/// ```
pub fn normalize_pair(symbol: &str, exchange: &str) -> Option<String> {
    match exchange {
        "binance" => exchanges::binance::normalize_pair(symbol),
        "bitfinex" => exchanges::bitfinex::normalize_pair(symbol),
        "bitget" => exchanges::bitget::normalize_pair(symbol),
        "bithumb" => Some(symbol.replace('-', "/")),
        "bitmex" => exchanges::bitmex::normalize_pair(symbol),
        "bitstamp" => exchanges::bitstamp::normalize_pair(symbol),
        "bitz" => Some(symbol.replace('_', "/").to_uppercase()),
        "bybit" => exchanges::bybit::normalize_pair(symbol),
        "coinbase_pro" => Some(symbol.replace('-', "/")),
        "deribit" => exchanges::deribit::normalize_pair(symbol),
        "dydx" => exchanges::dydx::normalize_pair(symbol),
        "ftx" => exchanges::ftx::normalize_pair(symbol),
        "gate" => {
            let (base, quote) = {
                let v: Vec<&str> = symbol.split('_').collect();
                (v[0].to_string(), v[1].to_string())
            };

            Some(format!("{}/{}", base, quote))
        }
        "huobi" => exchanges::huobi::normalize_pair(symbol),
        "kraken" => exchanges::kraken::normalize_pair(symbol),
        "kucoin" => exchanges::kucoin::normalize_pair(symbol),
        "mxc" | "mexc" => Some(symbol.replace('_', "/")),
        "okex" | "okx" => {
            let v: Vec<&str> = symbol.split('-').collect();
            Some(format!("{}/{}", v[0], v[1]))
        }
        "Poloniex" => Some(symbol.replace('_', "/")),
        "Upbit" => Some(symbol.replace('-', "/")),
        "zbg" => exchanges::zbg::normalize_pair(symbol),
        _ => panic!("Unknown exchange {}", exchange),
    }
}

/// Infer out market type from the symbol.
///
/// The `is_spot` parameter is not needed in most cases, but at some exchanges
///  (including binance, gate and mexc) a symbol might exist in both spot and
/// contract markets, for example:
/// * At binance `BTCUSDT` exists in both spot and linear_swap markets
/// * At gate `BTC_USDT` exists in both spot and linear_swap markets,
/// `BTC_USD` exists in both spot and inverse_swap markets
pub fn get_market_type(symbol: &str, exchange: &str, is_spot: Option<bool>) -> MarketType {
    match exchange {
        "binance" => exchanges::binance::get_market_type(symbol, is_spot),
        "bitfinex" => exchanges::bitfinex::get_market_type(symbol),
        "bitget" => exchanges::bitget::get_market_type(symbol),
        "bithumb" => MarketType::Spot,
        "bitmex" => exchanges::bitmex::get_market_type(symbol),
        "bitstamp" => MarketType::Spot,
        "bybit" => exchanges::bybit::get_market_type(symbol),
        "coinbase_pro" => MarketType::Spot,
        "deribit" => exchanges::deribit::get_market_type(symbol),
        "dydx" => MarketType::LinearSwap,
        "ftx" => exchanges::ftx::get_market_type(symbol),
        "gate" => exchanges::gate::get_market_type(symbol, is_spot),
        "huobi" => exchanges::huobi::get_market_type(symbol),
        "kraken" => exchanges::kraken::get_market_type(symbol),
        "kucoin" => exchanges::kucoin::get_market_type(symbol),
        "mxc" | "mexc" => exchanges::mexc::get_market_type(symbol, is_spot),
        "okex" | "okx" => exchanges::okx::get_market_type(symbol),
        "zbg" => exchanges::zbg::get_market_type(symbol),
        _ => MarketType::Unknown,
    }
}
