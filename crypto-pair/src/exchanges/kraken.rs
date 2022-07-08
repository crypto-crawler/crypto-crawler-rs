use std::collections::{BTreeSet, HashMap, HashSet};

use super::utils::http_get;
use crypto_market_type::MarketType;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::Value;

static SPOT_QUOTES: Lazy<HashSet<String>> = Lazy::new(|| {
    // offline data, in case the network is down
    let mut set: HashSet<String> = vec![
        "AUD", "CAD", "CHF", "DAI", "DOT", "ETH", "EUR", "GBP", "JPY", "USD", "USDC", "USDT",
        "XBT", "XET", "XXB", "ZAU", "ZCA", "ZEU", "ZGB", "ZJP", "ZUS",
    ]
    .into_iter()
    .map(|x| x.to_string())
    .collect();

    let from_online = fetch_spot_quotes();
    set.extend(from_online.into_iter());

    set
});

// see <https://docs.kraken.com/rest/#operation/getTradableAssetPairs>
fn fetch_spot_quotes() -> BTreeSet<String> {
    #[derive(Serialize, Deserialize)]
    struct Response {
        error: Vec<Value>,
        result: HashMap<String, SpotMarket>,
    }

    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct SpotMarket {
        altname: String,
        wsname: Option<String>,
        aclass_base: String,
        base: String,
        aclass_quote: String,
        quote: String,
        #[serde(flatten)]
        extra: HashMap<String, Value>,
    }

    if let Ok(txt) = http_get("https://api.kraken.com/0/public/AssetPairs") {
        let resp = serde_json::from_str::<Response>(&txt).unwrap();
        resp.result
            .into_values()
            .map(|m| m.quote)
            .map(|s| {
                if s.len() > 3 && (s.starts_with('X') || s.starts_with('Z')) {
                    s[1..].to_string()
                } else {
                    s
                }
            })
            .collect::<BTreeSet<String>>()
    } else {
        BTreeSet::new()
    }
}

pub(crate) fn normalize_currency(currency: &str) -> String {
    let uppercase = currency.to_uppercase();
    let mut currency = uppercase.as_str();
    // https://support.kraken.com/hc/en-us/articles/360001185506-How-to-interpret-asset-codes
    if currency.len() > 3 && (currency.starts_with('X') || currency.starts_with('Z')) {
        currency = &currency[1..]
    }

    if currency == "XBT" {
        "BTC"
    } else if currency == "XDG" {
        "DOGE"
    } else {
        currency
    }
    .to_string()
}

pub(crate) fn normalize_pair(symbol: &str) -> Option<String> {
    if symbol.contains('/') {
        // Spot
        let (base, quote) = {
            let v: Vec<&str> = symbol.split('/').collect();
            (v[0].to_string(), v[1].to_string())
        };

        Some(format!(
            "{}/{}",
            normalize_currency(&base),
            normalize_currency(&quote)
        ))
    } else if symbol.starts_with("pi_")
        || symbol.starts_with("fi_")
        || symbol.starts_with("PI_")
        || symbol.starts_with("FI_")
    {
        let pos = if let Some(pos) = symbol.find("usd") {
            pos
        } else if let Some(pos) = symbol.find("USD") {
            pos
        } else {
            panic!("Can not find usd or USD in symbol: {}", symbol);
        };
        let base = symbol[3..pos].to_uppercase();
        Some(format!("{}/USD", normalize_currency(&base),))
    } else if SPOT_QUOTES.contains(&symbol[(symbol.len() - 4)..]) {
        let base = &symbol[..(symbol.len() - 4)];
        let quote = &symbol[(symbol.len() - 4)..];
        Some(format!(
            "{}/{}",
            normalize_currency(base),
            normalize_currency(quote)
        ))
    } else if SPOT_QUOTES.contains(&symbol[(symbol.len() - 3)..]) {
        let base = &symbol[..(symbol.len() - 3)];
        let quote = &symbol[(symbol.len() - 3)..];
        Some(format!(
            "{}/{}",
            normalize_currency(base),
            normalize_currency(quote)
        ))
    } else {
        None
    }
}

pub(crate) fn get_market_type(symbol: &str) -> MarketType {
    if symbol.starts_with("pi_") || symbol.starts_with("PI_") {
        MarketType::InverseSwap
    } else if symbol.starts_with("fi_") || symbol.starts_with("FI_") {
        MarketType::InverseFuture
    } else {
        MarketType::Spot
    }
}

#[cfg(test)]
mod tests {
    use super::fetch_spot_quotes;

    #[test]
    fn spot_quotes() {
        let map = fetch_spot_quotes();
        for quote in map {
            println!("\"{}\",", quote);
        }
    }
}
