pub(crate) fn normalize_pair(symbol: &str) -> Option<String> {
    if symbol.ends_with("_USD-R") {
        let base = symbol.strip_suffix("_USD-R").unwrap();
        Some(format!("{}/USD", base))
    } else {
        Some(symbol.replace("_", "/").to_uppercase())
    }
}
