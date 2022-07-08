use std::collections::{BTreeSet, HashSet};

use super::utils::{http_get, normalize_pair_with_quotes};

use crypto_market_type::MarketType;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

static SPOT_QUOTES: Lazy<HashSet<String>> = Lazy::new(|| {
    // offline data, in case the network is down
    let mut set: HashSet<String> = vec![
        "AUD", "BIDR", "BKRW", "BNB", "BRL", "BTC", "BUSD", "BVND", "DAI", "DOGE", "DOT", "ETH",
        "EUR", "GBP", "GYEN", "IDRT", "NGN", "PAX", "RUB", "TRX", "TRY", "TUSD", "UAH", "USDC",
        "USDP", "USDS", "USDT", "UST", "VAI", "XRP", "ZAR",
    ]
    .into_iter()
    .map(|x| x.to_string())
    .collect();

    let from_online = fetch_spot_quotes();
    set.extend(from_online.into_iter());

    set
});

#[derive(Serialize, Deserialize)]
struct BinanceResponse {
    symbols: Vec<SpotMarket>,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct SpotMarket {
    quoteAsset: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

// see <https://binance-docs.github.io/apidocs/spot/en/#exchange-information>
fn fetch_spot_quotes() -> BTreeSet<String> {
    if let Ok(txt) = http_get("https://api.binance.com/api/v3/exchangeInfo") {
        let resp = serde_json::from_str::<BinanceResponse>(&txt).unwrap();
        resp.symbols
            .into_iter()
            .map(|m| m.quoteAsset)
            .collect::<BTreeSet<String>>()
    } else {
        BTreeSet::new()
    }
}

pub(crate) fn normalize_pair(symbol: &str) -> Option<String> {
    if let Some(base) = symbol.strip_suffix("USD_PERP") {
        // inverse swap
        Some(format!("{}/USD", base))
    } else if symbol.ends_with("-P") || symbol.ends_with("-C") {
        // option
        let pos = symbol.find('-').unwrap();
        let base = &symbol[..pos];
        Some(format!("{}/USDT", base))
    } else if symbol.len() > 7 && (symbol[(symbol.len() - 6)..]).parse::<i64>().is_ok() {
        // linear and inverse future
        let remove_date = &symbol[..symbol.len() - 7];
        if remove_date.ends_with("USDT") {
            let base = remove_date.strip_suffix("USDT").unwrap();
            Some(format!("{}/USDT", base))
        } else if remove_date.ends_with("USD") {
            let base = remove_date.strip_suffix("USD").unwrap();
            Some(format!("{}/USD", base))
        } else {
            panic!("Unsupported symbol {}", symbol);
        }
    } else {
        let quotes = &(*SPOT_QUOTES);
        normalize_pair_with_quotes(symbol, quotes)
    }
}

pub(crate) fn get_market_type(symbol: &str, is_spot: Option<bool>) -> MarketType {
    if symbol.ends_with("USD_PERP") {
        MarketType::InverseSwap
    } else if symbol.ends_with("-P") || symbol.ends_with("-C") {
        MarketType::EuropeanOption
    } else if symbol.len() > 7 && (symbol[(symbol.len() - 6)..]).parse::<i64>().is_ok() {
        // linear and inverse future
        let remove_date = &symbol[..symbol.len() - 7];
        if remove_date.ends_with("USDT") {
            MarketType::LinearFuture
        } else if remove_date.ends_with("USD") {
            MarketType::InverseFuture
        } else {
            MarketType::Unknown
        }
    } else if let Some(is_spot) = is_spot {
        if is_spot {
            MarketType::Spot
        } else {
            MarketType::LinearSwap
        }
    } else {
        MarketType::LinearSwap
    }
}

#[cfg(test)]
mod tests {
    use super::fetch_spot_quotes;

    #[test]
    fn spot_quotes() {
        let map = fetch_spot_quotes();
        for quote in map.iter() {
            if !super::SPOT_QUOTES.contains(quote) {
                println!("\"{}\",", quote);
            }
        }
    }

    #[test]
    fn normalize_pair() {
        assert_eq!("BDOT/DOT", super::normalize_pair("BDOTDOT").unwrap());
    }
}
