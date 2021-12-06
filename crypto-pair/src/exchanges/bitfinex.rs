use super::utils::http_get;
use crypto_market_type::MarketType;
use lazy_static::lazy_static;
use std::collections::{BTreeMap, HashMap};

lazy_static! {
    static ref BITFINEX_MAPPING: HashMap<String, String> = {
        // offline data, in case the network is down
        let mut set: HashMap<String, String> = vec![
            ("AAA", "TESTAAA"),
            ("AIX", "AI"),
            ("ALG", "ALGO"),
            ("AMP", "AMPL"),
            ("AMPF0", "AMPLF0"),
            ("ATO", "ATOM"),
            ("B21X", "B21"),
            ("BBB", "TESTBBB"),
            ("BCHABC", "XEC"),
            ("BTCF0", "BTC"),
            ("CNHT", "CNHt"),
            ("DAT", "DATA"),
            ("DOG", "MDOGE"),
            ("DSH", "DASH"),
            ("EDO", "PNT"),
            ("ETH2P", "ETH2Pending"),
            ("ETH2R", "ETH2Rewards"),
            ("ETH2X", "ETH2"),
            ("EUS", "EURS"),
            ("EUT", "EURt"),
            ("GNT", "GLM"),
            ("IDX", "ID"),
            ("IOT", "IOTA"),
            ("LBT", "LBTC"),
            ("LES", "LEO-EOS"),
            ("LET", "LEO-ERC20"),
            ("LNX", "LN-BTC"),
            ("MNA", "MANA"),
            ("OMN", "OMNI"),
            ("PAS", "PASS"),
            ("PBTCEOS", "pBTC-EOS"),
            ("PBTCETH", "PBTC-ETH"),
            ("PETHEOS", "pETH-EOS"),
            ("PLTCEOS", "PLTC-EOS"),
            ("PLTCETH", "PLTC-ETH"),
            ("QSH", "QASH"),
            ("QTM", "QTUM"),
            ("RBT", "RBTC"),
            ("REP", "REP2"),
            ("SNG", "SNGLS"),
            ("STJ", "STORJ"),
            ("TSD", "TUSD"),
            ("UDC", "USDC"),
            ("UST", "USDt"),
            ("USTF0", "USDt"),
            ("VSY", "VSYS"),
            ("WBT", "WBTC"),
            ("XAUT", "XAUt"),
            ("XCH", "XCHF"),
            ("YGG", "MCS"),
        ]
        .into_iter()
        .map(|x| (x.0.to_string(), x.1.to_string()))
        .collect();

        let from_online = fetch_currency_mapping();
        set.extend(from_online.into_iter());

        set
    };
}

// see <https://api-pub.bitfinex.com/v2/conf/pub:map:currency:sym>
fn fetch_currency_mapping() -> BTreeMap<String, String> {
    let mut mapping = BTreeMap::<String, String>::new();

    if let Ok(txt) = http_get("https://api-pub.bitfinex.com/v2/conf/pub:map:currency:sym") {
        let arr = serde_json::from_str::<Vec<Vec<Vec<String>>>>(&txt).unwrap();
        assert!(arr.len() == 1);

        for v in arr[0].iter() {
            assert!(v.len() == 2);
            mapping.insert(v[0].clone(), v[1].clone());
        }
    }

    mapping
}

pub(crate) fn normalize_currency(mut currency: &str) -> String {
    assert!(
        !currency.trim().is_empty(),
        "The currency must NOT be empty"
    );

    if currency.ends_with("F0") {
        currency = &currency[..(currency.len() - 2)]; // Futures only
    }
    if BITFINEX_MAPPING.contains_key(currency) {
        currency = BITFINEX_MAPPING[currency].as_str();
    }

    currency.to_uppercase()
}

pub(crate) fn normalize_pair(mut symbol: &str) -> Option<String> {
    if symbol.starts_with('t') {
        symbol = &symbol[1..]; // e.g., tBTCUSD, remove t
    };

    let (base, quote) = if symbol.contains(':') {
        let v: Vec<&str> = symbol.split(':').collect();
        (v[0].to_string(), v[1].to_string())
    } else {
        (
            symbol[..(symbol.len() - 3)].to_string(),
            symbol[(symbol.len() - 3)..].to_string(),
        )
    };

    Some(format!(
        "{}/{}",
        normalize_currency(&base),
        normalize_currency(&quote)
    ))
}

pub(crate) fn get_market_type(symbol: &str) -> MarketType {
    if symbol.ends_with("F0") || symbol.ends_with("f0") {
        MarketType::LinearSwap
    } else {
        MarketType::Spot
    }
}

#[cfg(test)]
mod tests {
    use super::fetch_currency_mapping;

    #[test]
    fn test_currency_mapping() {
        let map = fetch_currency_mapping();
        for (name, new_name) in map {
            println!("(\"{}\", \"{}\"),", name, new_name);
        }
    }
}
