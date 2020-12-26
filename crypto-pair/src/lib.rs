use lazy_static::lazy_static;
use std::collections::HashMap;
use std::collections::HashSet;

lazy_static! {
    static ref ALL_QUOTE_SYMBOLS: HashSet<&'static str> = vec![
        "BNB",
        "BTC",
        "BKRW",
        "BUSD",
        "CAD",
        "CHF",
        "CNHT",
        "CUSD",
        "DAI",
        "EOS",
        "EOSDT",
        "ETH",
        "EUR",
        "EUSD",
        "GBP",
        "HT",
        "HUSD",
        "IDRT",
        "JPY",
        "MX",
        "NGN",
        "ODIN",
        "OKB",
        "PAX",
        "PAXE",
        "RUB",
        "TRX",
        "TRY",
        "TUSD",
        "UAH",
        "USD",
        "USDC",
        "USDE",
        "USDK",
        "USDS",
        "USDT",
        "USN",
        "XCHF",
        "XLM",
        "XRP",
        "ZAR",
        "ZIG",
    ].into_iter().collect();

    // see https://api-pub.bitfinex.com/v2/conf/pub:map:currency:sym
    static ref BITFINEX_MAPPING: HashMap<&'static str, &'static str> = vec![
        ("AAA", "TESTAAA"),
        ("ABS", "ABYSS"),
        ("AIO", "AION"),
        ("ALG", "ALGO"),
        ("AMP", "AMPL"),
        ("ATO", "ATOM"),
        ("BAB", "BCH"),
        ("BBB", "TESTBBB"),
        ("CNHT", "CNHT"),
        ("CSX", "CS"),
        ("CTX", "CTXC"),
        ("DAD", "EDGE"),
        ("DAT", "DATA"),
        ("DOG", "MDOGE"),
        ("DRN", "DRGN"),
        ("DSH", "DASH"),
        ("DTX", "DT"),
        ("EDO", "PNT"),
        ("EUS", "EURS"),
        ("EUT", "EURT"),
        ("GSD", "GUSD"),
        ("IOS", "IOST"),
        ("IOT", "IOTA"),
        ("LBT", "LBTC"),
        ("MIT", "MITH"),
        ("MNA", "MANA"),
        ("NCA", "NCASH"),
        ("OMN", "OMNI"),
        ("PAS", "PASS"),
        ("POY", "POLY"),
        ("QSH", "QASH"),
        ("QTM", "QTUM"),
        ("RBT", "RBTC"),
        ("REP", "REP2"),
        ("SCR", "XD"),
        ("SNG", "SNGLS"),
        ("SPK", "SPANK"),
        ("STJ", "STORJ"),
        ("TSD", "TUSD"),
        ("UDC", "USDC"),
        ("USK", "USDK"),
        ("UST", "USDT"),
        ("UTN", "UTNP"),
        ("VSY", "VSYS"),
        ("WBT", "WBTC"),
        ("XAUT", "XAUT"),
        ("XCH", "XCHF"),
        ("YGG", "YEED"),
        ("YYW", "YOYOW"),
    ].into_iter().collect();

    static ref KRAKEN_QUOTE_SYMBOLS: HashSet<&'static str> = vec![
        "AUD",
        "BTC",
        "ETH",
        "EUR",
        "USD",
        "CAD",
        "CHF",
        "DAI",
        "GBP",
        "JPY",
        "USDC",
        "USDT",
    ].into_iter().collect();
}

const ALL_LENGTHS: [usize; 4] = [5, 4, 3, 2];

