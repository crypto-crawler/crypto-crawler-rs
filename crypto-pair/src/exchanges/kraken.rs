pub(crate) fn normalize_currency(mut currency: &str) -> String {
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
    let (base, quote) = {
        let v: Vec<&str> = symbol.split('/').collect();
        (v[0].to_string(), v[1].to_string())
    };

    Some(format!(
        "{}/{}",
        normalize_currency(&base),
        normalize_currency(&quote)
    ))
}
