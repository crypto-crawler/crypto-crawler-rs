use std::collections::HashSet;

use super::utils::{http_get, normalize_pair_with_quotes};
use lazy_static::lazy_static;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

lazy_static! {
    static ref SPOT_QUOTES: HashSet<String> = fetch_spot_quotes();
}

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
fn fetch_spot_quotes() -> HashSet<String> {
    let txt = http_get("https://api.huobi.pro/v1/common/symbols").unwrap();
    let resp = serde_json::from_str::<Response<SpotMarket>>(&txt).unwrap();
    resp.data
        .into_iter()
        .map(|m| m.quote_currency)
        .collect::<HashSet<String>>()
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
