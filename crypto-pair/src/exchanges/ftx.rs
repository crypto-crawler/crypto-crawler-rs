pub(crate) fn normalize_pair(symbol: &str) -> Option<String> {
    if symbol.ends_with("-PERP") {
        // linear swap
        let base = symbol.strip_suffix("-PERP").unwrap();
        Some(format!("{}/USD", base))
    } else if symbol.contains("-MOVE-") {
        let v: Vec<&str> = symbol.split('-').collect();
        Some(format!("{}-MOVE/USD", v[0]))
    } else if symbol.contains("BVOL/") || symbol.contains('/') {
        // BVOL and Spot
        Some(symbol.to_string())
    } else {
        // linear future
        let pos = symbol.rfind('-').unwrap();
        let base = &symbol[..pos];
        Some(format!("{}/USD", base))
    }
}
