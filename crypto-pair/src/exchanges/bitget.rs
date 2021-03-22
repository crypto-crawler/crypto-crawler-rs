pub(crate) fn normalize_pair(symbol: &str) -> Option<String> {
    if symbol.starts_with("cmt_") {
        // linear swap
        assert!(symbol.ends_with("usdt"));
        let base = &symbol[4..symbol.len() - 4];
        Some(format!("{}/usdt", base).to_uppercase())
    } else if symbol.contains('_') {
        // spot
        Some(symbol.replace("_", "/").to_uppercase())
    } else if symbol.ends_with("usd") {
        // inverse swap
        let base = symbol.strip_suffix("usd").unwrap();
        Some(format!("{}/usd", base).to_uppercase())
    } else {
        None
    }
}
