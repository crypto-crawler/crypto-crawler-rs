mod exchanges;

/// Normalize a trading currency.
///
/// # Arguments
///
/// * `currency` - The exchange-specific currency
/// * `exchange` - The normalized symbol
pub fn normalize_currency(symbol: &str, exchange: &str) -> String {
    match exchange {
        "bitfinex" => exchanges::bitfinex::normalize_currency(symbol),
        "bitmex" => exchanges::bitmex::normalize_currency(symbol),
        "kraken" => exchanges::kraken::normalize_currency(symbol),
        "kucoin" => exchanges::kucoin::normalize_currency(symbol),
        _ => symbol.to_uppercase(),
    }
}

/// Normalize a cryptocurrency trading pair.
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
        "bithumb" => Some(symbol.replace("-", "/")),
        "bitmex" => exchanges::bitmex::normalize_pair(symbol),
        "bitstamp" => exchanges::bitstamp::normalize_pair(symbol),
        "bitz" => Some(symbol.replace("_", "/").to_uppercase()),
        "bybit" => exchanges::bybit::normalize_pair(symbol),
        "coinbase_pro" => Some(symbol.replace("-", "/")),
        "deribit" => exchanges::deribit::normalize_pair(symbol),
        "ftx" => exchanges::ftx::normalize_pair(symbol),
        "gate" => exchanges::gate::normalize_pair(symbol),
        "huobi" => exchanges::huobi::normalize_pair(symbol),
        "kraken" => exchanges::kraken::normalize_pair(symbol),
        "kucoin" => exchanges::kucoin::normalize_pair(symbol),
        "mxc" => exchanges::mxc::normalize_pair(symbol),
        "okex" => {
            let v: Vec<&str> = symbol.split('-').collect();
            Some(format!("{}/{}", v[0], v[1]))
        }
        "Poloniex" => Some(symbol.replace("_", "/")),
        "Upbit" => Some(symbol.replace("-", "/")),
        "zbg" => exchanges::zbg::normalize_pair(symbol),
        _ => panic!("Unknown exchange {}", exchange),
    }
}
