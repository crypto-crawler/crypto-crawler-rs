use crypto_market_type::MarketType;

use once_cell::sync::Lazy;
use std::collections::HashSet;

static FIAT_CURRENCIES: Lazy<HashSet<String>> = Lazy::new(|| {
    // offline data, in case the network is down
    let set: HashSet<String> = vec![
        "BRL", "CHF", "CNH", "EUR", "INR", "MXN", "NZD", "SEK", "TRY", "TRY", "USD", "ZAR",
    ]
    .into_iter()
    .map(|x| x.to_string())
    .collect();

    set
});

// See <https://blog.bitmex.com/worlds-first-crypto-margined-perpetual-fx-swap-contracts/>
pub fn is_fx(symbol: &str) -> bool {
    let suffix = &symbol[symbol.len() - 3..];
    if FIAT_CURRENCIES.contains(suffix) {
        let base = &symbol[..symbol.len() - 3];
        base == "USDT" || FIAT_CURRENCIES.contains(base)
    } else {
        let prefix = &symbol[0..3];
        if FIAT_CURRENCIES.contains(prefix) {
            let quote = &symbol[3..];
            quote == "USDT" || FIAT_CURRENCIES.contains(quote)
        } else {
            false
        }
    }
}

pub(crate) fn normalize_currency(currency: &str) -> String {
    if currency == "XBT" { "BTC" } else { currency }.to_string()
}

pub(crate) fn normalize_pair(mut symbol: &str) -> Option<String> {
    if is_fx(symbol) {
        let suffix = &symbol[symbol.len() - 3..];
        let pair = if FIAT_CURRENCIES.contains(suffix) {
            format!("{}/{}", &symbol[0..(symbol.len() - 3)], suffix)
        } else {
            format!("{}/{}", &symbol[0..3], &symbol[3..])
        };
        return Some(pair);
    }

    if symbol.ends_with("_ETH") {
        symbol = symbol.strip_suffix("_ETH").unwrap();
    }
    if symbol[(symbol.len() - 2)..].parse::<f64>().is_ok() {
        symbol = &symbol[..(symbol.len() - 3)]
    }

    let (base, quote) = if symbol.ends_with("USD") {
        (
            symbol.strip_suffix("USD").unwrap().to_string(),
            "USD".to_string(),
        )
    } else if symbol.ends_with("_USDT") {
        // spot
        (
            symbol.strip_suffix("_USDT").unwrap().to_string(),
            "USDT".to_string(),
        )
    } else if symbol.ends_with("USDT") {
        (
            symbol.strip_suffix("USDT").unwrap().to_string(),
            "USDT".to_string(),
        )
    } else if symbol.ends_with("EUR") {
        (
            symbol.strip_suffix("EUR").unwrap().to_string(),
            "EUR".to_string(),
        )
    } else {
        let base_symbol = symbol;
        let quote_symbol = if base_symbol == "XBT" {
            "USD"
        } else if base_symbol == "ETHPOW" {
            "USDT"
        } else {
            "XBT"
        };
        (base_symbol.to_string(), quote_symbol.to_string())
    };

    Some(format!(
        "{}/{}",
        normalize_currency(&base),
        normalize_currency(&quote)
    ))
}

pub(crate) fn get_market_type(symbol: &str) -> MarketType {
    if symbol.ends_with("_USDT") {
        return MarketType::Spot;
    }
    if is_fx(symbol) {
        return MarketType::QuantoSwap;
    }

    let is_future = if let Some(pos) = symbol.rfind('_') {
        // e.g., ETHUSDM22_ETH
        let date = &symbol[(pos - 2)..pos];
        date.parse::<i64>().is_ok()
    } else {
        let date = &symbol[(symbol.len() - 2)..];
        date.parse::<i64>().is_ok()
    };
    let real_symbol = if is_future {
        if let Some(pos) = symbol.rfind('_') {
            &symbol[..(pos - 3)]
        } else {
            &symbol[..(symbol.len() - 3)]
        }
    } else {
        symbol
    };
    // 0, linear; 1, inverse; 2, quanto
    let linear_inverse_quanto = if real_symbol.ends_with("USDT") {
        0
    } else if real_symbol.starts_with("XBT") || symbol.ends_with("_ETH") {
        1
    } else if real_symbol.ends_with("USD") || real_symbol.ends_with("EUR") {
        2
    } else {
        // Settled in XBT, quoted in XBT
        0
    };

    match linear_inverse_quanto {
        0 => {
            if is_future {
                MarketType::LinearFuture
            } else {
                MarketType::LinearSwap
            }
        }
        1 => {
            if is_future {
                MarketType::InverseFuture
            } else {
                MarketType::InverseSwap
            }
        }
        2 => {
            if is_future {
                MarketType::QuantoFuture
            } else {
                MarketType::QuantoSwap
            }
        }
        _ => panic!("Impossible {}", symbol),
    }
}

#[cfg(test)]
mod tests {
    use crypto_market_type::MarketType;

    #[test]
    fn test_get_market_type() {
        assert_eq!(MarketType::Spot, super::get_market_type("XBT_USDT"));
        assert_eq!(MarketType::InverseSwap, super::get_market_type("XBTUSD"));
        assert_eq!(MarketType::LinearSwap, super::get_market_type("XBTUSDT"));
        assert_eq!(MarketType::InverseSwap, super::get_market_type("XBTEUR"));

        assert_eq!(MarketType::QuantoSwap, super::get_market_type("ETHUSD"));
        assert_eq!(MarketType::LinearSwap, super::get_market_type("ETHUSDT"));
        assert_eq!(
            MarketType::InverseFuture,
            super::get_market_type("ETHUSDM22_ETH")
        );
        assert_eq!(MarketType::QuantoSwap, super::get_market_type("EURUSD"));
        assert_eq!(MarketType::QuantoSwap, super::get_market_type("EURUSDT"));

        assert_eq!(
            MarketType::LinearFuture,
            super::get_market_type("ETHPOWZ22")
        );
        assert_eq!(MarketType::LinearFuture, super::get_market_type("ETHZ22"));
    }

    #[test]
    fn test_normalize_pair() {
        assert_eq!("BTC/USD", super::normalize_pair("XBTUSD").unwrap());
        assert_eq!("BTC/USDT", super::normalize_pair("XBT_USDT").unwrap());
        assert_eq!("BTC/USDT", super::normalize_pair("XBTUSDT").unwrap());
        assert_eq!("BTC/EUR", super::normalize_pair("XBTEUR").unwrap());

        assert_eq!("ETH/USD", super::normalize_pair("ETHUSD").unwrap());
        assert_eq!("ETH/USDT", super::normalize_pair("ETHUSDT").unwrap());
        assert_eq!("ETH/USD", super::normalize_pair("ETHUSDM22_ETH").unwrap());

        assert_eq!("EUR/USD", super::normalize_pair("EURUSD").unwrap());
        assert_eq!("EUR/USDT", super::normalize_pair("EURUSDT").unwrap());

        assert_eq!("ETHPOW/USDT", super::normalize_pair("ETHPOWZ22").unwrap());
        assert_eq!("ETH/BTC", super::normalize_pair("ETHZ22").unwrap());
    }
}
