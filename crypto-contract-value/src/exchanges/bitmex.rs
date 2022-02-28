use std::collections::{BTreeMap, HashMap};

use super::utils::http_get;
use crypto_market_type::MarketType;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::Value;

lazy_static! {
    // key = market_type + pair
    static ref CONTRACT_VALUES: HashMap<String, i64> = {
        // offline data, in case the network is down
        let mut m: HashMap<String, i64> = vec![
            ("linear_future.ADA/BTC", 1000000),
            ("linear_future.BTC/USDT", 1),
            ("linear_future.ETH/BTC", 1000),
            ("linear_future.ETH/USDT", 10),
            ("linear_future.XRP/BTC", 1000000),
            ("linear_swap.ADA/USDT", 10000),
            ("linear_swap.ALTMEXT/USDT", 100),
            ("linear_swap.AVAX/USDT", 100),
            ("linear_swap.BCH/USDT", 10),
            ("linear_swap.BNB/USDT", 100),
            ("linear_swap.BTC/USDT", 1),
            ("linear_swap.DEFIMEXT/USDT", 100),
            ("linear_swap.DOGE/USDT", 10000),
            ("linear_swap.DOT/USDT", 1000),
            ("linear_swap.EOS/USDT", 1000),
            ("linear_swap.ETH/USDT", 10),
            ("linear_swap.FTM/USDT", 10000),
            ("linear_swap.LINK/USDT", 1000),
            ("linear_swap.LTC/USDT", 100),
            ("linear_swap.LUNA/USDT", 100),
            ("linear_swap.MANA/USDT", 1000),
            ("linear_swap.MATIC/USDT", 10000),
            ("linear_swap.METAMEXT/USDT", 100),
            ("linear_swap.SAND/USDT", 1000),
            ("linear_swap.SHIB/USDT", 1000000),
            ("linear_swap.SOL/USDT", 100),
            ("linear_swap.XRP/USDT", 10000),
            ("quanto_future.ETH/USD", 100),
            ("quanto_swap.ADA/USD", 10000),
            ("quanto_swap.AVAX/USD", 1000),
            ("quanto_swap.AXS/USD", 100),
            ("quanto_swap.BCH/USD", 100),
            ("quanto_swap.BNB/USD", 100),
            ("quanto_swap.DOGE/USD", 100000),
            ("quanto_swap.DOT/USD", 1000),
            ("quanto_swap.EOS/USD", 10000),
            ("quanto_swap.ETH/USD", 100),
            ("quanto_swap.LINK/USD", 1000),
            ("quanto_swap.LTC/USD", 200),
            ("quanto_swap.LUNA/USD", 1000),
            ("quanto_swap.SOL/USD", 100),
            ("quanto_swap.XRP/USD", 20000),
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
            let market_type = crypto_pair::get_market_type(&instrument.symbol, "bitmex", None);
            let pair = crypto_pair::normalize_pair(&instrument.symbol, "bitmex").unwrap();
            mapping.insert(
                market_type.to_string() + "." + pair.as_str(),
                instrument.multiplier,
            );
        }
    }

    mapping
}

pub(crate) fn get_contract_value(market_type: MarketType, pair: &str) -> Option<f64> {
    let key = market_type.to_string() + "." + pair;
    match market_type {
        MarketType::InverseSwap | MarketType::InverseFuture => Some(1.0),
        MarketType::LinearFuture | MarketType::LinearSwap => {
            Some(CONTRACT_VALUES[&key] as f64 * 1e-8)
        }
        MarketType::QuantoSwap | MarketType::QuantoFuture => {
            Some(CONTRACT_VALUES[&key] as f64 * 1e-8)
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
