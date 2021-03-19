pub(crate) fn normalize_pair(symbol: &str) -> Option<String> {
    let (base, quote) = {
        let v: Vec<&str> = symbol.split('_').collect();
        (v[0].to_string(), v[1].to_string())
    };

    Some(format!("{}/{}", base, quote))
}
