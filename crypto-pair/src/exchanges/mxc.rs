pub(crate) fn normalize_pair(symbol: &str) -> Option<String> {
    Some(symbol.replace("_", "/"))
}
