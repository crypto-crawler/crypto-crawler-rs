use std::collections::{BTreeSet, HashSet};

use super::utils::{http_get, normalize_pair_with_quotes};
use lazy_static::lazy_static;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

lazy_static! {
    static ref SPOT_QUOTES: HashSet<String> = {
        // offline data, in case the network is down
        let mut set: HashSet<String> = vec![
            "AUD",
            "BIDR",
            "BKRW",
            "BNB",
            "BRL",
            "BTC",
            "BUSD",
            "BVND",
            "DAI",
            "ETH",
            "EUR",
            "GBP",
            "GYEN",
            "IDRT",
            "NGN",
            "PAX",
            "RUB",
            "TRX",
            "TRY",
            "TUSD",
            "UAH",
            "USDC",
            "USDP",
            "USDS",
            "USDT",
            "VAI",
            "XRP",
            "ZAR",
        ]
        .into_iter()
        .map(|x| x.to_string())
        .collect();

        let from_online = fetch_spot_quotes();
        set.extend(from_online.into_iter());

        set
    };
}

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
    } else if symbol.len() > 7 && (&symbol[(symbol.len() - 6)..]).parse::<i64>().is_ok() {
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
