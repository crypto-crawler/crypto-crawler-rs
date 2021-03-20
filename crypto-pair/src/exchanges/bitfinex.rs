use super::utils::http_get;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref BITFINEX_MAPPING: HashMap<String, String> = fetch_currency_mapping();
}

// see <https://api-pub.bitfinex.com/v2/conf/pub:map:currency:sym>
fn fetch_currency_mapping() -> HashMap<String, String> {
    let txt = http_get("https://api-pub.bitfinex.com/v2/conf/pub:map:currency:sym").unwrap();
    let arr = serde_json::from_str::<Vec<Vec<Vec<String>>>>(&txt).unwrap();
    assert!(arr.len() == 1);

    let mut mapping = HashMap::<String, String>::new();
    for v in arr[0].iter() {
        assert!(v.len() == 2);
        mapping.insert(v[0].clone(), v[1].clone());
    }
    mapping
}

pub(crate) fn normalize_currency(mut currency: &str) -> String {
    assert!(
        !currency.trim().is_empty(),
        "The currency must NOT be empty"
    );

    if currency.ends_with("F0") {
        currency = &currency[..(currency.len() - 2)]; // Futures only
    }
    if BITFINEX_MAPPING.contains_key(currency) {
        currency = BITFINEX_MAPPING[currency].as_str();
    }

    currency.to_uppercase()
}

pub(crate) fn normalize_pair(mut symbol: &str) -> Option<String> {
    if symbol.chars().next().unwrap() == 't' {
        symbol = &symbol[1..]; // e.g., tBTCUSD, remove t
    };

    let (base, quote) = if symbol.contains(':') {
        let v: Vec<&str> = symbol.split(':').collect();
        (v[0].to_string(), v[1].to_string())
    } else {
        (
            symbol[..(symbol.len() - 3)].to_string(),
            symbol[(symbol.len() - 3)..].to_string(),
        )
    };

    Some(format!(
        "{}/{}",
        normalize_currency(&base),
        normalize_currency(&quote)
    ))
}
