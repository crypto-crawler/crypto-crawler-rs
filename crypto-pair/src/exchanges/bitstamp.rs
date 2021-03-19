pub(crate) fn normalize_pair(symbol: &str) -> Option<String> {
    let (base, quote) = if symbol.ends_with("usdc") {
        (
            symbol.strip_suffix("usdc").unwrap().to_string(),
            symbol[(symbol.len() - 4)..].to_string(),
        )
    } else {
        (
            symbol[..(symbol.len() - 3)].to_string(),
            symbol[(symbol.len() - 3)..].to_string(),
        )
    };

    Some(format!("{}/{}", base, quote).to_uppercase())
}