/// Normalize a symbol.
///
/// # Arguments
///
/// * `symbol` - The original symbol from an exchange
/// * `exchange` - The normalized symbol
fn normalize_symbol(symbol: &str, exchange: &str) -> String {
    assert!(!symbol.trim().is_empty(), "The symbol must NOT be empty");
    assert!(
        !exchange.trim().is_empty(),
        "The exchange name must NOT be empty"
    );
    let mut symbol: &str = &symbol.to_uppercase();

    match exchange {
        "Bitfinex" => {
            if symbol.ends_with("F0") {
                symbol = &symbol[..(symbol.len() - 2)]; // Futures only
            }
            if BITFINEX_MAPPING.contains_key(symbol) {
                symbol = BITFINEX_MAPPING[symbol];
            }
            if symbol == "HOT" {
                symbol = "HYDRO";
            }
            if symbol == "ORS" {
                symbol = "ORSGROUP";
            }
        }
        "BitMEX" => {
            if symbol == "XBT" {
                symbol = "BTC";
            }
        }
        "Huobi" => {
            if symbol == "HOT" {
                symbol = "HYDRO";
            }
        }
        "Kraken" => {
            // https://support.kraken.com/hc/en-us/articles/360001185506-How-to-interpret-asset-codes
            if symbol.len() > 3 && (symbol.starts_with('X') || symbol.starts_with('Z')) {
                symbol = &symbol[1..]
            }
            if symbol == "XBT" {
                symbol = "BTC";
            }
            if symbol == "XDG" {
                symbol = "DOGE";
            }
        }
        "Newdex" | "WhaleEx" => {
            if symbol == "KEY" {
                symbol = "MYKEY";
            }
        }
        _ => (),
    }

    symbol.to_string()
}

/// Normalize a pair.
///
/// # Arguments
///
/// * `raw_pair` - The original pair
fn default_normalize_pair(raw_pair: &str) -> Option<String> {
    let raw_pair: &str = &raw_pair.to_uppercase();

    let split_by = |raw_pair: &str, delim: char| -> Option<String> {
        if raw_pair.matches(delim).count() != 1 {
            None
        } else {
            Some(raw_pair.replace(delim, "_"))
        }
    };

    if raw_pair.contains('_') {
        if raw_pair.matches('_').count() != 1 {
            None
        } else {
            Some(raw_pair.to_string())
        }
    } else if raw_pair.contains('-') {
        split_by(raw_pair, '-')
    } else if raw_pair.contains(':') {
        split_by(raw_pair, ':')
    } else if raw_pair.contains('/') {
        split_by(raw_pair, '/')
    } else {
        let mut quote_symbol: Option<&str> = None;

        for &length in &ALL_LENGTHS {
            if length >= raw_pair.len() {
                continue;
            }
            let symbol = &raw_pair[(raw_pair.len() - length)..];
            if ALL_QUOTE_SYMBOLS.contains(symbol) {
                quote_symbol = Some(symbol);
                break;
            }
        }
        if quote_symbol == None {
            return None;
        }

        let base_symbol = &raw_pair[..(raw_pair.len() - quote_symbol.unwrap().len())];

        Some(format!("{}_{}", base_symbol, quote_symbol.unwrap()))
    }
}

