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
        "brl", "btc", "eth", "eur", "gbp", "ht", "husd", "rub", "trx", "try", "uah", "usdc",
        "usdd", "usdt", "ust", "ustc",
    ]
    .into_iter()
    .map(|x| x.to_string())
    .collect();

    let from_online = fetch_spot_quotes();
    set.extend(from_online.into_iter());

    set
});

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct SpotMarket {
    base_currency: String,
    quote_currency: String,
    symbol: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct Response<T: Sized> {
    status: String,
    data: Vec<T>,
}

// see <https://huobiapi.github.io/docs/spot/v1/en/#get-all-supported-trading-symbol>
fn fetch_spot_quotes() -> BTreeSet<String> {
    if let Ok(txt) = http_get("https://api.huobi.pro/v1/common/symbols") {
        let resp = serde_json::from_str::<Response<SpotMarket>>(&txt).unwrap();
        resp.data
            .into_iter()
            .map(|m| m.quote_currency)
            .collect::<BTreeSet<String>>()
    } else {
        BTreeSet::new()
    }
}

pub(crate) fn normalize_pair(symbol: &str) -> Option<String> {
    if symbol.ends_with("-USD") || symbol.ends_with("-USDT") {
        // inverse and linear swap
        Some(symbol.replace('-', "/"))
    } else if symbol.contains("-C-") || symbol.contains("-P-") {
        // option
        let (base, quote) = {
            let v: Vec<&str> = symbol.split('-').collect();
            (v[0].to_uppercase(), v[1].to_uppercase())
        };
        Some(format!("{}/{}", base, quote))
    } else if symbol.ends_with("_CW")
        || symbol.ends_with("_NW")
        || symbol.ends_with("_CQ")
        || symbol.ends_with("_NQ")
    {
        // inverse future
        let base = &symbol[..symbol.len() - 3];
        Some(format!("{}/USD", base))
    } else {
        // spot
        let quotes = &(*SPOT_QUOTES);
        normalize_pair_with_quotes(symbol, quotes)
    }
}

pub(crate) fn get_market_type(symbol: &str) -> MarketType {
    if symbol.ends_with("-USD") {
        MarketType::InverseSwap
    } else if symbol.ends_with("-USDT") {
        MarketType::LinearSwap
    } else if symbol.contains("-C-") || symbol.contains("-P-") {
        MarketType::EuropeanOption
    } else if symbol.ends_with("_CW")
        || symbol.ends_with("_NW")
        || symbol.ends_with("_CQ")
        || symbol.ends_with("_NQ")
    {
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

    #[test]
    fn test_normalize_pair() {
        assert_eq!("MIR/UST", super::normalize_pair("mirust").unwrap());

        assert_eq!("BTT/TRX", super::normalize_pair("btttrx").unwrap());
    }
}
