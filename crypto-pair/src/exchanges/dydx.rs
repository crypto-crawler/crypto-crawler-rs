pub(crate) fn normalize_pair(symbol: &str) -> Option<String> {
    if symbol.contains('-') {
        let result = str::replace(symbol, "-", "/");
        Some(result)
    } else {
        None
    }
}
