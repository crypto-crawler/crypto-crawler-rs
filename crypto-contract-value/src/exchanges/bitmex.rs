use std::collections::{BTreeMap, HashMap};

use super::utils::http_get;
use crypto_market_type::MarketType;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::Value;

lazy_static! {
    static ref CONTRACT_VALUES: HashMap<String, i64> = {
        // offline data, in case the network is down
        let mut m: HashMap<String, i64> = vec![
            ("ADA/USD", 10000),
            ("ADA/USDT", 10000),
            ("ALTMEX/USD", 100),
            ("AVAX/USD", 1000),
            ("AXS/USD", 100),
            ("BCH/USD", 100),
            ("BCH/USDT", 10),
            ("BNB/USD", 100),
            ("BNB/USDT", 100),
            ("BTC/USDT", 1),
            ("DEFIMEX/USD", 100),
            ("DOGE/USD", 100000),
            ("DOGE/USDT", 10000),
            ("DOT/USD", 1000),
            ("EOS/USD", 10000),
            ("ETH/USD", 100),
            ("ETH/USDT", 10),
            ("FTM/USDT", 10000),
            ("LINK/USD", 1000),
            ("LTC/USD", 200),
            ("LTC/USDT", 100),
            ("LUNA/USD", 1000),
            ("SHIB/USDT", 1000000),
            ("SOL/USD", 100),
            ("SOL/USDT", 100),
            ("XRP/USD", 20000),
            ("XRP/USDT", 10000),
        ]
        .into_iter()
        .map(|x| (x.0.to_string(), x.1))
        .collect();

        let from_online = fetch_contract_values();
        for (pair, contract_value) in from_online {
            m.insert(pair, contract_value);
        }

        m
    };
}

#[derive(Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
struct Instrument {
    symbol: String,
    state: String,
    typ: String,
    quoteCurrency: String,
    multiplier: i64,
    isQuanto: bool,
    isInverse: bool,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

fn fetch_contract_values() -> BTreeMap<String, i64> {
    let mut mapping: BTreeMap<String, i64> = BTreeMap::new();

    if let Ok(text) = http_get("https://www.bitmex.com/api/v1/instrument/active") {
        let instruments: Vec<Instrument> = serde_json::from_str::<Vec<Instrument>>(&text)
            .unwrap()
            .into_iter()
            .filter(|x| x.state == "Open")
            .collect();

        for instrument in instruments.iter().filter(|x| !x.isInverse) {
            let pair = crypto_pair::normalize_pair(&instrument.symbol, "bitmex").unwrap();
            if !pair.ends_with("BTC") {
                mapping.insert(pair, instrument.multiplier);
            }
        }
    }

    mapping
}

pub(crate) fn get_contract_value(market_type: MarketType, pair: &str) -> Option<f64> {
    match market_type {
        MarketType::InverseSwap | MarketType::InverseFuture => Some(1.0),
        MarketType::LinearFuture | MarketType::LinearSwap => {
            if pair.ends_with("BTC") {
                Some(1e-8)
            } else {
                Some(CONTRACT_VALUES[pair] as f64 * 1e-8)
            }
        }
        MarketType::QuantoSwap | MarketType::QuantoFuture => {
            Some(CONTRACT_VALUES[pair] as f64 * 1e-8)
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::fetch_contract_values;

    #[test]
    fn quanto() {
        let mapping = fetch_contract_values();
        for (pair, contract_value) in &mapping {
            println!("(\"{}\", {}),", pair, contract_value);
        }
    }
}