/// Normalize a cryptocurrency trade pair.
///
/// # Arguments
///
/// * `raw_pair` - The original pair of an exchange
/// * `exchange` - The exchange name
pub fn normalize_pair(raw_pair: &str, exchange: &str) -> Option<String> {
    assert!(
        !raw_pair.trim().is_empty(),
        "The raw_pair must NOT be empty"
    );
    let mut raw_pair: &str = &raw_pair.to_uppercase();

    if exchange.is_empty() {
        return default_normalize_pair(raw_pair);
    }

    let (base_symbol, quote_symbol) = match exchange {
        "Bitfinex" => {
            if raw_pair.contains(':') {
                let v: Vec<&str> = raw_pair.split(':').collect();
                (v[0].to_string(), v[1].to_string())
            } else {
                (
                    raw_pair[..(raw_pair.len() - 3)].to_string(),
                    raw_pair[(raw_pair.len() - 3)..].to_string(),
                )
            }
        }
        "BitMEX" => {
            if raw_pair[..(raw_pair.len() - 2)].parse::<f64>().is_ok() {
                raw_pair = &raw_pair[..(raw_pair.len() - 3)]
            }

            if raw_pair.ends_with("USD") {
                (
                    raw_pair.strip_suffix("USD").unwrap().to_string(),
                    "USD".to_string(),
                )
            } else if raw_pair.ends_with("USDT") {
                (
                    raw_pair.strip_suffix("USDT").unwrap().to_string(),
                    "USDT".to_string(),
                )
            } else {
                let base_symbol = raw_pair;
                let quote_symbol = if base_symbol == "XBT" { "USD" } else { "XBT" };
                (base_symbol.to_string(), quote_symbol.to_string())
            }
        }
        "Bitstamp" => {
            if raw_pair.ends_with("EUR")
                || raw_pair.ends_with("BTC")
                || raw_pair.ends_with("GBP")
                || raw_pair.ends_with("USD")
            {
                (
                    raw_pair[..(raw_pair.len() - 3)].to_string(),
                    raw_pair[(raw_pair.len() - 3)..].to_string(),
                )
            } else if raw_pair.ends_with("USDC") {
                (
                    raw_pair.strip_suffix("USDC").unwrap().to_string(),
                    raw_pair[(raw_pair.len() - 4)..].to_string(),
                )
            } else {
                ("".to_string(), "".to_string())
            }
        }
        "Kraken" => {
            // https://github.com/ccxt/ccxt/blob/master/js/kraken.js#L322
            let safe_currency_code = |currency_id: &str| -> String {
                let mut result = currency_id;
                if currency_id.len() > 3 {
                    let first_char = currency_id.chars().next().unwrap();
                    if first_char == 'X' || first_char == 'Z' {
                        result = &currency_id[1..]
                    }
                }

                if result == "XBT" {
                    result = "BTC";
                }
                if result == "XDG" {
                    result = "DOGE";
                }

                result.to_string()
            };

            let mut base_symbol = safe_currency_code(&raw_pair[..(raw_pair.len() - 4)]);
            let mut quote_symbol = safe_currency_code(&raw_pair[(raw_pair.len() - 4)..]);
            let tmp: &str = &quote_symbol;
            // handle ICXETH
            if !KRAKEN_QUOTE_SYMBOLS.contains(tmp)
                || (base_symbol.len() == 2 && base_symbol != "SC")
            {
                base_symbol = safe_currency_code(&raw_pair[..(raw_pair.len() - 3)]);
                quote_symbol = safe_currency_code(&raw_pair[(raw_pair.len() - 3)..]);
            }

            let tmp: &str = &quote_symbol;
            if !KRAKEN_QUOTE_SYMBOLS.contains(tmp) {
                ("".to_string(), "".to_string())
                // throw new Error(`Failed to parse Kraken raw pair ${raw_pair}`);
            } else {
                (base_symbol, quote_symbol)
            }
        }
        "Newdex" => {
            if raw_pair.matches('-').count() == 2 {
                let v: Vec<&str> = raw_pair.split('-').collect();
                (v[1].to_string(), v[2].to_string())
            } else if raw_pair.matches('_').count() == 1 {
                let v: Vec<&str> = raw_pair.split('_').collect();
                (v[0].to_string(), v[1].to_string())
            } else {
                ("".to_string(), "".to_string())
                // throw new Error(`Failed to parse ${raw_pair} for Newdex`);
            }
        }
        "OKEx" => {
            let v: Vec<&str> = raw_pair.split('-').collect();
            (v[0].to_string(), v[1].to_string())
        }
        "Poloniex" => {
            let v: Vec<&str> = raw_pair.split('_').collect();
            (v[0].to_string(), v[1].to_string())
        }
        "Upbit" => {
            let v: Vec<&str> = raw_pair.split('-').collect();
            (v[0].to_string(), v[1].to_string())
        }
        _ => ("".to_string(), "".to_string()),
    };

    // default
    if base_symbol.is_empty() || quote_symbol.is_empty() {
        let normalized_pair = default_normalize_pair(raw_pair);
        if normalized_pair == None {
            None
            // throw new Error(`Failed to parse ${raw_pair} of exchange ${exchange}`);
        } else {
            let tmp = normalized_pair.unwrap();
            let v: Vec<&str> = tmp.split('_').collect();
            Some(format!(
                "{}_{}",
                normalize_symbol(v[0], exchange),
                normalize_symbol(v[1], exchange)
            ))
        }
    } else {
        Some(format!(
            "{}_{}",
            normalize_symbol(&base_symbol, exchange),
            normalize_symbol(&quote_symbol, exchange)
        ))
    }
}

#[cfg(test)]
mod tests;
