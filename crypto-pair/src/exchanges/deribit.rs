pub(crate) fn normalize_pair(symbol: &str) -> Option<String> {
    if let Some(pos) = symbol.find('-') {
        let base = &symbol[..pos];
        Some(format!("{}/USD", base))
    } else {
        None
    }
}
